use super::*;

fn execute_method(buf: &Vec<u8>) -> runtime::JVMValue {
    let lvt = runtime::LocalVariableTable {};
    let opt_ret = exec_method("DUMMY".to_string(), "DUMMY_DESC".to_string(), &buf, &lvt);
    match opt_ret {
        Some(value) => value,
        None => runtime::JVMValue::ObjRef {
            val: runtime::JVMObj::get_null(),
        },
    }
}

#[test]
fn adds_to_two() {
    let first_test = vec![
        opcode::Opcode::ICONST_1,
        opcode::Opcode::ICONST_1,
        opcode::Opcode::IADD,
        opcode::Opcode::IRETURN,
    ];
    let ret = match execute_method(&first_test) {
        runtime::JVMValue::Int { val: i } => i,
        _ => {
            println!("Unexpected, non-integer value encountered");
            0
        }
    };
    assert_eq!(2, ret);
}

#[test]
fn iconst_dup() {
    let buf = vec![
        opcode::Opcode::ICONST_1,
        opcode::Opcode::DUP,
        opcode::Opcode::IADD,
        opcode::Opcode::IRETURN,
    ];
    let ret = match execute_method(&buf) {
        runtime::JVMValue::Int { val: i } => i,
        _ => {
            println!("Unexpected, non-integer value encountered");
            0
        }
    };
    assert_eq!(2, ret);

    let buf2 = vec![
        opcode::Opcode::ICONST_1,
        opcode::Opcode::DUP,
        opcode::Opcode::IADD,
        opcode::Opcode::DUP,
        opcode::Opcode::IADD,
        opcode::Opcode::IRETURN,
    ];
    let ret2 = match execute_method(&buf2) {
        runtime::JVMValue::Int { val: i } => i,
        _ => {
            println!("Unexpected, non-integer value encountered");
            0
        }
    };
    assert_eq!(4, ret2);
}

#[test]
fn iconst_dup_nop_pop() {
    let buf = vec![
        opcode::Opcode::ICONST_1,
        opcode::Opcode::DUP,
        opcode::Opcode::NOP,
        opcode::Opcode::POP,
        opcode::Opcode::IRETURN,
    ];
    let ret = match execute_method(&buf) {
        runtime::JVMValue::Int { val: i } => i,
        _ => {
            println!("Unexpected, non-integer value encountered");
            0
        }
    };
    assert_eq!(1, ret);
}

#[test]
fn iconst_dup_x1() {
    let buf = vec![
        opcode::Opcode::ICONST_1,
        opcode::Opcode::ICONST_2,
        opcode::Opcode::DUP_X1,
        opcode::Opcode::IADD,
        opcode::Opcode::IADD,
        opcode::Opcode::IRETURN,
    ];
    let ret = match execute_method(&buf) {
        runtime::JVMValue::Int { val: i } => i,
        _ => {
            println!("Unexpected, non-integer value encountered");
            0
        }
    };
    assert_eq!(5, ret);

    let buf2 = vec![
        opcode::Opcode::ICONST_1,
        opcode::Opcode::ICONST_2,
        opcode::Opcode::DUP_X1,
        opcode::Opcode::IADD,
        opcode::Opcode::DUP_X1,
        opcode::Opcode::IADD,
        opcode::Opcode::IADD,
        opcode::Opcode::IRETURN,
    ];
    let ret2 = match execute_method(&buf2) {
        runtime::JVMValue::Int { val: i } => i,
        _ => {
            println!("Unexpected, non-integer value encountered");
            0
        }
    };
    assert_eq!(8, ret2);
}
