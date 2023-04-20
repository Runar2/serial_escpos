

use std::{io, str};
use tokio_util::codec::{Decoder};

use bytes::BytesMut;

use serialport::{self, SerialPortBuilder};
use tokio_serial::SerialPortBuilderExt;



#[cfg(unix)]
const DEFAULT_TTY: &str = "/dev/ttyUSB0";

struct LineCodec;

impl Decoder for LineCodec {
    type Item = String;
    type Error = io::Error;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        let newline = src.as_ref().iter().position(|b| *b == b'\n');
        if let Some(n) = newline {
            let line = src.split_to(n+1);
            return match str::from_utf8(line.as_ref()) {
                Ok(s) => Ok(Some(s.to_string())),
                Err(_) => Err(io::Error::new(io::ErrorKind::Other, "Invalid String")),
            };
        }
        Ok(None)
    }
}


fn main(){
    
}

fn list_ports(){
    let available_ports = serialport::available_ports()
        .expect("Failed to read available ports");
    for port in &available_ports{
        println!("Found port {}", port.port_name);
    }
    if available_ports.is_empty(){
        println!("No available ports found :-(");
    }
}

fn print_and_cut(_printer_read: SerialPortBuilder, _content: String, _cut: bool){
}

fn please_work(){
    let _async_printer = tokio_serial::new("/dev/ttyUSB0", 38400).open_native_async();
}