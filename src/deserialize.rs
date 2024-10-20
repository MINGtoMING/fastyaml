use pyo3::prelude::*;
use pyo3::types::PyDict;
use serde::de::{DeserializeSeed, Error, MapAccess, SeqAccess, Visitor};
use serde::Deserializer;
use smallvec::SmallVec;
use std::fmt::Formatter;

#[derive(Clone, Copy)]
pub(crate) struct PyLoader<'py> {
    py: Python<'py>,
}

impl<'py> PyLoader<'py> {
    pub(crate) fn new(py: Python<'py>) -> Self {
        PyLoader { py }
    }
}

impl<'de> Visitor<'de> for PyLoader<'_> {
    type Value = PyObject;

    fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
        write!(formatter, "a valid YAML value")
    }

    fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(v.to_object(self.py))
    }

    fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(v.to_object(self.py))
    }

    fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(v.to_object(self.py))
    }

    fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(v.to_object(self.py))
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(v.to_object(self.py))
    }

    fn visit_unit<E>(self) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(self.py.None())
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: SeqAccess<'de>,
    {
        let mut list: SmallVec<[Self::Value; 128]> = SmallVec::new();
        while let Some(elem) = seq.next_element_seed(self)? {
            list.push(elem);
        }
        Ok(list.to_object(self.py))
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: MapAccess<'de>,
    {
        let dict = PyDict::new_bound(self.py);
        while let Some(key) = map.next_key_seed(self)? {
            let value = map.next_value_seed(self)?;
            dict.set_item(key, value).unwrap();
        }
        Ok(dict.to_object(self.py))
    }
}

impl<'de> DeserializeSeed<'de> for PyLoader<'_> {
    type Value = PyObject;

    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(self)
    }
}
