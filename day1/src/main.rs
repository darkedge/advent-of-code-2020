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

// Okay, this use keyword makes sense, like importing a namespace.
use std::fs::File;
// Uhhhh.
use std::io::{self, BufRead};
use std::path::Path;
fn main() {
    // Ok seems like a special thing. Does it unwrap a result?
    // lines is the variable that is being declared.
    // read_lines (from below) returns...
    // Ooh. It's shorthand.
    //if let Ok(lines) = read_lines("../day1/input")
    // is equivalent to:
    let result = read_lines("../day1/input");
    // result is of type Result<...>.
    // This Result can be matched to Ok,
    // which can then be unwrapped.
    match result {
        Ok(lines) => {
            // Start of the body (could be placed after the "if let")

            // Consumes the iterator, returns an (Optional) String
            for line in lines {
                // Another if let.
                // So basically: If line can be matched to Ok,
                // unwrap to ip.
                if let Ok(ip) = line {
                    println!("{}", ip);
                }
            }
        }
        _ => {}
    }
}
// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
// So read_lines is polymorphic, I guess. We're passing a string.
// The where keyword here looks like a type check.

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
