use hagstrom_core::{action::KeyCode, message, Emulator};
use lazy_static::lazy_static;
use num_enum::TryFromPrimitiveError;
use std::{
    ffi::CStr,
    sync::{Arc, Mutex},
    time::Duration,
};

lazy_static! {
    static ref SESSION_EMULATOR: Arc<Mutex<Option<Emulator>>> = Arc::new(Mutex::new(None));
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
                let data = unsafe {
                    match convert_c_str(message) {
                        Ok(data) => data,
                        Err(response_code) => return response_code,
                    }
                };
                let packet = match data
                    .chars()
                    .map(|char| KeyCode::try_from(char as u8))
                    .collect::<Result<Vec<KeyCode>, TryFromPrimitiveError<KeyCode>>>()
                {
                    Ok(keycodes) => hagstrom_core::action::key::create_command(keycodes),
                    Err(_) => return ResponseCode::DataFormatting,
                };
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

unsafe fn convert_c_str<'a>(buffer: *const i8) -> Result<&'a str, ResponseCode> {
    let c_str = unsafe { CStr::from_ptr(buffer) };
    match std::str::from_utf8(c_str.to_bytes()) {
        Ok(data) => Ok(data),
        Err(_) => Err(ResponseCode::DataFormatting),
    }
}
