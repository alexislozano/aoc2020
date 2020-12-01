use crate::helpers::file::{read, write};

pub fn ex01() {
    let e = "01";
    let s = read(e);
    write(e, &sub1(&s).to_string(), &sub2(&s).to_string());
}

pub fn sub1(s: &str) -> usize {
    let mut numbers = s
        .split("\n")
        .map(|n| n.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();
    numbers.sort();
    let mut result = 0;
    for i in 0..numbers.len() {
        let ni = numbers[i];
        for j in (i + 1)..numbers.len() {
            let nj = numbers[j];
            if ni + nj > 2020 {
                break;
            } else if ni + nj == 2020 {
                result = ni * nj
            }
        }
    }
    result
}

pub fn sub2(s: &str) -> usize {
    let mut numbers = s
        .split("\n")
        .map(|n| n.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();
    numbers.sort();
    let mut result = 0;
    for i in 0..numbers.len() {
        let ni = numbers[i];
        for j in (i + 1)..numbers.len() {
            let nj = numbers[j];
            if ni + nj > 2020 {
                break;
            }
            for k in (j + 1)..numbers.len() {
                let nk = numbers[k];
                if ni + nj + nk > 2020 {
                    break;
                } else if ni + nj + nk == 2020 {
                    result = ni * nj * nk
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
        assert_eq!(sub1("1721\n979\n366\n299\n675\n1456"), 514579);
    }

    #[test]
    fn sub21() {
        assert_eq!(sub2("1721\n979\n366\n299\n675\n1456"), 241861950);
    }
}
