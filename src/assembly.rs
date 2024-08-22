use std::path::PathBuf;

pub enum AssemblyInstruction {
    Syscall(AssemblySysCalls),
    StaticAssemblyData(StaticAssemblyData),
    Add(String, String), // Add two registers or values
    Sub(String, String), // Subtract two registers or values
    Mov(String, String), // Move value from one register to another
    Push(String),        // Push a value onto the stack
    Pop(String),         // Pop a value from the stack
                         // Add more instructions as needed
}

// everything what comes into the .data section
pub enum StaticAssemblyData {
    /// Define Byte
    /// allocates 1 byte
    MakeDB(String),
    /// Define Word
    /// allocates 2 bytes
    MakeDW(String),
    /// Define Doubleword
    /// allocates 4 bytes
    MakeDD(String),
    /// Define Quadword
    /// allocates 8 bytes
    MakeDQ(String),
}

impl StaticAssemblyData {
    fn to_assembly(&self) -> Vec<String> {
        match self {
            StaticAssemblyData::MakeDB(v) => vec![format!("var DB {}", v)],
            StaticAssemblyData::MakeDW(v) => vec![format!("var DW {}", v)],
            StaticAssemblyData::MakeDD(v) => vec![format!("var DD {}", v)],
            StaticAssemblyData::MakeDQ(v) => vec![format!("var DQ {}", v)],
        }
    }
}

pub enum AssemblySysCalls {
    Exit(String),
}

// https://chromium.googlesource.com/chromiumos/docs/+/master/constants/syscalls.md
// TODO: May support more OS's
#[cfg(target_os = "linux")]
impl AssemblySysCalls {
    fn to_assembly(&self) -> Vec<String> {
        match self {
            AssemblySysCalls::Exit(code) => vec![
                "mov rax, 60".to_string(),
                format!("mov rdi, {}", code),
                "syscall".to_string(),
            ],
        }
    }
}

// Instruction map for efficient assembly generation
impl AssemblyInstruction {
    fn to_assembly(&self) -> Vec<String> {
        match self {
            AssemblyInstruction::Syscall(call) => call.to_assembly(),
            AssemblyInstruction::StaticAssemblyData(data) => data.to_assembly(),
            AssemblyInstruction::Add(reg1, reg2) => vec![format!("add {}, {}", reg1, reg2)],
            AssemblyInstruction::Sub(reg1, reg2) => vec![format!("sub {}, {}", reg1, reg2)],
            AssemblyInstruction::Mov(src, dst) => vec![format!("mov {}, {}", src, dst)],
            AssemblyInstruction::Push(value) => vec![format!("push {}", value)],
            AssemblyInstruction::Pop(reg) => vec![format!("pop {}", reg)],
            // Add assembly generation for other instructions here
        }
    }
}

pub fn write_into_file(instructions: Vec<AssemblyInstruction>, output_file: PathBuf) {
    let mut final_buffer = Vec::new();

    // final_buffer.push(get_data_section_header().to_string());
    final_buffer.push(get_assembly_header().to_string());
    for (i, instruction) in instructions.iter().enumerate() {
        final_buffer.extend(instruction.to_assembly());
        // We don't add line break on last instruction
        if i < instructions.len() - 1 {
            final_buffer.push("\n".to_string()); // Add newline for all but the last instruction
        }
    }

    std::fs::write(output_file, final_buffer.join("\n")).unwrap();
}

struct Stack<T> {
    data: Vec<T>,
}

impl<T> Stack<T> {
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }

    pub fn push(&mut self, value: T) {
        self.data.push(value);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.data.pop()
    }
}

pub const fn get_assembly_header() -> &'static str {
    "global _start\n_start:"
}

pub const fn get_data_section_header() -> &'static str {
    "section .data"
}
