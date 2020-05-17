#![feature(core_intrinsics)]
#![warn(
    clippy::doc_markdown,
    clippy::redundant_closure,
    clippy::explicit_iter_loop,
    clippy::match_same_arms,
    clippy::needless_borrow,
    clippy::print_stdout,
    clippy::integer_arithmetic,
    clippy::cast_possible_truncation,
    clippy::result_unwrap_used,
    clippy::result_map_unwrap_or_else,
    clippy::option_unwrap_used,
    clippy::option_map_unwrap_or_else,
    clippy::option_map_unwrap_or
)]
use pyo3::ffi::*;
use pyo3::prelude::*;
use pyo3::types::PyAny;
use pyo3::AsPyPointer;
use pyo3::Python;
use pyo3::{exceptions, wrap_pyfunction};
use serde::ser::{self, Serialize, SerializeMap, SerializeSeq};
use serde::Serializer;
use std::os::raw::c_char;
use std::sync::Once;

pub static mut NONE: *mut pyo3::ffi::PyObject = 0 as *mut pyo3::ffi::PyObject;
pub static mut TRUE: *mut pyo3::ffi::PyObject = 0 as *mut pyo3::ffi::PyObject;
pub static mut FALSE: *mut pyo3::ffi::PyObject = 0 as *mut pyo3::ffi::PyObject;

pub static mut STR_TYPE: *mut PyTypeObject = 0 as *mut PyTypeObject;
pub static mut INT_TYPE: *mut PyTypeObject = 0 as *mut PyTypeObject;
pub static mut BOOL_TYPE: *mut PyTypeObject = 0 as *mut PyTypeObject;
pub static mut NONE_TYPE: *mut PyTypeObject = 0 as *mut PyTypeObject;
pub static mut FLOAT_TYPE: *mut PyTypeObject = 0 as *mut PyTypeObject;
pub static mut LIST_TYPE: *mut PyTypeObject = 0 as *mut PyTypeObject;
pub static mut DICT_TYPE: *mut PyTypeObject = 0 as *mut PyTypeObject;

static INIT: Once = Once::new();

pub fn init_typerefs() {
    INIT.call_once(|| unsafe {
        NONE = Py_None();
        TRUE = Py_True();
        FALSE = Py_False();
        let unicode = PyUnicode_New(0, 255);
        STR_TYPE = (*unicode).ob_type;
        DICT_TYPE = (*PyDict_New()).ob_type;
        LIST_TYPE = (*PyList_New(0 as Py_ssize_t)).ob_type;
        NONE_TYPE = (*NONE).ob_type;
        BOOL_TYPE = (*TRUE).ob_type;
        INT_TYPE = (*PyLong_FromLongLong(0)).ob_type;
        FLOAT_TYPE = (*PyFloat_FromDouble(0.0)).ob_type;
    });
}

#[derive(Copy, Clone)]
pub enum ObjectType {
    Str,
    Int,
    Bool,
    None,
    Float,
    List,
    Dict,
    Unknown,
}

struct SerializePyObject {
    object: *mut pyo3::ffi::PyObject,
    object_type: ObjectType,
}

#[repr(C)]
pub struct PyASCIIObject {
    pub ob_refcnt: Py_ssize_t,
    pub ob_type: *mut PyTypeObject,
    pub length: Py_ssize_t,
    pub hash: Py_hash_t,
    pub state: u32,
    pub wstr: *mut c_char,
}

impl SerializePyObject {
    fn new(object: *mut pyo3::ffi::PyObject) -> Self {
        SerializePyObject {
            object,
            object_type: get_object_type(object),
        }
    }
}

fn get_object_type(object: *mut pyo3::ffi::PyObject) -> ObjectType {
    unsafe {
        let object_type = (*object).ob_type;
        if object_type == STR_TYPE {
            ObjectType::Str
        } else if object_type == FLOAT_TYPE {
            ObjectType::Float
        } else if object_type == BOOL_TYPE {
            ObjectType::Bool
        } else if object_type == INT_TYPE {
            ObjectType::Int
        } else if object_type == NONE_TYPE {
            ObjectType::None
        } else if object_type == LIST_TYPE {
            ObjectType::List
        } else if object_type == DICT_TYPE {
            ObjectType::Dict
        } else {
            ObjectType::Unknown
        }
    }
}

/// Convert a Python value to `serde_json::Value`
impl Serialize for SerializePyObject {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        unsafe {
            match self.object_type {
                ObjectType::Str => {
                    let str_size = (*self.object.cast::<PyASCIIObject>()).length;
                    let uni = self.object.cast::<PyASCIIObject>().offset(1) as *const u8;
                    let slice = std::str::from_utf8_unchecked(std::slice::from_raw_parts(
                        uni,
                        str_size as usize,
                    ));
                    serializer.serialize_str(slice)
                }
                ObjectType::Int => {
                    let val = PyLong_AsLongLong(self.object);
                    serializer.serialize_i64(val)
                }
                ObjectType::Float => serializer.serialize_f64(PyFloat_AS_DOUBLE(self.object)),
                ObjectType::Bool => serializer.serialize_bool(self.object == TRUE),
                ObjectType::None => serializer.serialize_unit(),
                ObjectType::Dict => {
                    let length = (*self.object.cast::<PyDictObject>()).ma_used as usize;
                    if std::intrinsics::unlikely(length == 0) {
                        serializer.serialize_map(Some(0))?.end()
                    } else {
                        let mut map = serializer.serialize_map(Some(length))?;
                        let mut pos = 0isize;
                        let mut key: *mut pyo3::ffi::PyObject = std::ptr::null_mut();
                        let mut value: *mut pyo3::ffi::PyObject = std::ptr::null_mut();
                        for _ in 0..length {
                            pyo3::ffi::_PyDict_Next(
                                self.object,
                                &mut pos,
                                &mut key,
                                &mut value,
                                std::ptr::null_mut(),
                            );
                            {
                                let str_size = (*key.cast::<PyASCIIObject>()).length;
                                let uni = key.cast::<PyASCIIObject>().offset(1) as *const u8;
                                let slice = std::str::from_utf8_unchecked(
                                    std::slice::from_raw_parts(uni, str_size as usize),
                                );
                                map.serialize_key(slice)?;
                            }

                            map.serialize_value(&SerializePyObject::new(value))?;
                        }
                        map.end()
                    }
                }
                ObjectType::List => {
                    let length = PyList_GET_SIZE(self.object) as usize;
                    if length == 0 {
                        serializer.serialize_seq(Some(0))?.end()
                    } else {
                        let mut sequence = serializer.serialize_seq(Some(length))?;
                        for i in 0..length {
                            let elem = *(*(self.object as *mut pyo3::ffi::PyListObject))
                                .ob_item
                                .add(i);
                            sequence.serialize_element(&SerializePyObject::new(elem))?
                        }
                        sequence.end()
                    }
                }
                _ => Err(ser::Error::custom("bar")),
            }
        }
    }
}

#[derive(Debug)]
enum JSONSchemaError {
    Compilation(jsonschema::CompilationError),
}

impl From<JSONSchemaError> for PyErr {
    fn from(error: JSONSchemaError) -> PyErr {
        exceptions::ValueError::py_err(format!("{:?}", error))
    }
}

#[pyfunction]
fn is_valid(schema: &PyAny, instance: &PyAny) -> PyResult<bool> {
    let schema = serde_json::to_value(SerializePyObject::new(schema.as_ptr()))
        .map_err(|err| exceptions::ValueError::py_err(err.to_string()))?;
    let instance = serde_json::to_value(SerializePyObject::new(instance.as_ptr()))
        .map_err(|err| exceptions::ValueError::py_err(err.to_string()))?;
    let compiled =
        jsonschema::JSONSchema::compile(&schema, None).map_err(JSONSchemaError::Compilation)?;
    Ok(compiled.is_valid(&instance))
}

#[pymodule]
fn jsonschema_rs(_py: Python, m: &PyModule) -> PyResult<()> {
    init_typerefs();
    m.add_wrapped(wrap_pyfunction!(is_valid))?;
    Ok(())
}
