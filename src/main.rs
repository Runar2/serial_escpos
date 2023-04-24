use std::{time::Duration, thread, io::{self, Write}};

use bytes::buf;
use serialport;





fn main() {
    let hex_dump = [29,40,65,2,0,49].as_slice();
    let cut_test = [29,86,66,10].as_slice();
    let output = "THIS is a test to see if anything works. 0123456789".as_bytes();
    let status_print = [29, 73, 69].as_slice();

    let ports = serialport::available_ports().expect("No ports found!");
    for p in ports {
        println!("{}", p.port_name);
    }
    let mut printer = serialport::new("/dev/ttyUSB0", 38400)
    .timeout(Duration::from_millis(10))
    .flow_control(serialport::FlowControl::Software)
    .parity(serialport::Parity::Even)
    .data_bits(serialport::DataBits::Eight)
    .stop_bits(serialport::StopBits::One)
    .open().expect("Failed to open port for some silly reason :-(");

    let port_name = printer.name().unwrap();
    let baud_rate = printer.baud_rate().unwrap();

    let mut printer_clone = printer.try_clone().expect("Failed to clone ");

    thread::spawn(move || loop {
        printer_clone
            .write_all(status_print)
            .expect("Failed to write to serial port");
        thread::sleep(Duration::from_millis(1000));
    });

    
            let mut serial_buf: Vec<u8> = vec![0; 1000];
            println!("Receiving data on {} at {} baud:", &port_name, &baud_rate);
            loop {
                match printer.read(serial_buf.as_mut_slice()) {
                    Ok(t) => io::stdout().write_all(&serial_buf[..t]).unwrap(),
                    Err(ref e) if e.kind() == io::ErrorKind::TimedOut => (),
                    Err(e) => eprintln!("{:?}", e),
                }
            }
        }
        
    

    

