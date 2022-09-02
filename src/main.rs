use serialport::SerialPort;
use std::{
    cell::RefCell,
    rc::Rc,
    sync::{Arc, Condvar, Mutex},
    thread,
    time::Duration,
};
use thiserror::Error;

mod error;
mod key_map;

// use error::Result;
use key_map::KeyCode;

const FLUSH_BYTE: u8 = 0x38;

fn main() -> Result<()> {
    let port = Rc::new(RefCell::new(
        serialport::new("COM3", 19_200)
            .timeout(Duration::from_millis(10))
            .open()?,
    ));

    // Flag for validating completion of tasks
    let finished_pair = Arc::new((Mutex::new(false), Condvar::new()));

    // Listen for SIGINT, setting the finish flag and notifying the condition variable upon
    // receival
    let finished_pair_clone = Arc::clone(&finished_pair);
    let sig_handler = move || -> std::result::Result<(), ctrlc::Error> {
        let (lock, cvar) = &*finished_pair_clone;
        *lock.lock()? = true;
        cvar.notify_one();
        Ok(())
    };
    ctrlc::set_handler(sig_handler)?;

    // if let Err(err) = self.start() {
    //     self.shutdown()?;
    //     return Err(err);
    // }

    if let Err(err) = run(Rc::clone(&port)) {
        flush(Rc::clone(&port))?;
        return Err(err)?;
    }

    // Wait for the thread to finish
    let (lock, cvar) = &*finished_pair;
    let mut started = lock.lock().unwrap();
    while !*started {
        started = cvar.wait(started).unwrap();
    }

    flush(port)
}

fn run(port: Rc<RefCell<Box<dyn SerialPort>>>) -> Result<()> {
    login(Rc::clone(&port))?;
    thread::sleep(Duration::from_millis(10000));
    write_document_gvim(Rc::clone(&port))?;
    thread::sleep(Duration::from_millis(2000));


    open_browser(port)
}

fn flush(port: Rc<RefCell<Box<dyn SerialPort>>>) -> Result<()> {
    port.borrow_mut().write(&vec![0x38])?;

    Ok(())
}

fn open_browser(port: Rc<RefCell<Box<dyn SerialPort>>>) -> Result<()> {
    port.borrow_mut()
        .write(&command![KeyCode::Super, KeyCode::R])?;
    thread::sleep(Duration::from_millis(1000));

    port.borrow_mut().write(&message!("Firefox"))?;
    thread::sleep(Duration::from_millis(1000));

    port.borrow_mut().write(&command!(KeyCode::Enter))?;

    Ok(())
}

fn write_document_notepad(port: Rc<RefCell<Box<dyn SerialPort>>>) -> Result<()> {
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

fn login(port: Rc<RefCell<Box<dyn SerialPort>>>) -> Result<()> {
    port.borrow_mut().write(&command!(KeyCode::Space))?;
    thread::sleep(Duration::from_millis(1000));

    port.borrow_mut().write(&message!("9009"))?;

    Ok(())
}

fn write_document_gvim(port: Rc<RefCell<Box<dyn SerialPort>>>) -> Result<()> {
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

type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
enum Error {
    #[error(transparent)]
    Lib(#[from] usb2usb::error::Error),
    #[error(transparent)]
    Ctrlc(#[from] ctrlc::Error),
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    SerialPort(#[from] serialport::Error),
}
