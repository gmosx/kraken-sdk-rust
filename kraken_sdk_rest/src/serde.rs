use serde::de::{self, Deserializer, Visitor};
use serde::Serializer;
use std::fmt;

struct F64Visitor;

impl<'de> Visitor<'de> for F64Visitor {
    type Value = f64;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("an f64 value as string")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        if let Ok(val) = value.parse::<f64>() {
            Ok(val)
        } else {
            Err(E::custom(format!("invalid f64 value: {}", value)))
        }
    }
}

pub fn string_as_f64<'de, D>(deserializer: D) -> Result<f64, D::Error>
where
    D: Deserializer<'de>,
{
    deserializer.deserialize_str(F64Visitor)
}

pub fn f64_as_string<S>(value: &f64, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str(value.to_string().as_str())
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize, Debug)]
    struct Test {
        #[serde(deserialize_with = "string_as_f64", serialize_with = "f64_as_string")]
        val: f64,
    }

    #[test]
    fn test_serialize_deserialize() {
        let val_test = Test { val: 123.123 };

        let serialized = serde_json::to_string(&val_test).expect("err");
        assert_eq!(serialized, "{\"val\":\"123.123\"}");

        let deserialized: Test = serde_json::from_str(&serialized).expect("err");
        assert_eq!(deserialized.val, 123.123);
    }
}
