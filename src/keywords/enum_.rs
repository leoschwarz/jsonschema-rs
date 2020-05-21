/// Docs: <https://tools.ietf.org/html/draft-handrews-json-schema-validation-01#section-6.1.2>
use super::{helpers, CompilationResult, Validate};
use crate::{
    compilation::{CompilationContext, JSONSchema},
    error::{error, no_error, CompilationError, ErrorIterator, ValidationError},
};
use serde_json::{Map, Value};

/// The value of this keyword MUST be an array. This array SHOULD have at least one element.
/// Elements in the array SHOULD be unique.
/// Elements in the array might be of any value, including null.
pub struct EnumValidator {
    options: Value,
    items: Vec<Value>,
}

impl EnumValidator {
    #[inline]
    pub(crate) fn compile(schema: &Value) -> CompilationResult {
        if let Value::Array(items) = schema {
            return Ok(Box::new(EnumValidator {
                options: schema.clone(),
                items: items.clone(),
            }));
        }
        Err(CompilationError::SchemaError)
    }
}

/// An instance validates successfully against this keyword if its value is equal to one of
/// the elements in this keyword's array value.
impl Validate for EnumValidator {
    fn validate<'a>(&self, schema: &'a JSONSchema, instance: &'a Value) -> ErrorIterator<'a> {
        if !self.is_valid(schema, instance) {
            return error(ValidationError::enumeration(instance, &self.options));
        }
        no_error()
    }

    fn is_valid(&self, _: &JSONSchema, instance: &Value) -> bool {
        self.items.iter().any(|item| helpers::equal(instance, item))
    }

    fn name(&self) -> String {
        format!(
            "enum: [{}]",
            self.items
                .iter()
                .map(|item| format!("{}", item))
                .collect::<Vec<String>>()
                .join(", ")
        )
    }
}

#[inline]
pub fn compile(
    _: &Map<String, Value>,
    schema: &Value,
    _: &CompilationContext,
) -> Option<CompilationResult> {
    Some(EnumValidator::compile(schema))
}
