//sequentials.rs
use std::fs::File;
use std::io::{Write, BufWriter, self, BufReader, Read};
use std::time::Instant;

use indicatif::{ProgressBar, ProgressStyle};

pub fn sequential_write_test(file_path: &str, size: usize, buffer_size: usize) -> io::Result<()> {
    let file = File::create(file_path)?;
    let mut writer = BufWriter::new(file);
    let data = vec![0u8; buffer_size];
    let mut total_written = 0usize;
    let start = Instant::now();

    let progress_bar = ProgressBar::new(size as u64);
    progress_bar.set_style(ProgressStyle::default_bar()
        .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({eta})")
        .progress_chars("#>-"));

    while total_written < size {
        writer.write_all(&data)?;
        total_written += buffer_size;
        progress_bar.set_position(total_written as u64);
    }

    progress_bar.finish_with_message("Write complete");

    let duration = start.elapsed();
    println!("Sequential write: {:.2} MB/s", (size as f64 / duration.as_secs_f64()) / (1024.0 * 1024.0));
    Ok(())
}

pub fn sequential_read_test(file_path: &str, buffer_size: usize) -> io::Result<()> {
    let file = File::open(file_path)?;
    let file_size = file.metadata()?.len() as usize; // Get file size before creating BufReader

    let mut reader = BufReader::new(file);
    let mut buffer = vec![0u8; buffer_size];
    let mut total_read = 0usize;
    let start = Instant::now();

    let progress_bar = ProgressBar::new(file_size as u64);
    progress_bar.set_style(ProgressStyle::default_bar()
        .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({eta})")
        .progress_chars("#>-"));

    loop {
        let bytes_read = reader.read(&mut buffer)?;
        if bytes_read == 0 {
            break; // End of file reached
        }
        total_read += bytes_read;
        progress_bar.set_position(total_read as u64);
    }

    progress_bar.finish_with_message("Read complete");

    let duration = start.elapsed();
    println!("Sequential read: {:.2} MB/s", (total_read as f64 / duration.as_secs_f64()) / (1024.0 * 1024.0));
    Ok(())
}


