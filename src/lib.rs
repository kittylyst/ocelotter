mod klass_parser;
mod opcode;
mod runtime;

static heap: runtime::SimpleLinkedJVMHeap = runtime::SimpleLinkedJVMHeap {};
static repo: runtime::ClassRepository = runtime::ClassRepository {};

pub fn exec_method(
    klass_name: String,
    desc: String,
    instr: &Vec<u8>,
    lvt: &runtime::LocalVariableTable,
) -> Option<runtime::JVMValue> {
    let mut current = 0;
    let mut eval = runtime::EvaluationStack::new();

    loop {
        let opt_ins = instr.get(current);
        let ins: u8 = match opt_ins {
            Some(value) => *value,
            None => panic!("Byte {} has no value", current),
        };
        current += 1;

        match ins {
            opcode::Opcode::ACONST_NULL => eval.aconst_null(),

            opcode::Opcode::ALOAD => {
                eval.push(lvt.aload(instr[current]));
                current += 1;
            }
            opcode::Opcode::ALOAD_0 => eval.push(lvt.aload(0)),

            opcode::Opcode::ALOAD_1 => eval.push(lvt.aload(1)),

            opcode::Opcode::ARETURN => break Some(eval.pop()),
            opcode::Opcode::ASTORE => {
                lvt.astore(instr[current], eval.pop());
                current += 1;
            }
            opcode::Opcode::ASTORE_0 => lvt.astore(0, eval.pop()),

            opcode::Opcode::ASTORE_1 => lvt.astore(1, eval.pop()),

            opcode::Opcode::BIPUSH => {
                eval.iconst(instr[current] as i32);
                current += 1;
            }
            opcode::Opcode::DADD => eval.dadd(),

            opcode::Opcode::DCONST_0 => eval.dconst(0.0),

            opcode::Opcode::DCONST_1 => eval.dconst(1.0),

            opcode::Opcode::DLOAD => {
                eval.push(lvt.dload(instr[current]));
                current += 1;
            }

            opcode::Opcode::DLOAD_0 => eval.push(lvt.dload(0)),

            opcode::Opcode::DLOAD_1 => eval.push(lvt.dload(1)),

            opcode::Opcode::DLOAD_2 => eval.push(lvt.dload(2)),

            opcode::Opcode::DLOAD_3 => eval.push(lvt.dload(3)),

            opcode::Opcode::DRETURN => break Some(eval.pop()),
            opcode::Opcode::DSTORE => {
                lvt.store(instr[current], eval.pop());
                current += 1;
            }
            opcode::Opcode::DSTORE_0 => lvt.store(0, eval.pop()),

            opcode::Opcode::DSTORE_1 => lvt.store(1, eval.pop()),

            opcode::Opcode::DSTORE_2 => lvt.store(2, eval.pop()),

            opcode::Opcode::DSTORE_3 => lvt.store(3, eval.pop()),

            opcode::Opcode::DSUB => eval.dsub(),

            opcode::Opcode::DUP => eval.dup(),

            opcode::Opcode::DUP_X1 => eval.dupX1(),

            // GETFIELD => {
            //     let cp_lookup = ((int) instr[current++] << 8) + (int) instr[current++];
            //     runtime::OCField field = repo.lookupField(klass_name, (short) cp_lookup);
            //     runtime::JVMValue receiver = eval.pop();
            //     // VERIFY: Should check to make sure receiver is an opcode::Opcode::A
            //     runtime::JVMObj obj = heap.findObject(receiver.value);
            //     eval.push(obj.getField(field));
            // },
            // GETSTATIC => {
            //     let cp_lookup = ((int) instr[current++] << 8) + (int) instr[current++];
            //     runtime::OCField f = repo.lookupField(klass_name, (short) cp_lookup);
            //     runtime::OCKlass fgKlass = f.getKlass();
            //     eval.push(fgKlass.getStaticField(f));
            // },
            opcode::Opcode::GOTO => {
                current += ((instr[current] as usize) << 8) + instr[current + 1] as usize
            }

            opcode::Opcode::I2D => eval.i2d(),

            opcode::Opcode::IADD => eval.iadd(),

            opcode::Opcode::IAND => eval.iand(),

            opcode::Opcode::ICONST_0 => eval.iconst(0),

            opcode::Opcode::ICONST_1 => eval.iconst(1),

            opcode::Opcode::ICONST_2 => eval.iconst(2),

            opcode::Opcode::ICONST_3 => eval.iconst(3),

            opcode::Opcode::ICONST_4 => eval.iconst(4),

            opcode::Opcode::ICONST_5 => eval.iconst(5),

            opcode::Opcode::ICONST_M1 => eval.iconst(-1),

            opcode::Opcode::IDIV => eval.idiv(),

            // opcode::Opcode::IF_ICMPEQ => {
            //     let jumpTo = (instr[current] as usize) << 8 + instr[current + 1] as usize;
            //     let v1 = match eval.pop() {
            //         runtime::JVMValue::ObjRef { val: v } => v,
            //         _ => println!("Value not of reference type found for IFNULL"),
            //     };
            //     let v2 = match eval.pop() {
            //         runtime::JVMValue::ObjRef { val: v } => v,
            //         _ => println!("Value not of reference type found for IFNULL"),
            //     };
            //     if v1 == v2 {
            //         current += jumpTo - 1; // The -1 is necessary as we've already inc'd current
            //     }
            // },
            // opcode::Opcode::IFEQ => {
            //     v = eval.pop();
            //     jumpTo = ((int) instr[current++] << 8) + (int) instr[current++];
            //     if (v.value == 0L) {
            //         current += jumpTo - 1; // The -1 is necessary as we've already inc'd current
            //     }
            // }    ,
            // opcode::Opcode::IFGE => {
            //     v = eval.pop();
            //     jumpTo = ((int) instr[current++] << 8) + (int) instr[current++];
            //     if (v.value >= 0L) {
            //         current += jumpTo - 1; // The -1 is necessary as we've already inc'd current
            //     }
            // } ,
            // opcode::Opcode::IFGT => {
            //     v = eval.pop();
            //     jumpTo = ((int) instr[current++] << 8) + (int) instr[current++];
            //     if (v.value > 0L) {
            //         current += jumpTo - 1; // The -1 is necessary as we've already inc'd current
            //     }
            // },
            // opcode::Opcode::IFLE => {
            //     v = eval.pop();
            //     jumpTo = ((int) instr[current++] << 8) + (int) instr[current++];
            //     if (v.value <= 0L) {
            //         current += jumpTo - 1; // The -1 is necessary as we've already inc'd current
            //     }
            // },
            // opcode::Opcode::IFLT => {
            //     v = eval.pop();
            //     jumpTo = ((int) instr[current++] << 8) + (int) instr[current++];
            //     if (v.value < 0L) {
            //         current += jumpTo - 1; // The -1 is necessary as we've already inc'd current
            //     }
            // },
            // opcode::Opcode::IFNE => {
            //     v = eval.pop();
            //     jumpTo = ((int) instr[current] << 8) + (int) instr[current + 1];
            //     if (v.value != 0L) {
            //         current += jumpTo - 1;  // The -1 is necessary as we've already inc'd current
            //     }
            // },
            opcode::Opcode::IFNONNULL => {
                let jumpTo = ((instr[current] as usize) << 8) + instr[current + 1] as usize;

                match eval.pop() {
                    runtime::JVMValue::ObjRef { val: v } => {
                        if !v.is_null() {
                            current += jumpTo;
                        } else {
                            current += 2;
                        }
                    }
                    _ => panic!(
                        "Value not of reference type found for IFNULL at {}",
                        current
                    ),
                };
            }
            opcode::Opcode::IFNULL => {
                let jumpTo = ((instr[current] as usize) << 8) + instr[current + 1] as usize;

                match eval.pop() {
                    runtime::JVMValue::ObjRef { val: v } => {
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
                        current
                    ),
                };
            }
            opcode::Opcode::IINC => {
                lvt.iinc(instr[current], instr[current + 1]);
                current += 2;
            }

            opcode::Opcode::ILOAD => {
                eval.push(lvt.iload(instr[current]));
                current += 1
            }

            opcode::Opcode::ILOAD_0 => eval.push(lvt.iload(0)),

            opcode::Opcode::ILOAD_1 => eval.push(lvt.iload(1)),

            opcode::Opcode::ILOAD_2 => eval.push(lvt.iload(2)),

            opcode::Opcode::ILOAD_3 => eval.push(lvt.iload(3)),

            opcode::Opcode::IMUL => eval.imul(),

            opcode::Opcode::INEG => eval.ineg(),

            opcode::Opcode::INVOKESPECIAL => {
                let cp_lookup = ((instr[current] as u16) << 8) + instr[current + 1] as u16;
                current += 2;
                dispatch_invoke(repo.lookupMethodExact(&klass_name, cp_lookup), &eval);
            }
            opcode::Opcode::INVOKESTATIC => {
                let cp_lookup = ((instr[current] as u16) << 8) + instr[current + 1] as u16;
                current += 2;
                dispatch_invoke(repo.lookupMethodExact(&klass_name, cp_lookup), &eval);
            }
            // FIXME DOES NOT ACTUALLY DO VIRTUAL LOOKUP YET
            opcode::Opcode::INVOKEVIRTUAL => {
                let cp_lookup = ((instr[current] as u16) << 8) + instr[current + 1] as u16;
                current += 2;
                dispatch_invoke(repo.lookupMethodVirtual(&klass_name, cp_lookup), &eval);
            }
            opcode::Opcode::IOR => eval.ior(),

            opcode::Opcode::IREM => eval.irem(),

            opcode::Opcode::IRETURN => break Some(eval.pop()),
            opcode::Opcode::ISTORE => {
                lvt.store(instr[current], eval.pop());
                current += 1;
            }
            opcode::Opcode::ISTORE_0 => lvt.store(0, eval.pop()),

            opcode::Opcode::ISTORE_1 => lvt.store(1, eval.pop()),

            opcode::Opcode::ISTORE_2 => lvt.store(2, eval.pop()),

            opcode::Opcode::ISTORE_3 => lvt.store(3, eval.pop()),

            opcode::Opcode::ISUB => eval.isub(),
            // Dummy implementation
            opcode::Opcode::LDC => {
                // System.out.print("Executing " + op + " with param bytes: ");
                // for (int i = current; i < current + num; i++) {
                //     System.out.print(instr[i] + " ");
                // }
                // current += num;
                // System.out.println();
            }

            // FIXME TEMP
            opcode::Opcode::MONITORENTER => {
                eval.pop();
            }
            opcode::Opcode::MONITOREXIT => {
                eval.pop();
            }

            opcode::Opcode::NEW => {
                let cp_lookup = ((instr[current] as u16) << 8) + instr[current + 1] as u16;
                current += 2;

                let klass: runtime::OCKlass = repo.lookupKlass(&klass_name, cp_lookup);
                eval.push(runtime::JVMValue::ObjRef {
                    val: heap.allocateObj(klass),
                });
            }
            opcode::Opcode::NOP => {
                ();
            }

            opcode::Opcode::POP => {
                eval.pop();
            }
            opcode::Opcode::POP2 => {
                let _discard: runtime::JVMValue = eval.pop();
                // FIXME Change to type match
                // if (discard.type == JVMType.J || discard.type == JVMType.D) {

                // }
                eval.pop();
            }
            opcode::Opcode::PUTFIELD => {
                let cp_lookup = ((instr[current] as u16) << 8) + instr[current + 1] as u16;
                current += 2;

                let putf: runtime::OCField = repo.lookupField(&klass_name, cp_lookup);
                let val: runtime::JVMValue = eval.pop();

                let recvp: runtime::JVMValue = eval.pop();
                // VERIFY: Should check to make sure receiver is an A
                // FIXME Match expression & destructure for recvp
                let obj = match recvp {
                    runtime::JVMValue::ObjRef { val: v } => v,
                    _ => panic!("Not an object ref at {}", current),
                };

                obj.putField(putf, val);
            }
            opcode::Opcode::PUTSTATIC => {
                let cp_lookup = ((instr[current] as u16) << 8) + instr[current + 1] as u16;
                current += 2;

                let puts: runtime::OCField = repo.lookupField(&klass_name, cp_lookup);
                let fKlass: runtime::OCKlass = puts.getKlass();
                let vals: runtime::JVMValue = eval.pop();
                fKlass.setStaticField(puts.getName(), vals);
            }
            opcode::Opcode::RETURN => break None,
            opcode::Opcode::SIPUSH => {
                let vtmp = ((instr[current] as i32) << 8) + instr[current + 1] as i32;
                eval.iconst(vtmp);
                current += 2;
            }
            opcode::Opcode::SWAP => {
                let val1: runtime::JVMValue = eval.pop();
                let val2: runtime::JVMValue = eval.pop();
                eval.push(val1);
                eval.push(val2);
            }
            // Disallowed opcodes
            opcode::Opcode::BREAKPOINT => break Some(runtime::JVMValue::Boolean { val: false }),
            opcode::Opcode::IMPDEP1 => break Some(runtime::JVMValue::Boolean { val: false }),
            opcode::Opcode::IMPDEP2 => break Some(runtime::JVMValue::Boolean { val: false }),
            opcode::Opcode::JSR => break Some(runtime::JVMValue::Boolean { val: false }),
            opcode::Opcode::JSR_W => break Some(runtime::JVMValue::Boolean { val: false }),
            opcode::Opcode::RET => break Some(runtime::JVMValue::Boolean { val: false }),

            _ => panic!(
                "Illegal opcode byte: {} encountered at position {}. Stopping.",
                ins,
                (current - 1)
            ),
        }
    }
}

fn dispatch_invoke(to_be_called: runtime::OCMethod, eval: &runtime::EvaluationStack) -> () {
    // Setup call

    // Invoke

    // Setup return value
    // let val : Option<runtime::JVMValue> = execMethod(toBeCalled, withVars);
    // FIXME convert to match expr
    // if (val != null)
    //     eval.push(val);

}

fn parse_class(bytes: Vec<u8>, fname: String) -> runtime::OCKlass {
    let mut parser = klass_parser::oc_parser::new(bytes, fname);
    parser.parse();
    // parser.klass();
    runtime::OCKlass {}
}

#[cfg(test)]
mod tests;
