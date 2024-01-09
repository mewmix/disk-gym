extern crate clap;
use clap::{Arg, App};

mod sequentials;
mod random;

fn main() {
    let matches = App::new("Disk Gym")
        .version("1.0")
        .author("Alexander Klein")
        .about("Tests write and read speeds on a specified disk")
        .arg(Arg::with_name("FILE")
             .help("Sets the file path to perform the benchmark")
             .required(true)
             .index(1))
        .arg(Arg::with_name("size")
             .short("s")
             .long("size")
             .help("Sets the size of the file to write in MB")
             .takes_value(true))
        .arg(Arg::with_name("test_type")
             .short("t")
             .long("test_type")
             .help("Type of test to perform: sequential or random")
             .takes_value(true))
        .arg(Arg::with_name("buffer_size")
             .short("b")
             .long("buffer_size")
             .help("Buffer size for read/write operations in bytes")
             .takes_value(true))
        .arg(Arg::with_name("num_operations")
             .short("n")
             .long("num_operations")
             .help("Number of operations for random read/write tests")
             .takes_value(true))
        .get_matches();

    let file_path = matches.value_of("FILE").unwrap();
    let file_size = matches.value_of("size").unwrap_or("10").parse::<usize>().unwrap() * 1024 * 1024; // Default to 10 MB
    let test_type = matches.value_of("test_type").unwrap_or("sequentials");
    let buffer_size = matches.value_of("buffer_size").unwrap_or("4096").parse::<usize>().unwrap(); // Default to 4096 bytes
    let num_operations = matches.value_of("num_operations").unwrap_or("1000").parse::<usize>().unwrap(); // Default to 1000 operations

    match test_type {
        "sequentials" => {
            sequentials::sequential_write_test(file_path, file_size, buffer_size).unwrap();
            sequentials::sequential_read_test(file_path, buffer_size).unwrap();
        },
        "random" => {
            random::random_write_test(file_path, file_size, buffer_size, num_operations).unwrap();
            random::random_read_test(file_path, file_size, buffer_size, num_operations).unwrap();
        },
        _ => println!("Invalid test type specified. Please choose 'sequentials' or 'random'."),
    }
}
