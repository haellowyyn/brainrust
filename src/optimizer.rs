use instruction::Instruction;

pub fn optimize(program: &[Instruction]) -> Vec<Instruction> {
    let mut opt_program = Vec::new();
    let mut count: usize = 0;
    let mut last_instr: Option<&Instruction> = None;

    for instr in program {
        if let Some(li) = last_instr {
            if *instr != *li {
                add_accumulated_instr(&mut opt_program, li, count);
                count = 0;
            }
        }

        last_instr = Some(&instr);
        count += 1;
    }
    add_accumulated_instr(&mut opt_program, last_instr.unwrap(), count);

    opt_program
}

fn add_accumulated_instr(program: &mut Vec<Instruction>, instr: &Instruction, count: usize) {
    if count != 1 && instr.is_accumulatable() {
        program.push(instr.to_accumulated(count).unwrap());
        return;
    }

    for _ in 0..count {
        program.push(instr.clone());
    }
}
