use ocelotter;

// mod runtime;
// use opcode;

#[test]
fn adds_to_two() {
    let first_test = vec![
        opcode::Opcode::ICONST_1,
        Opcode::ICONST_1,
        Opcode::IADD,
        Opcode::IRETURN,
    ];
    let lvt = ocelotter::runtime::LocalVariableTable {};
    let opt_ret = ocelotter::exec_method(
        "DUMMY".to_string(),
        "DUMMY_DESC".to_string(),
        &first_test,
        &lvt,
    );
    let ret_jvm = match opt_ret {
        Some(value) => value,
        None => JVMValue::ObjRef {},
    };
    let ret = match ret_jvm {
        JVMValue::Int { val: i } => i,
        _ => {
            println!("Unexpected, non-integer value encountered");
            0
        }
    };
    assert_eq!(2, ret);
}
