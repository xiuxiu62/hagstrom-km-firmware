use crate::error::Result;
use serialport::SerialPort;
use std::{io::Write, process, thread, time::Duration};

const FLUSH: u8 = 0x38;

pub struct Emulator(Box<dyn SerialPort>);

impl Emulator {
    pub fn new(port_id: &str) -> Result<Self> {
        Ok(Self(
            serialport::new(port_id, 19_200)
                .timeout(Duration::from_millis(10))
                .open()?,
        ))
    }

    pub fn write(&mut self, packet: Vec<u8>, sleep_duration: Duration) -> Result<()> {
        match packet.len() {
            0..=16 => self.0.write_all(&packet)?,
            _ => self.write_large_packet(packet)?,
        };
        thread::sleep(sleep_duration);

        Ok(())
    }

    pub fn write_byte(&mut self, byte: u8) -> Result<()> {
        let _ = self.0.write(&[byte])?;
        thread::sleep(Duration::from_millis(10));

        Ok(())
    }

    pub fn flush(&mut self) -> Result<()> {
        self.0.write_all(&[FLUSH])?;

        Ok(())
    }

    fn write_large_packet(&mut self, packet: Vec<u8>) -> Result<()> {
        packet.chunks(16).try_for_each(|chunk| -> Result<()> {
            self.0.write_all(chunk)?;
            thread::sleep(Duration::from_millis(100));

            Ok(())
        })
    }
}

impl Drop for Emulator {
    fn drop(&mut self) {
        if self.flush().is_err() {
            eprintln!("Failed to flush key buffer");

            process::exit(1);
        }
    }
}
