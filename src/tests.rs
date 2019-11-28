use std::cell::RefCell;
use std::path::Path;
use std::sync::Once;

use super::*;

use ocelotter_runtime::constant_pool::ACC_PUBLIC;
use ocelotter_util::file_to_bytes;

// Helper fns

static INIT: Once = Once::new();

fn init_repo() {
    INIT.call_once(|| {
        let mut repo = SharedKlassRepo::of();
        repo.bootstrap(exec_method);
        *REPO.lock().unwrap() = RefCell::new(repo);
    });
}

fn execute_simple_bytecode(buf: &Vec<u8>) -> JvmValue {
    let mut lvt = InterpLocalVars::of(10); // FIXME
    let opt_ret = exec_bytecode_method("DUMMY".to_string(), &buf, &mut lvt);
    match opt_ret {
        Some(value) => value,
        None => JvmValue::ObjRef {
            // FIXME ????
            val: 0, // object::OtObj::get_null(),
        },
    }
}

fn simple_parse_klass(cname: String) -> OtKlass {
    let mut path = "./resources/test/".to_string();
    path.push_str(&cname);
    path.push_str(".class");
    let bytes = match file_to_bytes(Path::new(&path)) {
        Ok(buf) => buf,
        _ => panic!("Error reading {}", cname),
    };
    let mut kname = cname;
    kname.push_str(".class");
    let mut parser = klass_parser::OtKlassParser::of(bytes, kname);
    parser.parse();
    let k = parser.klass();

    // Add our klass
    REPO.lock().unwrap().borrow_mut().add_klass(&k);
    k
}

/////////////////////////////////////////////////////////////////////////////

#[test]
fn bc_adds_to_two() {
    let first_test = vec![
        opcode::Opcode::ICONST_1,
        opcode::Opcode::ICONST_1,
        opcode::Opcode::IADD,
        opcode::Opcode::IRETURN,
    ];
    let ret = match execute_simple_bytecode(&first_test) {
        JvmValue::Int { val: i } => i,
        _ => {
            println!("Unexpected, non-integer value encountered");
            0
        }
    };
    assert_eq!(2, ret);
}

#[test]
fn bc_iconst_dup() {
    let buf = vec![
        opcode::Opcode::ICONST_1,
        opcode::Opcode::DUP,
        opcode::Opcode::IADD,
        opcode::Opcode::IRETURN,
    ];
    let ret = match execute_simple_bytecode(&buf) {
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
    let ret2 = match execute_simple_bytecode(&buf2) {
        JvmValue::Int { val: i } => i,
        _ => {
            println!("Unexpected, non-integer value encountered");
            0
        }
    };
    assert_eq!(4, ret2);
}

#[test]
fn bc_irem_works() {
    let buf = vec![
        opcode::Opcode::ICONST_5,
        opcode::Opcode::ICONST_3,
        opcode::Opcode::IREM,
        opcode::Opcode::IRETURN,
    ];
    let ret = match execute_simple_bytecode(&buf) {
        JvmValue::Int { val: i } => i,
        _ => {
            println!("Unexpected, non-integer value encountered");
            0
        }
    };
    assert_eq!(2, ret);
}

#[test]
fn bc_idiv_works() {
    let buf = vec![
        opcode::Opcode::ICONST_5,
        opcode::Opcode::ICONST_3,
        opcode::Opcode::IDIV,
        opcode::Opcode::IRETURN,
    ];
    let ret = match execute_simple_bytecode(&buf) {
        JvmValue::Int { val: i } => i,
        _ => {
            println!("Unexpected, non-integer value encountered");
            0
        }
    };
    assert_eq!(1, ret);
}

#[test]
fn bc_iconst_dup_nop_pop() {
    let buf = vec![
        opcode::Opcode::ICONST_1,
        opcode::Opcode::DUP,
        opcode::Opcode::NOP,
        opcode::Opcode::POP,
        opcode::Opcode::IRETURN,
    ];
    let ret = match execute_simple_bytecode(&buf) {
        JvmValue::Int { val: i } => i,
        _ => {
            println!("Unexpected, non-integer value encountered");
            0
        }
    };
    assert_eq!(1, ret);
}

#[test]
fn bc_iconst_dup_x1() {
    let buf = vec![
        opcode::Opcode::ICONST_1,
        opcode::Opcode::ICONST_2,
        opcode::Opcode::DUP_X1,
        opcode::Opcode::IADD,
        opcode::Opcode::IADD,
        opcode::Opcode::IRETURN,
    ];
    let ret = match execute_simple_bytecode(&buf) {
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
    let ret2 = match execute_simple_bytecode(&buf2) {
        JvmValue::Int { val: i } => i,
        _ => {
            println!("Unexpected, non-integer value encountered");
            0
        }
    };
    assert_eq!(8, ret2);
}

#[test]
fn bc_ifnonnull() {
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
    let ret = match execute_simple_bytecode(&buf) {
        JvmValue::Int { val: i } => i,
        _ => {
            println!("Unexpected, non-integer value encountered");
            0
        }
    };
    assert_eq!(2, ret);
}

#[test]
fn bc_ifnull() {
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
    let ret = match execute_simple_bytecode(&buf) {
        JvmValue::Int { val: i } => i,
        _ => {
            println!("Unexpected, non-integer value encountered");
            0
        }
    };
    assert_eq!(1, ret);
}

#[test]
fn bc_ifeq() {
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
    let ret = match execute_simple_bytecode(&buf) {
        JvmValue::Int { val: i } => i,
        _ => {
            println!("Unexpected, non-integer value encountered");
            0
        }
    };
    assert_eq!(3, ret);
}

#[test]
fn bc_goto() {
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
    let ret = match execute_simple_bytecode(&buf) {
        JvmValue::Int { val: i } => i,
        _ => {
            println!("Unexpected, non-integer value encountered");
            0
        }
    };
    assert_eq!(2, ret);
}

/////////////////////////////////////////////////////////////////
//
// Tests that actually load classes

#[test]
fn interp_invoke_simple() {
    init_repo();

    let k = simple_parse_klass("SampleInvoke".to_string());

    {
        let fq_meth = "SampleInvoke.bar:()I";
        let meth = k
            .get_method_by_name_and_desc(&fq_meth.to_string())
            .expect(&format!("{} not found", fq_meth));
        assert_eq!(ACC_PUBLIC | ACC_STATIC, meth.get_flags());

        let mut vars = InterpLocalVars::of(5);
        let ret = exec_method(&meth, &mut vars).unwrap();
        let ret2 = match ret {
            JvmValue::Int { val: i } => i,
            _ => panic!("Error executing SampleInvoke.bar:()I - non-int value returned"),
        };
        assert_eq!(7, ret2);
    }

    {
        let fq_meth = "SampleInvoke.foo:()I";
        let meth = k
            .get_method_by_name_and_desc(&fq_meth.to_string())
            .expect(&format!("{} not found", fq_meth));
        assert_eq!(ACC_PUBLIC | ACC_STATIC, meth.get_flags());

        let mut vars = InterpLocalVars::of(5);
        let ret = exec_method(&meth, &mut vars).unwrap();
        let ret2 = match ret {
            JvmValue::Int { val: i } => i,
            _ => panic!("Error executing SampleInvoke.foo:()I - non-int value returned"),
        };
        assert_eq!(9, ret2);
    }
}

#[test]
fn interp_iffer() {
    init_repo();

    let k = simple_parse_klass("Iffer".to_string());

    {
        let fq_meth = "Iffer.baz:()I";
        let meth = k
            .get_method_by_name_and_desc(&fq_meth.to_string())
            .expect(&format!("{} not found", fq_meth));
        assert_eq!(ACC_PUBLIC | ACC_STATIC, meth.get_flags());

        let mut vars = InterpLocalVars::of(5);
        let ret = exec_method(&meth, &mut vars).unwrap();
        let ret2 = match ret {
            JvmValue::Int { val: i } => i,
            _ => panic!("Error executing Iffer.baz:()I - non-int value returned"),
        };
        assert_eq!(3, ret2);
    }
}

#[test]
fn interp_array_set() {
    init_repo();
    let k = simple_parse_klass("ArraySimple".to_string());

    {
        let fqname = "ArraySimple.baz:()I".to_string();
        let meth = k.get_method_by_name_and_desc(&fqname).unwrap();

        assert_eq!(ACC_PUBLIC | ACC_STATIC, meth.get_flags());

        let mut vars = InterpLocalVars::of(5);
        let ret = exec_method(&meth, &mut vars).unwrap();
        let ret2 = match ret {
            JvmValue::Int { val: i } => i,
            _ => panic!("Error executing {} - non-int value returned", fqname),
        };
        assert_eq!(7, ret2);
    }
}

#[test]
fn interp_field_set() {
    init_repo();
    let k = simple_parse_klass("FieldHaver".to_string());

    {
        let fqname = "FieldHaver.main2:([Ljava/lang/String;)I".to_string();
        let meth = k.get_method_by_name_and_desc(&fqname).unwrap();

        assert_eq!(ACC_PUBLIC | ACC_STATIC, meth.get_flags());

        let mut vars = InterpLocalVars::of(5);
        let ret = match exec_method(&meth, &mut vars).unwrap() {
            JvmValue::Int { val: i } => i,
            _ => panic!("Error executing {} - non-int value returned", fqname),
        };
        assert_eq!(7, ret);
    }
}

#[test]
fn interp_system_current_timemillis() {
    init_repo();
    let k = simple_parse_klass("Main3".to_string());

    {
        let fqname = "Main3.main2:([Ljava/lang/String;)I";
        let meth = k
            .get_method_by_name_and_desc(&fqname.to_string())
            .expect(&format!("{} not found", fqname));
        assert_eq!(ACC_PUBLIC | ACC_STATIC, meth.get_flags());

        let mut vars = InterpLocalVars::of(5);
        let ret = exec_method(&meth, &mut vars).unwrap();
        let ctm1 = match ret {
            JvmValue::Int { val: i } => i,
            _ => panic!("Error executing {} - non-int value returned", fqname),
        };
        vars = InterpLocalVars::of(5);
        let opt_ret = exec_method(&meth, &mut vars);
        let ret2 = match opt_ret {
            Some(value) => value,
            None => panic!("Error executing {} - no value returned", fqname),
        };
        let ctm2 = match ret2 {
            JvmValue::Int { val: i } => i,
            _ => panic!("Error executing {} - non-int value returned", fqname),
        };
        assert_eq!(true, ctm2 >= ctm1, "System clock appears to go backwards");
    }
}

#[test]
fn interp_class_based_addition() {
    init_repo();
    let k = simple_parse_klass("AddFieldInteger".to_string());

    {
        let fqname = "AddFieldInteger.main2:([Ljava/lang/String;)I".to_string();
        let meth = k.get_method_by_name_and_desc(&fqname).unwrap();

        assert_eq!(ACC_PUBLIC | ACC_STATIC, meth.get_flags());

        let mut vars = InterpLocalVars::of(5);
        let ret = exec_method(&meth, &mut vars).unwrap();
        let ret2 = match ret {
            JvmValue::Int { val: i } => i,
            _ => panic!("Error executing {} - non-int value returned", fqname),
        };
        assert_eq!(7, ret2);
    }
}
