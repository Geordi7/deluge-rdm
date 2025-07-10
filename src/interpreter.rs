use crate::ast::Call;

pub fn interpret(program: Vec<Call>) {
    for call in program {
        execute(&call, 0);
    }
}

fn execute(call: &Call, indent: usize) {
    println!("{space}{}", call.name, space=" ".repeat(indent));
    for arg in &call.args {
        execute(arg, indent + 2);
    }
}
