use std::path::Path;

use super::*;

use ocelotter_runtime::constant_pool::ACC_PUBLIC;
// this crate is presumably old and not very good.
use assert_float_eq::{
    afe_is_f32_near, afe_is_f64_near, afe_near_error_msg, assert_f32_near, assert_f64_near,
};

use ocelotter_util::file_to_bytes;

// Helper fns

fn init_repo() -> SharedKlassRepo {
    let mut repo = SharedKlassRepo::of();
    repo.bootstrap(exec_method);
    repo
}

fn execute_simple_bytecode(buf: &[u8]) -> JvmValue {
    let mut repo = init_repo();
    let mut lvt = InterpLocalVars::of(10); // FIXME
    exec_bytecode_method(&mut repo, "DUMMY".to_string(), buf, &mut lvt)
        .unwrap_or(JvmValue::ObjRef(0)) // object::OtObj::get_null(),
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

    // Add our klass
    // &mut REPO.unwrap().add_klass(&k);
    parser.klass()
}

/////////////////////////////////////////////////////////////////////////////

#[test]
fn bc_adds_to_two() {
    let first_test = vec![
        opcode::ICONST_1,
        opcode::ICONST_1,
        opcode::IADD,
        opcode::IRETURN,
    ];
    let ret = match execute_simple_bytecode(&first_test) {
        JvmValue::Int(i) => i,
        _ => {
            println!("Unexpected, non-integer value encountered");
            0
        }
    };
    assert_eq!(2, ret);
}

#[test]
fn bc_iconst_dup() {
    let buf = vec![opcode::ICONST_1, opcode::DUP, opcode::IADD, opcode::IRETURN];
    let ret = match execute_simple_bytecode(&buf) {
        JvmValue::Int(i) => i,
        _ => {
            println!("Unexpected, non-integer value encountered");
            0
        }
    };
    assert_eq!(2, ret);

    let buf2 = vec![
        opcode::ICONST_1,
        opcode::DUP,
        opcode::IADD,
        opcode::DUP,
        opcode::IADD,
        opcode::IRETURN,
    ];
    let ret2 = match execute_simple_bytecode(&buf2) {
        JvmValue::Int(i) => i,
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
        opcode::ICONST_5,
        opcode::ICONST_3,
        opcode::IREM,
        opcode::IRETURN,
    ];
    let ret = match execute_simple_bytecode(&buf) {
        JvmValue::Int(i) => i,
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
        opcode::ICONST_5,
        opcode::ICONST_3,
        opcode::IDIV,
        opcode::IRETURN,
    ];
    let ret = match execute_simple_bytecode(&buf) {
        JvmValue::Int(i) => i,
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
        opcode::ICONST_1,
        opcode::DUP,
        opcode::NOP,
        opcode::POP,
        opcode::IRETURN,
    ];
    let ret = match execute_simple_bytecode(&buf) {
        JvmValue::Int(i) => i,
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
        opcode::ICONST_1,
        opcode::ICONST_2,
        opcode::DUP_X1,
        opcode::IADD,
        opcode::IADD,
        opcode::IRETURN,
    ];
    let ret = match execute_simple_bytecode(&buf) {
        JvmValue::Int(i) => i,
        _ => {
            println!("Unexpected, non-integer value encountered");
            0
        }
    };
    assert_eq!(5, ret);

    let buf2 = vec![
        opcode::ICONST_1,
        opcode::ICONST_2,
        opcode::DUP_X1,
        opcode::IADD,
        opcode::DUP_X1,
        opcode::IADD,
        opcode::IADD,
        opcode::IRETURN,
    ];
    let ret2 = match execute_simple_bytecode(&buf2) {
        JvmValue::Int(i) => i,
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
        opcode::ICONST_1,
        opcode::ACONST_NULL,
        opcode::IFNONNULL,
        0,
        4,
        opcode::POP,
        opcode::ICONST_2,
        opcode::IRETURN,
    ];
    let ret = match execute_simple_bytecode(&buf) {
        JvmValue::Int(i) => i,
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
        opcode::ICONST_1,
        opcode::ACONST_NULL,
        opcode::IFNULL,
        0,
        4,
        opcode::POP,
        opcode::ICONST_2,
        opcode::IRETURN,
    ];
    let ret = match execute_simple_bytecode(&buf) {
        JvmValue::Int(i) => i,
        _ => {
            println!("Unexpected, non-integer value encountered");
            0
        }
    };
    assert_eq!(2, ret);
}

#[test]
fn bc_ifeq() {
    let buf = vec![
        opcode::ICONST_1,
        opcode::ICONST_1,
        opcode::IADD,
        opcode::ICONST_2,
        opcode::IF_ICMPEQ,
        0,
        3,
        opcode::ICONST_4,
        // opcode::GOTO,
        // 0,
        // 12,
        opcode::ICONST_3,
        opcode::IRETURN,
    ];
    let ret = match execute_simple_bytecode(&buf) {
        JvmValue::Int(i) => i,
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
        opcode::ICONST_1,
        opcode::ICONST_1,
        opcode::IADD,
        opcode::GOTO,
        0,
        3,
        0xff,
        opcode::IRETURN,
    ];
    let ret = match execute_simple_bytecode(&buf) {
        JvmValue::Int(i) => i,
        _ => {
            println!("Unexpected, non-integer value encountered");
            0
        }
    };
    assert_eq!(2, ret);
}

#[test]
fn bc_lrem_works() {
    let buf = vec![
        opcode::BIPUSH,
        100,
        opcode::BIPUSH,
        100,
        opcode::IMUL,
        opcode::BIPUSH,
        100,
        opcode::IMUL,
        opcode::I2L,
        opcode::LSTORE_0,
        opcode::LLOAD_0,
        opcode::LLOAD_0,
        opcode::LMUL,
        opcode::BIPUSH,
        117,
        opcode::I2L,
        opcode::LREM,
        opcode::LRETURN,
    ];
    let ret = match execute_simple_bytecode(&buf) {
        JvmValue::Long(i) => i,
        _ => {
            println!("Unexpected, non-integer value encountered");
            0
        }
    };
    assert_eq!(ret, 1);
}

#[test]
fn bc_drem() {
    let buf = vec![
        opcode::DCONST_1,
        opcode::ICONST_2,
        opcode::I2D,
        opcode::DDIV,
        opcode::DCONST_1,
        opcode::ICONST_3,
        opcode::I2D,
        opcode::DDIV,
        opcode::DREM,
        opcode::DRETURN,
    ];
    let ret = match execute_simple_bytecode(&buf) {
        JvmValue::Double(i) => i,
        _ => {
            println!("Unexpected, non-double value encountered");
            0.0
        }
    };
    assert_f64_near!(ret, 1.0 / 6.0);
}

#[test]
fn bc_fdiv() {
    let buf = vec![
        opcode::ICONST_4,
        opcode::I2F,
        opcode::ICONST_3,
        opcode::I2F,
        opcode::FDIV,
        opcode::FRETURN,
    ];
    let ret = match execute_simple_bytecode(&buf) {
        JvmValue::Float(i) => i,
        _ => {
            println!("Unexpected, non-float value encountered");
            0.0
        }
    };
    assert_f32_near!(ret, 4.0 / 3.0);
}

#[test]
fn bc_frem() {
    let buf = vec![
        opcode::BIPUSH,
        17,
        opcode::I2F,
        opcode::ICONST_3,
        opcode::I2F,
        opcode::FDIV,
        opcode::ICONST_3,
        opcode::I2F,
        opcode::FREM,
        opcode::FRETURN,
    ];
    let ret = match execute_simple_bytecode(&buf) {
        JvmValue::Float(i) => i,
        _ => {
            println!("Unexpected, non-float value encountered");
            0.0
        }
    };
    assert_f32_near!(ret, 8.0 / 3.0);
}

/////////////////////////////////////////////////////////////////
//
// Tests for helper methods

#[test]
fn parse_signatures() {
    assert_eq!(0, OtKlass::parse_sig_for_args("()Z".to_string()).len());
    assert_eq!(0, OtKlass::parse_sig_for_args("()I".to_string()).len());
    assert_eq!(1, OtKlass::parse_sig_for_args("(I)V".to_string()).len());
    assert_eq!(1, OtKlass::parse_sig_for_args("([I)V".to_string()).len());
    assert_eq!(3, OtKlass::parse_sig_for_args("(D[II)V".to_string()).len());
    assert_eq!(1, OtKlass::parse_sig_for_args("([[I)V".to_string()).len());
    assert_eq!(
        0,
        OtKlass::parse_sig_for_args("()Ljava/lang/String;".to_string()).len()
    );
    assert_eq!(
        1,
        OtKlass::parse_sig_for_args("(Ljava/lang/String;)I".to_string()).len()
    );
    assert_eq!(
        1,
        OtKlass::parse_sig_for_args("([Ljava/lang/String;)I".to_string()).len()
    );
    assert_eq!(
        2,
        OtKlass::parse_sig_for_args(
            "(Ljava/io/FileDescriptor;I)Ljava/io/FileDescriptor;".to_string()
        )
        .len()
    );
}

/////////////////////////////////////////////////////////////////
//
// Tests that actually load classes

#[test]
fn interp_invoke_simple() {
    let mut repo = init_repo();
    let k = simple_parse_klass("SampleInvoke".to_string());
    repo.add_klass(&k);

    {
        let fq_meth = "SampleInvoke.bar:()I";
        let meth = k
            .get_method_by_name_and_desc(&fq_meth.to_string())
            .unwrap_or_else(|| panic!("{} not found", fq_meth));
        assert_eq!(ACC_PUBLIC | ACC_STATIC, meth.get_flags());

        let mut vars = InterpLocalVars::of(5);
        let ret = exec_method(&mut repo, meth, &mut vars).unwrap();
        let ret2 = match ret {
            JvmValue::Int(i) => i,
            _ => panic!("Error executing SampleInvoke.bar:()I - non-int value returned"),
        };
        assert_eq!(7, ret2);
    }

    {
        let fq_meth = "SampleInvoke.foo:()I";
        let meth = k
            .get_method_by_name_and_desc(&fq_meth.to_string())
            .unwrap_or_else(|| panic!("{} not found", fq_meth));
        assert_eq!(ACC_PUBLIC | ACC_STATIC, meth.get_flags());

        let mut vars = InterpLocalVars::of(5);
        let ret = exec_method(&mut repo, meth, &mut vars).unwrap();
        let ret2 = match ret {
            JvmValue::Int(i) => i,
            _ => panic!("Error executing SampleInvoke.foo:()I - non-int value returned"),
        };
        assert_eq!(9, ret2);
    }
}

#[test]
fn test_math_sin() {
    let mut repo = init_repo();
    let k = simple_parse_klass("TestMathSin".to_string());
    repo.add_klass(&k);

    {
        let fq_meth = "TestMathSin.main_ifge:()I";
        let meth = k
            .get_method_by_name_and_desc(&fq_meth.to_string())
            .unwrap_or_else(|| panic!("{} not found", fq_meth));
        assert_eq!(ACC_PUBLIC | ACC_STATIC, meth.get_flags());

        let mut vars = InterpLocalVars::of(5);
        let ret = exec_method(&mut repo, meth, &mut vars).unwrap();
        let ret2 = match ret {
            JvmValue::Int(i) => i,
            _ => panic!("Error executing {} - non-int value returned", fq_meth),
        };
        assert_eq!(1, ret2);
    }

    {
        let fq_meth = "TestMathSin.main_ifle:()I";
        let meth = k
            .get_method_by_name_and_desc(&fq_meth.to_string())
            .unwrap_or_else(|| panic!("{} not found", fq_meth));
        assert_eq!(ACC_PUBLIC | ACC_STATIC, meth.get_flags());

        let mut vars = InterpLocalVars::of(5);
        let ret = exec_method(&mut repo, meth, &mut vars).unwrap();
        let ret2 = match ret {
            JvmValue::Int(i) => i,
            _ => panic!("Error executing {} - non-int value returned", fq_meth),
        };
        assert_eq!(0, ret2);
    }

    {
        let fq_meth = "TestMathSin.main_ifnull:()I";
        let meth = k
            .get_method_by_name_and_desc(&fq_meth.to_string())
            .unwrap_or_else(|| panic!("{} not found", fq_meth));
        assert_eq!(ACC_PUBLIC | ACC_STATIC, meth.get_flags());

        let mut vars = InterpLocalVars::of(5);
        let ret = exec_method(&mut repo, meth, &mut vars).unwrap();
        let ret2 = match ret {
            JvmValue::Int(i) => i,
            _ => panic!("Error executing {} - non-int value returned", fq_meth),
        };
        assert_eq!(0, ret2);
    }
}

#[test]
fn interp_iffer() {
    let mut repo = init_repo();
    let k = simple_parse_klass("Iffer".to_string());
    repo.add_klass(&k);

    {
        let fq_meth = "Iffer.baz:()I";
        let meth = k
            .get_method_by_name_and_desc(&fq_meth.to_string())
            .unwrap_or_else(|| panic!("{} not found", fq_meth));
        assert_eq!(ACC_PUBLIC | ACC_STATIC, meth.get_flags());

        let mut vars = InterpLocalVars::of(5);
        let ret = exec_method(&mut repo, meth, &mut vars).unwrap();
        let ret2 = match ret {
            JvmValue::Int(i) => i,
            _ => panic!("Error executing Iffer.baz:()I - non-int value returned"),
        };
        assert_eq!(3, ret2);
    }
}

#[test]
fn interp_array_set() {
    let mut repo = init_repo();
    let k = simple_parse_klass("ArraySimple".to_string());
    repo.add_klass(&k);

    {
        let fqname = "ArraySimple.baz:()I".to_string();
        let meth = k.get_method_by_name_and_desc(&fqname).unwrap();

        assert_eq!(ACC_PUBLIC | ACC_STATIC, meth.get_flags());

        let mut vars = InterpLocalVars::of(5);
        let ret = exec_method(&mut repo, meth, &mut vars).unwrap();
        let ret2 = match ret {
            JvmValue::Int(i) => i,
            _ => panic!("Error executing {} - non-int value returned", fqname),
        };
        assert_eq!(7, ret2);
    }
}

#[test]
fn interp_field_set() {
    let mut repo = init_repo();
    let k = simple_parse_klass("FieldHaver".to_string());
    repo.add_klass(&k);

    {
        let fqname = "FieldHaver.main2:([Ljava/lang/String;)I".to_string();
        let meth = k.get_method_by_name_and_desc(&fqname).unwrap();

        assert_eq!(ACC_PUBLIC | ACC_STATIC, meth.get_flags());

        let mut vars = InterpLocalVars::of(5);
        let ret = match exec_method(&mut repo, meth, &mut vars).unwrap() {
            JvmValue::Int(i) => i,
            _ => panic!("Error executing {} - non-int value returned", fqname),
        };
        assert_eq!(7, ret);
    }
}

#[test]
fn interp_system_current_timemillis() {
    let mut repo = init_repo();
    let k = simple_parse_klass("Main3".to_string());
    repo.add_klass(&k);

    {
        let fqname = "Main3.main2:([Ljava/lang/String;)I";
        let meth = k
            .get_method_by_name_and_desc(&fqname.to_string())
            .unwrap_or_else(|| panic!("{} not found", fqname));
        assert_eq!(ACC_PUBLIC | ACC_STATIC, meth.get_flags());

        let mut vars = InterpLocalVars::of(5);
        let ret = exec_method(&mut repo, meth, &mut vars).unwrap();
        let ctm1 = match ret {
            JvmValue::Int(i) => i,
            _ => panic!("Error executing {} - non-int value returned", fqname),
        };
        vars = InterpLocalVars::of(5);
        let opt_ret = exec_method(&mut repo, meth, &mut vars);
        let ret2 = match opt_ret {
            Some(value) => value,
            None => panic!("Error executing {} - no value returned", fqname),
        };
        let ctm2 = match ret2 {
            JvmValue::Int(i) => i,
            _ => panic!("Error executing {} - non-int value returned", fqname),
        };
        assert!(ctm2 >= ctm1, "System clock appears to go backwards");
    }
}

#[test]
#[ignore]
fn interp_class_based_addition() {
    let mut repo = init_repo();
    let k = simple_parse_klass("AddFieldInteger".to_string());
    repo.add_klass(&k);

    {
        let fqname = "AddFieldInteger.main2:([Ljava/lang/String;)I".to_string();
        let meth = k.get_method_by_name_and_desc(&fqname).unwrap();

        assert_eq!(ACC_PUBLIC | ACC_STATIC, meth.get_flags());

        let mut vars = InterpLocalVars::of(5);
        let ret = exec_method(&mut repo, meth, &mut vars).unwrap();
        let ret2 = match ret {
            JvmValue::Int(i) => i,
            _ => panic!("Error executing {} - non-int value returned", fqname),
        };
        assert_eq!(7, ret2);
    }
}

#[test]
fn interp_ldc_based_addition() {
    let mut repo = init_repo();
    let k = simple_parse_klass("AddLdc".to_string());
    repo.add_klass(&k);

    {
        let fqname = "AddLdc.main2:([Ljava/lang/String;)I".to_string();
        let meth = k.get_method_by_name_and_desc(&fqname).unwrap();

        assert_eq!(ACC_PUBLIC | ACC_STATIC, meth.get_flags());

        let mut vars = InterpLocalVars::of(5);
        let ret = exec_method(&mut repo, meth, &mut vars).unwrap();
        let ret2 = match ret {
            JvmValue::Int(i) => i,
            _ => panic!("Error executing {} - non-int value returned", fqname),
        };
        assert_eq!(44451, ret2);
    }
}
