/// Docs: <https://tools.ietf.org/html/draft-handrews-json-schema-validation-01#section-6.1.3>
use super::{helpers, CompilationResult, Validate};
use crate::{
    compilation::{CompilationContext, JSONSchema},
    error::{error, no_error, ErrorIterator, ValidationError},
};
use serde_json::{Map, Value};

/// The value of this keyword MAY be of any type, including null.
pub struct ConstValidator {
    value: Value,
}

impl ConstValidator {
    #[inline]
    pub(crate) fn compile(value: &Value) -> CompilationResult {
        Ok(Box::new(ConstValidator {
            value: value.clone(),
        }))
    }
}

/// An instance validates successfully against this keyword if its value is equal to the value of the keyword.
impl Validate for ConstValidator {
    fn validate<'a>(&self, schema: &'a JSONSchema, instance: &'a Value) -> ErrorIterator<'a> {
        if self.is_valid(schema, instance) {
            no_error()
        } else {
            error(ValidationError::constant(instance, &self.value))
        }
    }

    fn is_valid(&self, _: &JSONSchema, instance: &Value) -> bool {
        helpers::equal(instance, &self.value)
    }

    fn name(&self) -> String {
        format!("const: {}", self.value)
    }
}
#[inline]
pub fn compile(
    _: &Map<String, Value>,
    schema: &Value,
    _: &CompilationContext,
) -> Option<CompilationResult> {
    Some(ConstValidator::compile(schema))
}
