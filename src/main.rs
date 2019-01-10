mod opcode;

// static lookup_bytecodes : [opcode::Opcode; 256] = make_bytecode_table();

fn main() {
    println!("Hello, world!");
    let op = opcode::Opcode::ALOAD;
    
}

fn exec_method(klass_name: String, desc: String, instr: Vec<u8>) -> Option<opcode::JVMValue> {
    let mut current = 0;

    loop {
        let ins =  opcode::Opcode::ALOAD; // lookup_bytecodes[0];
        current = current + 1;

        match ins {
            _ => break Some(opcode::JVMValue::Boolean { val: true })
        }
    }
}

// fn make_bytecode_table() -> [opcode::Opcode; 256] {
//     [opcode::Opcode::ALOAD; 256]
// }