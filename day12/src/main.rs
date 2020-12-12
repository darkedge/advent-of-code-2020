use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

/*
--- Day 12: Rain Risk ---

Your ferry made decent progress toward the island, but the storm came in faster than anyone
expected. The ferry needs to take evasive actions!

Unfortunately, the ship's navigation computer seems to be malfunctioning; rather than giving a route
directly to safety, it produced extremely circuitous instructions. When the captain uses the PA
system to ask if anyone can help, you quickly volunteer.

The navigation instructions (your puzzle input) consists of a sequence of single-character actions
paired with integer input values. After staring at them for a few minutes, you work out what they
probably mean:

    Action N means to move north by the given value.
    Action S means to move south by the given value.
    Action E means to move east by the given value.
    Action W means to move west by the given value.
    Action L means to turn left the given number of degrees.
    Action R means to turn right the given number of degrees.
    Action F means to move forward by the given value in the direction the ship is currently facing.

The ship starts by facing east. Only the L and R actions change the direction the ship is facing.
(That is, if the ship is facing east and the next instruction is N10, the ship would move north 10
    units, but would still move east if the following action were F.)

For example:

F10
N3
F7
R90
F11

These instructions would be handled as follows:

    F10 would move the ship 10 units east (because the ship starts by facing east) to east 10,
    north 0.
    N3 would move the ship 3 units north to east 10, north 3.
    F7 would move the ship another 7 units east (because the ship is still facing east) to east 17,
    north 3.
    R90 would cause the ship to turn right by 90 degrees and face south; it remains at east 17,
    north 3.
    F11 would move the ship 11 units south to east 17, south 8.

At the end of these instructions, the ship's Manhattan distance (sum of the absolute values of its east/west position and its north/south position) from its starting position is 17 + 8 = 25.

Figure out where the navigation instructions lead. What is the Manhattan distance between that location and the ship's starting position?

*/

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Action {
    North,
    South,
    East,
    West,
    Left,  // Degrees!
    Right, // Degrees!
    Forward,
}

fn degrees_to_direction(degrees: i32) -> Action {
    match degrees {
        0 => Action::East,
        90 => Action::North,
        180 => Action::West,
        270 => Action::South,
        _ => unreachable!(),
    }
}

fn direction_to_degrees(direction: Action) -> i32 {
    match direction {
        Action::East => 0,
        Action::North => 90,
        Action::West => 180,
        Action::South => 270,
        _ => unreachable!(),
    }
}

// East/North = +
fn forward(direction: Action) -> (i32, i32) {
    match direction {
        Action::East => (0, 1),
        Action::North => (1, 0),
        Action::West => (0, -1),
        Action::South => (-1, 0),
        _ => unreachable!(),
    }
}

fn parse_input() -> std::io::Result<Vec<(Action, i32)>> {
    let list = BufReader::new(File::open("input")?)
        .lines()
        .map(Result::unwrap)
        .collect::<Vec<_>>();
    Ok(list
        .iter()
        .map(|line| line.split_at(1))
        .map(|tuple| {
            (
                match tuple.0 {
                    "N" => Action::North,
                    "S" => Action::South,
                    "E" => Action::East,
                    "W" => Action::West,
                    "L" => Action::Left,
                    "R" => Action::Right,
                    "F" => Action::Forward,
                    _ => unreachable!(),
                },
                tuple.1.parse::<i32>().unwrap(),
            )
        })
        .collect::<Vec<_>>())
}

fn eval(
    direction: Action,
    action: Action,
    value: i32,
    position: (i32, i32),
) -> (Action, (i32, i32)) {
    match action {
        Action::North | Action::South | Action::East | Action::West => {
            let mut pos = position;
            let dir = forward(action);
            pos.0 += value * dir.0;
            pos.1 += value * dir.1;
            (direction, pos)
        }
        Action::Left | Action::Right => {
            let mut deg = direction_to_degrees(direction);
            deg += match action {
                Action::Left => value,
                Action::Right => -value,
                _ => unreachable!(),
            };
            while deg < 0 {
                deg += 360;
            }
            deg %= 360;
            (degrees_to_direction(deg), position)
        }
        Action::Forward => {
            let mut pos = position;
            let dir = forward(direction);
            pos.0 += value * dir.0;
            pos.1 += value * dir.1;
            (direction, pos)
        }
    }
}

fn part_one() -> std::io::Result<usize> {
    let actions = parse_input()?;
    //println! {"{:?}", actions};

    // We're starting in the east direction.
    let mut situation = (Action::East, (0, 0));
    for action in actions {
        //let old = situation.clone();
        situation = eval(situation.0, action.0, action.1, situation.1);
        //println!("{:?} + {:?} = {:?}", old, action, situation);
    }
    println!("{:?}", situation);

    Ok((situation.1 .0.abs() + situation.1 .1.abs()) as usize)
}

fn main() {
    println!("=== Advent of Code Day 12 ===");
    println!("Part One: {}", part_one().unwrap_or(0));
    //println!("Part Two: {}", part_two().unwrap_or(0));
}
