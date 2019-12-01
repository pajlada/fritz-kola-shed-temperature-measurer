use std::fs::OpenOptions;
use std::io::prelude::*;
use std::path::Path;

fn main() {
    let device = Path::new("/dev/hidraw11");

    // let mut file = File::open(&device).unwrap();
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .open(&device)
        .unwrap();

    // let get_firmware_identifier : [u8; 8] = [0x01, 0x86, 0xff, 0x01, 0x00, 0x00, 0x00, 0x00];
    let get_temperature: [u8; 8] = [0x01, 0x80, 0x33, 0x01, 0x00, 0x00, 0x00, 0x00];

    file.write_all(&get_temperature).unwrap();

    let mut result: [u8; 8] = [0x00; 8];

    file.read_exact(&mut result).unwrap();

    let mut result_slice: [u8; 2] = [0x00; 2];
    result_slice.copy_from_slice(&result[2..4]);

    // let relevant_result : [u8; 4] = [result[2..4]];

    let temp = i16::from_be_bytes(result_slice);

    let temp_float = temp as f32 / 100.0;

    println!("firmware result: {:?} - {}", result, temp_float);

    println!(
        "HELLO WORLD TODAY ON THE STREAM WE ARE READING STUFF FROM {}",
        device.display()
    );
}
