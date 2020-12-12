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

#[derive(Debug, Copy, Clone)]
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
        Action::East => (1, 0),
        Action::North => (0, 1),
        Action::West => (-1, 0),
        Action::South => (0, -1),
        _ => unreachable!(),
    }
}

#[derive(Debug, Copy, Clone)]
struct Instruction {
    action: Action,
    value: i32,
}

#[derive(Debug, Copy, Clone)]
struct Ship {
    x: i32,
    y: i32,
}

#[derive(Debug, Copy, Clone)]
struct Waypoint {
    x: i32,
    y: i32,
}

fn parse_input() -> std::io::Result<Vec<Instruction>> {
    let list = BufReader::new(File::open("input")?)
        .lines()
        .map(Result::unwrap)
        .collect::<Vec<_>>();
    Ok(list
        .iter()
        .map(|line| line.split_at(1))
        .map(|tuple| Instruction {
            action: match tuple.0 {
                "N" => Action::North,
                "S" => Action::South,
                "E" => Action::East,
                "W" => Action::West,
                "L" => Action::Left,
                "R" => Action::Right,
                "F" => Action::Forward,
                _ => unreachable!(),
            },
            value: tuple.1.parse::<i32>().unwrap(),
        })
        .collect::<Vec<_>>())
}

fn eval(direction: Action, instruction: Instruction, position: (i32, i32)) -> (Action, (i32, i32)) {
    match instruction.action {
        Action::North | Action::South | Action::East | Action::West => {
            let mut pos = position;
            let dir = forward(instruction.action);
            pos.0 += instruction.value * dir.0;
            pos.1 += instruction.value * dir.1;
            (direction, pos)
        }
        Action::Left | Action::Right => {
            let mut deg = direction_to_degrees(direction);
            deg += match instruction.action {
                Action::Left => instruction.value,
                Action::Right => -instruction.value,
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
            pos.0 += instruction.value * dir.0;
            pos.1 += instruction.value * dir.1;
            (direction, pos)
        }
    }
}

fn part_one() -> std::io::Result<usize> {
    let instructions = parse_input()?;
    //println! {"{:?}", instructions};

    // We're starting in the east direction.
    let mut situation = (Action::East, (0, 0));
    for instruction in instructions {
        //let old = situation.clone();
        situation = eval(situation.0, instruction, situation.1);
        //println!("{:?} + {:?} = {:?}", old, action, situation);
    }
    println!("{:?}", situation);

    Ok((situation.1 .0.abs() + situation.1 .1.abs()) as usize)
}

/*
--- Part Two ---

Before you can give the destination to the captain, you realize that the actual action meanings were
printed on the back of the instructions the whole time.

Almost all of the actions indicate how to move a waypoint which is relative to the ship's position:

    Action N means to move the waypoint north by the given value.
    Action S means to move the waypoint south by the given value.
    Action E means to move the waypoint east by the given value.
    Action W means to move the waypoint west by the given value.
    Action L means to rotate the waypoint around the ship left (counter-clockwise) the given number
    of degrees.
    Action R means to rotate the waypoint around the ship right (clockwise) the given number of
    degrees.
    Action F means to move forward to the waypoint a number of times equal to the given value.

The waypoint starts 10 units east and 1 unit north relative to the ship. The waypoint is relative to
the ship; that is, if the ship moves, the waypoint moves with it.

For example, using the same instructions as above:

    F10 moves the ship to the waypoint 10 times (a total of 100 units east and 10 units north),
        leaving the ship at east 100, north 10. The waypoint stays 10 units east and 1 unit north of
        the ship.
    N3 moves the waypoint 3 units north to 10 units east and 4 units north of the ship. The ship
        remains at east 100, north 10.
    F7 moves the ship to the waypoint 7 times (a total of 70 units east and 28 units north), leaving
        the ship at east 170, north 38. The waypoint stays 10 units east and 4 units north of the
        ship.
    R90 rotates the waypoint around the ship clockwise 90 degrees, moving it to 4 units east and 10
        units south of the ship. The ship remains at east 170, north 38.
    F11 moves the ship to the waypoint 11 times (a total of 44 units east and 110 units south),
        leaving the ship at east 214, south 72. The waypoint stays 4 units east and 10 units south
        of the ship.

After these operations, the ship's Manhattan distance from its starting position is 214 + 72 = 286.

Figure out where the navigation instructions actually lead. What is the Manhattan distance between that location and the ship's starting position?
*/

#[derive(Debug, Copy, Clone)]
struct Situation {
    ship: Ship,
    waypoint: Waypoint,
}

fn integer_sin(angle: i32) -> i32 {
    match angle {
        0 => 0,
        90 => 1,
        180 => 0,
        270 => -1,
        _ => unreachable!(),
    }
}

fn integer_cos(angle: i32) -> i32 {
    match angle {
        0 => 1,
        90 => 0,
        180 => -1,
        270 => 0,
        _ => unreachable!(),
    }
}

fn rotate(point: Waypoint, angle: i32) -> Waypoint {
    let cos_t = integer_cos(angle);
    let sin_t = integer_sin(angle);
    /*
    println!(
        "{:?}, angle = {}: cos = {}, sin = {}",
        point, angle, cos_t, sin_t
    );
    */
    Waypoint {
        x: point.x * cos_t - point.y * sin_t,
        y: point.x * sin_t + point.y * cos_t,
    }
}

fn eval_two(situation: Situation, instruction: Instruction) -> Situation {
    match instruction.action {
        Action::North | Action::South | Action::East | Action::West => {
            // move the waypoint
            let mut pos = situation.waypoint;
            let dir = forward(instruction.action);
            pos.x += instruction.value * dir.0;
            pos.y += instruction.value * dir.1;
            Situation {
                ship: situation.ship,
                waypoint: pos,
            }
        }
        Action::Left | Action::Right => {
            // rotate the waypoint around the ship
            let mut deg = match instruction.action {
                Action::Left => instruction.value,
                Action::Right => -instruction.value,
                _ => unreachable!(),
            };
            while deg < 0 {
                deg += 360;
            }
            deg %= 360;
            Situation {
                ship: situation.ship,
                waypoint: rotate(situation.waypoint, deg),
            }
        }
        Action::Forward => {
            // move forward to the waypoint a number of times equal to the given value
            let mut pos = situation.ship;
            pos.x += instruction.value * situation.waypoint.x;
            pos.y += instruction.value * situation.waypoint.y;
            Situation {
                ship: pos,
                waypoint: situation.waypoint,
            }
        }
    }
}

fn part_two() -> std::io::Result<usize> {
    let instructions = parse_input()?;
    //println! {"{:?}", actions};

    let mut situation = Situation {
        ship: Ship { x: 0, y: 0 },
        waypoint: Waypoint { x: 10, y: 1 },
    };
    for instruction in instructions {
        //let old = situation.clone();
        situation = eval_two(situation, instruction);
        //println!("{:?} + {:?} = {:?}", old, instruction, situation);
    }
    println!("{:?}", situation);

    Ok((situation.ship.x.abs() + situation.ship.y.abs()) as usize)
}

fn main() {
    println!("=== Advent of Code Day 12 ===");
    println!("Part One: {}", part_one().unwrap_or(0));
    println!("Part Two: {}", part_two().unwrap_or(0));
}
