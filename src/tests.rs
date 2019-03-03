use super::*;
use crate::runtime::constant_pool::ACC_PUBLIC;
use crate::runtime::constant_pool::ACC_STATIC;
use crate::runtime::object::OtObj;
use std::fs::File;
use std::io::Read;
use std::path::Path;

fn execute_method(buf: &Vec<u8>) -> runtime::JvmValue {
    let mut lvt = runtime::InterpLocalVars::of(10); // FIXME
    let opt_ret = exec_method("DUMMY".to_string(), &buf, &mut lvt);
    match opt_ret {
        Some(value) => value,
        None => runtime::JvmValue::ObjRef {
            val: 0, // runtime::object::OtObj::get_null(),
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
        runtime::JvmValue::Int { val: i } => i,
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
        runtime::JvmValue::Int { val: i } => i,
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
        runtime::JvmValue::Int { val: i } => i,
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
        runtime::JvmValue::Int { val: i } => i,
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
        runtime::JvmValue::Int { val: i } => i,
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
        runtime::JvmValue::Int { val: i } => i,
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
        runtime::JvmValue::Int { val: i } => i,
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
        runtime::JvmValue::Int { val: i } => i,
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
        runtime::JvmValue::Int { val: i } => i,
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
        runtime::JvmValue::Int { val: i } => i,
        _ => {
            println!("Unexpected, non-integer value encountered");
            0
        }
    };
    assert_eq!(1, ret);
}

#[test]
fn test_ifeq() {
    let buf = vec![
        Opcode::ICONST_1,
        Opcode::ICONST_1,
        Opcode::IADD,
        Opcode::ICONST_2,
        Opcode::IF_ICMPEQ,
        0,
        3,
        Opcode::ICONST_4,
        // Opcode::GOTO,
        // 0,
        // 12,
        Opcode::ICONST_3,
        Opcode::IRETURN,
    ];
    let ret = match execute_method(&buf) {
        runtime::JvmValue::Int { val: i } => i,
        _ => {
            println!("Unexpected, non-integer value encountered");
            0
        }
    };
    assert_eq!(3, ret);
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
        runtime::JvmValue::Int { val: i } => i,
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
    let mut parser = klass_parser::OtKlassParser::of(bytes, "Foo.class".to_string());
    parser.parse();
    assert_eq!(16, parser.get_pool_size());
    let k = parser.klass();
    assert_eq!("Foo", k.get_name());
    assert_eq!("java/lang/Object", k.get_super_name());
}

#[test]
fn test_read_simple_class() {
    let bytes = match file_to_bytes(Path::new("./resources/test/Foo2.class")) {
        Ok(buf) => buf,
        _ => panic!("Error reading Foo2"),
    };
    let mut parser = klass_parser::OtKlassParser::of(bytes, "Foo2.class".to_string());
    parser.parse();
    assert_eq!(30, parser.get_pool_size());
    let k = parser.klass();
    assert_eq!("Foo2", k.get_name());
    assert_eq!("java/lang/Object", k.get_super_name());
    assert_eq!(2, k.get_methods().len());
}

#[test]
fn test_invoke_simple() {
    let bytes = match file_to_bytes(Path::new("./resources/test/SampleInvoke.class")) {
        Ok(buf) => buf,
        _ => panic!("Error reading SampleInvoke"),
    };
    let mut parser = klass_parser::OtKlassParser::of(bytes, "SampleInvoke.class".to_string());
    parser.parse();
    assert_eq!(21, parser.get_pool_size());
    let mut k = parser.klass();
    assert_eq!("SampleInvoke", k.get_name());
    assert_eq!("java/lang/Object", k.get_super_name());
    assert_eq!(4, k.get_methods().len());

    let repo = CONTEXT.lock().unwrap().get_repo().add_klass(&mut k);

    {
        let meth = k.get_method_by_name_and_desc("SampleInvoke.bar:()I".to_string());
        assert_eq!(ACC_PUBLIC | ACC_STATIC, meth.get_flags());

        let opt_ret = exec_method2(meth);
        let ret = match opt_ret {
            Some(value) => value,
            None => panic!("Error executing SampleInvoke.bar:()I - no value returned"),
        };
        let ret2 = match ret {
            runtime::JvmValue::Int { val: i } => i,
            _ => panic!("Error executing SampleInvoke.bar:()I - non-int value returned"),
        };
        assert_eq!(7, ret2);
    }

    {
        let meth = k.get_method_by_name_and_desc("SampleInvoke.foo:()I".to_string());
        assert_eq!(ACC_PUBLIC | ACC_STATIC, meth.get_flags());

        let opt_ret = exec_method2(meth);
        let ret = match opt_ret {
            Some(value) => value,
            None => panic!("Error executing SampleInvoke.foo:()I - no value returned"),
        };
        let ret2 = match ret {
            runtime::JvmValue::Int { val: i } => i,
            _ => panic!("Error executing SampleInvoke.foo:()I - non-int value returned"),
        };
        assert_eq!(9, ret2);
    }
}

#[test]
fn test_iffer() {
    let bytes = match file_to_bytes(Path::new("./resources/test/Iffer.class")) {
        Ok(buf) => buf,
        _ => panic!("Error reading Iffer"),
    };
    let mut parser = klass_parser::OtKlassParser::of(bytes, "Iffer.class".to_string());
    parser.parse();
    let mut k = parser.klass();

    let repo = CONTEXT.lock().unwrap().get_repo().add_klass(&mut k);

    {
        let meth = k.get_method_by_name_and_desc("Iffer.baz:()I".to_string());
        assert_eq!(ACC_PUBLIC | ACC_STATIC, meth.get_flags());

        let opt_ret = exec_method2(meth);
        let ret = match opt_ret {
            Some(value) => value,
            None => panic!("Error executing Iffer.baz:()I - no value returned"),
        };
        let ret2 = match ret {
            runtime::JvmValue::Int { val: i } => i,
            _ => panic!("Error executing Iffer.baz:()I - non-int value returned"),
        };
        assert_eq!(3, ret2);
    }
}

#[test]
fn test_array_simple() {
    let bytes = match file_to_bytes(Path::new("./resources/test/ArraySimple.class")) {
        Ok(buf) => buf,
        _ => panic!("Error reading ArraySimple"),
    };
    let mut parser = klass_parser::OtKlassParser::of(bytes, "ArraySimple.class".to_string());
    parser.parse();
    let mut k = parser.klass();

    let repo = CONTEXT.lock().unwrap().get_repo().add_klass(&mut k);

    {
        let meth = k.get_method_by_name_and_desc("ArraySimple.baz:()I".to_string());
        assert_eq!(ACC_PUBLIC | ACC_STATIC, meth.get_flags());

        let opt_ret = exec_method2(meth);
        let ret = match opt_ret {
            Some(value) => value,
            None => panic!("Error executing ArraySimple.baz:()I - no value returned"),
        };
        let ret2 = match ret {
            runtime::JvmValue::Int { val: i } => i,
            _ => panic!("Error executing ArraySimple.baz:()I - non-int value returned"),
        };
        assert_eq!(7, ret2);
    }
}
