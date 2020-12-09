use crate::helpers::file::{read, write};

pub fn ex08() {
    let e = "08";
    let s = read(e);
    write(e, &sub1(&s).to_string(), &sub2(&s).to_string());
}

#[derive(Clone, Debug)]
enum Op {
    Acc(i16),
    Jmp(i16),
    Nop(i16),
}

impl From<&str> for Op {
    fn from(s: &str) -> Self {
        let words = s.split(" ").collect::<Vec<&str>>();
        let argument = words[1].parse::<i16>().unwrap();
        match words[0] {
            "acc" => Self::Acc(argument),
            "jmp" => Self::Jmp(argument),
            "nop" => Self::Nop(argument),
            _ => unreachable!(),
        }
    }
}

struct BootLoader {
    current_line: i16,
    acc: i16,
    visited_lines: Vec<i16>,
}

impl BootLoader {
    fn new() -> Self {
        Self {
            current_line: 0,
            acc: 0,
            visited_lines: vec![],
        }
    }

    fn run(&mut self, code: &Vec<Op>) -> bool {
        loop {
            if self.visited_lines.contains(&self.current_line) {
                break false;
            };
            if self.current_line as usize == code.len() {
                break true;
            }
            self.visited_lines.push(self.current_line);
            match code[self.current_line as usize] {
                Op::Acc(arg) => {
                    self.acc += arg;
                    self.current_line += 1
                }
                Op::Jmp(arg) => self.current_line += arg,
                Op::Nop(_) => self.current_line += 1,
            }
        }
    }
}

fn mutate(code: &Vec<Op>, count: usize) -> Vec<Op> {
    let mut c = 0;
    let mut new_code = vec![];
    for instruction in code.iter() {
        new_code.push(match instruction {
            Op::Jmp(argument) => {
                let new_instruction = if c == count {
                    Op::Nop(*argument)
                } else {
                    Op::Jmp(*argument)
                };
                c += 1;
                new_instruction
            }
            Op::Nop(argument) => {
                let new_instruction = if c == count {
                    Op::Jmp(*argument)
                } else {
                    Op::Nop(*argument)
                };
                c += 1;
                new_instruction
            }
            op => op.to_owned(),
        });
    }
    new_code
}

pub fn sub1(s: &str) -> i16 {
    let mut bl = BootLoader::new();
    let code = s
        .split("\n")
        .map(|instruction| Op::from(instruction))
        .collect::<Vec<Op>>();
    bl.run(&code);
    bl.acc
}

pub fn sub2(s: &str) -> i16 {
    let mut bl = BootLoader::new();
    let initial_code = s
        .split("\n")
        .map(|instruction| Op::from(instruction))
        .collect::<Vec<Op>>();
    let mut count = 0;
    let mut code = initial_code.to_owned();
    while !bl.run(&code) {
        code = mutate(&initial_code, count);
        count += 1;
        bl = BootLoader::new();
    }
    bl.acc
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sub11() {
        assert_eq!(
            sub1(
                "nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6"
            ),
            5
        )
    }

    #[test]
    fn sub21() {
        assert_eq!(
            sub2(
                "nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6"
            ),
            8
        )
    }
}
