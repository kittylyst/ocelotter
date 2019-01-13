use super::*;

#[test]
fn adds_to_two() {
    let first_test = vec![
        opcode::Opcode::ICONST_1,
        opcode::Opcode::ICONST_1,
        opcode::Opcode::IADD,
        opcode::Opcode::IRETURN,
    ];
    let lvt = runtime::LocalVariableTable {};
    let opt_ret = exec_method(
        "DUMMY".to_string(),
        "DUMMY_DESC".to_string(),
        &first_test,
        &lvt,
    );
    let ret_jvm = match opt_ret {
        Some(value) => value,
        None => runtime::JVMValue::ObjRef {
            val: runtime::JVMObj::get_null(),
        },
    };
    let ret = match ret_jvm {
        runtime::JVMValue::Int { val: i } => i,
        _ => {
            println!("Unexpected, non-integer value encountered");
            0
        }
    };
    assert_eq!(2, ret);
}
