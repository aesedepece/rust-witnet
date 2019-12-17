use std::{
    convert::{TryFrom, TryInto},
    fmt,
};

use serde::Serialize;
use serde_cbor::value::{from_value, Value};

use crate::{
    error::RadError,
    operators::{identity, string as string_operators, Operable, RadonOpCodes},
    script::RadonCall,
    types::{RadonType, RadonTypes},
};
use witnet_data_structures::radon_report::ReportContext;

pub const RADON_STRING_TYPE_NAME: &str = "RadonString";

#[derive(Clone, Debug, PartialEq, PartialOrd, Ord, Eq, Serialize, Default)]
pub struct RadonString {
    value: String,
}

impl RadonType<String> for RadonString {
    fn value(&self) -> String {
        self.value.clone()
    }

    fn radon_type_name() -> String {
        RADON_STRING_TYPE_NAME.to_string()
    }
}

impl TryFrom<Value> for RadonString {
    type Error = RadError;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        from_value::<String>(value)
            .map(Self::from)
            .map_err(|_| RadError::Decode {
                from: "serde_cbor::value::Value".to_string(),
                to: RADON_STRING_TYPE_NAME.to_string(),
            })
    }
}

impl TryFrom<RadonTypes> for RadonString {
    type Error = RadError;

    fn try_from(item: RadonTypes) -> Result<Self, Self::Error> {
        if let RadonTypes::String(rad_string) = item {
            Ok(rad_string)
        } else {
            let value = Value::try_from(item)?;
            value.try_into()
        }
    }
}

impl TryInto<Value> for RadonString {
    type Error = RadError;

    fn try_into(self) -> Result<Value, Self::Error> {
        Ok(Value::from(self.value()))
    }
}

impl From<String> for RadonString {
    fn from(value: String) -> Self {
        RadonString { value }
    }
}

impl<'a> From<&'a str> for RadonString {
    fn from(value: &'a str) -> Self {
        Self::from(String::from(value))
    }
}

impl Operable for RadonString {
    fn operate(&self, call: &RadonCall) -> Result<RadonTypes, RadError> {
        match call {
            (RadonOpCodes::Identity, None) => identity(RadonTypes::from(self.clone())),
            (RadonOpCodes::StringAsMixed, None) => {
                Ok(RadonTypes::from(string_operators::to_mixed(self.clone())))
            }
            (RadonOpCodes::StringParseJSON, None) => {
                string_operators::parse_json(self).map(RadonTypes::Mixed)
            }
            (RadonOpCodes::StringAsFloat, None) => string_operators::to_float(self)
                .map(RadonTypes::from)
                .map_err(Into::into),
            (RadonOpCodes::StringAsInteger, None) => string_operators::to_int(self)
                .map(RadonTypes::from)
                .map_err(Into::into),
            (RadonOpCodes::StringAsBoolean, None) => string_operators::to_bool(self)
                .map(RadonTypes::from)
                .map_err(Into::into),
            (RadonOpCodes::StringMatch, Some(args)) => {
                string_operators::string_match(self, args.as_slice()).map(RadonTypes::from)
            }
            (RadonOpCodes::StringLength, None) => {
                Ok(RadonTypes::from(string_operators::length(self)))
            }
            (RadonOpCodes::StringToLowerCase, None) => {
                Ok(RadonTypes::from(string_operators::to_lowercase(self)))
            }
            (RadonOpCodes::StringToUpperCase, None) => {
                Ok(RadonTypes::from(string_operators::to_uppercase(self)))
            }
            (op_code, args) => Err(RadError::UnsupportedOperator {
                input_type: RADON_STRING_TYPE_NAME.to_string(),
                operator: op_code.to_string(),
                args: args.to_owned(),
            }),
        }
    }

    fn operate_in_context(
        &self,
        call: &RadonCall,
        _context: &mut ReportContext,
    ) -> Result<RadonTypes, RadError> {
        self.operate(call)
    }
}

impl fmt::Display for RadonString {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, r#"{}("{}")"#, RADON_STRING_TYPE_NAME, self.value)
    }
}

#[test]
fn test_operate_identity() {
    let input = RadonString::from("Hello world!");
    let expected = RadonString::from("Hello world!").into();

    let call = (RadonOpCodes::Identity, None);
    let output = input.operate(&call).unwrap();

    assert_eq!(output, expected);
}

#[test]
fn test_operate_parsejson() {
    let valid_string = RadonString::from(r#"{ "Hello": "world" }"#);
    let invalid_string = RadonString::from(r#"{ "Not a JSON": }"#);

    let call = (RadonOpCodes::StringParseJSON, None);
    let valid_object = valid_string.operate(&call).unwrap();
    let invalid_object = invalid_string.operate(&call);

    assert!(if let RadonTypes::Mixed(bytes) = valid_object {
        if let serde_cbor::value::Value::Map(vector) = bytes.value() {
            if let Some((Value::Text(key), Value::Text(val))) = vector.iter().next() {
                key == "Hello" && val == "world"
            } else {
                false
            }
        } else {
            false
        }
    } else {
        false
    });

    assert!(if let Err(_error) = invalid_object {
        true
    } else {
        false
    });
}

#[test]
fn test_operate_unimplemented() {
    let input = RadonString::from("Hello world!");

    let call = (RadonOpCodes::Fail, None);
    let result = input.operate(&call);

    assert!(if let Err(_error) = result {
        true
    } else {
        false
    });
}

#[test]
fn test_serialize_radon_string() {
    use witnet_data_structures::radon_report::TypeLike;

    let input = RadonTypes::from(RadonString::from("Hello world!"));
    let expected: Vec<u8> = vec![108, 72, 101, 108, 108, 111, 32, 119, 111, 114, 108, 100, 33];

    let output = input.encode().unwrap();

    assert_eq!(output, expected);
}
