use super::*;

use ocelotter_runtime::constant_pool::ACC_PUBLIC;
use ocelotter_util::file_to_bytes;

use std::path::Path;

// Helper fns

fn execute_method(buf: &Vec<u8>) -> JvmValue {
    let mut lvt = InterpLocalVars::of(10); // FIXME
    let opt_ret = exec_bytecode_method("DUMMY".to_string(), &buf, &mut lvt);
    match opt_ret {
        Some(value) => value,
        None => JvmValue::ObjRef {
            val: 0, // object::OtObj::get_null(),
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
        JvmValue::Int { val: i } => i,
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
        JvmValue::Int { val: i } => i,
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
        JvmValue::Int { val: i } => i,
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
        JvmValue::Int { val: i } => i,
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
        JvmValue::Int { val: i } => i,
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
        JvmValue::Int { val: i } => i,
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
        JvmValue::Int { val: i } => i,
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
        JvmValue::Int { val: i } => i,
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
        JvmValue::Int { val: i } => i,
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
        JvmValue::Int { val: i } => i,
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
        JvmValue::Int { val: i } => i,
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
        JvmValue::Int { val: i } => i,
        _ => {
            println!("Unexpected, non-integer value encountered");
            0
        }
    };
    assert_eq!(2, ret);
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
    let k = parser.klass();
    assert_eq!("SampleInvoke", k.get_name());
    assert_eq!("java/lang/Object", k.get_super_name());
    assert_eq!(4, k.get_methods().len());

    // Bootstrap the equivalent of RT 
    CONTEXT.lock().unwrap().get_repo().bootstrap();
    // Add our klass
    CONTEXT.lock().unwrap().get_repo().add_klass(&k);

    {
        let meth = match k.get_method_by_name_and_desc(&"SampleInvoke.bar:()I".to_string()) {
            Some(value) => value.clone(),
            None => panic!("SampleInvoke.bar:()I not found"),
        };
        assert_eq!(ACC_PUBLIC | ACC_STATIC, meth.get_flags());

        let mut vars = InterpLocalVars::of(5);
        let opt_ret = exec_method(&meth, &mut vars);
        let ret = match opt_ret {
            Some(value) => value,
            None => panic!("Error executing SampleInvoke.bar:()I - no value returned"),
        };
        let ret2 = match ret {
            JvmValue::Int { val: i } => i,
            _ => panic!("Error executing SampleInvoke.bar:()I - non-int value returned"),
        };
        assert_eq!(7, ret2);
    }

    {
        let meth = match k.get_method_by_name_and_desc(&"SampleInvoke.foo:()I".to_string()) {
            Some(value) => value.clone(),
            None => panic!("SampleInvoke.bar:()I not found"),
        };

        assert_eq!(ACC_PUBLIC | ACC_STATIC, meth.get_flags());

        let mut vars = InterpLocalVars::of(5);
        let opt_ret = exec_method(&meth, &mut vars);
        let ret = match opt_ret {
            Some(value) => value,
            None => panic!("Error executing SampleInvoke.foo:()I - no value returned"),
        };
        let ret2 = match ret {
            JvmValue::Int { val: i } => i,
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
    let k = parser.klass();

    // Bootstrap the equivalent of RT 
    CONTEXT.lock().unwrap().get_repo().bootstrap();
    // Add our klass
    CONTEXT.lock().unwrap().get_repo().add_klass(&k);

    {
        let meth = match k.get_method_by_name_and_desc(&"Iffer.baz:()I".to_string()) {
            Some(value) => value.clone(),
            None => panic!("Iffer.baz:()I not found"),
        };

        assert_eq!(ACC_PUBLIC | ACC_STATIC, meth.get_flags());

        let mut vars = InterpLocalVars::of(5);
        let opt_ret = exec_method(&meth, &mut vars);
        let ret = match opt_ret {
            Some(value) => value,
            None => panic!("Error executing Iffer.baz:()I - no value returned"),
        };
        let ret2 = match ret {
            JvmValue::Int { val: i } => i,
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
    let k = parser.klass();

    // Bootstrap the equivalent of RT 
    CONTEXT.lock().unwrap().get_repo().bootstrap();
    // Add our klass
    CONTEXT.lock().unwrap().get_repo().add_klass(&k);

    {
        let meth = match k.get_method_by_name_and_desc(&"ArraySimple.baz:()I".to_string()) {
            Some(value) => value.clone(),
            None => panic!("ArraySimple.baz:()I not found"),
        };

        assert_eq!(ACC_PUBLIC | ACC_STATIC, meth.get_flags());

        let mut vars = InterpLocalVars::of(5);
        let opt_ret = exec_method(&meth, &mut vars);
        let ret = match opt_ret {
            Some(value) => value,
            None => panic!("Error executing ArraySimple.baz:()I - no value returned"),
        };
        let ret2 = match ret {
            JvmValue::Int { val: i } => i,
            _ => panic!("Error executing ArraySimple.baz:()I - non-int value returned"),
        };
        assert_eq!(7, ret2);
    }
}

#[test]
#[ignore]
fn test_system_current_timemillis() {
    let bytes = match file_to_bytes(Path::new("./resources/test/Main3.class")) {
        Ok(buf) => buf,
        _ => panic!("Error reading Main3"),
    };
    let mut parser = klass_parser::OtKlassParser::of(bytes, "Main3.class".to_string());
    parser.parse();
    let k = parser.klass();

    // Bootstrap the equivalent of RT 
    CONTEXT.lock().unwrap().get_repo().bootstrap();
    // Add our klass
    CONTEXT.lock().unwrap().get_repo().add_klass(&k);

    {
        let meth = match k.get_method_by_name_and_desc(&"Main3.main2:([Ljava/lang/String;)I".to_string()) {
            Some(value) => value.clone(),
            None => panic!("Main3.main2:([Ljava/lang/String;)I not found"),
        };

        assert_eq!(ACC_PUBLIC | ACC_STATIC, meth.get_flags());

        let mut vars = InterpLocalVars::of(5);
        let opt_ret = exec_method(&meth, &mut vars);
        let ret = match opt_ret {
            Some(value) => value,
            None => panic!("Error executing Main3.main2:([Ljava/lang/String;)I - no value returned"),
        };
        let ctm1 = match ret {
            JvmValue::Int { val: i } => i,
            _ => panic!("Error executing Main3.main2:([Ljava/lang/String;)I - non-int value returned"),
        };
        vars = InterpLocalVars::of(5);
        let opt_ret = exec_method(&meth, &mut vars);
        let ret2 = match opt_ret {
            Some(value) => value,
            None => panic!("Error executing Main3.main2:([Ljava/lang/String;)I - no value returned"),
        };
        let ctm2 = match ret2 {
            JvmValue::Int { val: i } => i,
            _ => panic!("Error executing Main3.main2:([Ljava/lang/String;)I - non-int value returned"),
        };
        assert_eq!(true, ctm2 >= ctm1, "System clock appears to go backwards");
    }
}
