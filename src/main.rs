mod opcode;

fn main() {
    println!("Hello, world!");
    let op = opcode::Opcode::ALOAD;
}

fn exec_method(klass_name: String, desc: String, instr: Vec<u8>) -> Option<opcode::JVMValue> {
    Some(opcode::JVMValue::Boolean { val: 0 })
}
