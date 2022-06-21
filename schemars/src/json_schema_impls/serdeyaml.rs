use crate::gen::SchemaGenerator;
use crate::schema::*;
use crate::{JsonSchema, Schema};
use serde_yaml::{Mapping, Number, Value};
use std::collections::BTreeMap;

impl JsonSchema for Value {
    no_ref_schema!();

    fn schema_name() -> String {
        "AnyValue".to_owned()
    }

    fn json_schema(_: &mut SchemaGenerator) -> Schema {
        Schema::Bool(true)
    }
}

forward_impl!(Mapping => BTreeMap<Value, Value>);

impl JsonSchema for Number {
    no_ref_schema!();

    fn schema_name() -> String {
        "Number".to_owned()
    }

    fn json_schema(_: &mut SchemaGenerator) -> Schema {
        SchemaObject {
            instance_type: Some(InstanceType::Number.into()),
            ..Default::default()
        }
        .into()
    }
}
