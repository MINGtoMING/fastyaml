#![feature(if_let_guard)]

pub mod deserialize;
pub mod serialize;

use crate::deserialize::PyLoader;
use crate::serialize::PyDumper;
use pyo3::prelude::*;
use pyo3::types::PyString;
use serde::de::DeserializeSeed;
use serde::Serialize;
use serde_yaml::{Deserializer, Serializer};

#[pyfunction]
pub fn loads(py: Python<'_>, s: &str) -> PyObject {
    let loader = PyLoader::new(py);
    let deserializer = Deserializer::from_str(s);
    let result = loader
        .deserialize(deserializer)
        .expect("Failed to deserialize");
    result
}

#[pyfunction]
pub fn dumps<'py>(py: Python<'py>, obj: &Bound<'py, PyAny>) -> Bound<'py, PyString> {
    let mut buffer = Vec::with_capacity(128);
    let mut serializer = Serializer::new(&mut buffer);
    let dumper = PyDumper::new(py, obj);
    dumper.serialize(&mut serializer).unwrap();
    let out = String::from_utf8(buffer).unwrap();
    PyString::new_bound(py, &out)
}

#[pymodule]
fn fastyaml(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(loads, m)?)?;
    m.add_function(wrap_pyfunction!(dumps, m)?)?;
    Ok(())
}
