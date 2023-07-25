use pyo3::prelude::*;

#[pymodule]
#[pyo3(name = "game_lib_pool")]
pub fn game_lib_pool(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Pool>()?;
    m.add_class::<PoolRef>()?;
    Ok(())
}

#[pyclass]
pub struct Pool{
    v: Vec<PyObject>
}

#[pymethods]
impl Pool{
    #[new]
    pub fn new() -> Self {
        Self{
            v: Vec::new()
        }
    }

    pub fn push(&mut self, elem: PyObject) -> PoolRef{
        self.v.push(elem);
        return PoolRef::new(self.v.len() - 1);
    }

    pub fn remove(&mut self, reference: &mut PoolRef){
        assert!(reference.index.is_some());

        self.v.swap_remove(reference.index.unwrap());
        reference.index = None;
    }

    pub fn get(&mut self, reference: &mut PoolRef) -> PyObject{
        assert!(reference.index.is_some());

        self.v[reference.index.unwrap()].clone()
    }
}

#[pyclass]
pub struct PoolRef{
    index: Option<usize>,
}

impl PoolRef{
    fn new(index: usize) -> Self{
        Self {index: Some(index)}
    }
}
