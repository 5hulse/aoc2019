use crate::load_input;

#[derive(Debug)]
struct Program {
    code: Vec<i32>,
    position: usize,
}

impl Program {
    pub fn new(code: Vec<i32>) -> Self {
        Program {
            code,
            position: 0,
        }
    }

    pub fn run_program(&mut self) {
        loop {
            let opcode = self.code[self.position];
            match opcode {
                1  => {
                    let pos1 = self.code[self.position + 1] as usize;
                    let pos2 = self.code[self.position + 2] as usize;
                    let register = self.code[self.position + 3] as usize;
                    self.code[register] = self.code[pos1] + self.code[pos2]
                }
                2  => {
                    let pos1 = self.code[self.position + 1] as usize;
                    let pos2 = self.code[self.position + 2] as usize;
                    let register = self.code[self.position + 3] as usize;
                    self.code[register] = self.code[pos1] * self.code[pos2]
                }
                99 => {
                    break
                }
                _ => {
                    panic!("Unexpected opcode!");
                }
            };
            self.position += 4;
        }
    }
}

pub fn run() {
    let contents = load_input("data/02.txt");
    let code: Vec<i32> = contents
        .trim()
        .split(",")
        .filter(|s| !s.is_empty())
        .map(|s| s.parse::<i32>().unwrap())
        .collect();
    let mut program = Program::new(code);
    program.run_program();
    println!("Value at position 0: {}", program.code[0]);
}

#[test]
fn test_examples() {
    let codes = [
        vec![1, 0, 0, 0, 99],
        vec![2, 3, 0, 3, 99],
        vec![2, 4, 4, 5, 99, 0],
        vec![1, 1, 1, 4, 99, 5, 6, 0, 99],

    ];
    let results = [
        vec![2, 0, 0, 0, 99],
        vec![2, 3, 0, 6, 99],
        vec![2, 4, 4, 5, 99, 9801],
        vec![30, 1, 1, 4, 2, 5, 6, 0, 99],
    ];
    for (init, res) in codes.iter().zip(results.iter()) {
        let mut program = Program::new(init.to_owned());
        program.run_program();
        assert_eq!(program.code, res.to_owned());
    }
}
