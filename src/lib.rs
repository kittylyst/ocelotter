#![deny(unreachable_patterns)]
mod klass_parser;
mod opcode;
mod runtime;
use object::*;
use opcode::*;
use runtime::*;

use crate::runtime::constant_pool::CpEntry;

#[macro_use]
extern crate lazy_static;

use std::sync::Mutex;

lazy_static! {
    static ref CONTEXT: Mutex<VmContext> = Mutex::new(VmContext::of());
}

pub fn exec_method2(meth: OtMethod) -> Option<JvmValue> {
    // HACK Replace with proper local var size by parsing class attributes properly
    let mut vars = InterpLocalVars::of(255);
    exec_method(meth.get_klass_name(), &meth.get_code(), &mut vars)
}

pub fn exec_method(
    klass_name: String,
    instr: &Vec<u8>,
    lvt: &mut InterpLocalVars,
) -> Option<JvmValue> {
    let mut current = 0;
    let mut eval = InterpEvalStack::of();

    // dbg!(instr);
    loop {
        let my_klass_name = klass_name.clone();
        let opt_ins = instr.get(current);
        let ins: u8 = match opt_ins {
            Some(value) => *value,
            None => panic!("Byte {} has no value", current),
        };
        current += 1;

        match ins {
            Opcode::ACONST_NULL => eval.aconst_null(),

            Opcode::ALOAD => {
                eval.push(lvt.load(instr[current]));
                current += 1;
            }
            Opcode::ALOAD_0 => eval.push(lvt.load(0)),

            Opcode::ALOAD_1 => eval.push(lvt.load(1)),

            Opcode::ARETURN => break Some(eval.pop()),
            Opcode::ASTORE => {
                lvt.store(instr[current], eval.pop());
                current += 1;
            }
            Opcode::ASTORE_0 => lvt.store(0, eval.pop()),

            Opcode::ASTORE_1 => lvt.store(1, eval.pop()),

            Opcode::BIPUSH => {
                eval.iconst(instr[current] as i32);
                current += 1;
            }
            Opcode::DADD => eval.dadd(),

            Opcode::DCONST_0 => eval.dconst(0.0),

            Opcode::DCONST_1 => eval.dconst(1.0),

            Opcode::DLOAD => {
                eval.push(lvt.load(instr[current]));
                current += 1;
            }

            Opcode::DLOAD_0 => eval.push(lvt.load(0)),

            Opcode::DLOAD_1 => eval.push(lvt.load(1)),

            Opcode::DLOAD_2 => eval.push(lvt.load(2)),

            Opcode::DLOAD_3 => eval.push(lvt.load(3)),

            Opcode::DRETURN => break Some(eval.pop()),
            Opcode::DSTORE => {
                lvt.store(instr[current], eval.pop());
                current += 1;
            }
            Opcode::DSTORE_0 => lvt.store(0, eval.pop()),

            Opcode::DSTORE_1 => lvt.store(1, eval.pop()),

            Opcode::DSTORE_2 => lvt.store(2, eval.pop()),

            Opcode::DSTORE_3 => lvt.store(3, eval.pop()),

            Opcode::DSUB => eval.dsub(),

            Opcode::DUP => eval.dup(),

            Opcode::DUP_X1 => eval.dupX1(),

            // GETFIELD => {
            //     let cp_lookup = ((int) instr[current++] << 8) + (int) instr[current++];
            //     OtField field = context.get_repo().lookupField(klass_name, (short) cp_lookup);
            //     JvmValue receiver = eval.pop();
            //     // VERIFY: Should check to make sure receiver is an Opcode::A
            //     OtObj obj = heap.findObject(receiver.value);
            //     eval.push(obj.getField(field));
            // },
            // GETSTATIC => {
            //     let cp_lookup = ((int) instr[current++] << 8) + (int) instr[current++];
            //     OtField f = context.get_repo().lookupField(klass_name, (short) cp_lookup);
            //     OtKlass fgKlass = f.getKlass();
            //     eval.push(fgKlass.getStaticField(f));
            // },
            Opcode::GOTO => {
                current += ((instr[current] as usize) << 8) + instr[current + 1] as usize
            }

            Opcode::I2D => eval.i2d(),

            Opcode::IADD => eval.iadd(),

            Opcode::IALOAD => {
                let pos_to_load = match eval.pop() {
                    JvmValue::Int { val: v } => v,
                    _ => panic!("Non-int seen on stack during IASTORE at {}", current - 1),
                };
                let arrayref = match eval.pop() {
                    JvmValue::ObjRef { val: v } => v,
                    _ => panic!("Non-objref seen on stack during IASTORE at {}", current - 1),
                };
                dbg!(arrayref.clone());

                let unwrapped_val = match arrayref {
                    OtObj::vm_arr_int {
                        mark: _,
                        klassid: _,
                        length: _,
                        elements: mut elts,
                    } => elts[pos_to_load as usize],
                    _ => panic!("Non-int[] seen on stack during IASTORE at {}", current - 1),
                };
                eval.push(JvmValue::Int { val: unwrapped_val });
            }

            Opcode::IAND => eval.iand(),

            Opcode::IASTORE => {
                let val_to_store = match eval.pop() {
                    JvmValue::Int { val: v } => v,
                    _ => panic!("Non-int seen on stack during IASTORE at {}", current - 1),
                };
                let pos_to_store = match eval.pop() {
                    JvmValue::Int { val: v } => v,
                    _ => panic!("Non-int seen on stack during IASTORE at {}", current - 1),
                };
                let arrayref = match eval.pop() {
                    JvmValue::ObjRef { val: v } => v,
                    _ => panic!("Non-objref seen on stack during IASTORE at {}", current - 1),
                };
                let arrc = arrayref.clone();
                dbg!(arrc.clone());

                match arrayref {
                    OtObj::vm_arr_int {
                        mark: _,
                        klassid: _,
                        length: _,
                        elements: mut elts,
                    } => {
                        elts[pos_to_store as usize] = val_to_store;
                        dbg!(val_to_store);
                    }
                    _ => panic!("Non-int[] seen on stack during IASTORE at {}", current - 1),
                };
                // let arrc2 = arrayref.clone();
                // dbg!(arrc2.clone());
            }

            Opcode::ICONST_0 => eval.iconst(0),

            Opcode::ICONST_1 => eval.iconst(1),

            Opcode::ICONST_2 => eval.iconst(2),

            Opcode::ICONST_3 => eval.iconst(3),

            Opcode::ICONST_4 => eval.iconst(4),

            Opcode::ICONST_5 => eval.iconst(5),

            Opcode::ICONST_M1 => eval.iconst(-1),

            Opcode::IDIV => eval.idiv(),

            Opcode::IF_ICMPEQ => {
                let jumpTo = (instr[current] as usize) << 8 + instr[current + 1] as usize;
                if massage_to_jvm_int_and_equate(eval.pop(), eval.pop()) {
                    current += jumpTo;
                } else {
                    current += 2;
                }
            }
            Opcode::IF_ICMPNE => {
                let jumpTo = (instr[current] as usize) << 8 + instr[current + 1] as usize;
                if massage_to_jvm_int_and_equate(eval.pop(), eval.pop()) {
                    current += 2;
                } else {
                    current += jumpTo;
                }
            }
            // Opcode::IFEQ => {
            //     let jumpTo = (instr[current] as usize) << 8 + instr[current + 1] as usize;
            //     let i = match eval.pop() {

            //     }
            //     if == 0 {
            //         current += jumpTo;
            //     } else {
            //         current += 2;
            //     }
            // }    ,
            // Opcode::IFGE => {
            //     v = eval.pop();
            //     jumpTo = ((int) instr[current++] << 8) + (int) instr[current++];
            //     if (v.value >= 0L) {
            //         current += jumpTo - 1; // The -1 is necessary as we've already inc'd current
            //     }
            // } ,
            // Opcode::IFGT => {
            //     v = eval.pop();
            //     jumpTo = ((int) instr[current++] << 8) + (int) instr[current++];
            //     if (v.value > 0L) {
            //         current += jumpTo - 1; // The -1 is necessary as we've already inc'd current
            //     }
            // },
            // Opcode::IFLE => {
            //     v = eval.pop();
            //     jumpTo = ((int) instr[current++] << 8) + (int) instr[current++];
            //     if (v.value <= 0L) {
            //         current += jumpTo - 1; // The -1 is necessary as we've already inc'd current
            //     }
            // },
            // Opcode::IFLT => {
            //     v = eval.pop();
            //     jumpTo = ((int) instr[current++] << 8) + (int) instr[current++];
            //     if (v.value < 0L) {
            //         current += jumpTo - 1; // The -1 is necessary as we've already inc'd current
            //     }
            // },
            // Opcode::IFNE => {
            //     v = eval.pop();
            //     jumpTo = ((int) instr[current] << 8) + (int) instr[current + 1];
            //     if (v.value != 0L) {
            //         current += jumpTo - 1;  // The -1 is necessary as we've already inc'd current
            //     }
            // },
            Opcode::IFNONNULL => {
                let jumpTo = ((instr[current] as usize) << 8) + instr[current + 1] as usize;

                match eval.pop() {
                    JvmValue::ObjRef { val: v } => {
                        if !v.is_null() {
                            current += jumpTo;
                        } else {
                            current += 2;
                        }
                    }
                    _ => panic!(
                        "Value not of reference type found for IFNULL at {}",
                        (current - 1)
                    ),
                };
            }
            Opcode::IFNULL => {
                let jumpTo = ((instr[current] as usize) << 8) + instr[current + 1] as usize;

                match eval.pop() {
                    JvmValue::ObjRef { val: v } => {
                        if v.is_null() {
                            // println!("Ins[curr]: {} and {}", instr[current], instr[current + 1]);
                            // println!("Attempting to jump by: {} from {}", jumpTo, current);
                            current += jumpTo;
                        } else {
                            current += 2;
                        }
                    }
                    _ => panic!(
                        "Value not of reference type found for IFNULL at {}",
                        (current - 1)
                    ),
                };
            }
            Opcode::IINC => {
                lvt.iinc(instr[current], instr[current + 1]);
                current += 2;
            }

            Opcode::ILOAD => {
                eval.push(lvt.load(instr[current]));
                current += 1
            }

            Opcode::ILOAD_0 => eval.push(lvt.load(0)),

            Opcode::ILOAD_1 => eval.push(lvt.load(1)),

            Opcode::ILOAD_2 => eval.push(lvt.load(2)),

            Opcode::ILOAD_3 => eval.push(lvt.load(3)),

            Opcode::IMUL => eval.imul(),

            Opcode::INEG => eval.ineg(),

            Opcode::INVOKESPECIAL => {
                let cp_lookup = ((instr[current] as u16) << 8) + instr[current + 1] as u16;
                current += 2;
                let current_klass = CONTEXT.lock().unwrap().get_repo().lookup_klass(&klass_name).clone();
                dbg!(current_klass.clone());
                dispatch_invoke(current_klass, cp_lookup, &mut eval, 1);
            }
            Opcode::INVOKESTATIC => {
                let cp_lookup = ((instr[current] as u16) << 8) + instr[current + 1] as u16;
                current += 2;
                let current_klass = CONTEXT.lock().unwrap().get_repo().lookup_klass(&klass_name).clone();
                dbg!(current_klass.clone());
                dispatch_invoke(current_klass, cp_lookup, &mut eval, 0);
            }
            // FIXME DOES NOT ACTUALLY DO VIRTUAL LOOKUP YET
            Opcode::INVOKEVIRTUAL => {
                // let cp_lookup = ((instr[current] as u16) << 8) + instr[current + 1] as u16;
                // current += 2;
                // dispatch_invoke(context.get_repo().lookup_method_virtual(&klass_name, cp_lookup), &eval);
            }
            Opcode::IOR => eval.ior(),

            Opcode::IREM => eval.irem(),

            Opcode::IRETURN => break Some(eval.pop()),
            Opcode::ISTORE => {
                lvt.store(instr[current], eval.pop());
                current += 1;
            }
            Opcode::ISTORE_0 => lvt.store(0, eval.pop()),

            Opcode::ISTORE_1 => lvt.store(1, eval.pop()),

            Opcode::ISTORE_2 => lvt.store(2, eval.pop()),

            Opcode::ISTORE_3 => lvt.store(3, eval.pop()),

            Opcode::ISUB => eval.isub(),
            // Dummy implementation
            Opcode::LDC => {
                // System.out.print("Executing " + op + " with param bytes: ");
                // for (int i = current; i < current + num; i++) {
                //     System.out.print(instr[i] + " ");
                // }
                // current += num;
                // System.out.println();
            }

            // FIXME TEMP
            Opcode::MONITORENTER => {
                eval.pop();
            }
            // FIXME TEMP
            Opcode::MONITOREXIT => {
                eval.pop();
            }
            Opcode::NEW => {
                let cp_lookup = ((instr[current] as u16) << 8) + instr[current + 1] as u16;
                current += 2;
                let current_klass = CONTEXT.lock().unwrap().get_repo().lookup_klass(&klass_name).clone();

                let klass_name = match current_klass.lookup_cp(cp_lookup) {
                    CpEntry::class { idx } => "DUMMY_CLASS".to_string(), // FIXME
                    _ => panic!(
                        "Non-class found in {} at CP index {}",
                        current_klass.get_name(),
                        cp_lookup
                    ),
                };

                eval.push(JvmValue::ObjRef {
                    val: CONTEXT.lock().unwrap().allocate_obj(&current_klass),
                });
            }
            Opcode::NEWARRAY => {
                let arr_type = instr[current];
                current += 1;

                // FIXME Other primitive array types needed
                let arr_ref = match arr_type {
                    // boolean: 4
                    // char: 5
                    // float: 6
                    // double: 7
                    // byte: 8
                    // short: 9
                    // int: 10
                    // long: 11
                    10 => match eval.pop() {
                        JvmValue::Int { val: arr_size } => {
                            CONTEXT.lock().unwrap().allocate_int_arr(arr_size)
                        }
                        _ => panic!("Not an int on the stack at {}", (current - 1)),
                    },
                    _ => panic!("Unsupported primitive array type at {}", (current - 1)),
                };

                eval.push(JvmValue::ObjRef { val: arr_ref });
            }

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
            Opcode::PUTFIELD => {
                let cp_lookup = ((instr[current] as u16) << 8) + instr[current + 1] as u16;
                current += 2;

                let putf = CONTEXT.lock().unwrap().get_repo().lookup_field(my_klass_name.clone(), cp_lookup);
                let val = eval.pop();

                let recvp: JvmValue = eval.pop();
                // VERIFY: Should check to make sure receiver is an A
                // FIXME Match expression & destructure for recvp
                let obj = match recvp {
                    JvmValue::ObjRef { val: v } => v,
                    _ => panic!("Not an object ref at {}", (current - 1)),
                };

                obj.put_field(putf, val);
            }
            Opcode::PUTSTATIC => {
                let cp_lookup = ((instr[current] as u16) << 8) + instr[current + 1] as u16;
                current += 2;

                let puts = CONTEXT.lock().unwrap().get_repo().lookup_field(my_klass_name.clone(), cp_lookup);
                let f_klass = puts.get_klass();
                f_klass.set_static_field(puts.get_name(), eval.pop());
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
        JvmValue::ObjRef { val: v } => panic!("Values found to have differing type for IF_ICMP*"),
    }
}

fn dispatch_invoke(
    current_klass: OtKlass,
    cp_lookup: u16,
    eval: &mut InterpEvalStack,
    additional_args: u8,
) -> () {
    let fq_name_desc = current_klass.cp_as_string(cp_lookup);
    let klz_idx = match current_klass.lookup_cp(cp_lookup) {
        CpEntry::methodref { clz_idx, nt_idx } => clz_idx,
        _ => panic!(
            "Non-methodref found in {} at CP index {}",
            current_klass.get_name(),
            cp_lookup
        ),
    };
    let dispatch_klass_name = current_klass.cp_as_string(klz_idx);
    let callee = CONTEXT.lock().unwrap().get_repo().lookup_method_exact(&dispatch_klass_name, fq_name_desc);

    // FIXME - General setup requires call args
    let mut vars = InterpLocalVars::of(255);
    if additional_args > 0 {
        vars.store(0, eval.pop());
    }
    match exec_method(callee.get_klass_name(), &callee.get_code(), &mut vars) {
        Some(val) => eval.push(val),
        None => (),
    }
}

fn parse_class(bytes: Vec<u8>, fname: String) -> OtKlass {
    let mut parser = klass_parser::oc_parser::of(bytes, fname);
    parser.parse();
    parser.klass()
}

#[cfg(test)]
mod tests;
