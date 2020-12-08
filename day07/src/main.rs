use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

/*
--- Day 7: Handy Haversacks ---

You land at the regional airport in time for your next flight. In fact, it looks like you'll even
have time to grab some food: all flights are currently delayed due to issues in luggage processing.

Due to recent aviation regulations, many rules (your puzzle input) are being enforced about bags and
their contents; bags must be color-coded and must contain specific quantities of other color-coded
bags. Apparently, nobody responsible for these regulations considered how long they would take to
enforce!

For example, consider the following rules:

light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.

These rules specify the required contents for 9 bag types. In this example, every faded blue bag is
empty, every vibrant plum bag contains 11 bags (5 faded blue and 6 dotted black), and so on.

You have a shiny gold bag. If you wanted to carry it in at least one other bag, how many different
bag colors would be valid for the outermost bag? (In other words: how many colors can, eventually,
contain at least one shiny gold bag?)

In the above rules, the following options would be available to you:

    A bright white bag, which can hold your shiny gold bag directly.
    A muted yellow bag, which can hold your shiny gold bag directly, plus some other bags.
    A dark orange bag, which can hold bright white and muted yellow bags, either of which could then
    hold your shiny gold bag.
    A light red bag, which can hold bright white and muted yellow bags, either of which could then
    hold your shiny gold bag.

So, in this example, the number of bag colors that can eventually contain at least one shiny gold
bag is 4.

How many bag colors can eventually contain at least one shiny gold bag? (The list of rules is quite
long; make sure you get all of it.)
*/

fn expand(map: &HashMap<String, HashSet<String>>, set: &HashSet<String>) -> HashSet<String> {
    let mut result = set.clone();
    for bag in set {
        if let Some(list) = map.get(bag) {
            result.extend(expand(map, list));
        }
    }

    result
}

fn part_one() -> std::io::Result<usize> {
    let file = File::open("input")?;

    let mut reverse_search = HashMap::new();

    let g = BufReader::new(file)
        .lines()
        .filter(|p| !p.as_ref().unwrap().ends_with("no other bags."));
    for line in g {
        if let Ok(bag) = line {
            let mut iter = bag
                .split(' ')
                .map(|p| p.replace(&[',', '.'][..], ""))
                .filter(|p| p != "bags" && p != "bag" && p != "contain")
                .peekable();

            let container = iter.next().unwrap() + " " + &iter.next().unwrap();
            //println!("{:?}", container);
            while iter.peek().is_some() {
                let _count = iter.next();
                let contents = iter.next().unwrap() + " " + &iter.next().unwrap();

                let reverse_set = reverse_search
                    .entry(contents.clone())
                    .or_insert(HashSet::new());
                reverse_set.insert(container.clone());
            }
        }
    }

    let mut result = 0;
    if let Some(a) = reverse_search.get("shiny gold") {
        result = expand(&reverse_search, a).len();
    }

    Ok(result)
}

/*
--- Part Two ---

It's getting pretty expensive to fly these days - not because of ticket prices, but because of the
ridiculous number of bags you need to buy!

Consider again your shiny gold bag and the rules from the above example:

    faded blue bags contain 0 other bags.
    dotted black bags contain 0 other bags.
    vibrant plum bags contain 11 other bags: 5 faded blue bags and 6 dotted black bags.
    dark olive bags contain 7 other bags: 3 faded blue bags and 4 dotted black bags.

So, a single shiny gold bag must contain 1 dark olive bag (and the 7 bags within it) plus 2 vibrant
plum bags (and the 11 bags within each of those): 1 + 1*7 + 2 + 2*11 = 32 bags!

Of course, the actual rules have a small chance of going several levels deeper than this example; be
sure to count all of the bags, even if the nesting becomes topologically impractical!

Here's another example:

shiny gold bags contain 2 dark red bags.
dark red bags contain 2 dark orange bags.
dark orange bags contain 2 dark yellow bags.
dark yellow bags contain 2 dark green bags.
dark green bags contain 2 dark blue bags.
dark blue bags contain 2 dark violet bags.
dark violet bags contain no other bags.

In this example, a single shiny gold bag must contain 126 other bags.

How many individual bags are required inside your single shiny gold bag?
*/
fn count_bags(
    map: &HashMap<String, HashSet<(String, usize)>>,
    set: &HashSet<(String, usize)>,
) -> usize {
    let mut result = 0;
    for tuple in set {
        result += tuple.1;
        if let Some(list) = map.get(&tuple.0) {
            result += tuple.1 * count_bags(map, list);
        }
    }

    result
}

fn part_two() -> std::io::Result<usize> {
    let file = File::open("input")?;

    let mut reverse_search = HashMap::new();

    let g = BufReader::new(file)
        .lines()
        .filter(|p| !p.as_ref().unwrap().ends_with("no other bags."));
    for line in g {
        if let Ok(bag) = line {
            let mut iter = bag
                .split(' ')
                .map(|p| p.replace(&[',', '.'][..], ""))
                .filter(|p| p != "bags" && p != "bag" && p != "contain")
                .peekable();

            let container = iter.next().unwrap() + " " + &iter.next().unwrap();
            //println!("{:?}", container);
            while iter.peek().is_some() {
                let count = iter.next().unwrap();
                let contents = iter.next().unwrap() + " " + &iter.next().unwrap();

                let reverse_set = reverse_search
                    .entry(container.clone())
                    .or_insert(HashSet::new());
                reverse_set.insert((contents.clone(), count.parse::<usize>().unwrap()));
            }
        }
    }

    let mut result = 0;
    if let Some(a) = reverse_search.get("shiny gold") {
        result = count_bags(&reverse_search, a);
    }

    Ok(result)
}

fn main() {
    println!("=== Advent of Code Day 7 ===");
    println!("Part One: {}", part_one().unwrap_or(0));
    println!("Part Two: {}", part_two().unwrap_or(0));
}
