use crate::helpers::file::{read, write};
use std::collections::HashMap;

pub fn ex16() {
    let e = "16";
    let s = read(e);
    write(e, &sub1(&s).to_string(), &sub2(&s).to_string());
}

#[derive(Debug)]
struct Rule {
    name: String,
    intervals: Vec<(usize, usize)>,
}

impl Rule {
    fn is_valid(&self, value: usize) -> bool {
        for interval in self.intervals.iter() {
            if value >= interval.0 && value <= interval.1 {
                return true;
            }
        }
        false
    }
}

impl From<&str> for Rule {
    fn from(s: &str) -> Self {
        let rule = s.split(": ").collect::<Vec<&str>>();
        let name = rule[0].to_string();
        let intervals = rule[1]
            .split(" or ")
            .map(|interval| {
                let interval = interval.split("-").collect::<Vec<&str>>();
                (
                    interval[0].parse::<usize>().unwrap(),
                    interval[1].parse::<usize>().unwrap(),
                )
            })
            .collect::<Vec<(usize, usize)>>();
        Self { name, intervals }
    }
}

pub fn sub1(s: &str) -> usize {
    let sections = s.split("\n\n").collect::<Vec<&str>>();
    let rules = sections[0]
        .split("\n")
        .map(|rule| Rule::from(rule))
        .collect::<Vec<Rule>>();
    let tickets = sections[2]
        .split("\n")
        .skip(1)
        .map(|ticket| {
            ticket
                .split(",")
                .map(|value| value.parse::<usize>().unwrap())
                .collect::<Vec<usize>>()
        })
        .collect::<Vec<Vec<usize>>>();
    let mut invalid_fields = vec![];
    for ticket in tickets {
        'check_value: for value in ticket {
            for rule in rules.iter() {
                if rule.is_valid(value) {
                    continue 'check_value;
                }
            }
            invalid_fields.push(value)
        }
    }
    invalid_fields.iter().sum::<usize>()
}

fn reduce(values: HashMap<String, Vec<usize>>) -> HashMap<String, usize> {
    let mut new_values = values.to_owned();
    let mut checking = 0;
    let mut alreay_done = vec![];
    while new_values.iter().any(|(_, row)| row.len() > 1) {
        for (_, value) in new_values.iter() {
            checking = value[0];
            if value.len() == 1 && !alreay_done.contains(&checking) {
                alreay_done.push(checking);
                break;
            }
        }
        let mut new_values2 = HashMap::new();
        for (name, row) in new_values.iter_mut() {
            new_values2.insert(
                name.to_owned(),
                if row.len() == 1 {
                    row.to_owned()
                } else {
                    row.iter()
                        .map(|el| *el)
                        .filter(|el| *el != checking)
                        .collect::<Vec<usize>>()
                },
            );
        }
        new_values = new_values2;
    }
    let mut result = HashMap::new();
    for (name, row) in new_values.iter() {
        result.insert(name.to_owned(), row[0]);
    }
    result
}

pub fn sub2(s: &str) -> usize {
    let sections = s.split("\n\n").collect::<Vec<&str>>();
    let rules = sections[0]
        .split("\n")
        .map(|rule| Rule::from(rule))
        .collect::<Vec<Rule>>();
    let your_ticket = sections[1]
        .split("\n")
        .skip(1)
        .map(|ticket| {
            ticket
                .split(",")
                .map(|value| value.parse::<usize>().unwrap())
                .collect::<Vec<usize>>()
        })
        .collect::<Vec<Vec<usize>>>()[0]
        .to_owned();
    let tickets = sections[2]
        .split("\n")
        .skip(1)
        .map(|ticket| {
            ticket
                .split(",")
                .map(|value| value.parse::<usize>().unwrap())
                .collect::<Vec<usize>>()
        })
        .collect::<Vec<Vec<usize>>>();
    let mut valid_tickets: Vec<Vec<usize>> = vec![];
    'check_ticket: for ticket in tickets.iter() {
        'check_value: for value in ticket {
            for rule in rules.iter() {
                if rule.is_valid(*value) {
                    continue 'check_value;
                }
            }
            continue 'check_ticket;
        }
        valid_tickets.push(ticket.to_owned());
    }
    let mut positions: HashMap<String, Vec<usize>> = HashMap::new();
    for rule in rules.iter() {
        for position in 0..rules.len() {
            if valid_tickets
                .iter()
                .all(|ticket| rule.is_valid(ticket[position]))
            {
                match positions.get_mut(&rule.name) {
                    Some(hs) => {
                        hs.push(position);
                    }
                    None => {
                        let mut hs = vec![];
                        hs.push(position);
                        positions.insert(rule.name.to_owned(), hs);
                    }
                };
            }
        }
    }
    let poss = reduce(positions);
    poss.iter()
        .map(|(name, _)| {
            if name.chars().take(9).collect::<String>() == "departure" {
                // index in ticket
                let t_index = poss[name];
                your_ticket[t_index]
            } else {
                1
            }
        })
        .product::<usize>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sub11() {
        assert_eq!(
            sub1(
                "class: 1-3 or 5-7
row: 6-11 or 33-44
seat: 13-40 or 45-50

your ticket:
7,1,14

nearby tickets:
7,3,47
40,4,50
55,2,20
38,6,12"
            ),
            71
        );
    }

    #[test]
    fn test_reduce() {
        let mut hm1 = HashMap::new();
        hm1.insert("1".to_string(), vec![1, 2, 3]);
        hm1.insert("2".to_string(), vec![1, 2]);
        hm1.insert("3".to_string(), vec![1]);
        hm1.insert("4".to_string(), vec![2, 3, 4]);
        let mut hm2 = HashMap::new();
        hm2.insert("1".to_string(), 3);
        hm2.insert("2".to_string(), 2);
        hm2.insert("3".to_string(), 1);
        hm2.insert("4".to_string(), 4);
        assert_eq!(reduce(hm1), hm2);
    }

    #[test]
    fn sub21() {
        assert_eq!(
            sub2(
                "departure_gate: 0-1 or 4-19
departure_time: 0-5 or 8-19
seat: 0-13 or 16-19

your ticket:
11,12,13

nearby tickets:
3,9,18
15,1,5
5,14,9
1,2,20"
            ),
            132
        )
    }
}
