use serialport::SerialPort;
use std::{cell::RefCell, rc::Rc, thread, time::Duration};

pub mod error;
pub mod key_map;

use error::Result;
use key_map::KeyCode;

const FLUSH_BYTE: u8 = 0x38;

// #[test]
fn open_browser() -> Result<()> {
    let port = RefCell::new(
        serialport::new("COM3", 19_200)
            .timeout(Duration::from_millis(10))
            .open()?,
    );

    port.borrow_mut()
        .write(&command![KeyCode::Super, KeyCode::R])?;
    thread::sleep(Duration::from_millis(1000));

    port.borrow_mut().write(&message!("Firefox"))?;
    thread::sleep(Duration::from_millis(1000));

    port.borrow_mut().write(&command!(KeyCode::Enter))?;

    Ok(())
}

// #[test]
fn write_document_notepad() -> Result<()> {
    let port = RefCell::new(
        serialport::new("COM3", 19_200)
            .timeout(Duration::from_millis(10))
            .open()?,
    );

    let file_name = "temp.txt";
    let contents = std::fs::read_to_string(format!("./data/{file_name}"))?;
    let body = message!(&contents);

    let write_byte = |byte: &u8| -> Result<()> {
        port.borrow_mut().write(&vec![*byte])?;
        thread::sleep(Duration::from_micros(10));

        Ok(())
    };

    port.borrow_mut()
        .write(&command![KeyCode::Super, KeyCode::R])?;
    thread::sleep(Duration::from_millis(1000));

    port.borrow_mut().write(&message!("Notepad"))?;
    thread::sleep(Duration::from_millis(1000));

    port.borrow_mut().write(&command![KeyCode::Enter])?;
    thread::sleep(Duration::from_millis(1000));

    body.iter().try_for_each(write_byte)?;
    thread::sleep(Duration::from_millis(1000));

    port.borrow_mut()
        .write(&command![KeyCode::Control, KeyCode::S])?;
    thread::sleep(Duration::from_millis(1000));

    port.borrow_mut().write(&vec![FLUSH_BYTE])?;
    thread::sleep(Duration::from_millis(500));

    file_name.as_bytes().iter().try_for_each(write_byte)?;
    thread::sleep(Duration::from_millis(1000));

    port.borrow_mut().write(&command![KeyCode::Enter])?;
    thread::sleep(Duration::from_millis(1000));

    port.borrow_mut()
        .write(&command![KeyCode::Alt, KeyCode::F4])?;
    thread::sleep(Duration::from_millis(1000));

    Ok(())
}

// #[test]
fn login() -> Result<()> {
    let port = RefCell::new(
        serialport::new("COM3", 19_200)
            .timeout(Duration::from_millis(10))
            .open()?,
    );

    port.borrow_mut().write(&command!(KeyCode::Space))?;
    thread::sleep(Duration::from_millis(1000));

    port.borrow_mut().write(&message!("9009"))?;

    Ok(())
}

#[test]
fn write_document_gvim() -> Result<()> {
    let port = RefCell::new(
        serialport::new("COM3", 19_200)
            .timeout(Duration::from_millis(10))
            .open()?,
    );

    // let file_name = "google-docs-manifest-generator.js";
    let file_name = "temp.txt";
    let contents = std::fs::read_to_string(format!("./data/{file_name}"))?;
    let body = message!(&contents);

    let write_byte = |byte: &u8| -> Result<()> {
        port.borrow_mut().write(&vec![*byte])?;
        thread::sleep(Duration::from_millis(10));

        Ok(())
    };

    // Launch Gvim
    port.borrow_mut().write(&command!(KeyCode::Super))?;
    thread::sleep(Duration::from_millis(1000));

    port.borrow_mut().write(&message!("gvim"))?;
    thread::sleep(Duration::from_millis(1000));

    port.borrow_mut().write(&command![KeyCode::Enter])?;
    thread::sleep(Duration::from_millis(1000));

    // Open new file
    port.borrow_mut().write(&message!(":o "))?;
    thread::sleep(Duration::from_millis(1000));

    port.borrow_mut()
        .write(&message!(format!("{file_name}").as_str()))?;
    // file_name.as_bytes().iter().try_for_each(write_byte)?;
    thread::sleep(Duration::from_millis(1000));

    port.borrow_mut().write(&command!(KeyCode::Enter))?;
    thread::sleep(Duration::from_millis(1000));

    // Enter insert mode
    port.borrow_mut().write(&message!("i"))?;
    thread::sleep(Duration::from_millis(1000));

    // Write file
    body.iter().try_for_each(write_byte)?;
    thread::sleep(Duration::from_millis(1000));

    // Enter normal mode
    port.borrow_mut().write(&command!(KeyCode::Escape))?;
    thread::sleep(Duration::from_millis(1000));

    // Save and exit
    // Enter insert mode
    port.borrow_mut().write(&message!(":wq"))?;
    thread::sleep(Duration::from_millis(1000));

    port.borrow_mut().write(&command!(KeyCode::Enter))?;
    thread::sleep(Duration::from_millis(1000));

    Ok(())
}
