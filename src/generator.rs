use crate::{
    assembly::{AssemblyInstruction, AssemblySysCalls},
    parser::Node,
};

pub fn parse_to_assembly(nodes: &Vec<Node>) -> Vec<AssemblyInstruction> {
    let mut instructions = Vec::new();

    for node in nodes {
        match node {
            Node::Expr(_) => todo!(),
            Node::Exit(exit) => instructions.push(AssemblyInstruction::Syscall(
                AssemblySysCalls::Exit(exit.expr.value.clone()),
            )),
            Node::Var(_) => todo!(),
        }
    }
    instructions
}
