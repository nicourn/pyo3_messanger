extern crate pyo3;
use pyo3::prelude::*;

use std::net::{TcpListener, TcpStream, SocketAddr};
use std::io::{Read, Write};
use std::str::from_utf8;
use std::{time, thread};

#[pyclass]
pub struct Server{
    listener: TcpListener,
    clients: Vec<(TcpStream, SocketAddr)>
}

#[pymethods]
impl Server{
    #[new]
    fn new() -> Self{
        Server{
            listener: TcpListener::bind("0.0.0.0:9090").unwrap(),
            clients: Vec::new()
        }
    }

    pub fn wait_client(&mut self, message: String, lim: usize){
        println!("Waiting...");
        while self.clients.len() < lim{
            let mut client = self.listener.accept().unwrap();
            println!("New client: {}", client.1);
            client.0.write(message.as_bytes()).unwrap();
            client.0.set_nonblocking(true).unwrap();
            self.clients.push(client)
        }
    }

    pub fn messaging(&mut self){
        let mut messages = Vec::new();
        let mut black_lists = Vec::new();
        loop {
            for client in self.clients.iter_mut(){
                let mut buff = [0; 256];
                match client.0.read(&mut buff){
                    Ok(size_message) => {
                        if buff.len() != 0 {
                            messages.push(Vec::from(&buff[..size_message]));
                            black_lists.push([client.1]);
                        }
                    }
                    
                    Err(_) => {}
                };
            }
            for (message, black_list) in messages.iter().zip(black_lists.iter()){
                self.send_message(message, black_list);

            }
            messages.clear();
            black_lists.clear();
            thread::sleep(time::Duration::from_secs_f32(0.1));
        }
        
    }

}

impl Server{
    fn send_message(&mut self, message: &[u8], black_list: &[SocketAddr]){
        for client in self.clients.iter_mut(){
            if !black_list.contains(&client.1){
                client.0.write(message).unwrap();
            }
        }
    }
}

///This module containe Server class
#[pymodule]
pub fn server(_p: Python, m: &PyModule) -> PyResult<()>{
    m.add_class::<Server>()?;

    Ok(())
}