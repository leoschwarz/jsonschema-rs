/// Docs: https://tools.ietf.org/html/draft-handrews-json-schema-validation-01#section-6.2.2
use super::{CompilationResult, Validate};
use crate::{
    compilation::{CompilationContext, JSONSchema},
    error::{error, no_error, CompilationError, ErrorIterator, ValidationError},
};
use serde_json::{Map, Value};

/// The value of "maximum" MUST be a number, representing an inclusive upper limit for
/// a numeric instance.
pub struct MaximumValidator {
    limit: f64,
}

impl MaximumValidator {
    #[inline]
    pub(crate) fn compile(schema: &Value) -> CompilationResult {
        if let Value::Number(limit) = schema {
            let limit = limit.as_f64().expect("Always valid");
            return Ok(Box::new(MaximumValidator { limit }));
        }
        Err(CompilationError::SchemaError)
    }
}

/// If the instance is a number, then this keyword validates only if
/// the instance is less than or exactly equal to "maximum".
impl Validate for MaximumValidator {
    fn validate<'a>(&self, _: &'a JSONSchema, instance: &'a Value) -> ErrorIterator<'a> {
        if let Value::Number(item) = instance {
            let item = item.as_f64().expect("Always valid");
            if item > self.limit {
                return error(ValidationError::maximum(instance, self.limit));
            }
        }
        no_error()
    }

    fn is_valid(&self, _: &JSONSchema, instance: &Value) -> bool {
        if let Value::Number(item) = instance {
            let item = item.as_f64().expect("Always valid");
            if item > self.limit {
                return false;
            }
        }
        true
    }

    fn name(&self) -> String {
        format!("maximum: {}", self.limit)
    }
}

#[inline]
pub fn compile(
    _: &Map<String, Value>,
    schema: &Value,
    _: &CompilationContext,
) -> Option<CompilationResult> {
    Some(MaximumValidator::compile(schema))
}
