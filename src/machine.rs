use instruction::Instruction;

pub fn execute_program(program: &[Instruction], input: &[u8]) -> Vec<u8> {
    let mut machine = Machine::new(program, input);
    machine.execute();
    machine.output
}

struct Machine {
    code: Vec<Instruction>,
    /// data array; grows as needed
    data: Vec<u8>,
    /// input; stored in reverse for more efficient consumption
    input: Vec<u8>,
    output: Vec<u8>,
    /// instruction pointer
    ip: usize,
    /// data pointer
    dp: usize,
}

impl Machine {
    fn new(program: &[Instruction], input: &[u8]) -> Machine {
        // We need to consume the input from first to last byte, but removing from the front of
        // a Vec is much less efficient than popping from the end. Thus, we store the input in
        // reverse.
        let mut input_vec = input.to_vec();
        input_vec.reverse();

        Machine {
            code: program.to_vec(),
            data: vec![0],
            input: input_vec,
            output: Vec::new(),
            ip: 0,
            dp: 0,
        }
    }

    fn execute(&mut self) {
        while self.ip < self.code.len() {
            use instruction::Instruction::*;

            match self.code[self.ip] {
                Next => self.exec_next(),
                Prev => self.exec_prev(),
                Inc => self.exec_inc(),
                Dec => self.exec_dec(),
                Put => self.exec_put(),
                Get => self.exec_get(),
                Skip => self.exec_skip(),
                Loop => self.exec_loop(),
                Fwd(n) => self.exec_fwd(n),
                Bwd(n) => self.exec_bwd(n),
                Add(b) => self.exec_add(b),
                Sub(b) => self.exec_sub(b),
            };

            self.ip += 1;
        }
    }

    // instruction handlers

    fn exec_next(&mut self) {
        self.dp += 1;
        if self.dp == self.data.len() {
            self.data.push(0);
        }
    }

    fn exec_prev(&mut self) {
        if self.dp == 0 {
            panic!("Data pointer moved below 0.");
        }
        self.dp -= 1;
    }

    fn exec_inc(&mut self) {
        let val = self.data[self.dp] as u16;
        self.data[self.dp] = (val + 1) as u8;
    }

    fn exec_dec(&mut self) {
        let val = self.data[self.dp] as i16;
        self.data[self.dp] = (val - 1) as u8;
    }

    fn exec_put(&mut self) {
        self.output.push(self.data[self.dp]);
    }

    fn exec_get(&mut self) {
        if let Some(byte) = self.input.pop() {
            self.data[self.dp] = byte;
        }
    }

    fn exec_skip(&mut self) {
        if self.data[self.dp] != 0 {
            return;
        }

        let mut depth = 1;
        while depth > 0 {
            self.ip += 1;
            if self.ip == self.code.len() {
                panic!("Matching Loop not found.");
            }
            match self.code[self.ip] {
                Instruction::Skip => depth += 1,
                Instruction::Loop => depth -= 1,
                _ => {}
            };
        }
    }

    fn exec_loop(&mut self) {
        if self.data[self.dp] == 0 {
            return;
        }

        let mut depth = 1;
        while depth > 0 {
            if self.ip == 0 {
                panic!("Matching Skip not found.");
            }
            self.ip -= 1;
            match self.code[self.ip] {
                Instruction::Skip => depth -= 1,
                Instruction::Loop => depth += 1,
                _ => {}
            };
        }
    }

    fn exec_fwd(&mut self, n: usize) {
        self.dp += n;
        if self.dp >= self.data.len() {
            self.data.resize(self.dp + 1, 0);
        }
    }

    fn exec_bwd(&mut self, n: usize) {
        if self.dp < n {
            panic!("Data pointer moved below 0.");
        }
        self.dp -= n;
    }

    fn exec_add(&mut self, b: u8) {
        let val = self.data[self.dp] as u16;
        self.data[self.dp] = (val + b as u16) as u8;
    }

    fn exec_sub(&mut self, b: u8) {
        let val = self.data[self.dp] as i16;
        self.data[self.dp] = (val - b as i16) as u8;
    }
}
