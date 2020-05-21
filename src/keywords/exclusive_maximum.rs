/// Docs: <https://tools.ietf.org/html/draft-handrews-json-schema-validation-01#section-6.2.3>
use super::{CompilationResult, Validate};
use crate::{
    compilation::{CompilationContext, JSONSchema},
    error::{error, no_error, CompilationError, ErrorIterator, ValidationError},
};
use serde_json::{Map, Value};

/// The value of "exclusiveMaximum" MUST be number, representing an exclusive upper limit for a numeric instance.
pub struct ExclusiveMaximumValidator {
    limit: f64,
}

impl ExclusiveMaximumValidator {
    #[inline]
    pub(crate) fn compile(schema: &Value) -> CompilationResult {
        if let Value::Number(limit) = schema {
            return Ok(Box::new(ExclusiveMaximumValidator {
                limit: limit.as_f64().expect("Always valid"),
            }));
        }
        Err(CompilationError::SchemaError)
    }
}

/// If the instance is a number, then the instance is valid only if it has a value strictly
/// less than (not equal to) "exclusiveMaximum".
impl Validate for ExclusiveMaximumValidator {
    fn validate<'a>(&self, _: &'a JSONSchema, instance: &'a Value) -> ErrorIterator<'a> {
        if let Value::Number(item) = instance {
            let item = item.as_f64().expect("Always valid");
            if item >= self.limit {
                return error(ValidationError::exclusive_maximum(instance, self.limit));
            }
        }
        no_error()
    }

    fn is_valid(&self, _: &JSONSchema, instance: &Value) -> bool {
        if let Value::Number(item) = instance {
            let item = item.as_f64().expect("Always valid");
            if item >= self.limit {
                return false;
            }
        }
        true
    }

    fn name(&self) -> String {
        format!("exclusiveMaximum: {}", self.limit)
    }
}

#[inline]
pub fn compile(
    _: &Map<String, Value>,
    schema: &Value,
    _: &CompilationContext,
) -> Option<CompilationResult> {
    Some(ExclusiveMaximumValidator::compile(schema))
}
