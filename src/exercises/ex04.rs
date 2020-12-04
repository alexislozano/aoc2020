use crate::helpers::file::{read, write};
use std::collections::HashMap;

pub fn ex04() {
    let e = "04";
    let s = read(e);
    write(e, &sub1(&s).to_string(), &sub2(&s).to_string());
}

struct Passport;

impl Passport {
    fn validate(s: &str, fields_validation: bool) -> bool {
        let mut hash: HashMap<String, String> = HashMap::new();
        let values = s
            .split("\n")
            .map(|row| {
                row.split(" ")
                    .map(|el| el.split(":").collect::<Vec<&str>>())
                    .collect::<Vec<Vec<&str>>>()
            })
            .collect::<Vec<Vec<Vec<&str>>>>()
            .concat();
        for el in values {
            hash.insert(el[0].to_string(), el[1].to_string());
        }
        match (
            hash.get("ecl"),
            hash.get("pid"),
            hash.get("eyr"),
            hash.get("hcl"),
            hash.get("byr"),
            hash.get("iyr"),
            hash.get("hgt"),
        ) {
            (
                Some(ecl),
                Some(pid),
                Some(eyr),
                Some(hcl),
                Some(byr),
                Some(iyr),
                Some(hgt),
            ) => {
                if fields_validation {
                    Ecl::validate(ecl)
                        && Pid::validate(pid)
                        && Eyr::validate(eyr)
                        && Hcl::validate(hcl)
                        && Byr::validate(byr)
                        && Iyr::validate(iyr)
                        && Hgt::validate(hgt)
                } else {
                    true
                }
            }
            _ => false,
        }
    }
}

struct Ecl;

impl Ecl {
    fn validate(s: &str) -> bool {
        match s {
            "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => true,
            _ => false,
        }
    }
}

struct Pid;

impl Pid {
    fn validate(s: &str) -> bool {
        s.len() == 9 && s.parse::<usize>().is_ok()
    }
}

struct Eyr;

impl Eyr {
    fn validate(s: &str) -> bool {
        s.len() == 4
            && match s.parse::<usize>() {
                Ok(date) => date >= 2020 && date <= 2030,
                _ => false,
            }
    }
}

struct Hcl;

impl Hcl {
    fn validate(s: &str) -> bool {
        s.len() == 7 && s.chars().nth(0).unwrap_or_default() == '#' && {
            for l in s.chars().skip(1) {
                match l {
                    '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8'
                    | '9' | 'a' | 'b' | 'c' | 'd' | 'e' | 'f' => (),
                    _ => return false,
                }
            }
            true
        }
    }
}

struct Byr;

impl Byr {
    fn validate(s: &str) -> bool {
        s.len() == 4
            && match s.parse::<usize>() {
                Ok(date) => date >= 1920 && date <= 2002,
                _ => false,
            }
    }
}

struct Iyr;

impl Iyr {
    fn validate(s: &str) -> bool {
        s.len() == 4
            && match s.parse::<usize>() {
                Ok(date) => date >= 2010 && date <= 2020,
                _ => false,
            }
    }
}

struct Hgt;

impl Hgt {
    fn validate(s: &str) -> bool {
        let (size, unit) =
            (s[..s.len() - 2].to_string(), s[s.len() - 2..].to_string());
        match size.parse::<usize>() {
            Ok(size) => match unit.as_str() {
                "cm" => size >= 150 && size <= 193,
                "in" => size >= 59 && size <= 76,
                _ => false,
            },
            _ => false,
        }
    }
}

pub fn sub1(s: &str) -> usize {
    s.split("\n\n")
        .map(|p| Passport::validate(p, false))
        .filter(|p| *p)
        .count()
}

pub fn sub2(s: &str) -> usize {
    s.split("\n\n")
        .map(|p| Passport::validate(p, true))
        .filter(|p| *p)
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sub11() {
        assert_eq!(sub1("ecl:gry pid:860033327 eyr:2020 hcl:#fffffd\nbyr:1937 iyr:2017 cid:147 hgt:183cm\n\niyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884\nhcl:#cfa07d byr:1929\n\nhcl:#ae17e1 iyr:2013\neyr:2024\necl:brn pid:760753108 byr:1931\nhgt:179cm\n\nhcl:#cfa07d eyr:2025 pid:166559648\niyr:2011 ecl:brn hgt:59in"), 2);
    }

    #[test]
    fn sub21() {
        assert_eq!(sub2("eyr:1972 cid:100\nhcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926\n\niyr:2019\nhcl:#602927 eyr:1967 hgt:170cm\necl:grn pid:012533040 byr:1946\n\nhcl:dab227 iyr:2012\necl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277\n\nhgt:59cm ecl:zzz\neyr:2038 hcl:74454a iyr:2023\npid:3556412378 byr:2007"), 0);
    }

    #[test]
    fn sub22() {
        assert_eq!(sub2("pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980\nhcl:#623a2f\n\neyr:2029 ecl:blu cid:129 byr:1989\niyr:2014 pid:896056539 hcl:#a97842 hgt:165cm\n\nhcl:#888785\nhgt:164cm byr:2001 iyr:2015 cid:88\npid:545766238 ecl:hzl\neyr:2022\n\niyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719"), 4);
    }
}
