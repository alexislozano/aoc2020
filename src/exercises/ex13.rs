use crate::helpers::file::{read, write};

pub fn ex13() {
    let e = "13";
    let s = read(e);
    write(e, &sub1(&s).to_string(), &sub2(&s).to_string());
}

pub fn sub1(s: &str) -> usize {
    let s = s.split("\n").collect::<Vec<&str>>();
    let depart = s[0].parse::<usize>().unwrap();
    let ids = s[1]
        .split(",")
        .filter(|id| *id != "x")
        .map(|id| id.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();
    let mut min_id = ids[0];
    let mut min_gap = min_id - depart % min_id;
    for id in ids.iter().skip(1) {
        let gap = id - depart % id;
        if gap < min_gap {
            min_gap = gap;
            min_id = *id;
            if gap == 0 {
                break;
            }
        }
    }
    min_id * min_gap
}

fn egcd(a: i64, b: i64) -> (i64, i64, i64) {
    if a == 0 {
        (b, 0, 1)
    } else {
        let (g, x, y) = egcd(b % a, a);
        (g, y - (b / a) * x, x)
    }
}

fn mod_inv(x: i64, n: i64) -> Option<i64> {
    let (g, x, _) = egcd(x, n);
    if g == 1 {
        Some((x % n + n) % n)
    } else {
        None
    }
}

fn chinese_remainder(residues: &[i64], modulii: &[i64]) -> Option<i64> {
    let prod = modulii.iter().product::<i64>();

    let mut sum = 0;

    for (&residue, &modulus) in residues.iter().zip(modulii) {
        let p = prod / modulus;
        sum += residue * mod_inv(p, modulus)? * p
    }

    Some(sum % prod)
}

pub fn sub2(s: &str) -> i64 {
    let s = s.split("\n").collect::<Vec<&str>>();
    let (residues, modulii): (Vec<i64>, Vec<i64>) = s[1]
        .split(",")
        .enumerate()
        .map(|(index, id)| (index as i64, id.parse::<i64>()))
        .filter(|(_, id)| id.is_ok())
        .map(|(index, id)| (index, id.unwrap()))
        .map(|(index, id)| (id - index, id))
        .unzip();
    chinese_remainder(&residues, &modulii).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sub11() {
        assert_eq!(
            sub1(
                "939
7,13,x,x,59,x,31,19"
            ),
            295
        )
    }

    #[test]
    fn sub21() {
        assert_eq!(
            sub2(
                "939
7,13,x,x,59,x,31,19"
            ),
            1068781
        )
    }

    #[test]
    fn sub22() {
        assert_eq!(
            sub2(
                "939
17,x,13,19"
            ),
            3417
        )
    }

    #[test]
    fn sub23() {
        assert_eq!(
            sub2(
                "939
67,7,59,61"
            ),
            754018
        )
    }

    #[test]
    fn sub24() {
        assert_eq!(
            sub2(
                "939
67,x,7,59,61"
            ),
            779210
        )
    }

    #[test]
    fn sub25() {
        assert_eq!(
            sub2(
                "939
67,7,x,59,61"
            ),
            1261476
        )
    }

    #[test]
    fn sub26() {
        assert_eq!(
            sub2(
                "939
1789,37,47,1889"
            ),
            1202161486
        )
    }
}
