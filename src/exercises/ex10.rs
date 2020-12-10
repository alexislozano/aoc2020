use crate::helpers::file::{read, write};
use std::collections::HashMap;

pub fn ex10() {
    let e = "10";
    let s = read(e);
    write(e, &sub1(&s).to_string(), &sub2(&s).to_string());
}

pub fn sub1(s: &str) -> usize {
    let mut ratings = s
        .split("\n")
        .map(|rating| rating.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();
    ratings.sort();
    let mut current_rating = 0;
    let mut one_jolt_diff = 0;
    let mut three_jolt_diff = 0;
    for rating in ratings.iter() {
        match rating - current_rating {
            1 => one_jolt_diff += 1,
            3 => three_jolt_diff += 1,
            _ => (),
        };
        current_rating = *rating;
    }
    three_jolt_diff += 1;
    one_jolt_diff * three_jolt_diff
}

fn count_paths(
    hash: &HashMap<usize, Vec<usize>>,
    from: &usize,
    to: &usize,
) -> usize {
    if from == to {
        1
    } else {
        match hash.get(from) {
            Some(children) => {
                let max = children.iter().max().unwrap();
                children
                    .iter()
                    .map(|child| count_paths(hash, child, max))
                    .sum::<usize>()
            }
            None => unreachable!(),
        }
    }
}

pub fn sub2(s: &str) -> usize {
    let mut ratings = s
        .split("\n")
        .map(|rating| rating.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();
    ratings.push(0);
    ratings.sort();
    let mut hash: HashMap<usize, Vec<usize>> = HashMap::new();
    for (index, rating) in ratings.iter().enumerate() {
        let children = ratings
            .iter()
            .skip(index + 1)
            .take(3)
            .filter(|r| *r - rating <= 3)
            .map(|r| *r)
            .collect::<Vec<usize>>();
        hash.insert(*rating, children.to_owned());
    }
    let mut result = 1;
    let mut from = 0;
    loop {
        match hash.get(&from) {
            None => unreachable!(),
            Some(children) => {
                if children.len() == 0 {
                    break;
                } else {
                    let max = children.iter().max().unwrap();
                    result *= count_paths(&hash, &from, max);
                    from = *max;
                }
            }
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sub11() {
        assert_eq!(
            sub1(
                "16
10
15
5
1
11
7
19
6
12
4"
            ),
            35
        );
    }

    #[test]
    fn sub12() {
        assert_eq!(
            sub1(
                "28
33
18
42
31
14
46
20
48
47
24
23
49
45
19
38
39
11
1
32
25
35
8
17
7
9
4
2
34
10
3"
            ),
            220
        );
    }

    #[test]
    fn sub21() {
        assert_eq!(
            sub2(
                "16
10
15
5
1
11
7
19
6
12
4"
            ),
            8
        );
    }

    #[test]
    fn sub22() {
        assert_eq!(
            sub2(
                "28
33
18
42
31
14
46
20
48
47
24
23
49
45
19
38
39
11
1
32
25
35
8
17
7
9
4
2
34
10
3"
            ),
            19208
        );
    }
}
