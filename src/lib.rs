mod klass_parser;
mod opcode;
mod runtime;

use opcode::*;
// use runtime::*;

static heap: runtime::shared_simple_heap = runtime::shared_simple_heap {};
static repo: runtime::shared_klass_repo = runtime::shared_klass_repo {};

pub fn exec_method2(mut meth: runtime::ot_method) -> Option<runtime::jvm_value> {
    let vars = runtime::interp_local_vars {};
    exec_method(
        meth.get_klass_name(),
        "DUMMY_DESC".to_string(),
        &meth.get_code(),
        &vars,
    )
}

pub fn exec_method(
    klass_name: String,
    _desc: String,
    instr: &Vec<u8>,
    lvt: &runtime::interp_local_vars,
) -> Option<runtime::jvm_value> {
    let mut current = 0;
    let mut eval = runtime::interp_eval_stack::new();
    let current_klass = repo.lookup_klass(klass_name.clone());

    dbg!(instr);
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
                eval.push(lvt.aload(instr[current]));
                current += 1;
            }
            Opcode::ALOAD_0 => eval.push(lvt.aload(0)),

            Opcode::ALOAD_1 => eval.push(lvt.aload(1)),

            Opcode::ARETURN => break Some(eval.pop()),
            Opcode::ASTORE => {
                lvt.astore(instr[current], eval.pop());
                current += 1;
            }
            Opcode::ASTORE_0 => lvt.astore(0, eval.pop()),

            Opcode::ASTORE_1 => lvt.astore(1, eval.pop()),

            Opcode::BIPUSH => {
                eval.iconst(instr[current] as i32);
                current += 1;
            }
            Opcode::DADD => eval.dadd(),

            Opcode::DCONST_0 => eval.dconst(0.0),

            Opcode::DCONST_1 => eval.dconst(1.0),

            Opcode::DLOAD => {
                eval.push(lvt.dload(instr[current]));
                current += 1;
            }

            Opcode::DLOAD_0 => eval.push(lvt.dload(0)),

            Opcode::DLOAD_1 => eval.push(lvt.dload(1)),

            Opcode::DLOAD_2 => eval.push(lvt.dload(2)),

            Opcode::DLOAD_3 => eval.push(lvt.dload(3)),

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
            //     runtime::ot_field field = repo.lookupField(klass_name, (short) cp_lookup);
            //     runtime::jvm_value receiver = eval.pop();
            //     // VERIFY: Should check to make sure receiver is an Opcode::A
            //     runtime::ot_obj obj = heap.findObject(receiver.value);
            //     eval.push(obj.getField(field));
            // },
            // GETSTATIC => {
            //     let cp_lookup = ((int) instr[current++] << 8) + (int) instr[current++];
            //     runtime::ot_field f = repo.lookupField(klass_name, (short) cp_lookup);
            //     runtime::ot_klass fgKlass = f.getKlass();
            //     eval.push(fgKlass.getStaticField(f));
            // },
            Opcode::GOTO => {
                current += ((instr[current] as usize) << 8) + instr[current + 1] as usize
            }

            Opcode::I2D => eval.i2d(),

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

            // Opcode::IF_ICMPEQ => {
            //     let jumpTo = (instr[current] as usize) << 8 + instr[current + 1] as usize;
            //     let v1 = match eval.pop() {
            //         runtime::jvm_value::ObjRef { val: v } => v,
            //         _ => println!("Value not of reference type found for IFNULL"),
            //     };
            //     let v2 = match eval.pop() {
            //         runtime::jvm_value::ObjRef { val: v } => v,
            //         _ => println!("Value not of reference type found for IFNULL"),
            //     };
            //     if v1 == v2 {
            //         current += jumpTo - 1; // The -1 is necessary as we've already inc'd current
            //     }
            // },
            // Opcode::IFEQ => {
            //     v = eval.pop();
            //     jumpTo = ((int) instr[current++] << 8) + (int) instr[current++];
            //     if (v.value == 0L) {
            //         current += jumpTo - 1; // The -1 is necessary as we've already inc'd current
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
                    runtime::jvm_value::ObjRef { val: v } => {
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
                    runtime::jvm_value::ObjRef { val: v } => {
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
                eval.push(lvt.iload(instr[current]));
                current += 1
            }

            Opcode::ILOAD_0 => eval.push(lvt.iload(0)),

            Opcode::ILOAD_1 => eval.push(lvt.iload(1)),

            Opcode::ILOAD_2 => eval.push(lvt.iload(2)),

            Opcode::ILOAD_3 => eval.push(lvt.iload(3)),

            Opcode::IMUL => eval.imul(),

            Opcode::INEG => eval.ineg(),

            // FIXME DOES NOT ACTUALLY DO EXACT LOOKUP YET
            Opcode::INVOKESPECIAL => {
                // let cp_lookup = ((instr[current] as u16) << 8) + instr[current + 1] as u16;
                // current += 2;
                // let cp_fq_name_desc = current_klass.lookup_cp(cp_lookup);
                // dispatch_invoke(repo.lookup_method_exact(dispatch_klass_name, local_name_desc), &eval);
            }
            Opcode::INVOKESTATIC => {
                let cp_lookup = ((instr[current] as u16) << 8) + instr[current + 1] as u16;
                current += 2;
                let fq_name_desc = match current_klass.lookup_cp(cp_lookup) {
                    runtime::cp_entry::methodref { clz_idx, nt_idx } => ".xxxx:()V", // FIXME
                    _ => panic!(
                        "Non-methodref found in {} at CP index {}",
                        current_klass.get_name(),
                        cp_lookup
                    ),
                };
                let (dispatch_klass_name, local_name_desc) =
                    runtime::split_name_desc(fq_name_desc.to_string());

                dispatch_invoke(
                    repo.lookup_method_exact(&dispatch_klass_name, local_name_desc),
                    &eval,
                );
            }
            // FIXME DOES NOT ACTUALLY DO VIRTUAL LOOKUP YET
            Opcode::INVOKEVIRTUAL => {
                // let cp_lookup = ((instr[current] as u16) << 8) + instr[current + 1] as u16;
                // current += 2;
                // dispatch_invoke(repo.lookup_method_virtual(&klass_name, cp_lookup), &eval);
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
            Opcode::MONITOREXIT => {
                eval.pop();
            }

            Opcode::NEW => {
                let cp_lookup = ((instr[current] as u16) << 8) + instr[current + 1] as u16;
                current += 2;
                // FIXME Should lookup CP_CLASS like this: current_klass.lookup_cp(cp_lookup)
                // Then match ...
                let klass_name = match current_klass.lookup_cp(cp_lookup) {
                    runtime::cp_entry::class { idx } => "DUMMY_CLASS".to_string(), // FIXME
                    _ => panic!(
                        "Non-class found in {} at CP index {}",
                        current_klass.get_name(),
                        cp_lookup
                    ),
                };

                let klass: runtime::ot_klass = repo.lookup_klass(klass_name);
                eval.push(runtime::jvm_value::ObjRef {
                    val: heap.allocate_obj(klass),
                });
            }
            Opcode::NOP => {
                ();
            }

            Opcode::POP => {
                eval.pop();
            }
            Opcode::POP2 => {
                let _discard: runtime::jvm_value = eval.pop();
                // FIXME Change to type match
                // if (discard.type == JVMType.J || discard.type == JVMType.D) {

                // }
                eval.pop();
            }
            Opcode::PUTFIELD => {
                let cp_lookup = ((instr[current] as u16) << 8) + instr[current + 1] as u16;
                current += 2;

                let putf: runtime::ot_field = repo.lookup_field(my_klass_name.clone(), cp_lookup);
                let val: runtime::jvm_value = eval.pop();

                let recvp: runtime::jvm_value = eval.pop();
                // VERIFY: Should check to make sure receiver is an A
                // FIXME Match expression & destructure for recvp
                let obj = match recvp {
                    runtime::jvm_value::ObjRef { val: v } => v,
                    _ => panic!("Not an object ref at {}", (current - 1)),
                };

                obj.put_field(putf, val);
            }
            Opcode::PUTSTATIC => {
                let cp_lookup = ((instr[current] as u16) << 8) + instr[current + 1] as u16;
                current += 2;

                let puts = repo.lookup_field(my_klass_name.clone(), cp_lookup);
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
            Opcode::BREAKPOINT => break Some(runtime::jvm_value::Boolean { val: false }),
            Opcode::IMPDEP1 => break Some(runtime::jvm_value::Boolean { val: false }),
            Opcode::IMPDEP2 => break Some(runtime::jvm_value::Boolean { val: false }),
            Opcode::JSR => break Some(runtime::jvm_value::Boolean { val: false }),
            Opcode::JSR_W => break Some(runtime::jvm_value::Boolean { val: false }),
            Opcode::RET => break Some(runtime::jvm_value::Boolean { val: false }),

            _ => panic!(
                "Illegal opcode byte: {} encountered at position {}. Stopping.",
                ins,
                (current - 1)
            ),
        }
    }
}

fn dispatch_invoke(_to_be_called: runtime::ot_method, _eval: &runtime::interp_eval_stack) -> () {
    // Setup call

    // Invoke

    // Setup return value
    // let val : Option<runtime::jvm_value> = execMethod(toBeCalled, withVars);
    // FIXME convert to match expr
    // if (val != null)
    //     eval.push(val);

}

fn parse_class(bytes: Vec<u8>, fname: String) -> runtime::ot_klass {
    let mut parser = klass_parser::oc_parser::new(bytes, fname);
    parser.parse();
    parser.klass()
}

#[cfg(test)]
mod tests;
