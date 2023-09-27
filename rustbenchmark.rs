use std::fs::File;
use std::io::{self, BufWriter, Read, Write, Seek, SeekFrom};
use std::time::Instant;

const DEVICE_PATH: &str = "/dev/mymem";
const NUM_TRIALS: usize = 10;
const ONE_BYTE: usize = 1;
const SIXTY_FOUR_BYTES: usize = 64;
const ONE_KB: usize = 1024;
const SIXTY_FOUR_KB: usize = 65536;
const FIVE_TWELVE_KB: usize = 524288;

// ...

fn benchmark(mut file: &File, size: usize) -> io::Result<()> {
    let mut buf = vec!['A' as u8; size];
    let mut writer = BufWriter::new(file);
    // Benchmark write
    let mut total_write_time = 0;
    for _ in 0..NUM_TRIALS {
        file.seek(SeekFrom::Start(0))?;
        let start = Instant::now();
        writer.write_all(&buf)?;
        let duration = start.elapsed().as_micros();
        total_write_time += duration;
        println!(
            "Size: {:6} bytes | Write time: {:>12} µs",
            size,
            duration
        );
    }

    // Benchmark read
    let mut total_read_time = 0;
    for _ in 0..NUM_TRIALS {
        file.seek(SeekFrom::Start(0))?;
        let start = Instant::now();
        file.read(&mut buf)?;
        let duration = start.elapsed().as_micros();
        total_read_time += duration;
        println!(
            "Size: {:6} bytes | Read time: {:>12} µs",
            size,
            duration
        );
    }

    println!(
        "Size: {:6} bytes | Avg write time: {:>12} µs | Avg read time: {:>12} µs",
        size,
        total_write_time / NUM_TRIALS as u128,
        total_read_time / NUM_TRIALS as u128
    );

    Ok(())
}


fn main() -> io::Result<()> {
    let file = File::options()
    .read(true)
    .write(true)
    .open(DEVICE_PATH)?;

    println!("Device opened successfully.");

    benchmark(&file, ONE_BYTE)?;
    benchmark(&file, SIXTY_FOUR_BYTES)?;
    benchmark(&file, ONE_KB)?;
    benchmark(&file, SIXTY_FOUR_KB)?;
    benchmark(&file, FIVE_TWELVE_KB)?;

    println!("Device closed successfully.");
    Ok(())
}
