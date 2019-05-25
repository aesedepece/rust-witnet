use crate::error::RadError;
use crate::types::boolean::RadonBoolean;
use crate::types::float::RadonFloat;
use crate::types::RadonType;
use rmpv::Value;

pub fn multiply(input: &RadonFloat, args: &[Value]) -> Result<RadonFloat, RadError> {
    let multiplier = args
        .first()
        .map(|ref value| value.as_f64())
        .unwrap()
        .unwrap();
    let result = RadonFloat::from(input.value() * multiplier);

    Ok(result)
}

pub fn greater_than(input: &RadonFloat, args: &[Value]) -> Result<RadonBoolean, RadError> {
    let other = args
        .first()
        .map(|ref value| value.as_f64())
        .unwrap()
        .unwrap();

    let result = RadonBoolean::from(input.value() > other);

    Ok(result)
}

pub fn less_than(input: &RadonFloat, args: &[Value]) -> Result<RadonBoolean, RadError> {
    let other = args
        .first()
        .map(|ref value| value.as_f64())
        .unwrap()
        .unwrap();

    let result = RadonBoolean::from(input.value() < other);

    Ok(result)
}
