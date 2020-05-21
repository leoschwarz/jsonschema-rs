/// Docs: https://tools.ietf.org/html/draft-handrews-json-schema-validation-01#section-6.1.1
use super::{CompilationResult, Validate};
use crate::{
    compilation::{CompilationContext, JSONSchema},
    error::{error, no_error, CompilationError, ErrorIterator, PrimitiveType, ValidationError},
};
use serde_json::{Map, Number, Value};

/// The value of this keyword MUST be either a string or an array.
/// If it is an array, elements of the array MUST be strings and MUST be unique.
pub struct MultipleTypesValidator {
    types: Vec<PrimitiveType>,
}

impl MultipleTypesValidator {
    #[inline]
    pub(crate) fn compile(items: &[Value]) -> CompilationResult {
        let mut types = Vec::with_capacity(items.len());
        for item in items {
            // String values MUST be one of the six primitive types ("null", "boolean", "object",
            // "array", "number", or "string"), or "integer" which matches any number with a zero fractional part.
            match item {
                Value::String(string) => match string.as_str() {
                    "integer" => types.push(PrimitiveType::Integer),
                    "null" => types.push(PrimitiveType::Null),
                    "boolean" => types.push(PrimitiveType::Boolean),
                    "string" => types.push(PrimitiveType::String),
                    "array" => types.push(PrimitiveType::Array),
                    "object" => types.push(PrimitiveType::Object),
                    "number" => types.push(PrimitiveType::Number),
                    _ => return Err(CompilationError::SchemaError),
                },
                _ => return Err(CompilationError::SchemaError),
            }
        }
        Ok(Box::new(MultipleTypesValidator { types }))
    }
}

/// An instance validates if and only if the instance is in any of the sets listed for this keyword.
impl Validate for MultipleTypesValidator {
    fn validate<'a>(&self, schema: &'a JSONSchema, instance: &'a Value) -> ErrorIterator<'a> {
        if self.is_valid(schema, instance) {
            no_error()
        } else {
            error(ValidationError::multiple_type_error(
                instance,
                self.types.clone(),
            ))
        }
    }
    fn is_valid(&self, _: &JSONSchema, instance: &Value) -> bool {
        for type_ in &self.types {
            match (type_, instance) {
                (PrimitiveType::Integer, Value::Number(num)) if is_integer(num) => return true,
                (PrimitiveType::Null, Value::Null)
                | (PrimitiveType::Boolean, Value::Bool(_))
                | (PrimitiveType::String, Value::String(_))
                | (PrimitiveType::Array, Value::Array(_))
                | (PrimitiveType::Object, Value::Object(_))
                | (PrimitiveType::Number, Value::Number(_)) => return true,
                (_, _) => continue,
            };
        }
        false
    }

    fn name(&self) -> String {
        format!(
            "type: [{}]",
            self.types
                .iter()
                .map(|type_| format!("{}", type_))
                .collect::<Vec<String>>()
                .join(", ")
        )
    }
}

pub struct NullTypeValidator {}

impl NullTypeValidator {
    #[inline]
    pub(crate) fn compile() -> CompilationResult {
        Ok(Box::new(NullTypeValidator {}))
    }
}

impl Validate for NullTypeValidator {
    fn validate<'a>(&self, schema: &'a JSONSchema, instance: &'a Value) -> ErrorIterator<'a> {
        if self.is_valid(schema, instance) {
            no_error()
        } else {
            error(ValidationError::single_type_error(
                instance,
                PrimitiveType::Null,
            ))
        }
    }
    fn is_valid(&self, _: &JSONSchema, instance: &Value) -> bool {
        instance.is_null()
    }

    fn name(&self) -> String {
        "type: null".to_string()
    }
}

pub struct BooleanTypeValidator {}

impl BooleanTypeValidator {
    #[inline]
    pub(crate) fn compile() -> CompilationResult {
        Ok(Box::new(BooleanTypeValidator {}))
    }
}

impl Validate for BooleanTypeValidator {
    fn validate<'a>(&self, schema: &'a JSONSchema, instance: &'a Value) -> ErrorIterator<'a> {
        if self.is_valid(schema, instance) {
            no_error()
        } else {
            error(ValidationError::single_type_error(
                instance,
                PrimitiveType::Boolean,
            ))
        }
    }
    fn is_valid(&self, _: &JSONSchema, instance: &Value) -> bool {
        instance.is_boolean()
    }

    fn name(&self) -> String {
        "type: boolean".to_string()
    }
}

pub struct StringTypeValidator {}

impl StringTypeValidator {
    #[inline]
    pub(crate) fn compile() -> CompilationResult {
        Ok(Box::new(StringTypeValidator {}))
    }
}

impl Validate for StringTypeValidator {
    fn validate<'a>(&self, schema: &'a JSONSchema, instance: &'a Value) -> ErrorIterator<'a> {
        if self.is_valid(schema, instance) {
            no_error()
        } else {
            error(ValidationError::single_type_error(
                instance,
                PrimitiveType::String,
            ))
        }
    }

    fn is_valid(&self, _: &JSONSchema, instance: &Value) -> bool {
        instance.is_string()
    }

    fn name(&self) -> String {
        "type: string".to_string()
    }
}

pub struct ArrayTypeValidator {}

impl ArrayTypeValidator {
    #[inline]
    pub(crate) fn compile() -> CompilationResult {
        Ok(Box::new(ArrayTypeValidator {}))
    }
}

impl Validate for ArrayTypeValidator {
    fn validate<'a>(&self, schema: &'a JSONSchema, instance: &'a Value) -> ErrorIterator<'a> {
        if self.is_valid(schema, instance) {
            no_error()
        } else {
            error(ValidationError::single_type_error(
                instance,
                PrimitiveType::Array,
            ))
        }
    }

    fn is_valid(&self, _: &JSONSchema, instance: &Value) -> bool {
        instance.is_array()
    }

    fn name(&self) -> String {
        "type: array".to_string()
    }
}

pub struct ObjectTypeValidator {}

impl ObjectTypeValidator {
    #[inline]
    pub(crate) fn compile() -> CompilationResult {
        Ok(Box::new(ObjectTypeValidator {}))
    }
}

impl Validate for ObjectTypeValidator {
    fn validate<'a>(&self, schema: &'a JSONSchema, instance: &'a Value) -> ErrorIterator<'a> {
        if self.is_valid(schema, instance) {
            no_error()
        } else {
            error(ValidationError::single_type_error(
                instance,
                PrimitiveType::Object,
            ))
        }
    }
    fn is_valid(&self, _: &JSONSchema, instance: &Value) -> bool {
        instance.is_object()
    }

    fn name(&self) -> String {
        "type: object".to_string()
    }
}

pub struct NumberTypeValidator {}

impl NumberTypeValidator {
    #[inline]
    pub(crate) fn compile() -> CompilationResult {
        Ok(Box::new(NumberTypeValidator {}))
    }
}

impl Validate for NumberTypeValidator {
    fn validate<'a>(&self, config: &'a JSONSchema, instance: &'a Value) -> ErrorIterator<'a> {
        if self.is_valid(config, instance) {
            no_error()
        } else {
            error(ValidationError::single_type_error(
                instance,
                PrimitiveType::Number,
            ))
        }
    }
    fn is_valid(&self, _: &JSONSchema, instance: &Value) -> bool {
        instance.is_number()
    }

    fn name(&self) -> String {
        "type: number".to_string()
    }
}

pub struct IntegerTypeValidator {}

impl IntegerTypeValidator {
    #[inline]
    pub(crate) fn compile() -> CompilationResult {
        Ok(Box::new(IntegerTypeValidator {}))
    }
}

impl Validate for IntegerTypeValidator {
    fn validate<'a>(&self, schema: &'a JSONSchema, instance: &'a Value) -> ErrorIterator<'a> {
        if self.is_valid(schema, instance) {
            no_error()
        } else {
            error(ValidationError::single_type_error(
                instance,
                PrimitiveType::Integer,
            ))
        }
    }

    fn is_valid(&self, _: &JSONSchema, instance: &Value) -> bool {
        if let Value::Number(num) = instance {
            return is_integer(num);
        }
        false
    }

    fn name(&self) -> String {
        "type: integer".to_string()
    }
}

fn is_integer(num: &Number) -> bool {
    num.is_u64() || num.is_i64() || num.as_f64().expect("Always valid").fract() == 0.
}

#[inline]
pub fn compile(
    _: &Map<String, Value>,
    schema: &Value,
    _: &CompilationContext,
) -> Option<CompilationResult> {
    match schema {
        Value::String(item) => compile_single_type(item.as_str()),
        Value::Array(items) => {
            if items.len() == 1 {
                if let Some(Value::String(item)) = items.iter().next() {
                    compile_single_type(item.as_str())
                } else {
                    Some(Err(CompilationError::SchemaError))
                }
            } else {
                Some(MultipleTypesValidator::compile(items))
            }
        }
        _ => Some(Err(CompilationError::SchemaError)),
    }
}

fn compile_single_type(item: &str) -> Option<CompilationResult> {
    match item {
        "integer" => Some(IntegerTypeValidator::compile()),
        "null" => Some(NullTypeValidator::compile()),
        "boolean" => Some(BooleanTypeValidator::compile()),
        "string" => Some(StringTypeValidator::compile()),
        "array" => Some(ArrayTypeValidator::compile()),
        "object" => Some(ObjectTypeValidator::compile()),
        "number" => Some(NumberTypeValidator::compile()),
        _ => Some(Err(CompilationError::SchemaError)),
    }
}
