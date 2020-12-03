/*
// Hello, world!
fn main() {
    println!("Hello, world!");
}
*/

// Print working directory
// We return a Result of ().
// () looks like a built-in void type.
// Note that the return value comes at the end of the signature.
// The namespaces can be omitted if we specify the modules with the "use" keyword.
fn main() -> std::io::Result<()> {
    // Note that with the question mark at the end,
    // We unwrap the Result immediately, without matching it.
    let path = std::env::current_dir()?;
    // println is in the global namespace, apparently.
    // The ! means that it is a Rust macro. We can define our own... sometime.
    // Nice that they have formatting using {} instead of specifiers.
    println!("The current directory is {}", path.display());

    // Note that there is no return statement here. So apparently the function will return
    // whatever is returned by the last statement.
    // What if we put a void statement after this?
    Ok(())
}

/*
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
    match result{
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
        },
        _ => {},
    }
}
// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
// So read_lines is polymorphic, I guess. We're passing a string.
// The where keyword here looks like a type check.

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
*/

// Note: Run "cargo run" from the Cargo.toml directory.

/*
// template

fn part_one() {
    match parser::read_lines("input") {
        Ok(lines) => {
            for line in lines {
                if let Ok(entry) = line {
                }
            }
        }
        _ => {
            println!("Error: Could not read lines!")
        }
    }
}

fn part_two() {
    match parser::read_lines("input") {
        Ok(lines) => {
            for line in lines {
                if let Ok(entry) = line {
                }
            }
        }
        _ => {
            println!("Error: Could not read lines!")
        }
    }
}
*/
