use crate::helpers::file::{read, write};

pub fn ex09() {
    let e = "09";
    let s = read(e);
    write(
        e,
        &sub1(&s, 25).to_string(),
        &sub2(&s, 756008079).to_string(),
    );
}

fn is_valid(numbers: &Vec<usize>, preamble_size: usize, index: usize) -> bool {
    for i in index - preamble_size..index {
        for j in i + 1..index {
            if numbers[i] + numbers[j] == numbers[index] {
                return true;
            }
        }
    }
    false
}

pub fn sub1(s: &str, preamble_size: usize) -> usize {
    let numbers = s
        .split("\n")
        .map(|number| number.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();
    let mut index = preamble_size;
    while is_valid(&numbers, preamble_size, index) {
        index += 1;
    }
    numbers[index]
}

fn contiguous(numbers: &Vec<usize>, invalid_number: usize) -> usize {
    let mut index_min = 0;
    let mut index_max = 0;
    let mut sum = (index_min..index_max).map(|i| numbers[i]).sum::<usize>();
    while sum != invalid_number {
        if sum < invalid_number {
            index_max += 1;
        } else if sum > invalid_number {
            index_min += 1;
            index_max = index_min;
        } else {
            break;
        }
        sum = (index_min..index_max).map(|i| numbers[i]).sum::<usize>();
    }
    let set = (index_min..index_max)
        .map(|i| numbers[i])
        .collect::<Vec<usize>>();
    set.iter().min().unwrap() + set.iter().max().unwrap()
}

pub fn sub2(s: &str, invalid_number: usize) -> usize {
    let numbers = s
        .split("\n")
        .map(|number| number.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();
    contiguous(&numbers, invalid_number)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sub11() {
        assert_eq!(
            sub1(
                "35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576",
                5
            ),
            127
        );
    }

    #[test]
    fn sub21() {
        assert_eq!(
            sub2(
                "35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576",
                127
            ),
            62
        );
    }
}
