use super::{CompilationResult, Validate};
use crate::{
    compilation::{CompilationContext, JSONSchema},
    error::{error, no_error, CompilationError, ErrorIterator, ValidationError},
};
use serde_json::{Map, Value};

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

impl Validate for MinLengthValidator {
    fn validate<'a>(&self, _: &'a JSONSchema, instance: &'a Value) -> ErrorIterator<'a> {
        if let Value::String(item) = instance {
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
