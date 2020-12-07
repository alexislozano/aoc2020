use crate::helpers::file::{read, write};
use std::collections::HashMap;

pub fn ex07() {
    let e = "07";
    let s = read(e);
    write(
        e,
        &sub1(&s, "shiny gold").to_string(),
        &sub2(&s, "shiny gold").to_string(),
    );
}

fn parse_sentence(s: &str) -> (String, HashMap<String, usize>) {
    let mut children: HashMap<String, usize> = HashMap::new();
    let s = s.split(" bags contain ").collect::<Vec<&str>>();
    let color = s[0];
    if s[1] == "no other bags." {
        (color.to_string(), children)
    } else {
        let s = s[1].split(", ").collect::<Vec<&str>>();
        for c in s.iter() {
            let words = c.split(" ").collect::<Vec<&str>>();
            let number = words[0].parse::<usize>().unwrap();
            let color = format!("{} {}", words[1], words[2]);
            children.insert(color, number);
        }
        (color.to_string(), children)
    }
}

fn parse_text_to_parents(s: &str) -> HashMap<String, HashMap<String, usize>> {
    let mut result: HashMap<String, HashMap<String, usize>> = HashMap::new();
    for sentence in s.split("\n") {
        let (parent, children) = parse_sentence(sentence);
        for (child, number) in children.iter() {
            match result.get_mut(child) {
                Some(h) => {
                    h.insert(parent.to_string(), *number);
                }
                None => {
                    let mut h: HashMap<String, usize> = HashMap::new();
                    h.insert(parent.to_string(), *number);
                    result.insert(child.to_string(), h);
                }
            }
        }
    }
    result
}

fn unpack(
    parents: &HashMap<String, HashMap<String, usize>>,
    color: &str,
    already_checked: &mut Vec<String>,
) -> usize {
    match parents.get(color) {
        Some(ps) => ps
            .iter()
            .map(|(color, _)| {
                if already_checked.contains(color) {
                    0
                } else {
                    already_checked.push(color.to_string());
                    1 + unpack(parents, color, already_checked)
                }
            })
            .sum(),
        None => 0,
    }
}

pub fn sub1(s: &str, color: &str) -> usize {
    let parents = parse_text_to_parents(s);
    let mut already_checked: Vec<String> = vec![];
    unpack(&parents, color, &mut already_checked)
}

fn parse_text_to_children(s: &str) -> HashMap<String, HashMap<String, usize>> {
    let mut result: HashMap<String, HashMap<String, usize>> = HashMap::new();
    for sentence in s.split("\n") {
        let (parent, children) = parse_sentence(sentence);
        for (child, number) in children.iter() {
            match result.get_mut(&parent) {
                Some(h) => {
                    h.insert(child.to_string(), *number);
                }
                None => {
                    let mut h: HashMap<String, usize> = HashMap::new();
                    h.insert(child.to_string(), *number);
                    result.insert(parent.to_string(), h);
                }
            }
        }
    }
    result
}

fn pack(
    children: &HashMap<String, HashMap<String, usize>>,
    color: &str,
) -> usize {
    match children.get(color) {
        Some(cs) => cs
            .iter()
            .map(|(color, number)| number + (number * pack(children, color)))
            .sum(),
        None => 0,
    }
}

pub fn sub2(s: &str, color: &str) -> usize {
    let children = parse_text_to_children(s);
    pack(&children, color)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constraint0() {
        let h: HashMap<String, usize> = HashMap::new();
        assert_eq!(
            parse_sentence("light red bags contain no other bags."),
            ("light red".to_string(), h)
        );
    }

    #[test]
    fn constraint1() {
        let mut h: HashMap<String, usize> = HashMap::new();
        h.insert("bright white".to_string(), 1);
        assert_eq!(
            parse_sentence("light red bags contain 1 bright white bag."),
            ("light red".to_string(), h)
        );
    }

    #[test]
    fn constraint2() {
        let mut h: HashMap<String, usize> = HashMap::new();
        h.insert("bright white".to_string(), 1);
        h.insert("muted yellow".to_string(), 2);
        assert_eq!(
            parse_sentence("light red bags contain 1 bright white bag, 2 muted yellow bags."),
            ("light red".to_string(), h)
        );
    }

    #[test]
    fn sub11() {
        assert_eq!(sub1("light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.", "shiny gold"), 4);
    }

    #[test]
    fn sub21() {
        assert_eq!(
            sub2(
                "shiny gold bags contain 2 dark red bags.
dark red bags contain 2 dark orange bags.
dark orange bags contain 2 dark yellow bags.
dark yellow bags contain 2 dark green bags.
dark green bags contain 2 dark blue bags.
dark blue bags contain 2 dark violet bags.
dark violet bags contain no other bags.",
                "shiny gold"
            ),
            126
        )
    }
}
