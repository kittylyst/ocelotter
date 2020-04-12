#![deny(unreachable_patterns)]

use ocelotter_runtime::constant_pool::*;
use ocelotter_runtime::interp_stack::InterpEvalStack;
use ocelotter_runtime::otklass::OtKlass;
use ocelotter_runtime::otmethod::OtMethod;
use ocelotter_runtime::klass_repo::SharedKlassRepo;
use ocelotter_runtime::*;

pub mod opcode;
use opcode::*;

pub fn exec_method(
    repo: &mut SharedKlassRepo,
    meth: &OtMethod,
    lvt: &mut InterpLocalVars,
) -> Option<JvmValue> {
    dbg!(meth.clone());
    // dbg!(meth.get_flags());
    if meth.is_native() {
        // Explicit type hint here to document the type of n_f
        let n_f: fn(&InterpLocalVars) -> Option<JvmValue> = meth.get_native_code().expect(
            &format!("Native code not found {}", meth.get_fq_name_desc()),
        );

        // FIXME Parameter passing
        n_f(lvt)
    } else {
        exec_bytecode_method(repo, meth.get_klass_name(), &meth.get_code(), lvt)
    }
}

pub fn exec_bytecode_method(
    repo: &mut SharedKlassRepo,
    klass_name: String,
    instr: &Vec<u8>,
    lvt: &mut InterpLocalVars,
) -> Option<JvmValue> {
    let mut current = 0;
    let mut eval = InterpEvalStack::of();

    loop {
        // let my_klass_name = klass_name.clone();
        let ins: u8 = *instr
            .get(current)
            .expect(&format!("Byte {} has no value", current));

        current += 1;

        dbg!(ins);
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

            Opcode::GETFIELD => {
                let cp_lookup = ((instr[current] as u16) << 8) + instr[current + 1] as u16;
                current += 2;

                let recvp: JvmValue = eval.pop();
                let obj_id = match recvp {
                    JvmValue::ObjRef { val: v } => v,
                    _ => panic!("Not an object ref at {}", (current - 1)),
                };
                let heap = HEAP.lock().unwrap();
                let obj = heap.get_obj(obj_id).clone();
                let getf = repo.lookup_instance_field(&klass_name, cp_lookup);

                let ret = obj.get_field_value(getf.get_offset() as usize);
                eval.push(ret);
            }
            Opcode::GETSTATIC => {
                let cp_lookup = ((instr[current] as u16) << 8) + instr[current + 1] as u16;
                current += 2;

                let getf = repo.lookup_static_field(&klass_name, cp_lookup).clone();
                let klass = repo.lookup_klass(&getf.get_klass_name()).clone();

                let ret = klass.get_static_field_value(&getf);
                eval.push(ret.clone());
            }
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
                let arrayid = match eval.pop() {
                    JvmValue::ObjRef { val: v } => v,
                    _ => panic!("Non-objref seen on stack during IASTORE at {}", current - 1),
                };
                dbg!(arrayid.clone());

                let unwrapped_val = match HEAP.lock().unwrap().get_obj(arrayid) {
                    ocelotter_runtime::object::OtObj::vm_arr_int {
                        id: _,
                        mark: _,
                        klassid: _,
                        length: _,
                        elements: elts,
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
                let obj_id = match eval.pop() {
                    JvmValue::ObjRef { val: v } => v,
                    _ => panic!("Non-objref seen on stack during IASTORE at {}", current - 1),
                };

                HEAP.lock()
                    .unwrap()
                    .iastore(obj_id, pos_to_store, val_to_store);
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
                let jump_to = (instr[current] as usize) << 8 + instr[current + 1] as usize;
                if massage_to_int_and_compare(eval.pop(), eval.pop(), |i: i32, j: i32| -> bool {
                    i == j
                }) {
                    current += jump_to;
                } else {
                    current += 2;
                }
            }
            Opcode::IF_ICMPGT => {
                let jump_to = (instr[current] as usize) << 8 + instr[current + 1] as usize;
                if massage_to_int_and_compare(eval.pop(), eval.pop(), |i: i32, j: i32| -> bool {
                    i > j
                }) {
                    current += jump_to;
                } else {
                    current += 2;
                }
            }

            Opcode::IF_ICMPLT => {
                let jump_to = (instr[current] as usize) << 8 + instr[current + 1] as usize;
                if massage_to_int_and_compare(eval.pop(), eval.pop(), |i: i32, j: i32| -> bool {
                    i < j
                }) {
                    current += jump_to;
                } else {
                    current += 2;
                }
            }
            Opcode::IF_ICMPNE => {
                let jump_to = (instr[current] as usize) << 8 + instr[current + 1] as usize;
                if massage_to_int_and_compare(eval.pop(), eval.pop(), |i: i32, j: i32| -> bool {
                    i == j
                }) {
                    current += 2;
                } else {
                    current += jump_to;
                }
            }
            // Opcode::IFEQ => {
            //     let jump_to = (instr[current] as usize) << 8 + instr[current + 1] as usize;
            //     let i = match eval.pop() {

            //     }
            //     if == 0 {
            //         current += jump_to;
            //     } else {
            //         current += 2;
            //     }
            // }    ,
            // Opcode::IFGE => {
            //     v = eval.pop();
            //     jump_to = ((int) instr[current++] << 8) + (int) instr[current++];
            //     if (v.value >= 0L) {
            //         current += jump_to - 1; // The -1 is necessary as we've already inc'd current
            //     }
            // } ,
            // Opcode::IFGT => {
            //     v = eval.pop();
            //     jump_to = ((int) instr[current++] << 8) + (int) instr[current++];
            //     if (v.value > 0L) {
            //         current += jump_to - 1; // The -1 is necessary as we've already inc'd current
            //     }
            // },
            // Opcode::IFLE => {
            //     v = eval.pop();
            //     jump_to = ((int) instr[current++] << 8) + (int) instr[current++];
            //     if (v.value <= 0L) {
            //         current += jump_to - 1; // The -1 is necessary as we've already inc'd current
            //     }
            // },
            // Opcode::IFLT => {
            //     v = eval.pop();
            //     jump_to = ((int) instr[current++] << 8) + (int) instr[current++];
            //     if (v.value < 0L) {
            //         current += jump_to - 1; // The -1 is necessary as we've already inc'd current
            //     }
            // },
            // Opcode::IFNE => {
            //     v = eval.pop();
            //     jump_to = ((int) instr[current] << 8) + (int) instr[current + 1];
            //     if (v.value != 0L) {
            //         current += jump_to - 1;  // The -1 is necessary as we've already inc'd current
            //     }
            // },
            Opcode::IFNONNULL => {
                let jump_to = ((instr[current] as usize) << 8) + instr[current + 1] as usize;

                match eval.pop() {
                    JvmValue::ObjRef { val: v } => {
                        if v > 0 {
                            current += jump_to;
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
                let jump_to = ((instr[current] as usize) << 8) + instr[current + 1] as usize;

                match eval.pop() {
                    JvmValue::ObjRef { val: v } => {
                        if v == 0 {
                            current += jump_to;
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
                let current_klass = repo.lookup_klass(&klass_name).clone();
                dispatch_invoke(repo, current_klass, cp_lookup, &mut eval, 1);
            }
            Opcode::INVOKESTATIC => {
                let cp_lookup = ((instr[current] as u16) << 8) + instr[current + 1] as u16;
                current += 2;
                let current_klass = repo.lookup_klass(&klass_name).clone();
                dbg!(current_klass.clone());
                dispatch_invoke(repo, current_klass, cp_lookup, &mut eval, 0);
            }
            Opcode::INVOKEVIRTUAL => {
                // FIXME DOES NOT ACTUALLY DO VIRTUAL LOOKUP YET
                let cp_lookup = ((instr[current] as u16) << 8) + instr[current + 1] as u16;
                current += 2;
                let current_klass = repo.lookup_klass(&klass_name).clone();
                dbg!(current_klass.clone());
                dispatch_invoke(repo, current_klass, cp_lookup, &mut eval, 1);
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
            Opcode::L2I => {
                match eval.pop() {
                    JvmValue::Long { val: v } => eval.push(JvmValue::Int { val: v as i32 }),
                    _ => panic!("Value not of long type found for L2I at {}", (current - 1)),
                };
            }
            Opcode::LDC => {
                let cp_lookup = instr[current] as u16;
                current += 1;
                let current_klass = repo.lookup_klass(&klass_name).clone();

                match current_klass.lookup_cp(cp_lookup) {
                    // FIXME Actually look up the class object properly
                    CpEntry::class { idx: _ } => eval.aconst_null(),
                    CpEntry::double { val: dcon } => eval.dconst(dcon),
                    CpEntry::integer { val: icon } => eval.iconst(icon),
                    // FIXME Actually look up the class object properly
                    CpEntry::string { idx: _ } => eval.aconst_null(),
                    _ => panic!(
                        "Non-handled entry found in LDC op {} at CP index {}",
                        current_klass.get_name(),
                        cp_lookup
                    ),
                }
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
                let current_klass = repo.lookup_klass(&klass_name).clone();

                let alloc_klass_name = match current_klass.lookup_cp(cp_lookup) {
                    // FIXME Find class name from constant pool of the current class
                    CpEntry::class { idx } => current_klass.cp_as_string(idx), // "DUMMY_CLASS".to_string(),
                    _ => panic!(
                        "Non-class found in {} at CP index {}",
                        current_klass.get_name(),
                        cp_lookup
                    ),
                };
                dbg!(alloc_klass_name.clone());
                let object_klass = repo.lookup_klass(&alloc_klass_name).clone();

                let obj_id = HEAP.lock().unwrap().allocate_obj(&object_klass);
                eval.push(JvmValue::ObjRef { val: obj_id });
            }
            Opcode::NEWARRAY => {
                let arr_type = instr[current];
                current += 1;

                // FIXME Other primitive array types needed
                let arr_id = match arr_type {
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
                            HEAP.lock().unwrap().allocate_int_arr(arr_size)
                        }
                        _ => panic!("Not an int on the stack at {}", (current - 1)),
                    },
                    _ => panic!("Unsupported primitive array type at {}", (current - 1)),
                };

                eval.push(JvmValue::ObjRef { val: arr_id });
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

                let val = eval.pop();

                let recvp: JvmValue = eval.pop();
                let obj_id = match recvp {
                    JvmValue::ObjRef { val: v } => v,
                    _ => panic!("Not an object ref at {}", (current - 1)),
                };

                let putf = repo.lookup_instance_field(&klass_name, cp_lookup);

                HEAP.lock().unwrap().put_field(obj_id, putf, val);
            }
            Opcode::PUTSTATIC => {
                let cp_lookup = ((instr[current] as u16) << 8) + instr[current + 1] as u16;
                current += 2;

                let puts = repo.lookup_static_field(&klass_name, cp_lookup);
                let klass_name = puts.get_klass_name();
                // FIXME IMPL IS BROKEN
                repo.put_static(klass_name, puts, eval.pop());
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

fn massage_to_int_and_compare(v1: JvmValue, v2: JvmValue, f: fn(i: i32, j: i32) -> bool) -> bool {
    match v1 {
        JvmValue::Int { val: i } => match v2 {
            JvmValue::Int { val: i1 } => f(i, i1),
            _ => panic!("Values found to have differing type for IF_ICMP*"),
        },
        _ => panic!("Values found to have the wrong type for IF_ICMP*"),
    }
}

fn dispatch_invoke(
    repo: &mut SharedKlassRepo,
    current_klass: OtKlass,
    cp_lookup: u16,
    eval: &mut InterpEvalStack,
    additional_args: u8,
) -> () {
    let fq_name_desc = current_klass.cp_as_string(cp_lookup);
    let klz_idx = match current_klass.lookup_cp(cp_lookup) {
        CpEntry::methodref { clz_idx, nt_idx: _ } => clz_idx,
        _ => panic!(
            "Non-methodref found in {} at CP index {}",
            current_klass.get_name(),
            cp_lookup
        ),
    };
    let dispatch_klass_name = current_klass.cp_as_string(klz_idx);

    let callee = repo.lookup_method_exact(&dispatch_klass_name, fq_name_desc);

    // FIXME - General setup requires call args from the stack
    let mut vars = InterpLocalVars::of(255);
    if additional_args > 0 {
        vars.store(0, eval.pop());
    }
    // Explicit use of match expression to be clear about the semantics
    match exec_method(repo, &callee, &mut vars) {
        Some(val) => eval.push(val),
        None => (),
    }
}

// fn parse_class(bytes: Vec<u8>, fname: String) -> OtKlass {
//     let mut parser = klass_parser::OtKlassParser::of(bytes, fname);
//     parser.parse();
//     parser.klass()
// }

#[cfg(test)]
mod tests;
