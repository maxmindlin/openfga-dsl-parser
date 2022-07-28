use crate::ast::*;
use serde_json::{json, Map, Value};

pub struct JsonTransformer<'d> {
    doc: &'d Document,
}

impl<'d> JsonTransformer<'d> {
    pub fn new(doc: &'d Document) -> Self {
        Self { doc }
    }

    pub fn serialize(self) -> String {
        let map = self.to_json_map();
        json!(map).to_string()
    }

    fn to_json_map(self) -> Map<String, Value> {
        let mut root = Map::new();

        let mut types: Vec<Map<String, Value>> = Vec::new();

        // loop through types, adding to vecs
        for ty in &self.doc.types {
            let type_obj = serialize_type_obj(ty);
            types.push(type_obj);
        }

        root.insert("type_definitions".into(), types.into());
        root
    }
}

fn serialize_type_obj(ty: &Type) -> Map<String, Value> {
    let mut type_obj = Map::new();
    type_obj.insert("type".into(), ty.kind.clone().into());

    let relations_obj = serialize_relations_obj(&ty.relations);
    type_obj.insert("relations".into(), relations_obj.into());
    type_obj
}

fn serialize_relations_obj(relations: &[Relation]) -> Map<String, Value> {
    let mut rel_obj = Map::new();
    for rel in relations {
        if rel.aliases.len() <= 1 {
            let mut rel_content = Map::new();
            for alias in &rel.aliases {
                let (key, obj) = serialize_alias_obj(&alias);
                rel_content.insert(key, obj);
            }
            rel_obj.insert(rel.kind.clone().into(), json!(rel_content));
        } else {
            let mut children = Vec::new();
            for alias in &rel.aliases {
                let (key, obj) = serialize_alias_obj(&alias);
                let out = json!({ key: obj });
                children.push(out);
            }
            let obj = json!({
                "union": {
                    "child": children
                }
            });
            rel_obj.insert(rel.kind.clone().into(), obj);
        }
    }
    rel_obj
}

fn serialize_alias_obj(alias: &Alias) -> (String, Value) {
    match &alias.kind {
        AliasKind::This => ("this".into(), json!({})),
        AliasKind::Named(name) => match &alias.parent {
            Some(parent) => (
                "tupleToUserset".into(),
                json!({
                    "tupleset": {
                        "object": "",
                        "relation": parent
                    },
                    "computedUserset": {
                        "object": "",
                        "relation": name
                    }
                }),
            ),
            None => (
                "computedUserset".into(),
                json!({
                    "object": "",
                    "relation": name
                }),
            ),
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_single_type() {
        let i = Document {
            types: vec![Type {
                kind: String::from("foo"),
                relations: Vec::new(),
            }],
        };
        let exp = json!({
            "type_definitions": [
                {
                    "type": "foo",
                    "relations": {}
                }
            ]
        });
        let res = JsonTransformer::new(&i).to_json_map();
        assert_eq!(json!(res), exp);
    }

    #[test]
    fn basic_self_relation() {
        let i = vec![Relation {
            kind: "foo".into(),
            aliases: vec![Alias {
                kind: AliasKind::This,
                parent: None,
            }],
        }];
        let exp = json!({
            "foo": {
                "this": {}
            }
        });
        let res = serialize_relations_obj(&i);
        assert_eq!(exp, json!(res));
    }

    #[test]
    fn basic_single_alias_relation() {
        let i = vec![Relation {
            kind: "foo".into(),
            aliases: vec![Alias {
                kind: AliasKind::Named("bar".into()),
                parent: None,
            }],
        }];
        let exp = json!({
            "foo": {
                "computedUserset": {
                    "object": "",
                    "relation": "bar"
                }
            }
        });
        let res = serialize_relations_obj(&i);
        assert_eq!(exp, json!(res));
    }

    #[test]
    fn self_plus_single_alias_relation() {
        let i = vec![Relation {
            kind: "foo".into(),
            aliases: vec![
                Alias {
                    kind: AliasKind::This,
                    parent: None,
                },
                Alias {
                    kind: AliasKind::Named("bar".into()),
                    parent: None,
                },
            ],
        }];
        let exp = json!({
            "foo": {
                "union": {
                    "child": [
                        {
                            "this": {}
                        },
                        {
                            "computedUserset": {
                                "object": "",
                                "relation": "bar"
                            }
                        }
                    ]
                }
            }
        });
        let res = serialize_relations_obj(&i);
        assert_eq!(exp, json!(res));
    }

    #[test]
    fn alias_relation_with_parent() {
        let i = vec![Relation {
            kind: "foo".into(),
            aliases: vec![Alias {
                kind: AliasKind::Named("bar".into()),
                parent: Some("parent".into()),
            }],
        }];
        let exp = json!({
            "foo": {
                "tupleToUserset": {
                    "tupleset": {
                        "object": "",
                        "relation": "parent",
                    },
                    "computedUserset": {
                        "object": "",
                        "relation": "bar",
                    }
                }
            }
        });
        let res = serialize_relations_obj(&i);
        assert_eq!(exp, json!(res));
    }

    #[test]
    fn big_one() {
        let i = Document {
            types: vec![
                Type {
                    kind: "domain".into(),
                    relations: vec![Relation {
                        kind: "member".into(),
                        aliases: vec![Alias {
                            kind: AliasKind::This,
                            parent: None,
                        }],
                    }],
                },
                Type {
                    kind: "folder".into(),
                    relations: vec![
                        Relation {
                            kind: "can_share".into(),
                            aliases: vec![Alias {
                                kind: AliasKind::Named("writer".into()),
                                parent: None,
                            }],
                        },
                        Relation {
                            kind: "owner".into(),
                            aliases: vec![
                                Alias {
                                    kind: AliasKind::This,
                                    parent: None,
                                },
                                Alias {
                                    kind: AliasKind::Named("owner".into()),
                                    parent: Some("parent_folder".into()),
                                },
                            ],
                        },
                    ],
                },
            ],
        };

        let exp = json!({
            "type_definitions": [{
                    "type": "domain",
                    "relations": {
                        "member": {
                            "this": {}
                        }
                    }
                },
                {
                    "type": "folder",
                    "relations": {
                        "can_share": {
                            "computedUserset": {
                                "object": "",
                                "relation": "writer"
                            }
                        },
                        "owner": {
                            "union": {
                                "child": [{
                                        "this": {}
                                    },
                                    {
                                        "tupleToUserset": {
                                            "tupleset": {
                                                "object": "",
                                                "relation": "parent_folder"
                                            },
                                            "computedUserset": {
                                                "object": "",
                                                "relation": "owner"
                                            }
                                        }
                                    }
                                ]
                            }
                        }
                    }
                }
            ]
        });

        let res = JsonTransformer::new(&i).to_json_map();
        assert_eq!(exp, json!(res));
    }
}
