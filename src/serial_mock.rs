use std::str;
use std::time::Duration;

// use serial::prelude::*;
use std::io::prelude::*;


struct SerialMock {
    name: String,
    rx_data: Vec<u8>,
}

impl SerialMock {
    pub fn open(name: &str) -> Self {
        Self {
            name: name.to_string(),
            rx_data: Vec::with_capacity(1024),
        }
    }

    /*
    pub fn set_timeout() {

    }
    */

    pub fn write(&mut self, buf: &[u8]) -> Result<usize, String> {
        Ok(buf.len())
    }

    pub fn read(&mut self, buf: &mut [u8]) -> Result<usize, String> {
        // TODO: Replace with proper buffer copy method
        // let mut i = 0;
        // for c in &self.rx_data {
        //     buf[i] = *c;
        //     i += 1;
        // }
        let i = self.rx_data.len();
        buf[0..i].copy_from_slice(&self.rx_data);
        self.rx_data.clear();

        Ok(i)
    }

    pub fn mock_rx_data(&mut self, data: &[u8]) {
        self.rx_data = data.to_vec();
    }
}

