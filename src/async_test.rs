use tokio::fs::File;
use tokio::io::{AsyncWriteExt, AsyncReadExt};
use tokio::task;
use std::time::Instant;
use log::{info, error};
use std::io;
use futures::future::join_all;

pub async fn async_write_test(file_path: &str, size: usize, buffer_size: usize) -> io::Result<()> {
    let data = vec![0u8; buffer_size];
    let num_operations = size / buffer_size;
    let start = Instant::now();
    let mut tasks = Vec::new();

    for _ in 0..num_operations {
        let mut file = File::create(file_path).await?;
        let data_clone = data.clone();

        let task = task::spawn(async move {
            file.write_all(&data_clone).await
        });

        tasks.push(task);
    }

    // Wait for all tasks to complete
    let results: Vec<_> = join_all(tasks).await.into_iter().collect();
    
    // Check results and handle errors
    for result in results {
        result??; // Double question mark to handle JoinError and io::Error
    }

    let duration = start.elapsed();
    let total_written = buffer_size * num_operations;
    let speed = (total_written as f64 / duration.as_secs_f64()) / (1024.0 * 1024.0);

    info!("Async write (concurrent): {:.2} MB/s", speed);
    Ok(())
}

pub async fn async_read_test(file_path: &str, size: usize, buffer_size: usize) -> io::Result<()> {
    let num_operations = size / buffer_size;
    let start = Instant::now();
    let mut tasks = Vec::new();

    for _ in 0..num_operations {
        let mut file = File::open(file_path).await?;
        let mut buffer = vec![0u8; buffer_size];

        let task = task::spawn(async move {
            file.read_exact(&mut buffer).await.map(|_| buffer.len())
        });

        tasks.push(task);
    }

    // Wait for all tasks to complete
    let results: Vec<_> = join_all(tasks).await.into_iter().collect();
    
    // Calculate total read and handle errors
    let mut total_read = 0;
    for result in results {
        total_read += result??; // Double question mark to handle JoinError and io::Error
    }

    let duration = start.elapsed();
    let speed = (total_read as f64 / duration.as_secs_f64()) / (1024.0 * 1024.0);

    info!("Async read (concurrent): {:.2} MB/s", speed);
    Ok(())
}
