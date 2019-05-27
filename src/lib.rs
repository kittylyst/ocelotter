#![deny(unreachable_patterns)]

use ocelotter_runtime::*;

pub mod opcode;
use opcode::*;

pub fn exec_method2(
    klass_name: String,
    instr: &Vec<u8>,
    lvt: &mut InterpLocalVars,
) -> Option<JvmValue> {
    let mut current = 0;
    let mut eval = InterpEvalStack::of();

    loop {
        let my_klass_name = klass_name.clone();
        let ins: u8 = match instr.get(current) {
            Some(value) => *value,
            // FIXME We don't know the name of the currently executing method!
            None => panic!("Byte {} has no value", current),
        };
        current += 1;

        dbg!(ins);
        match ins {
            Opcode::BIPUSH => {
                eval.iconst(instr[current] as i32);
                current += 1;
            }

            Opcode::DUP => eval.dup(),

            Opcode::DUP_X1 => eval.dupX1(),

            Opcode::IADD => eval.iadd(),

            Opcode::IAND => eval.iand(),

            Opcode::ICONST_0 => eval.iconst(0),

            Opcode::ICONST_1 => eval.iconst(1),

            Opcode::ICONST_2 => eval.iconst(2),

            Opcode::ICONST_3 => eval.iconst(3),

            Opcode::ICONST_4 => eval.iconst(4),

            Opcode::ICONST_5 => eval.iconst(5),

            Opcode::ICONST_M1 => eval.iconst(-1),

            Opcode::IDIV => eval.idiv(),

            Opcode::IMUL => eval.imul(),

            Opcode::INEG => eval.ineg(),

            Opcode::IOR => eval.ior(),

            Opcode::IREM => eval.irem(),

            Opcode::IRETURN => break Some(eval.pop()),

            Opcode::ISUB => eval.isub(),

            Opcode::NOP => {
                ();
            }

            Opcode::POP => {
                eval.pop();
            }
            Opcode::POP2 => {
                let _discard: JvmValue = eval.pop();
                // FIXME Change to type match
                // if (discard.type == JVMType.J || discard.type == JVMType.D) {

                // }
                eval.pop();
            }
            Opcode::RETURN => break None,
            Opcode::SIPUSH => {
                let vtmp = ((instr[current] as i32) << 8) + instr[current + 1] as i32;
                eval.iconst(vtmp);
                current += 2;
            }
            Opcode::SWAP => {
                let val1 = eval.pop();
                let val2 = eval.pop();
                eval.push(val1);
                eval.push(val2);
            }
            // Disallowed opcodes
            Opcode::BREAKPOINT => break Some(JvmValue::Boolean { val: false }),
            Opcode::IMPDEP1 => break Some(JvmValue::Boolean { val: false }),
            Opcode::IMPDEP2 => break Some(JvmValue::Boolean { val: false }),
            Opcode::JSR => break Some(JvmValue::Boolean { val: false }),
            Opcode::JSR_W => break Some(JvmValue::Boolean { val: false }),
            Opcode::RET => break Some(JvmValue::Boolean { val: false }),

            _ => panic!(
                "Illegal opcode byte: {} encountered at position {}. Stopping.",
                ins,
                (current - 1)
            ),
        }
    }
}

fn massage_to_jvm_int_and_equate(v1: JvmValue, v2: JvmValue) -> bool {
    match v1 {
        JvmValue::Boolean { val: b } => match v2 {
            JvmValue::Boolean { val: b1 } => b == b1,
            _ => panic!("Values found to have differing type for IF_ICMP*"),
        },
        JvmValue::Byte { val: b } => match v2 {
            JvmValue::Byte { val: b1 } => b == b1,
            _ => panic!("Values found to have differing type for IF_ICMP*"),
        },
        JvmValue::Short { val: s } => match v2 {
            JvmValue::Short { val: s1 } => s == s1,
            _ => panic!("Values found to have differing type for IF_ICMP*"),
        },
        JvmValue::Int { val: i } => match v2 {
            JvmValue::Int { val: i1 } => i == i1,
            _ => panic!("Values found to have differing type for IF_ICMP*"),
        },
        JvmValue::Long { val: i } => match v2 {
            JvmValue::Long { val: i1 } => i == i1,
            _ => panic!("Values found to have differing type for IF_ICMP*"),
        },
        JvmValue::Float { val: i } => match v2 {
            JvmValue::Float { val: i1 } => i == i1,
            _ => panic!("Values found to have differing type for IF_ICMP*"),
        },
        JvmValue::Double { val: i } => match v2 {
            JvmValue::Double { val: i1 } => i == i1,
            _ => panic!("Values found to have differing type for IF_ICMP*"),
        },
        JvmValue::Char { val: i } => match v2 {
            JvmValue::Char { val: i1 } => i == i1,
            _ => panic!("Values found to have differing type for IF_ICMP*"),
        },
        JvmValue::ObjRef { val: _ } => panic!("Values found to have differing type for IF_ICMP*"),
    }
}

#[cfg(test)]
mod tests;
