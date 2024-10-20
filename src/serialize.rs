use pyo3::prelude::*;
use pyo3::types::{PyBool, PyDict, PyFloat, PyInt, PyList, PyString, PyTuple};
use serde::ser::{SerializeMap, SerializeSeq};
use serde::{Serialize, Serializer};
use std::{f32, f64};

pub struct PyDumper<'py, 'a> {
    py: Python<'py>,
    obj: &'a Bound<'py, PyAny>,
}

impl<'py, 'a> PyDumper<'py, 'a> {
    #[inline(always)]
    pub fn new(py: Python<'py>, obj: &'a Bound<'py, PyAny>) -> Self {
        PyDumper { py, obj }
    }

    #[inline(always)]
    fn serialize_int<S>(&self, int: &Bound<'_, PyInt>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        if let Ok(x) = int.extract::<u128>() {
            match x {
                x if x <= u8::MAX as u128 => serializer.serialize_u8(x as u8),
                x if x <= u16::MAX as u128 => serializer.serialize_u16(x as u16),
                x if x <= u32::MAX as u128 => serializer.serialize_u32(x as u32),
                x if x <= u64::MAX as u128 => serializer.serialize_u64(x as u64),
                _ => serializer.serialize_u128(x),
            }
        } else if let Ok(x) = int.extract::<i128>() {
            match x {
                x if x >= i8::MIN as i128 && x <= i8::MAX as i128 => {
                    serializer.serialize_i8(x as i8)
                }
                x if x >= i16::MIN as i128 && x <= i16::MAX as i128 => {
                    serializer.serialize_i16(x as i16)
                }
                x if x >= i32::MIN as i128 && x <= i32::MAX as i128 => {
                    serializer.serialize_i32(x as i32)
                }
                x if x >= i64::MIN as i128 && x <= i64::MAX as i128 => {
                    serializer.serialize_i64(x as i64)
                }
                _ => serializer.serialize_i128(x),
            }
        } else {
            panic!("Unsupported integer type")
        }
    }

    #[inline(always)]
    fn serialize_float<S>(
        &self,
        float: &Bound<'_, PyFloat>,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        if let Ok(x) = float.extract::<f32>() {
            serializer.serialize_f32(x)
        } else if let Ok(x) = float.extract::<f64>() {
            serializer.serialize_f64(x)
        } else {
            panic!("Unsupported float type")
        }
    }
}

impl Serialize for PyDumper<'_, '_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self.obj {
            obj if obj.is_none() => serializer.serialize_unit(),
            obj if obj.is_instance_of::<PyBool>() => {
                serializer.serialize_bool(obj.is_truthy().unwrap())
            }
            obj if let Ok(int) = obj.downcast::<PyInt>() => self.serialize_int(int, serializer),
            obj if obj.is_instance_of::<PyList>() || obj.is_instance_of::<PyTuple>() => {
                let list = obj.downcast::<PyList>().unwrap();
                let mut seq = serializer.serialize_seq(Some(list.len()))?;
                for element in list {
                    seq.serialize_element(&PyDumper::new(self.py, &element))?;
                }
                seq.end()
            }
            obj if obj.is_instance_of::<PyDict>() => {
                let dict = obj.downcast::<PyDict>().unwrap();
                let mut map = serializer.serialize_map(Some(dict.len()))?;
                let mut int_buffer = itoa::Buffer::new();
                let mut float_buffer = dtoa::Buffer::new();
                for (key, value) in dict {
                    let key_str = if key.is_none() {
                        "null"
                    } else if key.is_instance_of::<PyBool>() {
                        match key.is_truthy().unwrap() {
                            true => "true",
                            false => "false",
                        }
                    } else if let Ok(s) = key.downcast::<PyString>() {
                        s.to_str().unwrap()
                    } else if let Ok(int) = key.downcast::<PyInt>() {
                        int_buffer.format(int.extract::<i128>().unwrap())
                    } else if let Ok(float) = key.downcast::<PyFloat>() {
                        float_buffer.format(float.extract::<f64>().unwrap())
                    } else {
                        panic!("Unsupported key type");
                    };
                    map.serialize_entry(key_str, &PyDumper::new(self.py, &value))?;
                }
                map.end()
            }
            obj if let Ok(s) = obj.downcast::<PyString>() => {
                serializer.serialize_str(s.to_str().unwrap())
            }
            obj if let Ok(float) = obj.downcast::<PyFloat>() => {
                self.serialize_float(float, serializer)
            }
            _ => panic!("Unsupported type"),
        }
    }
}
