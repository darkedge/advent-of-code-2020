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
struct OperationOne {
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
            let mut op = OperationOne {
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

/*
--- Part Two ---

For some reason, the sea port's computer system still can't communicate with your ferry's docking
program. It must be using version 2 of the decoder chip!

A version 2 decoder chip doesn't modify the values being written at all. Instead, it acts as a
memory address decoder. Immediately before a value is written to memory, each bit in the bitmask
modifies the corresponding bit of the destination memory address in the following way:

    If the bitmask bit is 0, the corresponding memory address bit is unchanged.
    If the bitmask bit is 1, the corresponding memory address bit is overwritten with 1.
    If the bitmask bit is X, the corresponding memory address bit is floating.

A floating bit is not connected to anything and instead fluctuates unpredictably. In practice, this
means the floating bits will take on all possible values, potentially causing many memory addresses
to be written all at once!

For example, consider the following program:

mask = 000000000000000000000000000000X1001X
mem[42] = 100
mask = 00000000000000000000000000000000X0XX
mem[26] = 1

When this program goes to write to memory address 42, it first applies the bitmask:

address: 000000000000000000000000000000101010  (decimal 42)
mask:    000000000000000000000000000000X1001X
result:  000000000000000000000000000000X1101X

After applying the mask, four bits are overwritten, three of which are different, and two of which
are floating. Floating bits take on every possible combination of values; with two floating bits,
four actual memory addresses are written:

000000000000000000000000000000011010  (decimal 26)
000000000000000000000000000000011011  (decimal 27)
000000000000000000000000000000111010  (decimal 58)
000000000000000000000000000000111011  (decimal 59)

Next, the program is about to write to memory address 26 with a different bitmask:

address: 000000000000000000000000000000011010  (decimal 26)
mask:    00000000000000000000000000000000X0XX
result:  00000000000000000000000000000001X0XX

This results in an address with three floating bits, causing writes to eight memory addresses:

000000000000000000000000000000010000  (decimal 16)
000000000000000000000000000000010001  (decimal 17)
000000000000000000000000000000010010  (decimal 18)
000000000000000000000000000000010011  (decimal 19)
000000000000000000000000000000011000  (decimal 24)
000000000000000000000000000000011001  (decimal 25)
000000000000000000000000000000011010  (decimal 26)
000000000000000000000000000000011011  (decimal 27)

The entire 36-bit address space still begins initialized to the value 0 at every address, and you
still need the sum of all values left in memory at the end of the program. In this example, the sum
is 208.

Execute the initialization program using an emulator for a version 2 decoder chip. What is the sum
of all values left in memory after it completes?
*/

#[derive(Debug, Clone)]
struct OperationTwo {
    operation_type: OperationType,
    // Mask
    or_mask: u64,
    and_mask: u64,
    floating_vec: Vec<u64>,
    // Mem
    address: u64,
    value: u64,
}

fn part_two() -> std::io::Result<u64> {
    let list = BufReader::new(File::open("input")?)
        .lines()
        .map(Result::unwrap)
        .collect::<Vec<_>>();
    let operations = list
        .iter()
        .map(|line| line.split(" = ").collect::<Vec<_>>())
        .map(|vec| {
            let left = vec.first().unwrap();
            let mut op = OperationTwo {
                operation_type: OperationType::Mask,
                or_mask: 0,
                and_mask: 0,
                floating_vec: vec![],
                address: 0,
                value: 0,
            };
            let right = vec.iter().skip(1).next().unwrap();
            if left.starts_with("mask") {
                let or_str = right.replace("X", "0");
                let mut or_mask = 0;
                for c in or_str.chars() {
                    or_mask <<= 1;
                    match c {
                        '1' => or_mask += 1,
                        _ => (),
                    }
                }
                op.or_mask = or_mask;
                
                let and_str = right.replace("0", "1");
                let mut and_mask = 0;
                for c in and_str.chars() {
                    and_mask <<= 1;
                    match c {
                        '1' => and_mask += 1,
                        _ => (),
                    }
                }
                op.and_mask = and_mask;

                for c in right.chars().enumerate() {
                    match c.1 {
                        'X' => {
                            op.floating_vec.push(1 << (35 - c.0));
                        }
                        _ => (),
                    }
                }
                // Debug
                //println!("{:b}", op.floating_vec.iter().fold(0, |acc, x| acc + x));
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

    let mut or_mask = 0;
    let mut and_mask = 0;
    let mut floating_vec = vec![];
    let mut memory: HashMap<u64, u64> = HashMap::new();
    for op in operations {
        match op.operation_type {
            OperationType::Mask => {
                or_mask = op.or_mask;
                and_mask = op.and_mask;
                floating_vec = op.floating_vec.clone(); // just clone the vector lol
            }
            OperationType::Memory => {
                let size = floating_vec.len();
                for i in 0..(1 << size) {
                    let mut sum = (op.address & and_mask) | or_mask;
                    for j in 0..size {
                        if ((i >> j) & 1) > 0 {
                            sum += floating_vec[j];
                        }
                    }
                    memory.insert(sum, op.value);
                }
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
    println!("Part Two: {}", part_two().unwrap_or(0));
}
