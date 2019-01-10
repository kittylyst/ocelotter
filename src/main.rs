mod opcode;

// static lookup_bytecodes : [opcode::Opcode; 256] = make_bytecode_table();

fn main() {
    println!("Hello, world!");
    let op = opcode::Opcode::ALOAD;
}

fn exec_method(klass_name: String, desc: String, instr: Vec<u8>) -> Option<opcode::JVMValue> {
    let mut current = 0;
    let eval: EvaluationStack = EvaluationStack::new();

    loop {
        let ins = instr.get(current);
        current += 1;

        match ins {
            ACONST_NULL => eval.aconst_null(),

            ALOAD => {
                eval.push(lvt.aload(instr[current]));
                current += 1;
            }
            ALOAD_0 => eval.push(lvt.aload(0)),

            ALOAD_1 => eval.push(lvt.aload(1)),

            ARETURN => break Some(eval.pop()),
            ASTORE => {
                lvt.astore(instr[current], eval.pop());
                current += 1;
            }
            ASTORE_0 => lvt.astore(0, eval.pop()),

            ASTORE_1 => lvt.astore(1, eval.pop()),

            BIPUSH => {
                eval.iconst(instr[current]);
                current += 1;
            }
            DADD => eval.dadd(),

            DCONST_0 => eval.dconst(0.0),

            DCONST_1 => eval.dconst(1.0),

            DLOAD => {
                eval.push(lvt.dload(instr[current]));
                current += 1;
            }

            DLOAD_0 => eval.push(lvt.dload(0)),

            DLOAD_1 => eval.push(lvt.dload(1)),

            DLOAD_2 => eval.push(lvt.dload(2)),

            DLOAD_3 => eval.push(lvt.dload(3)),

            DRETURN => break Some(eval.pop()),
            DSTORE => {
                lvt.store(instr[current], eval.pop());
                current += 1;
            }
            DSTORE_0 => lvt.store(0, eval.pop()),

            DSTORE_1 => lvt.store(1, eval.pop()),

            DSTORE_2 => lvt.store(2, eval.pop()),

            DSTORE_3 => lvt.store(3, eval.pop()),

            DSUB => eval.dsub(),

            DUP => eval.dup(),

            DUP_X1 => eval.dupX1(),

            // GETFIELD => {
            //     cpLookup = ((int) instr[current++] << 8) + (int) instr[current++];
            //     OCField field = repo.lookupField(klassName, (short) cpLookup);
            //     JVMValue receiver = eval.pop();
            //     // VERIFY: Should check to make sure receiver is an A
            //     JVMObj obj = heap.findObject(receiver.value);
            //     eval.push(obj.getField(field));
            // },
            // GETSTATIC => {
            //     cpLookup = ((int) instr[current++] << 8) + (int) instr[current++];
            //     OCField f = repo.lookupField(klassName, (short) cpLookup);
            //     OCKlass fgKlass = f.getKlass();
            //     eval.push(fgKlass.getStaticField(f));
            // },
            GOTO => current += 2 + (instr[current] << 8) + instr[current + 1],

            I2D => eval.i2d(),

            IADD => eval.iadd(),

            IAND => eval.iand(),

            ICONST_0 => eval.iconst(0),

            ICONST_1 => eval.iconst(1),

            ICONST_2 => eval.iconst(2),

            ICONST_3 => eval.iconst(3),

            ICONST_4 => eval.iconst(4),

            ICONST_5 => eval.iconst(5),

            ICONST_M1 => eval.iconst(-1),

            IDIV => eval.idiv(),

            // IF_ICMPEQ => {
            //     v = eval.pop();
            //     v2 = eval.pop();
            //     jumpTo = ((int) instr[current++] << 8) + (int) instr[current++];
            //     if (v.value == v2.value) {
            //         current += jumpTo - 1; // The -1 is necessary as we've already inc'd current
            //     }
            // },
            // IFEQ => {
            //     v = eval.pop();
            //     jumpTo = ((int) instr[current++] << 8) + (int) instr[current++];
            //     if (v.value == 0L) {
            //         current += jumpTo - 1; // The -1 is necessary as we've already inc'd current
            //     }
            // }    ,
            // IFGE => {
            //     v = eval.pop();
            //     jumpTo = ((int) instr[current++] << 8) + (int) instr[current++];
            //     if (v.value >= 0L) {
            //         current += jumpTo - 1; // The -1 is necessary as we've already inc'd current
            //     }
            // } ,
            // IFGT => {
            //     v = eval.pop();
            //     jumpTo = ((int) instr[current++] << 8) + (int) instr[current++];
            //     if (v.value > 0L) {
            //         current += jumpTo - 1; // The -1 is necessary as we've already inc'd current
            //     }
            // },
            // IFLE => {
            //     v = eval.pop();
            //     jumpTo = ((int) instr[current++] << 8) + (int) instr[current++];
            //     if (v.value <= 0L) {
            //         current += jumpTo - 1; // The -1 is necessary as we've already inc'd current
            //     }
            // },
            // IFLT => {
            //     v = eval.pop();
            //     jumpTo = ((int) instr[current++] << 8) + (int) instr[current++];
            //     if (v.value < 0L) {
            //         current += jumpTo - 1; // The -1 is necessary as we've already inc'd current
            //     }
            // },
            // IFNE => {
            //     v = eval.pop();
            //     jumpTo = ((int) instr[current] << 8) + (int) instr[current + 1];
            //     if (v.value != 0L) {
            //         current += jumpTo - 1;  // The -1 is necessary as we've already inc'd current
            //     }
            // },
            // IFNONNULL => {
            //     v = eval.pop();
            //     jumpTo = ((int) instr[current] << 8) + (int) instr[current + 1];
            //     // FIXME Check that this is of reference type
            //     if (v.value != 0L) {
            //         current += jumpTo - 1;  // The -1 is necessary as we've already inc'd current
            //     }
            // },
            // IFNULL => {
            //     v = eval.pop();
            //     jumpTo = ((int) instr[current] << 8) + (int) instr[current + 1];
            //     // FIXME Check that this is of reference type
            //     if (v.value == 0L) {
            //         current += jumpTo - 1;  // The -1 is necessary as we've already inc'd current
            //     }
            // },
            IINC => {
                lvt.iinc(instr[current], instr[current + 1]);
                current += 2;
            }

            ILOAD => {
                eval.push(lvt.iload(instr[current]));
                current += 1
            }

            ILOAD_0 => eval.push(lvt.iload(0)),

            ILOAD_1 => eval.push(lvt.iload(1)),

            ILOAD_2 => eval.push(lvt.iload(2)),

            ILOAD_3 => eval.push(lvt.iload(3)),

            IMUL => eval.imul(),

            INEG => eval.ineg(),

            INVOKESPECIAL => {
                cpLookup = (instr[current] << 8) + instr[current + 1];
                current += 2;
                dispatchInvoke(repo.lookupMethodExact(currentKlass, cpLookup), eval);
            }
            INVOKESTATIC => {
                cpLookup = (instr[current] << 8) + instr[current + 1];
                current += 2;
                dispatchInvoke(repo.lookupMethodExact(currentKlass, cpLookup), eval);
            }
            // FIXME DOES NOT ACTUALLY DO VIRTUAL LOOKUP YET
            INVOKEVIRTUAL => {
                cpLookup = (instr[current] << 8) + instr[current + 1];
                current += 2;
                dispatchInvoke(repo.lookupMethodVirtual(currentKlass, cpLookup), eval);
            }
            IOR => eval.ior(),

            IREM => eval.irem(),

            IRETURN => break Some(eval.pop()),
            ISTORE => {
                lvt.store(instr[current], eval.pop());
                current += 1;
            }
            ISTORE_0 => lvt.store(0, eval.pop()),

            ISTORE_1 => lvt.store(1, eval.pop()),

            ISTORE_2 => lvt.store(2, eval.pop()),

            ISTORE_3 => lvt.store(3, eval.pop()),

            ISUB => eval.isub(),
            // FIXME TEMP
            MONITORENTER | MONITOREXIT => eval.pop(),

            NEW => {
                cpLookup = (instr[current] << 8) + instr[current + 1];
                current += 2;

                let klass: OCKlass = repo.lookupKlass(currentKlass, cpLookup);
                eval.push(entryRef(heap.allocateObj(klass)));
            }
            NOP => 1,

            POP => eval.pop(),

            POP2 => {
                let discard: Opcode::JVMValue = eval.pop();
                // FIXME Change to type match
                // if (discard.type == JVMType.J || discard.type == JVMType.D) {

                // }
                eval.pop();
            }
            PUTFIELD => {
                cpLookup = (instr[current] << 8) + instr[current + 1];
                current += 2;

                let putf: OCField = repo.lookupField(klassName, cpLookup);
                let val: Opcode::JVMValue = eval.pop();

                let recvp: Opcode::JVMValue = eval.pop();
                // VERIFY: Should check to make sure receiver is an A
                let objp: JVMObj = heap.findObject(recvp.value);
                objp.putField(putf, val);
            }
            PUTSTATIC => {
                cpLookup = (instr[current] << 8) + instr[current + 1];
                current += 2;

                let puts: OCField = repo.lookupField(klassName, cpLookup);
                let fKlass: OCKlass = puts.getKlass();
                let vals: Opcode::JVMValue = eval.pop();
                fKlass.setStaticField(puts.getName(), vals);
            }
            RETURN => break None,
            SIPUSH => {
                eval.iconst((instr[current] << 8) + instr[current + 1]);
                current += 2;
            }
            SWAP => {
                let val1: Opcode::JVMValue = eval.pop();
                let val2: Opcode::JVMValue = eval.pop();
                eval.push(val1);
                eval.push(val2);
            }
            // Dummy implementation
            LDC => {
                // System.out.print("Executing " + op + " with param bytes: ");
                // for (int i = current; i < current + num; i++) {
                //     System.out.print(instr[i] + " ");
                // }
                // current += num;
                // System.out.println();
            }
            // Disallowed opcodes
            BREAKPOINT | IMPDEP1 | IMPDEP2 | JSR | JSR_W | RET => {
                break Some(opcode::JVMValue::Boolean { val: true })
            }
            // throw new IllegalArgumentException("Illegal opcode byte: " + (b & 0xff) + " encountered at position " + (current - 1) + ". Stopping."),
            _ => break Some(opcode::JVMValue::Boolean { val: true }),
        }
    }
}

// fn make_bytecode_table() -> [opcode::Opcode; 256] {
//     [opcode::Opcode::ALOAD; 256]
// }
