use hagstrom_core::{
    action::{KeyCode, MouseAction},
    command,
    error::Result,
    message, Emulator,
};
use std::time::Duration;

fn main() -> Result<()> {
    let port = if cfg!(windows) {
        "COM3"
    } else {
        "/dev/ttyUSB0"
    };
    let mut emulator = Emulator::new(port)?;

    login(&mut emulator)?;
    write_document_gvim(&mut emulator)?;
    open_firefox(&mut emulator)?;

    mouse_test(&mut emulator)
}

fn login(emulator: &mut Emulator) -> Result<()> {
    emulator.write(command!(KeyCode::Space), Duration::from_millis(999))?;
    emulator.write(message!("9009"), Duration::from_millis(10000))?;

    Ok(())
}

fn open_firefox(emulator: &mut Emulator) -> Result<()> {
    emulator.write(
        command![KeyCode::Super, KeyCode::R],
        Duration::from_millis(1000),
    )?;
    emulator.write(message!("firefox"), Duration::from_millis(1000))?;
    emulator.write(command![KeyCode::Enter], Duration::ZERO)?;

    Ok(())
}

fn write_document_gvim(emulator: &mut Emulator) -> Result<()> {
    // let file_name = "google-docs-manifest-generator.js";
    let file_name = "temp.txt";
    let contents = std::fs::read_to_string(format!("./data/{file_name}"))?;

    let sleep_duration = Duration::from_millis(1000);
    // Launch Gvim
    emulator.write(command!(KeyCode::Super), sleep_duration)?;
    emulator.write(message!("gvim"), sleep_duration)?;
    emulator.write(command!(KeyCode::Enter), Duration::from_millis(5000))?;

    // Open new file
    emulator.write(message!(format!(":o {file_name}").as_str()), sleep_duration)?;
    emulator.write(command!(KeyCode::Enter), sleep_duration)?;

    // Write file
    emulator.write(message!("i"), sleep_duration)?;
    emulator.write(message!(&contents), sleep_duration)?;

    // Save and exit
    emulator.write(command!(KeyCode::Escape), sleep_duration)?;
    emulator.write(message!(":wq"), sleep_duration)?;
    emulator.write(command!(KeyCode::Enter), sleep_duration)?;

    Ok(())
}

fn mouse_test(emulator: &mut Emulator) -> Result<()> {
    let sleep_duration = Duration::from_millis(500);

    emulator.write(MouseAction::Move(100, 100).as_packet(), sleep_duration)?;
    emulator.write(MouseAction::LeftClick.as_packet(), sleep_duration)?;
    emulator.write(MouseAction::LeftClick.as_packet(), sleep_duration)?;

    Ok(())
}

// fn open_browser(port: Rc<RefCell<Box<dyn SerialPort>>>) -> Result<()> {
//     port.borrow_mut()
//         .write_all(&command![KeyCode::Super, KeyCode::R])?;
//     thread::sleep(Duration::from_millis(1000));

//     port.borrow_mut().write_all(&message!("Firefox"))?;
//     thread::sleep(Duration::from_millis(1000));

//     port.borrow_mut().write_all(&command!(KeyCode::Enter))?;

//     Ok(())
// }

// type Result<T> = std::result::Result<T, Error>;

// #[derive(Debug, Error)]
// enum Error {
//     #[error(transparent)]
//     Lib(#[from] usb2usb::error::Error),
//     #[error(transparent)]
//     Io(#[from] std::io::Error),
//     #[error(transparent)]
//     SerialPort(#[from] serialport::Error),

// }
