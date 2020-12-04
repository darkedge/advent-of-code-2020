/*
--- Day 4: Passport Processing ---

You arrive at the airport only to realize that you grabbed your North Pole Credentials instead of
your passport. While these documents are extremely similar, North Pole Credentials aren't issued by
a country and therefore aren't actually valid documentation for travel in most of the world.

It seems like you're not the only one having problems, though; a very long line has formed for the
automatic passport scanners, and the delay could upset your travel itinerary.

Due to some questionable network security, you realize you might be able to solve both of these
problems at the same time.

The automatic passport scanners are slow because they're having trouble detecting which passports
have all required fields. The expected fields are as follows:

    byr (Birth Year)
    iyr (Issue Year)
    eyr (Expiration Year)
    hgt (Height)
    hcl (Hair Color)
    ecl (Eye Color)
    pid (Passport ID)
    cid (Country ID)

Passport data is validated in batch files (your puzzle input). Each passport is represented as a
sequence of key:value pairs separated by spaces or newlines. Passports are separated by blank lines.

Here is an example batch file containing four passports:

ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in

The first passport is valid - all eight fields are present. The second passport is invalid - it is
missing hgt (the Height field).

The third passport is interesting; the only missing field is cid, so it looks like data from North
Pole Credentials, not a passport at all! Surely, nobody would mind if you made the system
temporarily ignore missing cid fields. Treat this "passport" as valid.

The fourth passport is missing two fields, cid and byr. Missing cid is fine, but missing any other
field is not, so this passport is invalid.

According to the above rules, your improved system would report 2 valid passports.

Count the number of valid passports - those that have all required fields. Treat cid as optional.
In your batch file, how many passports are valid?
*/
use std::fs::File;
use std::io::prelude::*;

struct Passport<'a> {
    birth_year: Option<&'a str>,      // byr
    issue_year: Option<&'a str>,      // iyr
    expiration_year: Option<&'a str>, // eyr
    height: Option<&'a str>,          // hgt
    hair_color: Option<&'a str>,      // hcl
    eye_color: Option<&'a str>,       // ecl
    passport_id: Option<&'a str>,     // pid
    country_id: Option<&'a str>,      // cid
}

fn parse_passports<'a>(list: &'a Vec<&'a str>) -> Vec<Passport> {
    let mut passports = Vec::new();

    for entry in list {
        let mut birth_year: Option<&str> = None;
        let mut issue_year: Option<&str> = None;
        let mut expiration_year: Option<&str> = None;
        let mut height: Option<&str> = None;
        let mut hair_color: Option<&str> = None;
        let mut eye_color: Option<&str> = None;
        let mut passport_id: Option<&str> = None;
        let mut country_id: Option<&str> = None;

        /*
         * pid:8729818647 hcl:z
         * ecl:#ae70eb cid:168 hgt:161cm iyr:2030
         * eyr:2020 byr:2022
         * (possible newline)
         */
        let fields = entry.split(|c| c == ' ' || c == '\n');
        for field in fields {
            if field.is_empty() {
                continue;
            }
            // pid:8729818647
            //println!("parsing field: {}", field);
            let mut tokens = field.split(':');
            let key = tokens.next();
            match key {
                Some("byr") => birth_year = tokens.next(),
                Some("iyr") => issue_year = tokens.next(),
                Some("eyr") => expiration_year = tokens.next(),
                Some("hgt") => height = tokens.next(),
                Some("hcl") => hair_color = tokens.next(),
                Some("ecl") => eye_color = tokens.next(),
                Some("pid") => passport_id = tokens.next(),
                Some("cid") => country_id = tokens.next(),
                Some(x) => println!("Found invalid token: {}", x),
                _ => println!("Parse error!"),
            }
        }

        let passport = Passport {
            birth_year,
            issue_year,
            expiration_year,
            height,
            hair_color,
            eye_color,
            passport_id,
            country_id,
        };

        passports.push(passport);
    }

    return passports;
}

fn part_one() {
    if let Ok(mut file) = File::open("input") {
        // Read the whole file into a string.
        let mut contents = String::new();
        let _ = file.read_to_string(&mut contents);

        // Passports seem to be delimited by two newlines.
        // Create a vector of strings for passports.
        let passports: Vec<&str> = contents.split("\n\n").collect();
        println!("Number of passports: {}", passports.len());

        // Now we can split each password into fields.
        let mut parsed = parse_passports(&passports);

        // Valid passports must contain certain values
        parsed.retain(|x| {
            x.birth_year.is_some()
                && x.issue_year.is_some()
                && x.expiration_year.is_some()
                && x.height.is_some()
                && x.hair_color.is_some()
                && x.eye_color.is_some()
                && x.passport_id.is_some()
        });

        println!("Number of valid passports: {}", parsed.len());
    }
}

fn part_two() {}

fn main() {
    println!("=== Advent of Code Day 4 ===");
    part_one();
    part_two();
}
