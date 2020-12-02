use crate::helpers::file::{read, write};

pub fn ex02() {
    let e = "02";
    let s = read(e);
    write(e, &sub1(&s).to_string(), &sub2(&s).to_string());
}

struct PolicyPassword {
    min: u8,
    max: u8,
    letter: char,
    password: String,
}

pub fn sub1(s: &str) -> usize {
    s.split("\n")
        .map(|pp| {
            let values = pp.split(" ").collect::<Vec<&str>>();
            let minmax = values[0]
                .split("-")
                .map(|x| x.parse::<u8>().unwrap())
                .collect::<Vec<u8>>();
            let min = minmax[0];
            let max = minmax[1];
            let letter = values[1].chars().nth(0).unwrap();
            let password = values[2].to_string();
            PolicyPassword {
                min,
                max,
                letter,
                password,
            }
        })
        .filter(|pp| {
            let c = pp.password.chars().filter(|l| *l == pp.letter).count();
            c >= pp.min as usize && c <= pp.max as usize
        })
        .collect::<Vec<PolicyPassword>>()
        .len()
}

pub fn sub2(s: &str) -> usize {
    s.split("\n")
        .map(|pp| {
            let values = pp.split(" ").collect::<Vec<&str>>();
            let minmax = values[0]
                .split("-")
                .map(|x| x.parse::<u8>().unwrap())
                .collect::<Vec<u8>>();
            let min = minmax[0];
            let max = minmax[1];
            let letter = values[1].chars().nth(0).unwrap();
            let password = values[2].to_string();
            PolicyPassword {
                min,
                max,
                letter,
                password,
            }
        })
        .filter(|pp| {
            let first = pp.password.chars().nth(pp.min as usize - 1).unwrap();
            let end = pp.password.chars().nth(pp.max as usize - 1).unwrap();
            (first == pp.letter) ^ (end == pp.letter)
        })
        .collect::<Vec<PolicyPassword>>()
        .len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sub11() {
        assert_eq!(sub1("1-3 a: abcde\n1-3 b: cdefg\n2-9 c: ccccccccc"), 2);
    }

    #[test]
    fn sub21() {
        assert_eq!(sub2("1-3 a: abcde\n1-3 b: cdefg\n2-9 c: ccccccccc"), 1);
    }
}
