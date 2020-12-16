use crate::helpers::file::{read, write};
use std::collections::HashMap;

pub fn ex15() {
    let e = "15";
    let s = read(e);
    write(
        e,
        &sub1(&s, 2020).to_string(),
        &sub1(&s, 30000000).to_string(),
    );
}

pub fn sub1(s: &str, max_count: usize) -> usize {
    let starting_numbers = s.split(",").map(|n| n.parse::<usize>().unwrap());
    let mut h: HashMap<usize, usize> = HashMap::new();
    let mut count = 0;
    let mut last_number = 0;
    for n in starting_numbers {
        last_number = n;
        count += 1;
        h.insert(last_number, count);
    }
    while count < max_count {
        count += 1;
        last_number = match h.get_mut(&last_number) {
            Some(n) => {
                let age = count - *n - 1;
                h.insert(last_number, count - 1);
                age
            }
            None => {
                h.insert(last_number, count - 1);
                0
            }
        };
    }
    last_number
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sub11() {
        assert_eq!(sub1("0,3,6", 2020), 436);
    }

    #[test]
    fn sub12() {
        assert_eq!(sub1("1,3,2", 2020), 1);
    }

    #[test]
    fn sub13() {
        assert_eq!(sub1("2,1,3", 2020), 10);
    }

    #[test]
    fn sub14() {
        assert_eq!(sub1("1,2,3", 2020), 27);
    }

    #[test]
    fn sub15() {
        assert_eq!(sub1("2,3,1", 2020), 78);
    }

    #[test]
    fn sub16() {
        assert_eq!(sub1("3,2,1", 2020), 438);
    }

    #[test]
    fn sub17() {
        assert_eq!(sub1("3,1,2", 2020), 1836);
    }

    #[test]
    fn sub21() {
        assert_eq!(sub1("0,3,6", 30000000), 175594);
    }
}
