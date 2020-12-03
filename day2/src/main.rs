/**
--- Day 2: Password Philosophy ---

Your flight departs in a few days from the coastal airport; the easiest way down to the coast from
here is via toboggan.

The shopkeeper at the North Pole Toboggan Rental Shop is having a bad day. "Something's wrong with
our computers; we can't log in!" You ask if you can take a look.

Their password database seems to be a little corrupted: some of the passwords wouldn't have been
allowed by the Official Toboggan Corporate Policy that was in effect when they were chosen.

To try to debug the problem, they have created a list (your puzzle input) of passwords (according to
the corrupted database) and the corporate policy when that password was set.

For example, suppose you have the following list:

1-3 a: abcde
1-3 b: cdefg
2-9 c: ccccccccc

Each line gives the password policy and then the password. The password policy indicates the lowest
and highest number of times a given letter must appear for the password to be valid. For example,
1-3 a means that the password must contain a at least 1 time and at most 3 times.

In the above example, 2 passwords are valid. The middle password, cdefg, is not; it contains no
instances of b, but needs at least 1. The first and third passwords are valid: they contain one a or
nine c, both within the limits of their respective policies.

How many passwords are valid according to their policies?
*/
mod parser;

fn part_one() {
    match parser::read_lines("input") {
        Ok(lines) => {
            let mut num_ok: i32 = 0;
            for line in lines {
                if let Ok(entry) = line {
                    // Parse line
                    let mut tokens = entry.split(|c| c == '-' || c == ':' || c == ' ');
                    let min = tokens.next().unwrap().parse::<i32>().unwrap();
                    let max = tokens.next().unwrap().parse::<i32>().unwrap();
                    let c = tokens.next().unwrap().chars().next().unwrap();
                    tokens.next();
                    let password = tokens.next().unwrap();

                    // Check if password passes, without standard library
                    let mut count: i32 = 0;
                    for d in password.chars() {
                        if d == c {
                            count += 1;
                        }
                    }

                    if count >= min && count <= max {
                        // println! {"{} OK", entry};
                        num_ok += 1;
                    } else {
                        // println! {"{} FAIL", entry};
                    }
                }
            } // end password list

            println! {"Number of passwords OK: {}", num_ok};
        }
        _ => {
            println!("Error: Could not read lines!")
        }
    }
}

fn part_two() {
    match parser::read_lines("input") {
        Ok(lines) => {
            let mut num_ok: i32 = 0;
            for line in lines {
                if let Ok(entry) = line {
                    // println! {"Parsing password: {}", entry};
                    // Parse line
                    let mut tokens = entry.split(|c| c == '-' || c == ':' || c == ' ');

                    // These are 1-based!
                    let pos0 = tokens.next().unwrap().parse::<i32>().unwrap() - 1;
                    let pos1 = tokens.next().unwrap().parse::<i32>().unwrap() - 1;
                    let c = tokens.next().unwrap().chars().next().unwrap() as u8;
                    tokens.next();
                    let password = tokens.next().unwrap();

                    // Index the string as ASCII bytes
                    let chars = password.as_bytes();

                    let has_pos0 = chars[pos0 as usize] == c;
                    let has_pos1 = chars[pos1 as usize] == c;
                    let valid = has_pos0 ^ has_pos1;
                    if valid {
                        num_ok += 1;
                    }
                }
            } // end password list

            println! {"Number of passwords OK: {}", num_ok};
        }
        _ => {
            println!("Error: Could not read lines!")
        }
    }
}

fn main() {
    println!("=== Advent of Code Day 1 ===");
    part_one();
    part_two();
}
