use pyo3::prelude::*;

#[pymodule]
#[pyo3(name = "game_lib_pool")]
pub fn game_lib_pool(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Pool>()?;
    m.add_class::<PoolRef>()?;
    m.add_class::<PoolIter>()?;
    Ok(())
}

#[derive(Debug, Clone)]
#[pyclass]
pub struct Pool{
    v: Vec<PyObject>,
    keys: Vec<Py<PoolRef>>
}

#[pymethods]
impl Pool{
    #[new]
    pub fn new() -> Self {
        Self{
            v: Vec::new(),
            keys: Vec::new()
        }
    }

    pub fn push(&mut self, elem: PyObject) -> Py<PoolRef>{
        let this_index = self.v.len();
        let py_key: Py<PoolRef> = Python::with_gil(|py| {
            return Py::new(py, PoolRef::new(this_index));
        }).unwrap();

        self.v.push(elem);
        self.keys.push(py_key.clone());

        return py_key;
    }

    pub fn remove(&mut self, reference: Py<PoolRef>){
        let mut this_index = 0;
        Python::with_gil(|py| {
            this_index = reference.borrow(py).key.expect("using dropped key");
            reference.borrow_mut(py).key = None;
        });

        self.v.swap_remove(this_index);
        self.keys.swap_remove(this_index);
        
        if self.v.len() > this_index{
            Python::with_gil(|py| {
                self.keys[this_index].borrow_mut(py).key = Some(this_index);
            });
        }
    }

    pub fn get(&mut self, reference: Py<PoolRef>) -> PyObject{
        let mut this_index = 0;
        Python::with_gil(|py| {
            this_index = reference.borrow(py).key.unwrap();
        });

        self.v[this_index].clone()
    }

    pub fn __iter__(slf: PyRef<'_, Self>) -> PyResult<Py<PoolIter>>{
        let iter = PoolIter { inner: slf.v.clone().into_iter() };
        return Py::new(slf.py(), iter);
    }
}

#[pyclass]
pub struct PoolRef{
    key: Option<usize>,
}

impl PoolRef{
    fn new(key: usize) -> Self{
        Self {key: Some(key)}
    }

    pub fn is_removed(&self) -> bool{
        self.key.is_none()
    }
}

#[derive(Debug, Clone)]
#[pyclass]
pub struct PoolIter{
    inner: std::vec::IntoIter<PyObject>,
}

#[pymethods]
impl PoolIter{
    pub fn __iter__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
        slf
    }

    pub fn __next__(mut slf: PyRefMut<'_, Self>) -> Option<PyObject> {
        slf.inner.next()
    }
}