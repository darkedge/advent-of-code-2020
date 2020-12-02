/*
--- Day 1: Report Repair ---

After saving Christmas five years in a row, you've decided to take a vacation at a nice resort on a
tropical island. Surely, Christmas will go on without you.

The tropical island has its own currency and is entirely cash-only. The gold coins used there have a
little picture of a starfish; the locals just call them stars. None of the currency exchanges seem
to have heard of them, but somehow, you'll need to find fifty of these coins by the time you arrive
so you can pay the deposit on your room.

To save your vacation, you need to get all fifty stars by December 25th.

Collect stars by solving puzzles. Two puzzles will be made available on each day in the Advent
calendar; the second puzzle is unlocked when you complete the first. Each puzzle grants one star.
Good luck!

Before you leave, the Elves in accounting just need you to fix your expense report (your puzzle
input); apparently, something isn't quite adding up.

Specifically, they need you to find the two entries that sum to 2020 and then multiply those two
numbers together.

For example, suppose your expense report contained the following:

1721
979
366
299
675
1456

In this list, the two entries that sum to 2020 are 1721 and 299. Multiplying them together produces
1721 * 299 = 514579, so the correct answer is 514579.

Of course, your expense report is much larger. Find the two entries that sum to 2020; what do you
get if you multiply them together?
*/

/**
 * Ok, so: initial ideas
 * 1. Brute force - sum every x with y
 * 2. Second list with 2020 - x, check for duplicates
 * 3. a. Sort the list: start with two pointers at the low end of the list
 *    b. increment first pointer and check if sum is 2020
 *    c. keep incrementing first pointer until overshoot
 *    d. increment second pointer once, start decrementing first pointer until undershoot
 *    e. keep going until we find 2020
 *
 * I like 2 best, for now.
 * Note: We know what the data looks like! We can and should that that to our advantage.
 * Of course, we could parse the data offline and not even touch code, but that's no fun.
 *
 * From a quick glance it looks like most numbers are over 1010. This reduces the amount of entries
 * that we need to take a look at.
 * But bah, computers are so fast, even brute forcing this list should literally be done
 * in a millisecond.
 */
use std::fs::File;

// use std::io::{self, BufRead};
use std::io;
use std::io::BufRead;

use std::path::Path;

fn main() {
    println!("=== Advent of Code Day 1 ===");
    match read_lines("input") {
        Ok(lines) => {
            // Lines is an iterator, we don't know how many lines there are
            // because we parse as we go.
            for line in lines {
                if let Ok(entry) = line {
                    if let Ok(parsed_entry) = entry.parse::<i32>() {
                        println!("{}", parsed_entry);
                    }
                }
            }
        }
        _ => {
            println!("Error: Could not read lines!")
        }
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>, // Trait bound: P must be convertible to &Path
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
