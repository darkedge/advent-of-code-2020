use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

/*
--- Day 17: Conway Cubes ---

As your flight slowly drifts through the sky, the Elves at the Mythical Information Bureau at the
North Pole contact you. They'd like some help debugging a malfunctioning experimental energy source
aboard one of their super-secret imaging satellites.

The experimental energy source is based on cutting-edge technology: a set of Conway Cubes contained
in a pocket dimension! When you hear it's having problems, you can't help but agree to take a look.

The pocket dimension contains an infinite 3-dimensional grid. At every integer 3-dimensional
coordinate (x,y,z), there exists a single cube which is either active or inactive.

In the initial state of the pocket dimension, almost all cubes start inactive. The only exception to
this is a small flat region of cubes (your puzzle input); the cubes in this region start in the
specified active (#) or inactive (.) state.

The energy source then proceeds to boot up by executing six cycles.

Each cube only ever considers its neighbors: any of the 26 other cubes where any of their
coordinates differ by at most 1. For example, given the cube at x=1,y=2,z=3, its neighbors include
the cube at x=2,y=2,z=2, the cube at x=0,y=2,z=3, and so on.

During a cycle, all cubes simultaneously change their state according to the following rules:

    If a cube is active and exactly 2 or 3 of its neighbors are also active, the cube remains
        active. Otherwise, the cube becomes inactive.
    If a cube is inactive but exactly 3 of its neighbors are active, the cube becomes active.
        Otherwise, the cube remains inactive.

The engineers responsible for this experimental energy source would like you to simulate the pocket
dimension and determine what the configuration of cubes should be at the end of the six-cycle boot
process.

For example, consider the following initial state:

.#.
..#
###

Even though the pocket dimension is 3-dimensional, this initial state represents a small
2-dimensional slice of it. (In particular, this initial state defines a 3x3x1 region of the
3-dimensional space.)

Simulating a few cycles from this initial state produces the following configurations, where the
result of each cycle is shown layer-by-layer at each given z coordinate (and the frame of view
follows the active cells in each cycle):

Before any cycles:

z=0
.#.
..#
###


After 1 cycle:

z=-1
#..
..#
.#.

z=0
#.#
.##
.#.

z=1
#..
..#
.#.


After 2 cycles:

z=-2
.....
.....
..#..
.....
.....

z=-1
..#..
.#..#
....#
.#...
.....

z=0
##...
##...
#....
....#
.###.

z=1
..#..
.#..#
....#
.#...
.....

z=2
.....
.....
..#..
.....
.....


After 3 cycles:

z=-2
.......
.......
..##...
..###..
.......
.......
.......

z=-1
..#....
...#...
#......
.....##
.#...#.
..#.#..
...#...

z=0
...#...
.......
#......
.......
.....##
.##.#..
...#...

z=1
..#....
...#...
#......
.....##
.#...#.
..#.#..
...#...

z=2
.......
.......
..##...
..###..
.......
.......
.......

After the full six-cycle boot process completes, 112 cubes are left in the active state.

Starting with your given initial configuration, simulate six cycles. How many cubes are left in the
active state after the sixth cycle?
*/

#[derive(Debug, Clone, Copy, Hash)]
struct Position {
    x: i32,
    y: i32,
    z: i32,
}

// We let the inactive state be the 3x3x3 space around active cubes.
#[derive(Debug, Clone)]
struct State {
    active: HashSet<Position>,
    inactive: HashSet<Position>,
}

impl PartialEq for Position {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z
    }
}
impl Eq for Position {}

fn match_inactive(active: &HashSet<Position>) -> HashSet<Position> {
    let mut inactive: HashSet<Position> = HashSet::new();
    for cube in active {
        for x in -1..=1 {
            for y in -1..=1 {
                for z in -1..=1 {
                    if !((x == 0) && (y == 0) && (z == 0)) {
                        let neighbor = Position {
                            x: cube.x + x,
                            y: cube.y + y,
                            z: cube.z + z,
                        };

                        if !active.contains(&neighbor) {
                            inactive.insert(neighbor);
                        }
                    }
                }
            }
        }
    }

    inactive
}

fn parse_input() -> std::io::Result<State> {
    // x will be line width, growing right
    // y will be number of lines, growing down
    // z = 0, growing "up"
    let lines = BufReader::new(File::open("input")?)
        .lines()
        .map(|line| line.unwrap())
        .collect::<Vec<_>>();

    let mut active: HashSet<Position> = HashSet::new();

    for (y, line) in lines.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                active.insert(Position {
                    x: x as i32,
                    y: y as i32,
                    z: 0,
                });
            }
        }
    }

    let inactive = match_inactive(&active);
    Ok(State { active, inactive })
}

fn cycle(state: &State) -> State {
    let mut active: HashSet<Position> = HashSet::new();

    // evaluate active cubes
    for cube in &state.active {
        let mut num_neighbors = 0;
        for x in -1..=1 {
            for y in -1..=1 {
                for z in -1..=1 {
                    if !((x == 0) && (y == 0) && (z == 0)) {
                        let neighbor = Position {
                            x: cube.x + x,
                            y: cube.y + y,
                            z: cube.z + z,
                        };

                        if state.active.contains(&neighbor) {
                            //println!("{:?} has neighbor {:?}", cube, neighbor);
                            num_neighbors += 1;
                        }
                    }
                }
            }
        }
        if (num_neighbors == 2) || (num_neighbors == 3) {
            //println!("Inserting cube!");
            active.insert(*cube);
        //println!("{:?} stays active", cube);
        } else {
            //println!("{:?} now inactive", cube);
        }
    }

    // evaluate inactive cubes
    for cube in &state.inactive {
        let mut num_neighbors = 0;
        //println!("Checking {:?}", cube);
        for x in -1..=1 {
            for y in -1..=1 {
                for z in -1..=1 {
                    if !((x == 0) && (y == 0) && (z == 0)) {
                        let neighbor = Position {
                            x: cube.x + x,
                            y: cube.y + y,
                            z: cube.z + z,
                        };

                        if state.active.contains(&neighbor) {
                            //println!("Empty {:?} has neighbor {:?}", cube, neighbor);
                            num_neighbors += 1;
                        }
                    }
                }
            }
        }
        if num_neighbors == 3 {
            active.insert(*cube);
            //println!("{:?} now active", cube);
        }
    }

    let inactive = match_inactive(&active);
    State { active, inactive }
}

fn print_state(state: &State) {
    // Get extents
    let mut min_x = i32::MAX;
    let mut max_x = i32::MIN;
    let mut min_y = i32::MAX;
    let mut max_y = i32::MIN;
    let mut min_z = i32::MAX;
    let mut max_z = i32::MIN;

    for cube in &state.active {
        if cube.x < min_x {
            min_x = cube.x;
        }
        if cube.y < min_y {
            min_y = cube.y;
        }
        if cube.z < min_z {
            min_z = cube.z;
        }
        if cube.x > max_x {
            max_x = cube.x;
        }
        if cube.y > max_y {
            max_y = cube.y;
        }
        if cube.z > max_z {
            max_z = cube.z;
        }
    }

    for z in min_z..=max_z {
        println!("z={}", z);
        for y in min_y..=max_y {
            for x in min_x..=max_x {
                let position = Position { x, y, z };
                if state.active.contains(&position) {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!();
        }
        println!();
    }
}

fn part_one() -> std::io::Result<usize> {
    let mut state = parse_input()?;
    print_state(&state);

    for i in 1..=6 {
        state = cycle(&state);
        println!("After {} cycles:\n", i);
        print_state(&state);
    }

    Ok(0)
}

fn main() {
    println!("=== Advent of Code Day 10 ===");
    println!("Part One: {}", part_one().unwrap_or(0));
    //println!("Part Two: {}", part_two().unwrap_or(0));
}
