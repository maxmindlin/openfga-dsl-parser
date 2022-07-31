use openfga_dsl_parser::*;
use serde_json::Value;

#[test]
fn parses_full_doc() {
    let i = "type team
  relations
    define member as self
type repo
  relations
    define admin as self or repo_admin from owner
    define maintainer as self or admin
    define owner as self
    define reader as self or triager or repo_reader from owner
    define triager as self or writer
    define writer as self or maintainer or repo_writer from owner
type org
  relations
    define billing_manager as self or owner
    define member as self or owner
    define owner as self
    define repo_admin as self
    define repo_reader as self
    define repo_writer as self
type app
  relations
    define app_manager as self or owner from owner
    define owner as self";
    let exp_raw = r#"{
  "type_definitions": [
    {
      "type": "team",
      "relations": {
        "member": {
          "this": {}
        }
      }
    },
    {
      "type": "repo",
      "relations": {
        "admin": {
          "union": {
            "child": [
              {
                "this": {}
              },
              {
                "tupleToUserset": {
                  "tupleset": {
                    "object": "",
                    "relation": "owner"
                  },
                  "computedUserset": {
                    "object": "",
                    "relation": "repo_admin"
                  }
                }
              }
            ]
          }
        },
        "maintainer": {
          "union": {
            "child": [
              {
                "this": {}
              },
              {
                "computedUserset": {
                  "object": "",
                  "relation": "admin"
                }
              }
            ]
          }
        },
        "owner": {
          "this": {}
        },
        "reader": {
          "union": {
            "child": [
              {
                "this": {}
              },
              {
                "computedUserset": {
                  "object": "",
                  "relation": "triager"
                }
              },
              {
                "tupleToUserset": {
                  "tupleset": {
                    "object": "",
                    "relation": "owner"
                  },
                  "computedUserset": {
                    "object": "",
                    "relation": "repo_reader"
                  }
                }
              }
            ]
          }
        },
        "triager": {
          "union": {
            "child": [
              {
                "this": {}
              },
              {
                "computedUserset": {
                  "object": "",
                  "relation": "writer"
                }
              }
            ]
          }
        },
        "writer": {
          "union": {
            "child": [
              {
                "this": {}
              },
              {
                "computedUserset": {
                  "object": "",
                  "relation": "maintainer"
                }
              },
              {
                "tupleToUserset": {
                  "tupleset": {
                    "object": "",
                    "relation": "owner"
                  },
                  "computedUserset": {
                    "object": "",
                    "relation": "repo_writer"
                  }
                }
              }
            ]
          }
        }
      }
    },
    {
      "type": "org",
      "relations": {
        "billing_manager": {
          "union": {
            "child": [
              {
                "this": {}
              },
              {
                "computedUserset": {
                  "object": "",
                  "relation": "owner"
                }
              }
            ]
          }
        },
        "member": {
          "union": {
            "child": [
              {
                "this": {}
              },
              {
                "computedUserset": {
                  "object": "",
                  "relation": "owner"
                }
              }
            ]
          }
        },
        "owner": {
          "this": {}
        },
        "repo_admin": {
          "this": {}
        },
        "repo_reader": {
          "this": {}
        },
        "repo_writer": {
          "this": {}
        }
      }
    },
    {
      "type": "app",
      "relations": {
        "app_manager": {
          "union": {
            "child": [
              {
                "this": {}
              },
              {
                "tupleToUserset": {
                  "tupleset": {
                    "object": "",
                    "relation": "owner"
                  },
                  "computedUserset": {
                    "object": "",
                    "relation": "owner"
                  }
                }
              }
            ]
          }
        },
        "owner": {
          "this": {}
        }
      }
    }
  ]
}"#;
    let exp: Value = serde_json::from_str(exp_raw).unwrap();

    let mut parser = Parser::new(&i);
    let doc = parser.parse_document().unwrap();

    let res = json::JsonTransformer::new(&doc)
        .serialize();
    assert_eq!(exp, serde_json::from_str::<Value>(&res).unwrap());
}
