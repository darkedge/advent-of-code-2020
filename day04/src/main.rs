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
use std::fmt;
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

impl fmt::Debug for Passport<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Point")
            .field("birth_year", &self.birth_year)
            .field("issue_year", &self.issue_year)
            .field("expiration_year", &self.expiration_year)
            .field("height", &self.height)
            .field("hair_color", &self.hair_color)
            .field("eye_color", &self.eye_color)
            .field("passport_id", &self.passport_id)
            .field("country_id", &self.country_id)
            .finish()
    }
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

fn part_one(passports: &Vec<Passport>) -> i32 {
    let mut count = 0;

    for x in passports {
        if x.birth_year.is_some()
            && x.issue_year.is_some()
            && x.expiration_year.is_some()
            && x.height.is_some()
            && x.hair_color.is_some()
            && x.eye_color.is_some()
            && x.passport_id.is_some()
        {
            count += 1;
        }
    }

    count
}

fn validate(validate: &dyn Fn(&Vec<Passport>) -> i32) {
    if let Ok(mut file) = File::open("input") {
        // Read the whole file into a string.
        let mut contents = String::new();
        let _ = file.read_to_string(&mut contents);

        // Passports seem to be delimited by two newlines.
        // Create a vector of strings for passports.
        let passports: Vec<&str> = contents.split("\n\n").collect();
        println!("Number of passports: {}", passports.len());

        // Now we can split each password into fields.
        let parsed = parse_passports(&passports);

        // Valid passports must contain certain values
        println!("Number of valid passports: {}", validate(&parsed));
    }
}

/*
--- Part Two ---

The line is moving more quickly now, but you overhear airport security talking about how passports
with invalid data are getting through. Better add some data validation, quick!

You can continue to ignore the cid field, but each other field has strict rules about what values
are valid for automatic validation:

    byr (Birth Year) - four digits; at least 1920 and at most 2002.
    iyr (Issue Year) - four digits; at least 2010 and at most 2020.
    eyr (Expiration Year) - four digits; at least 2020 and at most 2030.
    hgt (Height) - a number followed by either cm or in:
        If cm, the number must be at least 150 and at most 193.
        If in, the number must be at least 59 and at most 76.
    hcl (Hair Color) - a # followed by exactly six characters 0-9 or a-f.
    ecl (Eye Color) - exactly one of: amb blu brn gry grn hzl oth.
    pid (Passport ID) - a nine-digit number, including leading zeroes.
    cid (Country ID) - ignored, missing or not.

Your job is to count the passports where all required fields are both present and valid according to
the above rules. Here are some example values:

byr valid:   2002
byr invalid: 2003

hgt valid:   60in
hgt valid:   190cm
hgt invalid: 190in
hgt invalid: 190

hcl valid:   #123abc
hcl invalid: #123abz
hcl invalid: 123abc

ecl valid:   brn
ecl invalid: wat

pid valid:   000000001
pid invalid: 0123456789

Here are some invalid passports:

eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946

hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007

Here are some valid passports:

pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719

Count the number of valid passports - those that have all required fields and valid values. Continue
to treat cid as optional. In your batch file, how many passports are valid?
*/
use regex::Regex;

fn regex_is_match<'t>(
    option_string: Option<&'t str>,
    re: &regex::Regex,
) -> Option<regex::Match<'t>> {
    if let Some(string) = option_string {
        //println!("String exists: {}", string);
        if let Some(captures) = re.captures(string) {
            return captures.get(1);
        }
    }

    None
}

fn regex_match(option_string: Option<&str>, re: &regex::Regex, min: i32, max: i32) -> bool {
    if let Some(match1) = regex_is_match(option_string, re) {
        //println!("Regex passed");
        if let Ok(val) = match1.as_str().parse::<i32>() {
            //println!("Checking if {} is between {} and {}", val, min, max);
            return val >= min && val <= max;
        }
    }

    false
}

fn part_two(parsed: &Vec<Passport>) -> i32 {
    /*
    byr (Birth Year) - four digits; at least 1920 and at most 2002.
    iyr (Issue Year) - four digits; at least 2010 and at most 2020.
    eyr (Expiration Year) - four digits; at least 2020 and at most 2030.
    hgt (Height) - a number followed by either cm or in:
        If cm, the number must be at least 150 and at most 193.
        If in, the number must be at least 59 and at most 76.
    hcl (Hair Color) - a # followed by exactly six characters 0-9 or a-f.
    ecl (Eye Color) - exactly one of: amb blu brn gry grn hzl oth.
    pid (Passport ID) - a nine-digit number, including leading zeroes.
    cid (Country ID) - ignored, missing or not.
    */
    // byr, iyr, eyr
    let re_four_digits = Regex::new(r"(\d{4})").unwrap();
    let re_height_cm = Regex::new(r"(\d+)cm").unwrap();
    let re_height_in = Regex::new(r"(\d+)in").unwrap();
    let re_hair_color = Regex::new(r"#([\da-f]{6})").unwrap();
    // we're gonna use matching for the eye color.
    let re_passport_id = Regex::new(r"(\b\d{9}\b)").unwrap();

    let mut num_valid = 0;
    for passport in parsed {
        //println!("\nValidating passport: {:?}", passport);
        if !regex_match(passport.birth_year, &re_four_digits, 1920, 2002) {
            //println!("Failing birth_year {:?}", passport.birth_year);
            continue;
        }
        if !regex_match(passport.issue_year, &re_four_digits, 2010, 2020) {
            //println!("Failing issue_year {:?}", passport.issue_year);
            continue;
        }
        if !regex_match(passport.expiration_year, &re_four_digits, 2020, 2030) {
            //println!("Failing expiration_year {:?}", passport.expiration_year);
            continue;
        }
        if !(regex_match(passport.height, &re_height_cm, 150, 193)
            || regex_match(passport.height, &re_height_in, 59, 76))
        {
            //println!("Failing height {:?}", passport.height);
            continue;
        }
        if regex_is_match(passport.hair_color, &re_hair_color).is_none() {
            //println!("Failing hair_color {:?}", passport.hair_color);
            continue;
        }
        match passport.eye_color {
            Some("amb") => {}
            Some("blu") => {}
            Some("brn") => {}
            Some("gry") => {}
            Some("grn") => {}
            Some("hzl") => {}
            Some("oth") => {}
            _ => {
                //println!("Failing eye_color {:?}", passport.eye_color);
                continue;
            }
        }
        if regex_is_match(passport.passport_id, &re_passport_id).is_none() {
            //println!("Failing passport_id {:?}", passport.passport_id);
            continue;
        }
        println!("Passport OK: {:?}", passport);

        num_valid += 1;
    }

    num_valid
}

fn main() {
    println!("=== Advent of Code Day 4 ===");
    println!("= Part One =");
    validate(&part_one);
    println!("= Part Two =");
    validate(&part_two);
}
