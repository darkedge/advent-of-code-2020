use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

/*
--- Day 11: Seating System ---

Your plane lands with plenty of time to spare. The final leg of your journey is a ferry that goes
directly to the tropical island where you can finally start your vacation. As you reach the waiting
area to board the ferry, you realize you're so early, nobody else has even arrived yet!

By modeling the process people use to choose (or abandon) their seat in the waiting area, you're
pretty sure you can predict the best place to sit. You make a quick map of the seat layout (your
    puzzle input).

The seat layout fits neatly on a grid. Each position is either floor (.), an empty seat (L), or an
occupied seat (#). For example, the initial seat layout might look like this:

L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL

Now, you just need to model the people who will be arriving shortly. Fortunately, people are
entirely predictable and always follow a simple set of rules. All decisions are based on the number
of occupied seats adjacent to a given seat (one of the eight positions immediately up, down, left,
right, or diagonal from the seat). The following rules are applied to every seat simultaneously:

    If a seat is empty (L) and there are no occupied seats adjacent to it, the seat becomes
        occupied.
    If a seat is occupied (#) and four or more seats adjacent to it are also occupied, the seat
        becomes empty.
    Otherwise, the seat's state does not change.

Floor (.) never changes; seats don't move, and nobody sits on the floor.

After one round of these rules, every seat in the example layout becomes occupied:

#.##.##.##
#######.##
#.#.#..#..
####.##.##
#.##.##.##
#.#####.##
..#.#.....
##########
#.######.#
#.#####.##

After a second round, the seats with four or more occupied adjacent seats become empty again:

#.LL.L#.##
#LLLLLL.L#
L.L.L..L..
#LLL.LL.L#
#.LL.LL.LL
#.LLLL#.##
..L.L.....
#LLLLLLLL#
#.LLLLLL.L
#.#LLLL.##

This process continues for three more rounds:

#.##.L#.##
#L###LL.L#
L.#.#..#..
#L##.##.L#
#.##.LL.LL
#.###L#.##
..#.#.....
#L######L#
#.LL###L.L
#.#L###.##

#.#L.L#.##
#LLL#LL.L#
L.L.L..#..
#LLL.##.L#
#.LL.LL.LL
#.LL#L#.##
..L.L.....
#L#LLLL#L#
#.LLLLLL.L
#.#L#L#.##

#.#L.L#.##
#LLL#LL.L#
L.#.L..#..
#L##.##.L#
#.#L.LL.LL
#.#L#L#.##
..L.L.....
#L#L##L#L#
#.LLLLLL.L
#.#L#L#.##

At this point, something interesting happens: the chaos stabilizes and further applications of these
rules cause no seats to change state! Once people stop moving around, you count 37 occupied seats.

Simulate your seating area by applying the seating rules repeatedly until no seats change state. How
many seats end up occupied?
*/

#[derive(Debug, Copy, Clone, PartialEq)]
enum Position {
    Floor,
    EmptySeat,
    OccupiedSeat,
}

fn part_one() -> std::io::Result<usize> {
    let list = BufReader::new(File::open("input")?)
        .lines()
        .map(Result::unwrap)
        .collect::<Vec<_>>();

    let width = list.first().unwrap().chars().count();
    let height = list.len();
    println!("width: {}, height: {}", width, height);

    let mut seating_area_chunks = vec![Position::Floor; width * height];
    let mut seating_area_vec = seating_area_chunks
        .as_mut_slice()
        .chunks_mut(width)
        .collect::<Vec<_>>();
    let seating_area = seating_area_vec.as_mut_slice();

    for (i, line) in list.iter().enumerate() {
        for (j, c) in line.chars().enumerate() {
            seating_area[i][j] = match c {
                '.' => Position::Floor,
                'L' => Position::EmptySeat,
                '#' => Position::OccupiedSeat,
                _ => unreachable!(),
            }
        }
    }

    //println!("{:?}", seating_area);

    let mut num_prev_seats = 0;
    let mut num_iterations = 0;
    loop {
        let mut num_seats = 0;
        //evaluate
        let mut iteration_chunks = vec![Position::Floor; width * height];
        let mut iteration_vec = iteration_chunks
            .as_mut_slice()
            .chunks_mut(width)
            .collect::<Vec<_>>();
        let iteration = iteration_vec.as_mut_slice();
        // Iterate cells
        for y in 0..height {
            for x in 0..width {
                // Closure to check adjacent cells
                let adjacent_empty = || {
                    // 3x3
                    //print!("{},{} - ", y, x);
                    for i in -1..=1 {
                        for j in -1..=1 {
                            if !(i == 0 && j == 0) {
                                // Range check
                                let p = y as i32 + i;
                                let q = x as i32 + j;
                                if p >= 0 && p < height as i32 && q >= 0 && q < width as i32 {
                                    //print!("{},{}; ", p, q);
                                    // Check for occupied seat
                                    if seating_area[p as usize][q as usize]
                                        == Position::OccupiedSeat
                                    {
                                        //println!();
                                        return false;
                                    }
                                }
                            }
                        }
                    }
                    //println!();
                    return true;
                };
                let adjacent_occupied = || {
                    // 3x3
                    let mut num_occupied = 0;
                    for i in -1..=1 {
                        for j in -1..=1 {
                            if !(i == 0 && j == 0) {
                                // Range check
                                let p = y as i32 + i;
                                let q = x as i32 + j;
                                if p >= 0 && p < height as i32 && q >= 0 && q < width as i32 {
                                    // Check for occupied seat
                                    if seating_area[p as usize][q as usize]
                                        == Position::OccupiedSeat
                                    {
                                        num_occupied += 1;
                                        if num_occupied == 4 {
                                            return true;
                                        }
                                    }
                                }
                            }
                        }
                    }
                    return false;
                };

                // rules
                let pos = seating_area[y][x];
                iteration[y][x] = match pos {
                    Position::EmptySeat => {
                        if adjacent_empty() {
                            Position::OccupiedSeat
                        } else {
                            pos
                        }
                    }
                    Position::OccupiedSeat => {
                        if adjacent_occupied() {
                            Position::EmptySeat
                        } else {
                            pos
                        }
                    }
                    _ => pos,
                };
                if iteration[y][x] == Position::OccupiedSeat {
                    num_seats += 1;
                }
            }
        }
        //println!("{:?}", iteration);
        num_iterations += 1;
        /*
        println!(
            "num_iterations: {}, num_seats: {}, num_prev_seats: {}",
            num_iterations, num_seats, num_prev_seats
        );
        */
        /*
        for y in 0..height {
            for x in 0..width {
                print!(
                    "{}",
                    match iteration[y][x] {
                        Position::Floor => '.',
                        Position::EmptySeat => 'L',
                        Position::OccupiedSeat => '#',
                        _ => unreachable!(),
                    }
                );
            }
            println!();
        }
        println!();
        */

        // DEBUG
        /*
        if num_iterations == 2 {
            break;
        }
        */
        if num_seats == num_prev_seats {
            break;
        }
        num_prev_seats = num_seats;
        for y in 0..height {
            for x in 0..width {
                seating_area[y][x] = iteration[y][x];
            }
        }
    }
    println!("Iterations: {}", num_iterations);

    Ok(num_prev_seats)
}

fn main() {
    println!("=== Advent of Code Day 11 ===");
    println!("Part One: {}", part_one().unwrap_or(0));
    //println!("Part Two: {}", part_two().unwrap_or(0));
}
