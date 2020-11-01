mod at;
mod serial_mock;

use regex::{Regex, RegexBuilder};
use std::str;
use std::time::Duration;

use serial::prelude::*;
use std::io::prelude::*;


fn main() {
    println!("Hello, world!");

    // test2();
    // test3();
}

/*
fn test2() {
    /*
    ATI
    TOBY-L210-03S-01

    OK
    */
    let mut port = SerialMock::open("/dev/dummy");
    port.write(b"ATI").unwrap();

    port.mock_rx_data(b"ATI\r\nTOBY-L210-03S-01\r\nOK\r\n");

    let mut read_buffer = [0u8; 1024];
    let bytes = port.read(&mut read_buffer[..]).unwrap();
    println!("got {} bytes", bytes);
    let text = str::from_utf8(&read_buffer[0..bytes]).unwrap();
    println!("content: {}", text);
}
*/

fn test3() {
    let mut port: serial::SystemPort = serial::open("/dev/ttyACM0").unwrap();
/*
    let settings = serial::PortSettings {
        baud_rate: serial::BaudRate::from_speed(115200),
        char_size: serial::Bits8,
        parity: serial::ParityNone,
        stop_bits: serial::Stop1,
        flow_control: serial::FlowNone,
    };
    port.configure(&settings).unwrap();
*/
    port.set_timeout(Duration::from_millis(1000)).unwrap();
    // port.flush();

    port.write(b"ATI\n").unwrap();

    loop {
        let mut read_buffer = [0u8; 1024];
        // let bytes = serial_port.read(&mut read_buffer[..]).ok();
        if let Ok(bytes_read) = port.read(&mut read_buffer[..]) {
            let data = read_buffer[0..bytes_read].to_vec();
            println!("got {} bytes", bytes_read);
            println!("content: {:?}", str::from_utf8(&read_buffer[0..bytes_read]).unwrap());
        }
        else {
//            println!("no data");
        }
    }
}
