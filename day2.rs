struct State {
    mem: Vec<usize>,
    pc: usize,
    done: bool,
}
impl State {
    fn new(memory: Vec<usize>) -> State {
        State {
            mem: memory,
            pc: 0,
            done: false,
        }
    }

    fn new_with_input(memory: Vec<usize>, noun: usize, verb: usize) -> State {
        let mut s = State::new(memory);
        s.set(noun, verb);
        s
    }

    fn set(&mut self, noun: usize, verb: usize) {
        self.mem[1] = noun;
        self.mem[2] = verb;
    }

    fn op(&self) -> usize {
        self.mem[self.pc]
    }

    fn params(&self) -> (usize, usize, usize) {
        (
            self.mem[self.pc + 1],
            self.mem[self.pc + 2],
            self.mem[self.pc + 3],
        )
    }

    fn run(mut self) -> usize {
        while !self.done {
            self = step(self);
        }
        self.mem[0]
    }
}

fn main() {
    #[allow(clippy::unreadable_literal)]
    let memory: Vec<usize> = vec![
        1, 12, 2, 3, 1, 1, 2, 3, 1, 3, 4, 3, 1, 5, 0, 3, 2, 6, 1, 19, 1, 19, 9, 23, 1, 23, 9, 27,
        1, 10, 27, 31, 1, 13, 31, 35, 1, 35, 10, 39, 2, 39, 9, 43, 1, 43, 13, 47, 1, 5, 47, 51, 1,
        6, 51, 55, 1, 13, 55, 59, 1, 59, 6, 63, 1, 63, 10, 67, 2, 67, 6, 71, 1, 71, 5, 75, 2, 75,
        10, 79, 1, 79, 6, 83, 1, 83, 5, 87, 1, 87, 6, 91, 1, 91, 13, 95, 1, 95, 6, 99, 2, 99, 10,
        103, 1, 103, 6, 107, 2, 6, 107, 111, 1, 13, 111, 115, 2, 115, 10, 119, 1, 119, 5, 123, 2,
        10, 123, 127, 2, 127, 9, 131, 1, 5, 131, 135, 2, 10, 135, 139, 2, 139, 9, 143, 1, 143, 2,
        147, 1, 5, 147, 0, 99, 2, 0, 14, 0,
    ];

    let target = 19_690_720;
    for noun in 0..=99 {
        for verb in 0..=99 {
            let result = State::new_with_input(memory.clone(), noun, verb).run();
            if result == target {
                println!(
                    "Found match at {}, {}, 100 * noun + verb = {}",
                    noun,
                    verb,
                    100 * noun + verb
                );
            }
        }
    }

    // println!("Last 0 state: {}", state.run());
    // println!("Memory:\n{:#?}", state.mem);
}

fn step(state: State) -> State {
    match state.op() {
        1 => add(state),
        2 => mul(state),
        99 => halt(state),
        _ => error(state),
    }
}

fn add(mut state: State) -> State {
    let instr_len = 4;
    let (a, b, c) = state.params();
    state.mem[c] = state.mem[a] + state.mem[b];
    state.pc += instr_len;
    state
}

fn mul(mut state: State) -> State {
    let instr_len = 4;
    let (a, b, c) = state.params();
    state.mem[c] = state.mem[a] * state.mem[b];
    state.pc += instr_len;
    state
}

fn halt(mut state: State) -> State {
    state.done = true;
    state
}

fn error(state: State) -> State {
    println!("Invalid opcode {} at {}", state.op(), state.pc);
    state
}
