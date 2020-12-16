use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

/*
--- Day 16: Ticket Translation ---

As you're walking to yet another connecting flight, you realize that one of the legs of your
re-routed trip coming up is on a high-speed train. However, the train ticket you were given is in a
language you don't understand. You should probably figure out what it says before you get to the
train station after the next flight.

Unfortunately, you can't actually read the words on the ticket. You can, however, read the numbers,
and so you figure out the fields these tickets must have and the valid ranges for values in those
fields.

You collect the rules for ticket fields, the numbers on your ticket, and the numbers on other nearby
tickets for the same train service (via the airport security cameras) together into a single
document you can reference (your puzzle input).

The rules for ticket fields specify a list of fields that exist somewhere on the ticket and the
valid ranges of values for each field. For example, a rule like class: 1-3 or 5-7 means that one of
the fields in every ticket is named class and can be any value in the ranges 1-3 or 5-7 (inclusive,
    such that 3 and 5 are both valid in this field, but 4 is not).

Each ticket is represented by a single line of comma-separated values. The values are the numbers on
the ticket in the order they appear; every ticket has the same format. For example, consider this
ticket:

.--------------------------------------------------------.
| ????: 101    ?????: 102   ??????????: 103     ???: 104 |
|                                                        |
| ??: 301  ??: 302             ???????: 303      ??????? |
| ??: 401  ??: 402           ???? ????: 403    ????????? |
'--------------------------------------------------------'

Here, ? represents text in a language you don't understand. This ticket might be represented as
101,102,103,104,301,302,303,401,402,403; of course, the actual train tickets you're looking at are
much more complicated. In any case, you've extracted just the numbers in such a way that the first
number is always the same specific field, the second number is always a different specific field,
and so on - you just don't know what each position actually means!

Start by determining which tickets are completely invalid; these are tickets that contain values
which aren't valid for any field. Ignore your ticket for now.

For example, suppose you have the following notes:

class: 1-3 or 5-7
row: 6-11 or 33-44
seat: 13-40 or 45-50

your ticket:
7,1,14

nearby tickets:
7,3,47
40,4,50
55,2,20
38,6,12

It doesn't matter which position corresponds to which field; you can identify invalid nearby tickets
by considering only whether tickets contain values that are not valid for any field. In this
example, the values on the first nearby ticket are all valid for at least one field. This is not
true of the other three nearby tickets: the values 4, 55, and 12 are are not valid for any field.
Adding together all of the invalid values produces your ticket scanning error rate:
4 + 55 + 12 = 71.

Consider the validity of the nearby tickets you scanned. What is your ticket scanning error rate?
*/

#[derive(Debug, Clone)]
struct Range {
    min: i32, // inclusive
    max: i32, // inclusive
}

#[derive(Debug, Clone)]
struct Rule {
    name: String,
    range_first: Range,
    range_second: Range,
}

#[derive(Debug, Clone)]
struct Ticket {
    values: Vec<i32>,
}

#[derive(Debug, Clone)]
struct Input {
    rules: Vec<Rule>,
    ticket_mine: Ticket,
    tickets_nearby: Vec<Ticket>,
}

fn extract_range(string: &str) -> Range {
    let split_second = string.split("-").map(|x| x.parse::<i32>().unwrap());
    let range: Vec<_> = split_second
        .clone()
        .zip(split_second.clone().skip(1))
        .map(|tuple| Range {
            min: tuple.0,
            max: tuple.1,
        })
        .collect();
    range.first().unwrap().clone()
}

fn parse_ticket(string: &str) -> Ticket {
    Ticket {
        values: string
            .split(",")
            .map(|x| x.parse::<i32>().unwrap())
            .collect(),
    }
}

fn parse_input() -> std::io::Result<Input> {
    let mut rules: Vec<Rule> = Vec::new();
    let mut tickets_nearby: Vec<Ticket> = Vec::new();

    let input_lines: Vec<_> = BufReader::new(File::open("input")?)
        .lines()
        .map(Result::unwrap)
        .filter(|x| !x.is_empty() && !x.contains("nearby tickets:"))
        .collect();

    // rules
    let mut it = input_lines.iter();
    loop {
        let line = it.next().unwrap();
        //println!("{:?}", line);
        if line.contains("your ticket:") {
            break;
        }

        let mut split_name = line.split(": ");
        let name = split_name.next().unwrap().to_owned();
        let split_or: Vec<_> = split_name.next().unwrap().split(" or ").collect();
        let range_first = extract_range(split_or[0]);
        let range_second = extract_range(split_or[1]);

        rules.push(Rule {
            name,
            range_first,
            range_second,
        })
    }
    //println!("{:?}", rules);

    // ticket_mine
    let ticket_mine = parse_ticket(it.next().unwrap());

    // tickets_nearby
    for line in it {
        tickets_nearby.push(parse_ticket(line))
    }

    Ok(Input {
        rules,
        ticket_mine,
        tickets_nearby,
    })
}

fn part_one() -> std::io::Result<i32> {
    let input = parse_input()?;
    //println!("{:?}", input);

    let mut sum = 0;
    for ticket in &input.tickets_nearby {
        sum += ticket.values.iter().fold(0, |acc, x| {
            let mut has_rule = false;
            for rule in &input.rules {
                if (x >= &rule.range_first.min && x <= &rule.range_first.max)
                    || (x >= &rule.range_second.min && x <= &rule.range_second.max)
                {
                    has_rule = true;
                    break;
                }
            }
            if !has_rule {
                return acc + *x;
            }
            acc
        });
    }

    Ok(sum)
}

fn main() {
    println!("=== Advent of Code Day 16 ===");
    println!("Part One: {}", part_one().unwrap_or(0));
    //println!("Part Two: {}", part_two().unwrap_or(0));
}
