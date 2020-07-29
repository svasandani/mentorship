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

    // store prefs in a 2d Vec
    let mut group_a_preferences: Vec<usize> = Vec::with_capacity(num_people * num_people);

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

    // store prefs in a 2d Vec
    let mut group_b_preferences: Vec<usize> = Vec::with_capacity(num_people * num_people);

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

fn stable_match(num_people: usize, group_a_preferences: Vec<usize>, group_b_preferences: Vec<usize>) -> Vec<usize> {
    let max = usize::MAX;

    // store group_a's provisional engagements (we eventually return this)
    let mut provisional_matches: Vec<usize> = vec![max; num_people];

    // store group_b's offers in a similar array
    let mut group_b_offers: Vec<usize> = vec![max; num_people];

    let mut round = 0;

    // while there are unmatched members in group A
    while (provisional_matches.contains(&max)) {
        // each member of group A proposes a match to its most preferred
        let mut a = 0;
        // for each person in group A
        while a < num_people {
            // get the highest ranking preference so far
            // and offer them a match
            let a_best_match = group_a_preferences[a * num_people + round];
            let b_current_best_offer = group_b_offers[a_best_match];

            // if the person in group B has had no offer
            if b_current_best_offer == max {
                // we provisionally match the two
                provisional_matches[a] = a_best_match;
                group_b_offers[a_best_match] = a;
            } else {
                // otherwise
                // we see if A is better than B's current best offer
                let mut better: bool = false;
                let mut i = a_best_match * num_people;

                // for each of B's preferences
                while i <  (a_best_match + 1) * num_people {
                    // if we reach A first, then A is better
                    if group_b_preferences[i] == a {
                        better = true;
                        break;
                    } else if group_b_preferences[i] == group_b_offers[a_best_match] {
                        // but if we reach the current match, then A is not better
                        better = false;
                        break;
                    }
                }

                // if A is better 
                if better {
                    // unmatch the previous best_match
                    provisional_matches[group_b_offers[a_best_match]] = max;
                    // match the current best_match
                    provisional_matches[a] = a_best_match;
                    group_b_offers[a_best_match] = a;
                }
                // otherwise do nothing
                
            }

            a = a + 1;
        }

        round = round + 1;
    }

    // once there are no more -1 values, we have our stable match
    return provisional_matches;
}
