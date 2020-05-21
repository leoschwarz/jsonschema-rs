/// Docs: <https://tools.ietf.org/html/draft-handrews-json-schema-validation-01#section-6.2.5>
use super::{CompilationResult, Validate};
use crate::{
    compilation::{CompilationContext, JSONSchema},
    error::{error, no_error, CompilationError, ErrorIterator, ValidationError},
};
use serde_json::{Map, Value};

/// The value of "exclusiveMinimum" MUST be number, representing an exclusive lower limit for a numeric instance.
pub struct ExclusiveMinimumValidator {
    limit: f64,
}

impl ExclusiveMinimumValidator {
    #[inline]
    pub(crate) fn compile(schema: &Value) -> CompilationResult {
        if let Value::Number(limit) = schema {
            let limit = limit.as_f64().expect("Always valid");
            return Ok(Box::new(ExclusiveMinimumValidator { limit }));
        }
        Err(CompilationError::SchemaError)
    }
}

/// If the instance is a number, then the instance is valid only if it has a value strictly
/// greater than (not equal to) "exclusiveMinimum".
impl Validate for ExclusiveMinimumValidator {
    fn validate<'a>(&self, _: &'a JSONSchema, instance: &'a Value) -> ErrorIterator<'a> {
        if let Value::Number(item) = instance {
            let item = item.as_f64().expect("Always valid");
            if item <= self.limit {
                return error(ValidationError::exclusive_minimum(instance, self.limit));
            }
        }
        no_error()
    }

    fn is_valid(&self, _: &JSONSchema, instance: &Value) -> bool {
        if let Value::Number(item) = instance {
            let item = item.as_f64().expect("Always valid");
            if item <= self.limit {
                return false;
            }
        }
        true
    }

    fn name(&self) -> String {
        format!("exclusiveMinimum: {}", self.limit)
    }
}
#[inline]
pub fn compile(
    _: &Map<String, Value>,
    schema: &Value,
    _: &CompilationContext,
) -> Option<CompilationResult> {
    Some(ExclusiveMinimumValidator::compile(schema))
}
