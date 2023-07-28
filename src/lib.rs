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
    index_to_key_dict: Vec<PoolKey>,
    index_dict: Vec<usize>,
    usable_keys: Vec<PoolKey>
}

#[pymethods]
impl Pool{
    #[new]
    pub fn new() -> Self {
        Self{
            v: Vec::new(),
            index_to_key_dict: Vec::new(),
            index_dict: Vec::new(),
            usable_keys: Vec::new(),
        }
    }

    pub fn push(&mut self, elem: PyObject) -> PoolRef{
        let this_index = self.v.len();
        self.v.push(elem);

        if self.usable_keys.is_empty(){
            let this_key = PoolKey(this_index);

            self.index_dict.push(this_index);
            self.index_to_key_dict.push(this_key);

            return PoolRef::new(this_key);
        }else{
            let this_key = self.usable_keys.pop().unwrap();

            self.index_dict[this_key.0] = this_index;
            self.index_to_key_dict.push(this_key);
            
            return PoolRef::new(this_key);
        }
    }

    pub fn remove(&mut self, reference: &mut PoolRef){
        assert!(reference.key.is_some());

        let this_key = reference.key.unwrap();
        let this_index = self.index_dict[this_key.0];

        if self.v.len() == 1{
            self.usable_keys.push(this_key);
            self.v.pop().unwrap();
            return;
        }

        let last_index = self.v.len() - 1;
        let last_key = self.index_to_key_dict[last_index];

        self.v.swap_remove(this_index);
        self.index_to_key_dict.pop();

        self.index_dict[last_key.0] = this_index;
        self.index_to_key_dict[this_index] = last_key;
        self.usable_keys.push(this_key);
        reference.key = None
    }

    pub fn get(&mut self, reference: &mut PoolRef) -> PyObject{
        assert!(reference.key.is_some());

        let this_key = reference.key.unwrap();
        let this_index = self.index_dict[this_key.0];
        self.v[this_index].clone()
    }

    pub fn __iter__(slf: PyRef<'_, Self>) -> PyResult<Py<PoolIter>>{
        let iter = PoolIter { inner: slf.v.clone().into_iter() };
        return Py::new(slf.py(), iter);
    }

    /* 
    pub fn log(&self){
        println!("{:?}", self.v);
        println!("index_dict: {:?}", self.index_dict);
        println!("index_to_key_dict: {:?}", self.index_to_key_dict);
        println!("usable_keys: {:?}", self.usable_keys);
    }*/
}

#[derive(Debug, Clone, Copy)]
struct PoolKey(usize);

#[pyclass]
pub struct PoolRef{
    key: Option<PoolKey>,
}

impl PoolRef{
    fn new(key: PoolKey) -> Self{
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