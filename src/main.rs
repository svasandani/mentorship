use std::env;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

fn main() -> io::Result<()> {
    // read filename from args
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let f = File::open(filename)?;
    let mut reader = BufReader::new(f);
    let mut buffer = String::new();

    // read a line into buffer
    reader.read_line(&mut buffer)?;

    // panic for invalid syntax
    // TODO - fail gracefully
    let num_people: usize = buffer.trim().parse().unwrap();
    println!("{}", num_people);

    let mut group_a_preferences = Vec::with_capacity(num_people);

    let mut i = 0;
    while i < num_people {
        group_a_preferences.push(5);
        i = i + 1;
    }

    println!("{:?}", group_a_preferences);

    Ok(())
}
