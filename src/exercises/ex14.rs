use crate::helpers::file::{read, write};
use std::collections::HashMap;

pub fn ex14() {
    let e = "14";
    let s = read(e);
    write(e, &sub1(&s).to_string(), &sub2(&s).to_string());
}

#[derive(PartialEq, Eq, Hash, Clone)]
struct U36 {
    bits: Vec<bool>,
}

impl From<&str> for U36 {
    fn from(s: &str) -> Self {
        let number = s.parse::<u64>().unwrap();
        let mut bits_str = format!("{:b}", number);
        for _ in 0..36 - bits_str.len() {
            bits_str = format!("0{}", bits_str);
        }
        let bits = bits_str
            .chars()
            .map(|c| match c {
                '0' => false,
                _ => true,
            })
            .collect::<Vec<bool>>();
        if bits.len() != 36 {
            panic!("U36 != 36");
        }
        Self { bits }
    }
}

impl From<&U36> for usize {
    fn from(u: &U36) -> Self {
        u.bits
            .iter()
            .rev()
            .enumerate()
            .map(
                |(index, value)| {
                    if *value {
                        2usize.pow(index as u32)
                    } else {
                        0
                    }
                },
            )
            .sum::<usize>()
    }
}

#[derive(PartialEq)]
enum MaskBit {
    Zero,
    One,
    Undefined,
}

impl From<char> for MaskBit {
    fn from(c: char) -> Self {
        match c {
            '0' => Self::Zero,
            '1' => Self::One,
            _ => Self::Undefined,
        }
    }
}

struct Mask {
    bits: Vec<MaskBit>,
}

impl From<&str> for Mask {
    fn from(s: &str) -> Self {
        let bits = s
            .chars()
            .map(|c| MaskBit::from(c))
            .collect::<Vec<MaskBit>>();
        if bits.len() != 36 {
            panic!("U36 != 36");
        }
        Self { bits }
    }
}

impl Mask {
    fn new() -> Self {
        let bits = (0..36)
            .map(|_| MaskBit::Undefined)
            .collect::<Vec<MaskBit>>();
        Self { bits }
    }
}

enum Instruction {
    Mask(Mask),
    Mem(U36, U36),
}

impl From<&str> for Instruction {
    fn from(s: &str) -> Self {
        let values = s.split(" = ").collect::<Vec<&str>>();
        match (values[0], values[1]) {
            ("mask", mask) => Self::Mask(Mask::from(mask)),
            (op, value) => {
                let address = U36::from(
                    op.split("[").collect::<Vec<&str>>()[1]
                        .split("]")
                        .collect::<Vec<&str>>()[0],
                );
                let value = U36::from(value);
                Self::Mem(address, value)
            }
        }
    }
}

struct Computer {
    mask: Mask,
    memory: HashMap<U36, U36>,
}

impl Computer {
    fn new() -> Self {
        Self {
            mask: Mask::new(),
            memory: HashMap::new(),
        }
    }

    fn mask(&self, value: U36) -> U36 {
        let bits = self
            .mask
            .bits
            .iter()
            .zip(value.bits)
            .map(|(m, v)| match m {
                MaskBit::Undefined => v,
                MaskBit::One => true,
                MaskBit::Zero => false,
            })
            .collect::<Vec<bool>>();
        U36 { bits }
    }

    fn run(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::Mask(mask) => self.mask = mask,
            Instruction::Mem(address, value) => {
                let new_value = self.mask(value);
                self.memory.insert(address, new_value);
            }
        }
    }

    fn mask2(&self, address: U36) -> Vec<U36> {
        let address_pattern = self
            .mask
            .bits
            .iter()
            .zip(address.bits)
            .map(|(m, a)| match m {
                MaskBit::Undefined => MaskBit::Undefined,
                MaskBit::One => MaskBit::One,
                MaskBit::Zero => {
                    if a {
                        MaskBit::One
                    } else {
                        MaskBit::Zero
                    }
                }
            })
            .collect::<Vec<MaskBit>>();
        let undefined_nb = address_pattern
            .iter()
            .filter(|m| **m == MaskBit::Undefined)
            .count();
        let mut addresses = vec![];
        for i in 0..2usize.pow(undefined_nb as u32) {
            let mut chars = format!("{:b}", i);
            while chars.len() < undefined_nb {
                chars = format!("0{}", chars);
            }
            let mut address = vec![];
            let mut index = 0;
            for m in address_pattern.iter() {
                address.push(match m {
                    MaskBit::Undefined => {
                        index += 1;
                        chars.chars().nth(index - 1).unwrap() == '1'
                    }
                    MaskBit::One => true,
                    MaskBit::Zero => false,
                });
            }
            addresses.push(U36 { bits: address });
        }
        addresses
    }

    fn run2(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::Mask(mask) => self.mask = mask,
            Instruction::Mem(address, value) => {
                let new_addresses = self.mask2(address);
                for address in new_addresses.into_iter() {
                    self.memory.insert(address, value.to_owned());
                }
            }
        }
    }

    fn sum(&self) -> usize {
        self.memory
            .iter()
            .map(|(_, value)| usize::from(value))
            .sum::<usize>()
    }
}

pub fn sub1(s: &str) -> usize {
    let mut computer = Computer::new();
    let program = s.split("\n").map(|line| Instruction::from(line));
    for instruction in program {
        computer.run(instruction);
    }
    computer.sum()
}

pub fn sub2(s: &str) -> usize {
    let mut computer = Computer::new();
    let program = s.split("\n").map(|line| Instruction::from(line));
    for instruction in program {
        computer.run2(instruction);
    }
    computer.sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sub11() {
        assert_eq!(
            sub1(
                "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
mem[8] = 11
mem[7] = 101
mem[8] = 0"
            ),
            165
        );
    }

    #[test]
    fn sub21() {
        assert_eq!(
            sub2(
                "mask = 000000000000000000000000000000X1001X
mem[42] = 100
mask = 00000000000000000000000000000000X0XX
mem[26] = 1"
            ),
            208
        );
    }
}
