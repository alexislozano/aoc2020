use crate::helpers::file::{read, write};
use std::collections::HashMap;

pub fn ex06() {
    let e = "06";
    let s = read(e);
    write(e, &sub1(&s).to_string(), &sub2(&s).to_string());
}

pub fn sub1(s: &str) -> usize {
    s.split("\n\n")
        .map(|group| {
            let mut h: HashMap<char, usize> = HashMap::new();
            for c in group.replace("\n", "").chars() {
                h.insert(
                    c,
                    match h.get(&c) {
                        Some(value) => value + 1,
                        None => 1,
                    },
                );
            }
            h.len()
        })
        .sum::<usize>()
}

pub fn sub2(s: &str) -> usize {
    s.split("\n\n")
        .map(|group| {
            let mut h: HashMap<char, usize> = HashMap::new();
            for c in group.replace("\n", "").chars() {
                h.insert(
                    c,
                    match h.get(&c) {
                        Some(value) => value + 1,
                        None => 1,
                    },
                );
            }
            h.values()
                .filter(|v| **v == group.split("\n").count())
                .count()
        })
        .sum::<usize>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sub11() {
        assert_eq!(sub1("abcx\nabcy\nabcz"), 6);
    }

    #[test]
    fn sub12() {
        assert_eq!(sub1("abc\n\na\nb\nc\n\nab\nac\n\na\na\na\na\n\nb"), 11);
    }

    #[test]
    fn sub21() {
        assert_eq!(sub2("abcx\nabcy\nabcz"), 3);
    }

    #[test]
    fn sub22() {
        assert_eq!(sub2("abc\n\na\nb\nc\n\nab\nac\n\na\na\na\na\n\nb"), 6);
    }
}
