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
    let mut num_people_string = String::new();
    // throwaway string for better formatting in input text (fix this?)
    let mut throwaway = String::new();

    // read a line into buffer
    reader.read_line(&mut num_people_string)?;

    // panic for invalid syntax
    // TODO - fail gracefully
    let num_people: usize = num_people_string.trim().parse().unwrap();
    println!("{}", num_people);

    // read group A prefs

    reader.read_line(&mut throwaway)?;

    let mut group_a_preferences: Vec<i32> = Vec::with_capacity(num_people * num_people);

    let mut a = 0;
    // for each person in group A
    while a < num_people {
        let mut prefs_string = String::new();

        reader.read_line(&mut prefs_string)?;

        let prefs = prefs_string.split(" ");

        // record their preferences for people in group B
        for p in prefs {
            group_a_preferences.push(p.trim().parse().unwrap())
        }

        a = a + 1;
    }

    println!("{:?}", group_a_preferences);

    // repeat the above for group B

    reader.read_line(&mut throwaway)?;

    let mut group_b_preferences: Vec<i32> = Vec::with_capacity(num_people * num_people);

    let mut b = 0;
    // for each person in group B
    while b < num_people {
        let mut prefs_string = String::new();

        reader.read_line(&mut prefs_string)?;

        let prefs = prefs_string.split(" ");

        // record their preferences for people in group A
        for p in prefs {
            group_b_preferences.push(p.trim().parse().unwrap())
        }

        b = b + 1;
    }

    println!("{:?}", group_b_preferences);

    println!("{:?}", stable_match(num_people, group_a_preferences, group_b_preferences));

    Ok(())
}

fn stable_match(num_people: usize, group_a: Vec<i32>, group_b: Vec<i32>) -> Vec<i32> {
    Vec::with_capacity(num_people * num_people)
}
