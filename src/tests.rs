use super::*;
use std::fs::File;
use std::io::Read;
use std::path::Path;

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
fn irem_works() {
    let buf = vec![
        opcode::Opcode::ICONST_5,
        opcode::Opcode::ICONST_3,
        opcode::Opcode::IREM,
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
}

#[test]
fn idiv_works() {
    let buf = vec![
        opcode::Opcode::ICONST_5,
        opcode::Opcode::ICONST_3,
        opcode::Opcode::IDIV,
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

#[test]
fn test_ifnonnull() {
    let buf = vec![
        opcode::Opcode::ICONST_1,
        opcode::Opcode::ACONST_NULL,
        opcode::Opcode::IFNONNULL,
        0,
        4,
        opcode::Opcode::POP,
        opcode::Opcode::ICONST_2,
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
}

#[test]
fn test_ifnull() {
    let buf = vec![
        opcode::Opcode::ICONST_1,
        opcode::Opcode::ACONST_NULL,
        opcode::Opcode::IFNULL,
        0,
        4,
        opcode::Opcode::POP,
        opcode::Opcode::ICONST_2,
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
fn test_goto() {
    let buf = vec![
        opcode::Opcode::ICONST_1,
        opcode::Opcode::ICONST_1,
        opcode::Opcode::IADD,
        opcode::Opcode::GOTO,
        0,
        3,
        0xff,
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
}

// Helper fn
fn file_to_bytes(path: &Path) -> Result<Vec<u8>, std::io::Error> {
    File::open(path).and_then(|mut file| {
        let mut bytes = Vec::new();
        file.read_to_end(&mut bytes)?;
        Ok(bytes)
    })
}

#[test]
fn test_read_header() {
    let bytes = match file_to_bytes(Path::new("./resources/test/Foo.class")) {
        Ok(buf) => buf,
        _ => panic!("Error reading Foo"),
    };
    let mut parser = klass_parser::oc_parser::new(bytes, "Foo.class".to_string());
    parser.parse();
    assert_eq!(17, parser.get_pool_size());
    let mut k = parser.klass();
    assert_eq!("Foo", k.get_name());
    assert_eq!("java/lang/Object", k.get_super_name());
}
