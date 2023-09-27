use std::fs::File;
use std::io::Read;
use std::io::{self, BufWriter, Write, Seek, SeekFrom};

const DEVICE_PATH: &str = "/dev/mymem";

fn main() -> io::Result<()> {
    let mut file = File::options()
    .read(true)
    .write(true)
    .open(DEVICE_PATH)?;


    println!("Device opened successfully.");
    {
        let mut writer = BufWriter::new(&file);
        writer.write_all(b"f bruh")?;
    } // writer goes out of scope and flushes automatically

    file.seek(SeekFrom::Start(0))?;
    let mut buffer = [0; 18];
    let out = file.read(&mut buffer)?;

    let result_str = String::from_utf8(buffer[..out].to_vec()).unwrap();

    println!("Received from kernel: {}", result_str);

    Ok(())
}
