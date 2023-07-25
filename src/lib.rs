use pyo3::prelude::*;

#[pymodule]
pub fn pool(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Pool>()?;
    m.add_class::<PoolRef>()?;
    Ok(())
}

#[pyclass]
pub struct Pool{
    v: Vec<PyObject>
}

impl Pool{
    pub fn push(&mut self, elem: PyObject) -> PoolRef{
        self.v.push(elem);
        return PoolRef::new(self.v.len() - 1);
    }

    fn remove(&mut self, reference: &mut PoolRef){
        assert!(reference.index.is_some());

        self.v.swap_remove(reference.index.unwrap());
        reference.index = None;
    }

    fn get(&mut self, reference: &mut PoolRef) -> PyObject{
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
