use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

/*
--- Day 14: Docking Data ---

As your ferry approaches the sea port, the captain asks for your help again. The computer system
that runs this port isn't compatible with the docking program on the ferry, so the docking
parameters aren't being correctly initialized in the docking program's memory.

After a brief inspection, you discover that the sea port's computer system uses a strange bitmask
system in its initialization program. Although you don't have the correct decoder chip handy, you
can emulate it in software!

The initialization program (your puzzle input) can either update the bitmask or write a value to
memory. Values and memory addresses are both 36-bit unsigned integers. For example, ignoring
bitmasks for a moment, a line like mem[8] = 11 would write the value 11 to memory address 8.

The bitmask is always given as a string of 36 bits, written with the most significant bit
(representing 2^35) on the left and the least significant bit (2^0, that is, the 1s bit) on the
right. The current bitmask is applied to values immediately before they are written to memory:
a 0 or 1 overwrites the corresponding bit in the value, while an X leaves the bit in the value
unchanged.

For example, consider the following program:

mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
mem[8] = 11
mem[7] = 101
mem[8] = 0

This program starts by specifying a bitmask (mask = ....). The mask it specifies will overwrite two
bits in every written value: the 2s bit is overwritten with 0, and the 64s bit is overwritten
with 1.

The program then attempts to write the value 11 to memory address 8. By expanding everything out to
individual bits, the mask is applied as follows:

value:  000000000000000000000000000000001011  (decimal 11)
mask:   XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
result: 000000000000000000000000000001001001  (decimal 73)

So, because of the mask, the value 73 is written to memory address 8 instead. Then, the program
tries to write 101 to address 7:

value:  000000000000000000000000000001100101  (decimal 101)
mask:   XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
result: 000000000000000000000000000001100101  (decimal 101)

This time, the mask has no effect, as the bits it overwrote were already the values the mask tried
to set. Finally, the program tries to write 0 to address 8:

value:  000000000000000000000000000000000000  (decimal 0)
mask:   XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
result: 000000000000000000000000000001000000  (decimal 64)

64 is written to address 8 instead, overwriting the value that was there previously.

To initialize your ferry's docking program, you need the sum of all values left in memory after the
initialization program completes. (The entire 36-bit address space begins initialized to the value 0
at every address.) In the above example, only two values in memory are not zero - 101 (at address 7)
and 64 (at address 8) - producing a sum of 165.

Execute the initialization program. What is the sum of all values left in memory after it completes?
*/

#[derive(Debug, Clone)]
enum OperationType {
    Mask,
    Memory,
}

#[derive(Debug, Clone)]
struct Operation {
    operation_type: OperationType,
    // Mask
    and_mask: u64, // Has ones at all other places (binary AND)
    or_mask: u64,  // Has zeroes at all other places (binary OR)
    // Mem
    address: u64,
    value: u64,
}

fn part_one() -> std::io::Result<u64> {
    let list = BufReader::new(File::open("input")?)
        .lines()
        .map(Result::unwrap)
        .collect::<Vec<_>>();
    let operations = list
        .iter()
        .map(|line| line.split(" = ").collect::<Vec<_>>())
        .map(|vec| {
            let left = vec.first().unwrap();
            let mut op = Operation {
                operation_type: OperationType::Mask,
                or_mask: 0,
                and_mask: 0,
                address: 0,
                value: 0,
            };
            let right = vec.iter().skip(1).next().unwrap();
            if left.starts_with("mask") {
                let and_str = right.replace("X", "1");
                let mut and_mask = 0;
                for c in and_str.chars() {
                    and_mask <<= 1;
                    match c {
                        '1' => and_mask += 1,
                        '0' => (),
                        _ => unreachable!(),
                    }
                }
                op.and_mask = and_mask;

                let or_str = right.replace("X", "0");
                let mut or_mask = 0;
                for c in or_str.chars() {
                    or_mask <<= 1;
                    match c {
                        '1' => or_mask += 1,
                        '0' => (),
                        _ => unreachable!(),
                    }
                }
                op.or_mask = or_mask;
            } else {
                op.operation_type = OperationType::Memory;
                op.address = left
                    .split(|x| x == '[' || x == ']')
                    .skip(1)
                    .next()
                    .unwrap()
                    .parse::<u64>()
                    .unwrap();
                op.value = right.parse::<u64>().unwrap();
            }
            op
        })
        .collect::<Vec<_>>();

    let mut and_mask = 0;
    let mut or_mask = 0;
    let mut memory: HashMap<u64, u64> = HashMap::new();
    for op in operations {
        match op.operation_type {
            OperationType::Mask => {
                and_mask = op.and_mask;
                or_mask = op.or_mask;
            }
            OperationType::Memory => {
                let mut value = op.value;
                value &= and_mask;
                value |= or_mask;
                memory.insert(op.address, value);
            }
        }
    }

    let mut sum = 0;
    for kv in memory {
        sum += kv.1;
    }

    Ok(sum)
}

fn main() {
    println!("=== Advent of Code Day 14 ===");
    println!("Part One: {}", part_one().unwrap_or(0));
    //println!("Part Two: {}", part_two().unwrap_or(0));
}
