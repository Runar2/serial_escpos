

use std::{io,env, str, time::Duration};
use futures::StreamExt;
use tokio_util::codec::{Decoder, Encoder};

use bytes::BytesMut;

use serialport::{self, SerialPortBuilder};
use tokio_serial::SerialPortBuilderExt;

use tokio;



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

impl Encoder<String> for LineCodec {
    type Error = io::Error;

    fn encode(&mut self, item: String, dst: &mut BytesMut) -> Result<(), Self::Error> {
        Ok(())
    }
}


#[tokio::main]
async fn main() -> tokio_serial::Result<()>{
    let mut args = env::args();
    let tty_path = args.nth(1).unwrap_or_else(|| DEFAULT_TTY.into());

    let mut port = tokio_serial::new(tty_path, 38400)
        .data_bits(serialport::DataBits::Eight)
        .flow_control(serialport::FlowControl::Software)
        .parity(serialport::Parity::Even)
        .stop_bits(serialport::StopBits::One)
        .timeout(Duration::from_millis(10))
        .open_native_async()?;

    #[cfg(unix)]
    port.set_exclusive(false).expect("Failed to to set port exclusive to false");

    let mut reader = LineCodec.framed(port);

    while let Some(line_result) = reader.next().await {
        let line = line_result.expect("Failed to read line :-(");
        println!("{}", line);
    }
    Ok(())
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

