use hagstrom_core::{
    action::{KeyCode, MouseAction, ScrollDirection, ScrollMagnitude},
    message, Emulator,
};
use lazy_static::lazy_static;
use num_enum::TryFromPrimitiveError;
use std::{
    ffi::CStr,
    sync::{Arc, Mutex},
    time::Duration,
};

lazy_static! {
    static ref SESSION_EMULATOR: Arc<Mutex<Option<Emulator>>> = Arc::new(Mutex::new(None));
    static ref EMULATORS: Arc<Mutex<Vec<Emulator>>> = Arc::new(Mutex::new(vec![]));
}

#[repr(C)]
enum ResponseCode {
    Ok = 0,
    Uninitialized = 1,
    DataFormatting = 2,
    DeviceNotFound = 3,
    LockPoisoned = 4,
}

#[no_mangle]
extern "C" fn initialize_emulator(serial_port: *const i8) -> ResponseCode {
    let serial_port = unsafe {
        match convert_c_str(serial_port) {
            Ok(data) => data,
            Err(response_code) => return response_code,
        }
    };

    match Emulator::new(serial_port) {
        Ok(emulator) => match SESSION_EMULATOR.lock().as_mut() {
            Ok(session_emulator) => {
                **session_emulator = Some(emulator);

                ResponseCode::Ok
            }
            Err(_) => ResponseCode::LockPoisoned,
        },
        Err(_) => ResponseCode::DeviceNotFound,
    }
}

// #[no_mangle]
// extern "C" fn initialize_emulator(serial_port: *const i8) -> ResponseCode {
//     let serial_port = unsafe {
//         match convert_c_str(serial_port) {
//             Ok(data) => data,
//             Err(response_code) => return response_code,
//         }
//     };

//     match Emulator::new(serial_port) {
//         Ok(emulator) => match EMULATORS.lock().as_mut() {
//             Ok(emulators) => {
//                 emulators.push(emulator);

//                 ResponseCode::Ok
//             }
//             Err(_) => ResponseCode::LockPoisoned,
//         },
//         Err(_) => ResponseCode::DeviceNotFound,
//     }
// }

fn send_packet<F>(packet_callback: F, sleep_duration: Duration) -> ResponseCode
where
    F: FnOnce() -> Result<Vec<u8>, ResponseCode>,
{
    let Ok(mut emulator_lock) = SESSION_EMULATOR.lock() else {
        return ResponseCode::LockPoisoned;
    };

    let Some(emulator) = emulator_lock.as_mut() else {
        return ResponseCode::Uninitialized;
    };

    let packet = match packet_callback() {
        Ok(packet) => packet,
        Err(response_code) => return response_code,
    };

    match emulator.write(packet, sleep_duration) {
        Ok(_) => ResponseCode::Ok,
        Err(_) => ResponseCode::DataFormatting,
    }
}

#[no_mangle]
extern "C" fn write_message(message: *const i8, sleep_duration: u64) -> ResponseCode {
    match SESSION_EMULATOR.lock() {
        Ok(mut emulator) => match emulator.as_mut() {
            Some(emulator) => {
                let data = unsafe {
                    match convert_c_str(message) {
                        Ok(data) => data,
                        Err(response_code) => return response_code,
                    }
                };
                let packet = message!(data);
                let duration = Duration::from_millis(sleep_duration);

                match emulator.write(packet, duration) {
                    Ok(_) => ResponseCode::Ok,
                    Err(_) => ResponseCode::DataFormatting,
                }
            }
            None => ResponseCode::Uninitialized,
        },
        Err(_) => ResponseCode::LockPoisoned,
    }
}

#[no_mangle]
extern "C" fn write_command(message: *const i8, sleep_duration: u64) -> ResponseCode {
    match SESSION_EMULATOR.lock() {
        Ok(mut emulator) => match emulator.as_mut() {
            Some(emulator) => {
                let duration = Duration::from_millis(sleep_duration);
                let data = unsafe {
                    match convert_c_str(message) {
                        Ok(data) => data,
                        Err(response_code) => return response_code,
                    }
                };

                let Ok(packet) = data
                    .chars()
                    .map(|char| KeyCode::try_from(char as u8))
                    .collect::<Result<Vec<KeyCode>, TryFromPrimitiveError<KeyCode>>>()
                    .map(|keycodes| hagstrom_core::action::key::create_command(keycodes)) else {
                     return ResponseCode::DataFormatting;
                };

                match emulator.write(packet, duration) {
                    Ok(_) => ResponseCode::Ok,
                    Err(_) => ResponseCode::DataFormatting,
                }
            }
            None => ResponseCode::Uninitialized,
        },
        Err(_) => ResponseCode::LockPoisoned,
    }
}

#[no_mangle]
extern "C" fn mouse_move(x: u16, y: u16, sleep_duration: u64) -> ResponseCode {
    println!("Move to: ({x}, {y})");

    ResponseCode::Ok
    //     match SESSION_EMULATOR.lock() {
    //         Ok(mut emulator) => {
    //             todo!()
    //         }
    //         Err(_) => ResponseCode::LockPoisoned,
    //     }
}

#[no_mangle]
extern "C" fn mouse_click(button: u8, sleep_duration: u64) -> ResponseCode {
    // let callback = || {
    // let Ok(button) =
    // }
}

#[no_mangle]
extern "C" fn mouse_scroll(direction: u8, magnitude: u8, sleep_duration: u64) -> ResponseCode {
    let callback = || -> Result<Vec<u8>, ResponseCode> {
        let Ok(direction) = ScrollDirection::try_from(direction) else {
            return Err(ResponseCode::DataFormatting);
        };

        let Ok(magnitude) = ScrollMagnitude::try_from(magnitude) else {
            return Err(ResponseCode::DataFormatting);
        };

        Ok(MouseAction::Scroll(direction, magnitude).as_packet())
    };

    send_packet(callback, Duration::from_millis(sleep_duration))
}

unsafe fn convert_c_str<'a>(buffer: *const i8) -> Result<&'a str, ResponseCode> {
    let c_str = unsafe { CStr::from_ptr(buffer) };
    match std::str::from_utf8(c_str.to_bytes()) {
        Ok(data) => Ok(data),
        Err(_) => Err(ResponseCode::DataFormatting),
    }
}

// fn emulator_mut<'a>() -> Result<&'a mut Emulator, ResponseCode> {
//     let Ok(mut emulator_lock) = SESSION_EMULATOR.lock() else {
//         return Err(ResponseCode::LockPoisoned);
//     };

//     emulator_lock.as_mut().ok_or(ResponseCode::Uninitialized)
// }
