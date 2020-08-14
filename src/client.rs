extern crate pyo3;
use pyo3::prelude::*;

use std::net::TcpStream;
use std::io::{Read, Write};

#[pyclass]
pub struct Client{
    connecter: TcpStream
}

#[pymethods]
impl Client{
    #[new]
    fn new(addr: String) -> Self{
        let connecter = TcpStream::connect(addr).unwrap();
        connecter.set_nonblocking(true).unwrap();
        Client{connecter}
    }

    pub fn send_message(&mut self, message: String){
        self.connecter.write(message.as_bytes()).unwrap();
    }

    pub fn read_message(&mut self) -> PyResult<String>{
        let mut buff = String::new();
        self.connecter.read_to_string(&mut buff).unwrap_or_default();

        Ok(buff.trim().to_string())
    }
}

#[pymodule]
pub fn client(_p: Python, m: &PyModule) -> PyResult<()>{
    m.add_class::<Client>()?;

    Ok(())
}