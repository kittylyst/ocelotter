#![deny(unreachable_patterns)]
#![allow(dead_code)]
#![allow(unused_variables)]

use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};

use crate::interpreter::interp_stack::InterpEvalStack;
use crate::interpreter::object::OtObj;
use crate::interpreter::opcode;
use crate::interpreter::values::*;
use crate::klass::constant_pool::*;
use crate::klass::otklass::OtKlass;
use crate::klass::otmethod::OtMethod;
use crate::OtKlassComms;

pub fn start_new_jthread(f_name: String, tx: Sender<OtKlassComms>) {
    let thread_tx = tx.clone();

    // FIXME Real main() signature required, dummying for ease of testing
    let main_str: String = f_name.clone() + ".main2:([Ljava/lang/String;)I";

    let (tx_main, rx_main): (Sender<OtKlass>, Receiver<OtKlass>) = mpsc::channel();
    // Send the main klass name and receive back the klass
    thread_tx.send(OtKlassComms {
        kname: f_name.clone(),
        reply_via: tx_main,
    });
    let main_klass = rx_main.recv().unwrap();

    let main = main_klass
        .get_method_by_name_and_desc(&main_str)
        .unwrap_or_else(|| panic!("Error: Main method not found {}", main_str.clone()));

    // FIXME Parameter passing
    let mut vars = InterpLocalVars::of(5);

    let return_val = exec_method(tx, main, &mut vars);
    let ret = match return_val {
        Some(JvmValue::Int(i)) => i,
        _ => panic!(
            "Error executing {} - non-int value returned",
            f_name.clone()
        ),
    };

    println!("Ret: {}", ret);
}

// Transmits a class name, receives a klass
pub fn exec_method(
    tx: Sender<OtKlassComms>,
    meth: &OtMethod,
    lvt: &mut InterpLocalVars,
) -> Option<JvmValue> {
    let thread_tx = tx.clone();

    if meth.is_native() {
        // Explicit type hint here to document the type of n_f
        let n_f: fn(&InterpLocalVars) -> Option<JvmValue> = meth
            .get_native_code()
            .unwrap_or_else(|| panic!("Native code not found {}", meth.get_fq_name_desc()));

        // FIXME Parameter passing
        n_f(lvt)
    } else {
        exec_bytecode_method(thread_tx, meth.get_klass_name(), &meth.get_code(), lvt)
    }
}

pub fn exec_bytecode_method(
    tx: Sender<OtKlassComms>,
    klass_name: String,
    instr: &[u8],
    lvt: &mut InterpLocalVars,
) -> Option<JvmValue> {
    let thread_tx = tx.clone();

    let mut current = 0;
    let mut eval = InterpEvalStack::of();

    // println!("Getting to interpreter loop");
    loop {
        // let my_klass_name = klass_name.clone();
        let ins: u8 = *instr
            .get(current)
            .unwrap_or_else(|| panic!("Byte {} has no value", current));

        current += 1;

        // dbg!(ins);
        match ins {
            opcode::ACONST_NULL => eval.aconst_null(),

            opcode::ALOAD => {
                eval.push(lvt.load(instr[current]));
                current += 1;
            }
            opcode::ALOAD_0 => eval.push(lvt.load(0)),

            opcode::ALOAD_1 => eval.push(lvt.load(1)),

            opcode::ALOAD_2 => eval.push(lvt.load(2)),

            opcode::ALOAD_3 => eval.push(lvt.load(3)),

            opcode::ARETURN => break Some(eval.pop()),
            opcode::ASTORE => {
                lvt.store(instr[current], eval.pop());
                current += 1;
            }
            opcode::ASTORE_0 => lvt.store(0, eval.pop()),

            opcode::ASTORE_1 => lvt.store(1, eval.pop()),

            opcode::ASTORE_2 => lvt.store(2, eval.pop()),

            opcode::ASTORE_3 => lvt.store(3, eval.pop()),

            opcode::BIPUSH => {
                eval.iconst(instr[current] as i32);
                current += 1;
            }

            opcode::D2F => {
                match eval.pop() {
                    JvmValue::Double(v) => eval.push(JvmValue::Float(v as f32)),
                    _ => panic!("Value not of long type found for D2F at {}", (current - 1)),
                };
            }

            opcode::D2I => {
                match eval.pop() {
                    JvmValue::Double(v) => eval.push(JvmValue::Int(v as i32)),
                    _ => panic!("Value not of long type found for D2I at {}", (current - 1)),
                };
            }

            opcode::D2L => {
                match eval.pop() {
                    JvmValue::Double(v) => eval.push(JvmValue::Long(v as i64)),
                    _ => panic!("Value not of long type found for D2L at {}", (current - 1)),
                };
            }

            opcode::DADD => eval.dadd(),

            opcode::DCMPG => eval.dcmpg(),

            opcode::DCMPL => eval.dcmpl(),

            opcode::DCONST_0 => eval.dconst(0.0),

            opcode::DCONST_1 => eval.dconst(1.0),

            opcode::DDIV => eval.ddiv(),

            opcode::DLOAD => {
                eval.push(lvt.load(instr[current]));
                current += 1;
            }

            opcode::DLOAD_0 => eval.push(lvt.load(0)),

            opcode::DLOAD_1 => eval.push(lvt.load(1)),

            opcode::DLOAD_2 => eval.push(lvt.load(2)),

            opcode::DLOAD_3 => eval.push(lvt.load(3)),

            opcode::DMUL => eval.dmul(),

            opcode::DNEG => eval.dneg(),

            opcode::DREM => eval.drem(),

            opcode::DRETURN => break Some(eval.pop()),

            opcode::DSTORE => {
                lvt.store(instr[current], eval.pop());
                current += 1;
            }

            opcode::DSTORE_0 => lvt.store(0, eval.pop()),

            opcode::DSTORE_1 => lvt.store(1, eval.pop()),

            opcode::DSTORE_2 => lvt.store(2, eval.pop()),

            opcode::DSTORE_3 => lvt.store(3, eval.pop()),

            opcode::DSUB => eval.dsub(),

            opcode::DUP => eval.dup(),

            opcode::DUP_X1 => eval.dup_x1(),

            //            opcode::DUP2 => eval.dup2(),
            opcode::F2D => eval.f2d(),

            opcode::F2I => eval.f2i(),

            opcode::F2L => eval.f2l(),

            opcode::FADD => eval.fadd(),

            opcode::FCMPG => eval.fcmpg(),

            opcode::FCMPL => eval.fcmpl(),

            opcode::FCONST_0 => eval.fconst(0.0),

            opcode::FCONST_1 => eval.fconst(1.0),

            opcode::FCONST_2 => eval.fconst(2.0),

            opcode::FDIV => eval.fdiv(),

            opcode::FLOAD => {
                eval.push(lvt.load(instr[current]));
                current += 1;
            }

            opcode::FLOAD_0 => eval.push(lvt.load(0)),

            opcode::FLOAD_1 => eval.push(lvt.load(1)),

            opcode::FLOAD_2 => eval.push(lvt.load(2)),

            opcode::FLOAD_3 => eval.push(lvt.load(3)),

            opcode::FMUL => eval.fmul(),

            opcode::FNEG => eval.fneg(),

            opcode::FREM => eval.frem(),

            opcode::FRETURN => break Some(eval.pop()),

            opcode::FSTORE => {
                lvt.store(instr[current], eval.pop());
                current += 1;
            }

            opcode::FSTORE_0 => lvt.store(0, eval.pop()),

            opcode::FSTORE_1 => lvt.store(1, eval.pop()),

            opcode::FSTORE_2 => lvt.store(2, eval.pop()),

            opcode::FSTORE_3 => lvt.store(3, eval.pop()),

            opcode::FSUB => eval.fsub(),

            opcode::GETFIELD => {
                let cp_lookup = ((instr[current] as u16) << 8) + instr[current + 1] as u16;
                current += 2;

                let recvp: JvmValue = eval.pop();
                let obj_id = match recvp {
                    JvmValue::ObjRef(v) => v,
                    _ => panic!("Not an object ref at {}", (current - 1)),
                };
                let heap = HEAP.lock().unwrap();
                let obj = heap.get_obj(obj_id);
                let getf =
                    OtKlass::lookup_instance_field(thread_tx.clone(), &klass_name, cp_lookup);

                let ret = obj.get_field_value(getf.get_offset() as usize);
                eval.push(ret);
            }
            opcode::GETSTATIC => {
                let cp_lookup = ((instr[current] as u16) << 8) + instr[current + 1] as u16;
                current += 2;

                let getf =
                    OtKlass::lookup_static_field(thread_tx.clone(), &klass_name, cp_lookup).clone();
                let klass =
                    OtKlass::lookup_klass(thread_tx.clone(), &getf.get_klass_name()).clone();

                let ret = klass.get_static(&getf);
                eval.push(ret);
            }
            opcode::GOTO => {
                current += ((instr[current] as usize) << 8) + instr[current + 1] as usize
            }
            opcode::GOTO_W => {
                current += ((instr[current] as usize) << 24)
                    + ((instr[current + 1] as usize) << 16)
                    + ((instr[current + 2] as usize) << 8)
                    + instr[current + 3] as usize
            }

            opcode::I2B => eval.i2b(),

            opcode::I2C => eval.i2c(),

            opcode::I2D => eval.i2d(),

            opcode::I2F => eval.i2f(),

            opcode::I2L => eval.i2l(),

            opcode::I2S => eval.i2s(),

            opcode::IADD => eval.iadd(),

            opcode::IALOAD => {
                let pos_to_load = match eval.pop() {
                    JvmValue::Int(v) => v,
                    _ => panic!("Non-int seen on stack during IASTORE at {}", current - 1),
                };
                let arrayid = match eval.pop() {
                    JvmValue::ObjRef(v) => v,
                    _ => panic!("Non-objref seen on stack during IASTORE at {}", current - 1),
                };
                dbg!(arrayid);

                let unwrapped_val = match HEAP.lock().unwrap().get_obj(arrayid) {
                    OtObj::VmArrInt {
                        id: _,
                        mark: _,
                        klassid: _,
                        length: _,
                        elements: elts,
                    } => elts[pos_to_load as usize],
                    _ => panic!("Non-int[] seen on stack during IASTORE at {}", current - 1),
                };
                eval.push(JvmValue::Int(unwrapped_val));
            }

            opcode::IAND => eval.iand(),

            opcode::IASTORE => {
                let val_to_store = match eval.pop() {
                    JvmValue::Int(v) => v,
                    _ => panic!("Non-int seen on stack during IASTORE at {}", current - 1),
                };
                let pos_to_store = match eval.pop() {
                    JvmValue::Int(v) => v,
                    _ => panic!("Non-int seen on stack during IASTORE at {}", current - 1),
                };
                let obj_id = match eval.pop() {
                    JvmValue::ObjRef(v) => v,
                    _ => panic!("Non-objref seen on stack during IASTORE at {}", current - 1),
                };

                HEAP.lock()
                    .unwrap()
                    .iastore(obj_id, pos_to_store, val_to_store);
            }

            opcode::ICONST_0 => eval.iconst(0),

            opcode::ICONST_1 => eval.iconst(1),

            opcode::ICONST_2 => eval.iconst(2),

            opcode::ICONST_3 => eval.iconst(3),

            opcode::ICONST_4 => eval.iconst(4),

            opcode::ICONST_5 => eval.iconst(5),

            opcode::ICONST_M1 => eval.iconst(-1),

            opcode::IDIV => eval.idiv(),

            opcode::IF_ICMPEQ => {
                let jump_to = ((instr[current] as usize) << 8) + instr[current + 1] as usize;
                if massage_to_int_and_compare(eval.pop(), eval.pop(), |i: i32, j: i32| -> bool {
                    i == j
                }) {
                    current += jump_to - 1;
                } else {
                    current += 2;
                }
            }

            opcode::IF_ICMPGE => {
                let jump_to = ((instr[current] as usize) << 8) + instr[current + 1] as usize;
                if massage_to_int_and_compare(eval.pop(), eval.pop(), |i: i32, j: i32| -> bool {
                    i >= j
                }) {
                    current += jump_to - 1;
                } else {
                    current += 2;
                }
            }

            opcode::IF_ICMPGT => {
                let jump_to = ((instr[current] as usize) << 8) + instr[current + 1] as usize;
                if massage_to_int_and_compare(eval.pop(), eval.pop(), |i: i32, j: i32| -> bool {
                    i > j
                }) {
                    current += jump_to - 1;
                } else {
                    current += 2;
                }
            }

            opcode::IF_ICMPLE => {
                let jump_to = ((instr[current] as usize) << 8) + instr[current + 1] as usize;
                if massage_to_int_and_compare(eval.pop(), eval.pop(), |i: i32, j: i32| -> bool {
                    i <= j
                }) {
                    current += jump_to - 1;
                } else {
                    current += 2;
                }
            }

            opcode::IF_ICMPLT => {
                let jump_to = ((instr[current] as usize) << 8) + instr[current + 1] as usize;
                if massage_to_int_and_compare(eval.pop(), eval.pop(), |i: i32, j: i32| -> bool {
                    i < j
                }) {
                    current += jump_to - 1;
                } else {
                    current += 2;
                }
            }

            opcode::IF_ICMPNE => {
                let jump_to = ((instr[current] as usize) << 8) + instr[current + 1] as usize;
                if massage_to_int_and_compare(eval.pop(), eval.pop(), |i: i32, j: i32| -> bool {
                    i == j
                }) {
                    current += 2;
                } else {
                    current += jump_to - 1;
                }
            }
            opcode::IFEQ => {
                let jump_to = ((instr[current] as usize) << 8) + instr[current + 1] as usize;
                let i = match eval.pop() {
                    JvmValue::Int(v) => v,
                    _ => panic!("Non-int seen on stack during IFEQ at {}", current - 1),
                };
                if i == 0 {
                    current += jump_to - 1;
                } else {
                    current += 2;
                }
            }
            opcode::IFGE => {
                let jump_to = ((instr[current] as usize) << 8) + instr[current + 1] as usize;
                let v = match eval.pop() {
                    JvmValue::Int(i) => i,
                    _ => panic!("Non-int seen on stack during IFGE at {}", current - 1),
                };
                //                dbg!(v);
                //                dbg!(current, jump_to);
                if v >= 0 {
                    current += jump_to - 1;
                } else {
                    current += 2;
                }
            }
            opcode::IFGT => {
                let jump_to = ((instr[current] as usize) << 8) + instr[current + 1] as usize;
                let v = match eval.pop() {
                    JvmValue::Int(v) => v,
                    _ => panic!("Non-int seen on stack during IFGT at {}", current - 1),
                };
                if v > 0 {
                    current += jump_to - 1;
                } else {
                    current += 2;
                }
            }
            opcode::IFLE => {
                let jump_to = ((instr[current] as usize) << 8) + instr[current + 1] as usize;
                let v = match eval.pop() {
                    JvmValue::Int(i) => i,
                    _ => panic!("Non-int seen on stack during IFLE at {}", current - 1),
                };
                //                dbg!(v);
                //                dbg!(current, jump_to);
                //                dbg!(instr[current], instr[current + 1]);
                if v <= 0 {
                    current += jump_to - 1;
                } else {
                    current += 2;
                }
            }
            opcode::IFLT => {
                let jump_to = ((instr[current] as usize) << 8) + instr[current + 1] as usize;
                let v = match eval.pop() {
                    JvmValue::Int(v) => v,
                    _ => panic!("Non-int seen on stack during IFGT at {}", current - 1),
                };
                if v < 0 {
                    current += jump_to - 1;
                } else {
                    current += 2;
                }
            }
            opcode::IFNE => {
                let jump_to = ((instr[current] as usize) << 8) + instr[current + 1] as usize;
                let i = match eval.pop() {
                    JvmValue::Int(v) => v,
                    _ => panic!("Non-int seen on stack during IFEQ at {}", current - 1),
                };
                if i == 0 {
                    current += jump_to - 1;
                } else {
                    current += 2;
                }
            }
            opcode::IFNONNULL => {
                let jump_to = ((instr[current] as usize) << 8) + instr[current + 1] as usize;

                match eval.pop() {
                    JvmValue::ObjRef(v) => {
                        if v > 0 {
                            current += jump_to - 1;
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
            opcode::IFNULL => {
                let jump_to = ((instr[current] as usize) << 8) + instr[current + 1] as usize;

                match eval.pop() {
                    JvmValue::ObjRef(v) => {
                        if v == 0 {
                            current += jump_to - 1;
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
            opcode::IINC => {
                lvt.iinc(instr[current], instr[current + 1]);
                current += 2;
            }

            opcode::ILOAD => {
                eval.push(lvt.load(instr[current]));
                current += 1
            }

            opcode::ILOAD_0 => eval.push(lvt.load(0)),

            opcode::ILOAD_1 => eval.push(lvt.load(1)),

            opcode::ILOAD_2 => eval.push(lvt.load(2)),

            opcode::ILOAD_3 => eval.push(lvt.load(3)),

            opcode::IMUL => eval.imul(),

            opcode::INEG => eval.ineg(),

            opcode::INVOKESPECIAL => {
                let cp_lookup = ((instr[current] as u16) << 8) + instr[current + 1] as u16;
                current += 2;
                let current_klass = OtKlass::lookup_klass(thread_tx.clone(), &klass_name).clone();
                dispatch_invoke(thread_tx.clone(), current_klass, cp_lookup, &mut eval, 1);
            }
            opcode::INVOKESTATIC => {
                let cp_lookup = ((instr[current] as u16) << 8) + instr[current + 1] as u16;
                current += 2;
                let current_klass = OtKlass::lookup_klass(thread_tx.clone(), &klass_name).clone();
                //                dbg!(cp_lookup);
                let arg_count = current_klass.get_method_arg_count(cp_lookup);
                dispatch_invoke(
                    thread_tx.clone(),
                    current_klass,
                    cp_lookup,
                    &mut eval,
                    arg_count,
                );
            }
            opcode::INVOKEVIRTUAL => {
                // FIXME DOES NOT ACTUALLY DO VIRTUAL LOOKUP YET
                let cp_lookup = ((instr[current] as u16) << 8) + instr[current + 1] as u16;
                current += 2;
                let current_klass = OtKlass::lookup_klass(thread_tx.clone(), &klass_name).clone();
                dbg!(current_klass.clone());
                dispatch_invoke(thread_tx.clone(), current_klass, cp_lookup, &mut eval, 1);
            }
            opcode::IOR => eval.ior(),

            opcode::IREM => eval.irem(),

            opcode::IRETURN => break Some(eval.pop()),

            opcode::ISHL => eval.ishl(),

            opcode::ISHR => eval.ishr(),

            opcode::ISTORE => {
                lvt.store(instr[current], eval.pop());
                current += 1;
            }
            opcode::ISTORE_0 => lvt.store(0, eval.pop()),

            opcode::ISTORE_1 => lvt.store(1, eval.pop()),

            opcode::ISTORE_2 => lvt.store(2, eval.pop()),

            opcode::ISTORE_3 => lvt.store(3, eval.pop()),

            opcode::ISUB => eval.isub(),

            opcode::IUSHR => eval.iushr(),

            opcode::IXOR => eval.ixor(),

            opcode::L2D => eval.l2d(),

            opcode::L2F => eval.l2f(),

            opcode::L2I => eval.l2i(),

            opcode::LADD => eval.ladd(),

            opcode::LAND => eval.land(),

            opcode::LCMP => eval.lcmp(),

            opcode::LCONST_0 => eval.lconst(0),

            opcode::LCONST_1 => eval.lconst(1),

            opcode::LDC => {
                let cp_lookup = instr[current] as u16;
                current += 1;
                let current_klass = OtKlass::lookup_klass(thread_tx.clone(), &klass_name).clone();

                match current_klass.lookup_cp(cp_lookup) {
                    // FIXME Actually look up the class object properly
                    CpEntry::Class(_) => eval.aconst_null(),
                    CpEntry::Double(dcon) => eval.dconst(dcon),
                    CpEntry::Integer(icon) => eval.iconst(icon),
                    // FIXME Actually look up the class object properly
                    CpEntry::String(idx) => eval.aconst_null(),
                    _ => panic!(
                        "Non-handled entry found in LDC op {} at CP index {}",
                        current_klass.get_name(),
                        cp_lookup
                    ),
                }
            }
            opcode::LDC2_W => {
                let cp_lookup = ((instr[current] as u16) << 8) + instr[current + 1] as u16;
                current += 2;
                let current_klass = OtKlass::lookup_klass(thread_tx.clone(), &klass_name).clone();

                let entry: CpEntry = current_klass.lookup_cp(cp_lookup);
                //                dbg!("Index: {} of type {}", cp_lookup, entry.name());
                //                dbg!(current_klass.clone());
                match entry {
                    // FIXME Actually look up the class object properly
                    CpEntry::Class(_) => eval.aconst_null(),
                    CpEntry::Double(dcon) => eval.dconst(dcon),
                    CpEntry::Integer(icon) => eval.iconst(icon),
                    // FIXME Actually look up the string object properly
                    CpEntry::String(_) => eval.aconst_null(),
                    _ => panic!(
                        "Non-handled entry found in LDC op {} at CP index {}",
                        current_klass.get_name(),
                        cp_lookup
                    ),
                }
            }

            opcode::LDIV => eval.ldiv(),

            opcode::LLOAD => {
                eval.push(lvt.load(instr[current]));
                current += 1
            }

            opcode::LLOAD_0 => eval.push(lvt.load(0)),

            opcode::LLOAD_1 => eval.push(lvt.load(1)),

            opcode::LLOAD_2 => eval.push(lvt.load(2)),

            opcode::LLOAD_3 => eval.push(lvt.load(3)),

            opcode::LMUL => eval.lmul(),

            opcode::LNEG => eval.lneg(),

            opcode::LOR => eval.lor(),

            opcode::LREM => eval.lrem(),

            opcode::LRETURN => break Some(eval.pop()),

            opcode::LSHL => eval.lshl(),

            opcode::LSHR => eval.lshr(),

            opcode::LSTORE => {
                lvt.store(instr[current], eval.pop());
                current += 1;
            }

            opcode::LSTORE_0 => lvt.store(0, eval.pop()),

            opcode::LSTORE_1 => lvt.store(1, eval.pop()),

            opcode::LSTORE_2 => lvt.store(2, eval.pop()),

            opcode::LSTORE_3 => lvt.store(3, eval.pop()),

            opcode::LSUB => eval.lsub(),

            opcode::LUSHR => eval.lushr(),

            opcode::LXOR => eval.lxor(),

            // FIXME TEMP
            opcode::MONITORENTER => {
                eval.pop();
            }
            // FIXME TEMP
            opcode::MONITOREXIT => {
                eval.pop();
            }
            opcode::NEW => {
                let cp_lookup = ((instr[current] as u16) << 8) + instr[current + 1] as u16;
                current += 2;
                let current_klass = OtKlass::lookup_klass(thread_tx.clone(), &klass_name).clone();

                let alloc_klass_name = match current_klass.lookup_cp(cp_lookup) {
                    // FIXME Find class name from constant pool of the current class
                    CpEntry::Class(c) => current_klass.cp_as_string(c.0), // "DUMMY_CLASS".to_string(),
                    _ => panic!(
                        "Non-class found in {} at CP index {}",
                        current_klass.get_name(),
                        cp_lookup
                    ),
                };
                //                dbg!(alloc_klass_name.clone());
                let object_klass =
                    OtKlass::lookup_klass(thread_tx.clone(), &alloc_klass_name).clone();

                let obj_id = HEAP.lock().unwrap().allocate_obj(&object_klass);
                eval.push(JvmValue::ObjRef(obj_id));
            }
            opcode::NEWARRAY => {
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
                        JvmValue::Int(arr_size) => HEAP.lock().unwrap().allocate_int_arr(arr_size),
                        _ => panic!("Not an int on the stack at {}", (current - 1)),
                    },
                    _ => panic!("Unsupported primitive array type at {}", (current - 1)),
                };

                eval.push(JvmValue::ObjRef(arr_id));
            }
            opcode::NOP => (),
            opcode::POP => {
                eval.pop();
            }
            opcode::POP2 => {
                let _discard: JvmValue = eval.pop();
                // FIXME Change to type match
                // if (discard.type == JVMType.J || discard.type == JVMType.D) {

                // }
                eval.pop();
            }
            opcode::PUTFIELD => {
                let cp_lookup = ((instr[current] as u16) << 8) + instr[current + 1] as u16;
                current += 2;

                let val = eval.pop();

                let recvp: JvmValue = eval.pop();
                let obj_id = match recvp {
                    JvmValue::ObjRef(v) => v,
                    _ => panic!("Not an object ref at {}", (current - 1)),
                };

                let putf =
                    OtKlass::lookup_instance_field(thread_tx.clone(), &klass_name, cp_lookup);

                HEAP.lock().unwrap().put_field(obj_id, putf, val);
            }
            opcode::PUTSTATIC => {
                let cp_lookup = ((instr[current] as u16) << 8) + instr[current + 1] as u16;
                current += 2;

                let puts = OtKlass::lookup_static_field(thread_tx.clone(), &klass_name, cp_lookup);
                let klass =
                    OtKlass::lookup_klass(thread_tx.clone(), &puts.get_klass_name()).clone();

                klass.put_static(&puts, eval.pop());
            }
            opcode::RETURN => break None,
            opcode::SIPUSH => {
                let vtmp = ((instr[current] as i32) << 8) + instr[current + 1] as i32;
                eval.iconst(vtmp);
                current += 2;
            }
            opcode::SWAP => {
                let val1 = eval.pop();
                let val2 = eval.pop();
                eval.push(val1);
                eval.push(val2);
            }
            // Disallowed opcodes
            opcode::BREAKPOINT => break Some(JvmValue::Boolean(false)),
            opcode::IMPDEP1 => break Some(JvmValue::Boolean(false)),
            opcode::IMPDEP2 => break Some(JvmValue::Boolean(false)),
            opcode::JSR => break Some(JvmValue::Boolean(false)),
            opcode::JSR_W => break Some(JvmValue::Boolean(false)),
            opcode::RET => break Some(JvmValue::Boolean(false)),

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
        JvmValue::Int(i) => match v2 {
            JvmValue::Int(i1) => f(i, i1),
            _ => panic!("Values found to have differing type for IF_ICMP*"),
        },
        _ => panic!("Values found to have the wrong type for IF_ICMP*"),
    }
}

fn dispatch_invoke(
    tx: Sender<OtKlassComms>,
    current_klass: OtKlass,
    cp_lookup: u16,
    eval: &mut InterpEvalStack,
    additional_args: u8,
) {
    let fq_name_desc = current_klass.cp_as_string(cp_lookup);
    let klz_idx = match current_klass.lookup_cp(cp_lookup) {
        CpEntry::MethodRef(mr) => mr.clz_idx,
        _ => panic!(
            "Non-methodref found in {} at CP index {}",
            current_klass.get_name(),
            cp_lookup
        ),
    };
    let dispatch_klass_name = current_klass.cp_as_string(klz_idx);

    let callee = OtKlass::lookup_method_exact(tx.clone(), &dispatch_klass_name, fq_name_desc);

    // FIXME - General setup requires call args from the stack
    let mut vars = InterpLocalVars::of(255);
    if additional_args > 0 {
        vars.store(0, eval.pop());
    }
    // Explicit use of match expression to be clear about the semantics
    match exec_method(tx, &callee, &mut vars) {
        Some(val) => eval.push(val),
        None => (),
    }
}

// #[cfg(test)]
// mod tests;
