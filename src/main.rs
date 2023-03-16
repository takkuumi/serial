use std::{thread, time::Duration};

use anyhow::Context;
use anyhow::Ok;
use serialport::{DataBits, FlowControl, StopBits};

fn main() {
    // if let Err(err) = set_ndid() {
    //     eprintln!("Error: {:#}", err)
    // }

    if let Err(err) = read_ndid() {
        eprintln!("Error: {:#}", err)
    }

    if let Err(err) = send_serialport() {
        eprintln!("Error: {:#}", err)
    }
}

pub fn send_serialport() -> anyhow::Result<()> {
    let mut port = serialport::new("/dev/tty.usbserial-14340", 115200)
        .data_bits(DataBits::Eight)
        .stop_bits(StopBits::One)
        .flow_control(FlowControl::None)
        .timeout(core::time::Duration::from_secs(10))
        .open()
        .context("Failed to open serial port")?;

    let mut i: u16 = 0;
    loop {
        let mut data = Vec::<u8>::new();

        data.extend_from_slice("AT+NDRPT=0001,13,".as_bytes());

        data.extend_from_slice(&i.to_be_bytes());

        data.extend_from_slice("010100000001\r\n".as_bytes());

        if let core::result::Result::Ok(_s) = port.write(&data) {
            let mut buffer = [0; 200];

            if let core::result::Result::Ok(size) = port.read(&mut buffer) {
                let resp = String::from_utf8_lossy(&buffer);
                if resp.contains("DATA") {
                    eprintln!("response:{},INDEX:{}", resp, i);
                }
            }
        }
        if i == 10000 {
            break;
        }
        i += 1;
        thread::sleep(Duration::from_secs(1));
    }

    // let size = port
    //     .write("AT+MODE=2\r\n".as_bytes())
    //     .expect("Failed to write to serial port");
    // eprintln!("{}", size);

    // let size = port
    //     .write("AT+NDID=0002\r\n".as_bytes())
    //     .expect("Failed to write to serial port");
    // eprintln!("{}", size);

    // let size = port
    //     .write("AT+NDRESET\r\n".as_bytes())
    //     .expect("Failed to write to serial port");
    // eprintln!("{}", size);

    // let size = port
    //     .write("AT+REBOOT\r\n".as_bytes())
    //     .expect("Failed to write to serial port");
    // eprintln!("{}", size);

    // let size = port
    //     .write("AT+NDID\r\n".as_bytes())
    //     .expect("Failed to write to serial port");
    // eprintln!("{}", size);

    // Read the four bytes back from the cloned port

    Ok(())
}

pub fn set_ndid() -> anyhow::Result<()> {
    let mut port = serialport::new("/dev/tty.usbserial-14340", 115200)
        .data_bits(DataBits::Eight)
        .stop_bits(StopBits::One)
        .flow_control(FlowControl::None)
        .timeout(core::time::Duration::from_secs(10))
        .open()
        .context("Failed to open serial port")?;

    let size = port
        .write("AT+NDID=0002\r\n".as_bytes())
        .expect("Failed to write to serial port");
    eprintln!("{}", size);

    let size = port
        .write("AT+NDRESET\r\n".as_bytes())
        .expect("Failed to write to serial port");
    eprintln!("{}", size);

    let size = port
        .write("AT+REBOOT\r\n".as_bytes())
        .expect("Failed to write to serial port");
    eprintln!("{}", size);

    // Read the four bytes back from the cloned port
    let mut buffer = [0; 200];
    let res = port.read(&mut buffer)?;
    eprintln!("res size {}", res);

    let res2 = String::from_utf8_lossy(&buffer);
    eprintln!("res2 {}", res2);

    Ok(())
}

fn read_ndid() -> anyhow::Result<()> {
    let mut port = serialport::new("/dev/tty.usbserial-14340", 115200)
        .data_bits(DataBits::Eight)
        .stop_bits(StopBits::One)
        .flow_control(FlowControl::None)
        .timeout(core::time::Duration::from_secs(10))
        .open()
        .context("Failed to open serial port")?;

    let size = port
        .write("AT+NDID\r\n".as_bytes())
        .expect("Failed to write to serial port");
    eprintln!("{}", size);

    // Read the four bytes back from the cloned port
    let mut buffer = [0; 200];
    let res = port.read(&mut buffer)?;
    eprintln!("res size {}", res);

    let res2 = String::from_utf8_lossy(&buffer);
    eprintln!("res2 {}", res2);

    Ok(())
}
