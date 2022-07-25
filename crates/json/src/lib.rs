use ast::*;
use serde_json::{Map, Value};

pub struct JsonTransformer<'d> {
    doc: &'d Document,
}

impl<'d> JsonTransformer<'d> {
    pub fn new(doc: &'d Document) -> Self {
        Self { doc }
    }

    pub fn to_json(self) -> Map<String, Value> {
        unimplemented!()
    }
}
