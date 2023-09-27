use std::fs::File;
use std::io::{self, BufWriter, Write, Read, Seek, SeekFrom};

const DEVICE_PATH: &str = "/dev/mymem";

fn main() -> io::Result<()> {
    let mut file = File::options()
        .read(true)
        .write(true)
        .open(DEVICE_PATH)?;

    println!("Device opened successfully.");

    // First write
    {
        let mut writer = BufWriter::new(&file);
        writer.write_all(b"First write: Hello, kernel!")?;
    } // writer goes out of scope and flushes automatically

    file.seek(SeekFrom::Start(0))?;
    let mut buffer = [0; 256];
    let out = file.read(&mut buffer)?;
    let result_str = String::from_utf8(buffer[..out].to_vec()).unwrap();
    println!("Received from kernel after first write: {}", result_str);

    {
        let message = b"Second write: Hello again, kernel!";
        let mut writer = BufWriter::new(&file);
        writer.write_all(message)?;
        println!("Written message: {}", String::from_utf8_lossy(message));
    }
    
    file.seek(SeekFrom::Start(0))?;
    let outp = file.read(&mut buffer)?;

    let result_str2 = String::from_utf8(buffer[..outp].to_vec()).unwrap();
    println!("Received from kernel after second write: {}", result_str2);

    Ok(())
}
