/// Docs: https://tools.ietf.org/html/draft-handrews-json-schema-validation-01#section-6.3.2
use super::{CompilationResult, Validate};
use crate::{
    compilation::{CompilationContext, JSONSchema},
    error::{error, no_error, CompilationError, ErrorIterator, ValidationError},
};
use serde_json::{Map, Value};

/// The value of this keyword MUST be a non-negative integer.
pub struct MinLengthValidator {
    limit: u64,
}

impl MinLengthValidator {
    #[inline]
    pub(crate) fn compile(schema: &Value) -> CompilationResult {
        if let Some(limit) = schema.as_u64() {
            return Ok(Box::new(MinLengthValidator { limit }));
        }
        Err(CompilationError::SchemaError)
    }
}

/// A string instance is valid against this keyword if its length is greater than, or equal to,
/// the value of this keyword.
impl Validate for MinLengthValidator {
    fn validate<'a>(&self, _: &'a JSONSchema, instance: &'a Value) -> ErrorIterator<'a> {
        if let Value::String(item) = instance {
            // The length of a string instance is defined as the number of its characters
            // as defined by RFC 7159.
            if (item.chars().count() as u64) < self.limit {
                return error(ValidationError::min_length(instance, self.limit));
            }
        }
        no_error()
    }

    fn is_valid(&self, _: &JSONSchema, instance: &Value) -> bool {
        if let Value::String(item) = instance {
            if (item.chars().count() as u64) < self.limit {
                return false;
            }
        }
        true
    }

    fn name(&self) -> String {
        format!("minLength: {}", self.limit)
    }
}

#[inline]
pub fn compile(
    _: &Map<String, Value>,
    schema: &Value,
    _: &CompilationContext,
) -> Option<CompilationResult> {
    Some(MinLengthValidator::compile(schema))
}
