use std::path::PathBuf;

pub enum AssemblyInstruction {
    Exit(String),    // Code
    MakeInt(String), // Int value
}

pub fn write_into_file(instructions: Vec<AssemblyInstruction>, output_file: PathBuf) {
    let mut final_buffer = Vec::new();

    final_buffer.push(get_assembly_header().to_string());
    for instruction in instructions {
        for assembly in get_instruction_assembly(instruction) {
            final_buffer.push(assembly.to_string());
            // final_buffer.push("\n");
        }
    }
    std::fs::write(output_file, final_buffer.join("\n")).unwrap();
}

// https://chromium.googlesource.com/chromiumos/docs/+/master/constants/syscalls.md
pub fn get_instruction_assembly(instruction: AssemblyInstruction) -> Vec<String> {
    let mut buffer = Vec::new();
    let mut stack = Stack::new();
    match instruction {
        AssemblyInstruction::Exit(code) => {
            buffer.push("mov rax, 60".to_string());
            buffer.push(format!("mov rdi, {}", code));
            buffer.push("syscall".to_string());
        }
        AssemblyInstruction::MakeInt(value) => {
            buffer.push(format!("mov rax, {}", value));
            buffer.push(stack.push("rax"));
        }
    }
    buffer
}

pub struct Stack {
    current_size: u32,
}

impl Stack {
    pub fn new() -> Self {
        Self { current_size: 0 }
    }

    pub fn push(&mut self, loc: &str) -> String {
        self.current_size += 1;
        format!("push {}", loc)
    }
    pub fn pop(&mut self, loc: &str) -> String {
        self.current_size -= 1;
        format!("pop {}", loc)
    }
}

pub const fn get_assembly_header() -> &'static str {
    "global _start\n_start:"
}
