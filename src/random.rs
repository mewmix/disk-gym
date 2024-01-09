//random.rs
use std::fs::File;
use std::io::{self, Seek, SeekFrom, Write, Read};
use std::time::Instant;
use rand::Rng;

use indicatif::{ProgressBar, ProgressStyle};

pub fn random_write_test(file_path: &str, file_size: usize, buffer_size: usize, num_operations: usize) -> io::Result<()> {
    let mut file = File::create(file_path)?;
    let data = vec![0u8; buffer_size];
    let mut rng = rand::thread_rng();
    let mut total_time = 0u128;
    let mut total_written = 0usize;

    let progress_bar = ProgressBar::new(num_operations as u64);
    progress_bar.set_style(ProgressStyle::default_bar()
        .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta})")
        .progress_chars("#>-"));

    for _ in 0..num_operations {
        let pos = rng.gen_range(0..file_size) as u64;
        file.seek(SeekFrom::Start(pos))?;

        let start = Instant::now();
        file.write_all(&data)?;
        total_time += start.elapsed().as_micros();
        total_written += buffer_size;

        progress_bar.inc(1);
    }

    progress_bar.finish_with_message("Random write complete");

    let write_speed = (total_written as f64 / 1024.0 / 1024.0) / (total_time as f64 / 1_000_000.0);
    println!("Random write speed: {:.2} MB/s", write_speed);
    Ok(())
}



pub fn random_read_test(file_path: &str, file_size: usize, buffer_size: usize, num_operations: usize) -> io::Result<()> {
    let mut file = File::open(file_path)?;
    let mut buffer = vec![0u8; buffer_size];
    let mut rng = rand::thread_rng();
    let mut total_time = 0u128;
    let mut total_read = 0usize;

    let progress_bar = ProgressBar::new(num_operations as u64);
    progress_bar.set_style(ProgressStyle::default_bar()
        .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta})")
        .progress_chars("#>-"));

    for _ in 0..num_operations {
        let pos = rng.gen_range(0..file_size - buffer_size) as u64;
        file.seek(SeekFrom::Start(pos))?;

        let start = Instant::now();
        match file.read(&mut buffer) {
            Ok(bytes_read) => {
                total_time += start.elapsed().as_micros();
                total_read += bytes_read;
            },
            Err(e) if e.kind() == io::ErrorKind::UnexpectedEof => break, // Stop if EOF is reached
            Err(e) => return Err(e),
        }

        progress_bar.inc(1);
    }

    progress_bar.finish_with_message("Random read complete");

    let read_speed = (total_read as f64 / 1024.0 / 1024.0) / (total_time as f64 / 1_000_000.0);
    println!("Random read speed: {:.2} MB/s", read_speed);
    Ok(())
}

