use serialport::SerialPort;
use std::{thread, time::Duration};

mod key_map;

use key_map::KeyCode;

type DynResult<T> = Result<T, Box<dyn std::error::Error>>;

const FLUSH_BYTE: u8 = 0x38;

fn main() -> DynResult<()> {
    let mut port = serialport::new("COM3", 19_200)
        .timeout(Duration::from_millis(10))
        .open()?;

    write_document(&mut port)
}

fn main_back() -> DynResult<()> {
    let mut port = serialport::new("COM3", 19_200)
        .timeout(Duration::from_millis(10))
        .open()?;

    let windows_r = command![KeyCode::Super, KeyCode::R];
    let chrome = message!("Chrome");
    let firefox = message!("Firefox");
    // let chrome = message![KeyCode::C, KeyCode::H, KeyCode::R, KeyCode::O, KeyCode::M, KeyCode::E];
    // let firefox = message![KeyCode::F, KeyCode::I, KeyCode::R, KeyCode::E, KeyCode::F, KeyCode::O, KeyCode::X];
    let enter = command![KeyCode::Enter];

    port.write(&windows_r)?;
    thread::sleep(Duration::from_millis(1000));
    port.write(&firefox)?;
    thread::sleep(Duration::from_millis(1000));
    port.write(&enter)?;

    Ok(())
}

fn write_document(port: &mut Box<dyn SerialPort>) -> DynResult<()> {
    let contents = std::fs::read_to_string("./data/google-docs-manifest-generator.js")?;
    // let contents = std::fs::read_to_string("./data/temp.js")?;
    let body = message!(&contents);

    port.write(&command![KeyCode::Super, KeyCode::R])?;
    thread::sleep(Duration::from_millis(1000));
    port.write(&message!("Notepad"))?;
    thread::sleep(Duration::from_millis(1000));
    port.write(&command![KeyCode::Enter])?;
    thread::sleep(Duration::from_millis(1000));
    for (i, byte) in body.iter().enumerate() {
        if i % 2 == 0 {
            port.write(&vec![FLUSH_BYTE])?;
        }

        port.write(&vec![*byte])?;
        thread::sleep(Duration::from_millis(1));
    }

    Ok(())
}
