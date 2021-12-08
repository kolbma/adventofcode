#![deny(clippy::all)]
#![deny(unsafe_code)]
#![deny(warnings)]

use std::{fs::File, io::Read};

const INPUT_FILE: &str = "./data/input";

const PASSPORT_FIELDS: &[&str] = &["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid", "cid"];
const EYE_COLOR: &[&str] = &["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];

fn main() {
    let input = get_input(INPUT_FILE);

    let count = count_valid_passports(&input, false);
    println!("valid passports: {}", count);

    let count = count_valid_passports(&input, true);
    println!("valid passports part 2: {}", count);
}

#[inline(always)]
fn get_input(path: &str) -> String {
    // read text file
    let mut file = File::open(path).expect("file not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("something went wrong reading the file");
    contents
}

fn count_valid_passports(input: &str, is_part2: bool) -> usize {
    let mut check_fields = 0u8;
    let mut nr = 0usize;
    input
        .lines()
        .filter(|&line| {
            nr += 1;
            let cur_check_fields = parse_line(line, is_part2);
            if cur_check_fields == 0 {
                check_fields = 0;
            } else {
                check_fields |= cur_check_fields;
            }
            if check_fields == 0x7f || check_fields == 0xff {
                check_fields = 0;
                true
            } else {
                false
            }
        })
        .count()
}

#[inline(always)]
fn parse_line(line: &str, is_part2: bool) -> u8 {
    let mut check_fields = 0u8;

    line.trim().split(' ').for_each(|field| {
        let fields = field.split(':').collect::<Vec<&str>>();
        let field = fields[0];
        if let Some(pos) = PASSPORT_FIELDS.iter().position(|&x| x == field) {
            if !is_part2 || is_field_valid(field, fields[1]) {
                check_fields |= 1 << pos;
            }
        }
    });

    check_fields
}

#[inline(always)]
fn is_field_valid(field: &str, value: &str) -> bool {
    match field {
        "byr" => {
            let value = value.parse::<u16>().unwrap_or_default();
            value >= 1920 && value <= 2002
        }
        "iyr" => {
            let value = value.parse::<u16>().unwrap_or_default();
            value >= 2010 && value <= 2020
        }
        "eyr" => {
            let value = value.parse::<u16>().unwrap_or_default();
            value >= 2020 && value <= 2030
        }
        "hgt" => {
            if let Some(cm) = value.strip_suffix("cm") {
                let value = cm.parse::<u16>().unwrap_or_default();
                value >= 150 && value <= 193
            } else if let Some(inch) = value.strip_suffix("in") {
                let value = inch.parse::<u16>().unwrap_or_default();
                value >= 59 && value <= 76
            } else {
                false
            }
        }
        "hcl" => {
            if let Some(color) = value.strip_prefix("#") {
                color.len() == 6
                    && color
                        .find(|c| c < '0' || (c > '9' && c < 'a') || c > 'f')
                        .is_none()
            } else {
                false
            }
        }
        "ecl" => EYE_COLOR.contains(&value),
        "pid" => value.matches(char::is_numeric).count() == 9,
        "cid" => true,
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ["byr=0 =>0x01", "iyr=1 =>0x02", "eyr=2 =>0x04", "hgt=3 =>0x08",
    //  "hcl=4 =>0x10", "ecl=5 =>0x20", "pid=6 =>0x40", "cid=7 =>0x80"]
    const TEST_INPUT: &str = r" ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
                                byr:1937 iyr:2017 cid:147 hgt:183cm
                                
                                iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
                                hcl:#cfa07d byr:1929
                                
                                hcl:#ae17e1 iyr:2013
                                eyr:2024
                                ecl:brn pid:760753108 byr:1931
                                hgt:179cm
                                
                                hcl:#cfa07d eyr:2025 pid:166559648
                                iyr:2011 ecl:brn hgt:59in";

    #[test]
    fn test_get_input() {
        let input = get_input(INPUT_FILE);
        assert!(input.len() > 0);
    }

    #[test]
    fn test_count_valid_passports() {
        let count = count_valid_passports(TEST_INPUT, false);
        assert_eq!(count, 2);
    }

    #[test]
    fn test_parse_line() {
        let check_fields = parse_line(TEST_INPUT.lines().collect::<Vec<&str>>()[0], false);
        assert_eq!(check_fields, 0x74);

        let check_fields = parse_line(TEST_INPUT.lines().collect::<Vec<&str>>()[9], false);
        assert_eq!(check_fields, 0x08);

        let check_fields = parse_line(TEST_INPUT.lines().collect::<Vec<&str>>()[10], false);
        assert_eq!(check_fields, 0x0);

        let check_fields = parse_line(TEST_INPUT.lines().collect::<Vec<&str>>()[11], false);
        assert_eq!(check_fields, 0x54);

        let check_fields = parse_line(TEST_INPUT.lines().collect::<Vec<&str>>()[12], false);
        assert_eq!(check_fields, 0x2A);
    }

    #[test]
    fn test_is_field_valid() {
        assert!(is_field_valid("byr", "1920"));
        assert!(is_field_valid("byr", "2002"));
        assert!(!is_field_valid("byr", "1919"));
        assert!(!is_field_valid("byr", "2003"));

        assert!(is_field_valid("iyr", "2010"));
        assert!(is_field_valid("iyr", "2020"));
        assert!(!is_field_valid("iyr", "2009"));
        assert!(!is_field_valid("iyr", "2021"));

        assert!(is_field_valid("eyr", "2020"));
        assert!(is_field_valid("eyr", "2030"));
        assert!(!is_field_valid("eyr", "2019"));
        assert!(!is_field_valid("eyr", "2031"));

        assert!(is_field_valid("hgt", "150cm"));
        assert!(is_field_valid("hgt", "193cm"));
        assert!(!is_field_valid("hgt", "149cm"));
        assert!(!is_field_valid("hgt", "194cm"));
        assert!(!is_field_valid("hgt", "170"));

        assert!(is_field_valid("hgt", "59in"));
        assert!(is_field_valid("hgt", "76in"));
        assert!(!is_field_valid("hgt", "58in"));
        assert!(!is_field_valid("hgt", "77in"));
        assert!(!is_field_valid("hgt", "60"));

        assert!(is_field_valid("hcl", "#ffffff"));
        assert!(is_field_valid("hcl", "#000000"));
        assert!(is_field_valid("hcl", "#0a9d1f"));
        assert!(!is_field_valid("hcl", "000000"));
        assert!(!is_field_valid("hcl", "ffffff"));
        assert!(!is_field_valid("hcl", "#000"));
        assert!(!is_field_valid("hcl", "#fffffv"));

        for ecl in EYE_COLOR {
            assert!(is_field_valid("ecl", ecl));
        }
        assert!(!is_field_valid("ecl", "err"));

        assert!(is_field_valid("pid", "023456789"));
        assert!(!is_field_valid("pid", "12345678"));
        assert!(!is_field_valid("pid", "12345678a"));
        assert!(!is_field_valid("pid", "0123456789"));
    }

    #[test]
    fn test_invalid_passports() {
        const TEST_INVALID_PASSPORTS: &str = r"
                eyr:1972 cid:100
                hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926
                
                iyr:2019
                hcl:#602927 eyr:1967 hgt:170cm
                ecl:grn pid:012533040 byr:1946
                
                hcl:dab227 iyr:2012
                ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277
                
                hgt:59cm ecl:zzz
                eyr:2038 hcl:74454a iyr:2023
                pid:3556412378 byr:2007";

        let count = count_valid_passports(TEST_INVALID_PASSPORTS, true);
        assert_eq!(count, 0);
    }

    #[test]
    fn test_valid_passports() {
        const TEST_VALID_PASSPORTS: &str = r"
                pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
                hcl:#623a2f
                
                eyr:2029 ecl:blu cid:129 byr:1989
                iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm
                
                hcl:#888785
                hgt:164cm byr:2001 iyr:2015 cid:88
                pid:545766238 ecl:hzl
                eyr:2022
                
                iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719";

        let count = count_valid_passports(TEST_VALID_PASSPORTS, true);
        assert_eq!(count, 4);
    }
}
