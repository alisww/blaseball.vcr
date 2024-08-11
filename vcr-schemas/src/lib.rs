mod schemas;
pub use schemas::*;

// lord this is sinful
pub mod serde_json_borsh {
    use std::collections::BTreeMap;

    use borsh::BorshSerialize;
    #[derive(BorshSerialize)]
    enum FakeValue {
        Null,
        Bool(bool),
        PosInt(u64),
        NegInt(i64),
        Float(f64),
        String(String),
        Array(Vec<FakeValue>),
        Object(BTreeMap<String, FakeValue>),
    }

    impl From<&serde_json::Value> for FakeValue {
        fn from(value: &serde_json::Value) -> Self {
            use serde_json::Value::*;
            match value {
                Null => FakeValue::Null,
                Bool(b) => FakeValue::Bool(*b),
                Number(n) if n.is_f64() => FakeValue::Float(n.as_f64().unwrap()),
                Number(n) if n.is_u64() => FakeValue::PosInt(n.as_u64().unwrap()),
                Number(n) if n.is_i64() => FakeValue::NegInt(n.as_i64().unwrap()),
                Number(n) => FakeValue::Float(n.as_f64().unwrap()),
                String(s) => FakeValue::String(s.clone()),
                Array(v) => FakeValue::Array(v.iter().map(FakeValue::from).collect()),
                Object(map) => FakeValue::Object(BTreeMap::from_iter(
                    map.iter().map(|(k, v)| (k.clone(), FakeValue::from(v))),
                )),
            }
        }
    }

    pub fn serialize_json_value<W: borsh::io::Write>(
        obj: &serde_json::Value,
        writer: &mut W,
    ) -> Result<(), borsh::io::Error> {
        <FakeValue as BorshSerialize>::serialize(&FakeValue::from(obj), writer)
    }

    pub fn serialize_json_value_opt<W: borsh::io::Write>(
        obj: &Option<serde_json::Value>,
        writer: &mut W,
    ) -> Result<(), borsh::io::Error> {
        <Option<FakeValue> as BorshSerialize>::serialize(&obj.as_ref().map(FakeValue::from), writer)
    }

    pub fn serialize_json_value_vecopt<W: borsh::io::Write>(
        obj: &Option<Vec<serde_json::Value>>,
        writer: &mut W,
    ) -> Result<(), borsh::io::Error> {
        <Option<Vec<FakeValue>> as BorshSerialize>::serialize(
            &obj.as_ref()
                .map(|v| v.iter().map(FakeValue::from).collect()),
            writer,
        )
    }

    pub fn serialize_json_value_vec<W: borsh::io::Write>(
        obj: &Vec<serde_json::Value>,
        writer: &mut W,
    ) -> Result<(), borsh::io::Error> {
        <Vec<FakeValue> as BorshSerialize>::serialize(
            &obj.iter().map(FakeValue::from).collect(),
            writer,
        )
    }
}
