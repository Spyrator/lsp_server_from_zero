use std::fmt;

use serde::{
    de::{self, Visitor},
    ser::SerializeMap,
    Deserialize, Deserializer, Serialize, Serializer,
};

use super::{errors::JsonRpcError, request::StringOrNumber};

// return a response
#[derive(Debug, Serialize, Deserialize)]
pub struct JsonRpcResponse {
    pub jsonrpc: String,

    #[serde(flatten)]
    pub result: JsonRpcResult,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<StringOrNumber>,
}

// ok variant is just string so far, but that's ok
// spec: string | number | boolean | array | object | null, we define it later
#[derive(Debug)]
pub struct JsonRpcResult(pub Result<String, JsonRpcError>);

impl Serialize for JsonRpcResult {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map = serializer.serialize_map(Some(1))?;
        match &self.0 {
            Ok(value) => map.serialize_entry("result", &value)?,
            Err(error) => map.serialize_entry("error", &error)?,
        }
        return map.end();
    }
}

impl<'de> Deserialize<'de> for JsonRpcResult {
    fn deserialize<D>(deserializer: D) -> Result<JsonRpcResult, D::Error>
    where
        D: Deserializer<'de>,
    {
        // This part could also be generated independently by:

        #[derive(Deserialize)]
        #[serde(field_identifier, rename_all = "lowercase")]
        enum Field {
            Result,
            Error,
        }

        struct JsonRpcResultVisitor;

        impl<'de> Visitor<'de> for JsonRpcResultVisitor {
            type Value = JsonRpcResult;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct JsonRpcResult")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: de::MapAccess<'de>,
            {
                let mut result = None;
                let mut error = None;
                while let Some(key) = map.next_key()? {
                    match key {
                        Field::Result => {
                            if result.is_some() {
                                return Err(de::Error::duplicate_field("result"));
                            }
                            result = Some(map.next_value()?);
                        }
                        Field::Error => {
                            if error.is_some() {
                                return Err(de::Error::duplicate_field("error"));
                            }
                            error = Some(map.next_value()?);
                        }
                    }
                }

                if let Some(res) = result {
                    return Ok(JsonRpcResult(Ok(res)));
                };

                if let Some(err) = error {
                    return Ok(JsonRpcResult(Err(err)));
                };

                return Err(de::Error::missing_field("`result` or `error`"));
            }
        }

        const FIELDS: &'static [&'static str] = &["result", "error"];
        deserializer.deserialize_struct("JsonRpcResult", FIELDS, JsonRpcResultVisitor)
    }
}
