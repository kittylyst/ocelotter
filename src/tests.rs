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
    let ret_jvm = execute_method(&first_test);
    let ret = match ret_jvm {
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
    let ret_jvm = execute_method(&buf);
    let ret = match ret_jvm {
        runtime::JVMValue::Int { val: i } => i,
        _ => {
            println!("Unexpected, non-integer value encountered");
            0
        }
    };
    assert_eq!(2, ret);
}

// @Test
// public void iconst_dup() {
//     byte[] buf = {ICONST_1.B(), DUP.B(), IADD.B(), IRETURN.B()};
//     JVMValue res = im.execMethod("", "main:()V", buf, new LocalVars());

//     assertEquals("Return type should be int", JVMType.I, res.type);
//     assertEquals("Return value should be 2", 2, (int) res.value);

//     byte[] buf2 = {ICONST_1.B(), DUP.B(), IADD.B(), DUP.B(), IADD.B(), IRETURN.B()};
//     res = im.execMethod("", "main:()V", buf2, new LocalVars());

//     assertEquals("Return type should be int", JVMType.I, res.type);
//     assertEquals("Return value should be 4", 4, (int) res.value);
// }

// @Test
// public void iconst_dup_nop_pop() {
//     byte[] buf = {ICONST_1.B(), DUP.B(), NOP.B(), POP.B(), IRETURN.B()};
//     JVMValue res = im.execMethod("", "main:()V", buf, new LocalVars());

//     assertEquals("Return type should be int", JVMType.I, res.type);
//     assertEquals("Return value should be 1", 1, (int) res.value);

//     byte[] buf2 = {ICONST_1.B(), DUP.B(), NOP.B(), POP.B(), POP.B(), RETURN.B()};
//     res = im.execMethod("", "main:()V", buf2, new LocalVars());

//     assertNull("Return should be null", res);
// }

// @Test
// public void iconst_dup_x1() {
//     byte[] buf = {ICONST_1.B(), ICONST_2.B(), DUP_X1.B(), IADD.B(), IADD.B(), IRETURN.B()};
//     JVMValue res = im.execMethod("", "main:()V", buf, new LocalVars());

//     assertEquals("Return type should be int", JVMType.I, res.type);
//     assertEquals("Return value should be 2", 5, (int) res.value);

//     byte[] buf2 = {ICONST_1.B(), ICONST_2.B(), DUP_X1.B(), IADD.B(), DUP_X1.B(), IADD.B(), IADD.B(), IRETURN.B()};
//     res = im.execMethod("", "main:()V", buf2, new LocalVars());

//     assertEquals("Return type should be int", JVMType.I, res.type);
//     assertEquals("Return value should be 4", 8, (int) res.value);
// }
