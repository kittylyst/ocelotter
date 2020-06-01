#![deny(unreachable_patterns)]

use crate::JvmValue;

pub struct InterpEvalStack {
    stack: Vec<JvmValue>,
}

impl InterpEvalStack {
    pub fn of() -> InterpEvalStack {
        InterpEvalStack { stack: Vec::new() }
    }

    pub fn push(&mut self, val: JvmValue) -> () {
        let s = &mut self.stack;
        s.push(val);
    }

    pub fn pop(&mut self) -> JvmValue {
        let s = &mut self.stack;
        match s.pop() {
            Some(value) => value,
            None => panic!("pop() on empty stack"),
        }
    }

    pub fn aconst_null(&mut self) -> () {
        self.push(JvmValue::ObjRef {
            val: 0, // OtObj::get_null(),
        });
    }

    //
    // I opcodes - int
    //

    pub fn iconst(&mut self, v: i32) -> () {
        self.push(JvmValue::Int { val: v });
    }

    pub fn i2d(&mut self) -> () {
        let i1 = match self.pop() {
            JvmValue::Int { val: i } => i,
            _ => panic!("Unexpected, non-integer value encountered"),
        };
        self.push(JvmValue::Double { val: i1 as f64 });
    }

    pub fn i2l(&mut self) -> () {
        let i1 = match self.pop() {
            JvmValue::Int { val: i } => i,
            _ => panic!("Unexpected, non-integer value encountered"),
        };
        self.push(JvmValue::Long { val: i1 as i64 });
    }


    pub fn iadd(&mut self) -> () {
        // For a runtime checking interpreter - type checks would go here...
        let i1 = match self.pop() {
            JvmValue::Int { val: i } => i,
            _ => panic!("Unexpected, non-integer value encountered"),
        };
        let i2 = match self.pop() {
            JvmValue::Int { val: i } => i,
            _ => panic!("Unexpected, non-integer value encountered"),
        };

        self.push(JvmValue::Int { val: i1 + i2 });
    }

    pub fn isub(&mut self) -> () {
        // For a runtime checking interpreter - type checks would go here...
        let i1 = match self.pop() {
            JvmValue::Int { val: i } => i,
            _ => panic!("Unexpected, non-integer value encountered"),
        };
        let i2 = match self.pop() {
            JvmValue::Int { val: i } => i,
            _ => panic!("Unexpected, non-integer value encountered"),
        };

        self.push(JvmValue::Int { val: i1 - i2 });
    }

    pub fn imul(&mut self) -> () {
        // For a runtime checking interpreter - type checks would go here...
        let i1 = match self.pop() {
            JvmValue::Int { val: i } => i,
            _ => panic!("Unexpected, non-integer value encountered"),
        };
        let i2 = match self.pop() {
            JvmValue::Int { val: i } => i,
            _ => panic!("Unexpected, non-integer value encountered"),
        };

        self.push(JvmValue::Int { val: i1 * i2 });
    }

    pub fn irem(&mut self) -> () {
        // For a runtime checking interpreter - type checks would go here...
        let i1 = match self.pop() {
            JvmValue::Int { val: i } => i,
            _ => panic!("Unexpected, non-integer value encountered"),
        };
        let i2 = match self.pop() {
            JvmValue::Int { val: i } => i,
            _ => panic!("Unexpected, non-integer value encountered"),
        };

        self.push(JvmValue::Int { val: i2 % i1 });
    }

    pub fn idiv(&mut self) -> () {
        // For a runtime checking interpreter - type checks would go here...
        let i1 = match self.pop() {
            JvmValue::Int { val: i } => i,
            _ => panic!("Unexpected, non-integer value encountered"),
        };
        let i2 = match self.pop() {
            JvmValue::Int { val: i } => i,
            _ => panic!("Unexpected, non-integer value encountered"),
        };

        self.push(JvmValue::Int { val: i2 / i1 });
    }

    pub fn iand(&mut self) -> () {
        let i1 = match self.pop() {
            JvmValue::Int { val: i } => i,
            _ => panic!("Unexpected, non-integer value encountered"),
        };
        let i2 = match self.pop() {
            JvmValue::Int { val: i } => i,
            _ => panic!("Unexpected, non-integer value encountered"),
        };
        self.push(JvmValue::Int { val: i1 & i2 });
    }

    pub fn ineg(&mut self) -> () {
        let i1 = match self.pop() {
            JvmValue::Int { val: i } => i,
            _ => panic!("Unexpected, non-integer value encountered"),
        };
        self.push(JvmValue::Int { val: -i1 });
    }

    pub fn ior(&mut self) -> () {
        let i1 = match self.pop() {
            JvmValue::Int { val: i } => i,
            _ => panic!("Unexpected, non-integer value encountered"),
        };
        let i2 = match self.pop() {
            JvmValue::Int { val: i } => i,
            _ => panic!("Unexpected, non-integer value encountered"),
        };
        self.push(JvmValue::Int { val: i1 | i2 });
    }

    pub fn ixor(&mut self) -> () {
        let i1 = match self.pop() {
            JvmValue::Int { val: i } => i,
            _ => panic!("Unexpected, non-integer value encountered"),
        };
        let i2 = match self.pop() {
            JvmValue::Int { val: i } => i,
            _ => panic!("Unexpected, non-integer value encountered"),
        };
        self.push(JvmValue::Int { val: i1 ^ i2 });

    }

    pub fn ishl(&mut self) -> () {
        let i1 = match self.pop() {
            JvmValue::Int { val: i } => i,
            _ => panic!("Unexpected, non-integer value encountered"),
        };
        let i2 = match self.pop() {
            JvmValue::Int { val: i } => i,
            _ => panic!("Unexpected, non-integer value encountered"),
        };
        self.push(JvmValue::Int { val: i1 << i2 });
    }

    pub fn ishr(&mut self) -> () {
        let i1 = match self.pop() {
            JvmValue::Int { val: i } => i,
            _ => panic!("Unexpected, non-integer value encountered"),
        };
        let i2 = match self.pop() {
            JvmValue::Int { val: i } => i,
            _ => panic!("Unexpected, non-integer value encountered"),
        };
        self.push(JvmValue::Int { val: i1 >> i2 });
    }


    //
    // L opcodes - long
    //

    pub fn lconst(&mut self, v: i64) -> () {
        self.push(JvmValue::Long { val: v });
    }

    pub fn ladd(&mut self) -> () {
        // For a runtime checking interpreter - type checks would go here...
        let i1 = match self.pop() {
            JvmValue::Long { val: i } => i,
            _ => panic!("Unexpected, non-integer value encountered"),
        };
        let i2 = match self.pop() {
            JvmValue::Long { val: i } => i,
            _ => panic!("Unexpected, non-integer value encountered"),
        };

        self.push(JvmValue::Long { val: i1 + i2 });
    }

    pub fn lsub(&mut self) -> () {
        // For a runtime checking interpreter - type checks would go here...
        let i1 = match self.pop() {
            JvmValue::Long { val: i } => i,
            _ => panic!("Unexpected, non-integer value encountered"),
        };
        let i2 = match self.pop() {
            JvmValue::Long { val: i } => i,
            _ => panic!("Unexpected, non-integer value encountered"),
        };

        self.push(JvmValue::Long { val: i1 - i2 });
    }

    pub fn ldiv(&mut self) -> () {
        // For a runtime checking interpreter - type checks would go here...
        let i1 = match self.pop() {
            JvmValue::Long { val: i } => i,
            _ => panic!("Unexpected, non-integer value encountered"),
        };
        let i2 = match self.pop() {
            JvmValue::Long { val: i } => i,
            _ => panic!("Unexpected, non-integer value encountered"),
        };

        self.push(JvmValue::Long { val: i2 / i1 });
    }

    pub fn lmul(&mut self) -> () {
        // For a runtime checking interpreter - type checks would go here...
        let i1 = match self.pop() {
            JvmValue::Long { val: i } => i,
            _ => panic!("Unexpected, non-integer value encountered"),
        };
        let i2 = match self.pop() {
            JvmValue::Long { val: i } => i,
            _ => panic!("Unexpected, non-integer value encountered"),
        };

        self.push(JvmValue::Long { val: i2 * i1 });
    }

    pub fn lneg(&mut self) -> () {
        let i1 = match self.pop() {
            JvmValue::Long { val: i } => i,
            _ => panic!("Unexpected, non-integer value encountered"),
        };
        self.push(JvmValue::Long { val: -i1 });
    }

    pub fn land(&mut self) -> () {
        let i1 = match self.pop() {
            JvmValue::Long { val: i } => i,
            _ => panic!("Unexpected, non-integer value encountered"),
        };
        let i2 = match self.pop() {
            JvmValue::Long { val: i } => i,
            _ => panic!("Unexpected, non-integer value encountered"),
        };
        self.push(JvmValue::Long { val: i1 & i2 });
    }

    pub fn lor(&mut self) -> () {
        let i1 = match self.pop() {
            JvmValue::Long { val: i } => i,
            _ => panic!("Unexpected, non-integer value encountered"),
        };
        let i2 = match self.pop() {
            JvmValue::Long { val: i } => i,
            _ => panic!("Unexpected, non-integer value encountered"),
        };
        self.push(JvmValue::Long { val: i1 | i2 });
    }

    pub fn lxor(&mut self) -> () {
        let i1 = match self.pop() {
            JvmValue::Long { val: i } => i,
            _ => panic!("Unexpected, non-integer value encountered"),
        };
        let i2 = match self.pop() {
            JvmValue::Long { val: i } => i,
            _ => panic!("Unexpected, non-integer value encountered"),
        };
        self.push(JvmValue::Long { val: i1 ^ i2 });
    }

    pub fn lshl(&mut self) -> () {
        let i1 = match self.pop() {
            JvmValue::Long { val: i } => i,
            _ => panic!("Unexpected, non-integer value encountered"),
        };
        let i2 = match self.pop() {
            JvmValue::Long { val: i } => i,
            _ => panic!("Unexpected, non-integer value encountered"),
        };
        self.push(JvmValue::Long { val: i1 << i2 });
    }

    pub fn lshr(&mut self) -> () {
        let i1 = match self.pop() {
            JvmValue::Long { val: i } => i,
            _ => panic!("Unexpected, non-integer value encountered"),
        };
        let i2 = match self.pop() {
            JvmValue::Long { val: i } => i,
            _ => panic!("Unexpected, non-integer value encountered"),
        };
        self.push(JvmValue::Long { val: i1 >> i2 });
    }

    pub fn lcmp(&mut self) {
        let v2 = match self.pop() {
            JvmValue::Long{ val: v } => v,
            _ => panic!("Non-long seen on stack during LCMP"),
        };
        let v1 = match self.pop() {
            JvmValue::Long { val: v } => v,
            _ => panic!("Non-double seen on stack during LCMP"),
        };
        let mut out = JvmValue::Int { val: 0 };
        if v1 > v2 {
            out = JvmValue::Int { val: 1 };
        }
        if v1 < v2 {
            out = JvmValue::Int { val: -1 };
        }
        self.push(out);
    }


    //
    // F opcodes - float
    //

    pub fn f2d(&mut self) -> () {
        let i1 = match self.pop() {
            JvmValue::Float { val: i } => i,
            _ => panic!("Unexpected, non-integer value encountered"),
        };
        self.push(JvmValue::Double { val: i1 as f64 });
    }

    pub fn fadd(&mut self) -> () {
        // For a runtime checking interpreter - type checks would go here...
        let i1 = match self.pop() {
            JvmValue::Float { val: i } => i,
            _ => panic!("Unexpected, non-double value encountered"),
        };
        let i2 = match self.pop() {
            JvmValue::Float { val: i } => i,
            _ => panic!("Unexpected, non-double value encountered"),
        };

        self.push(JvmValue::Float { val: i1 + i2 });
    }

    pub fn fsub(&mut self) -> () {
        // For a runtime checking interpreter - type checks would go here...
        let i1 = match self.pop() {
            JvmValue::Float { val: i } => i,
            _ => panic!("Unexpected, non-double value encountered"),
        };
        let i2 = match self.pop() {
            JvmValue::Float { val: i } => i,
            _ => panic!("Unexpected, non-double value encountered"),
        };

        self.push(JvmValue::Float { val: i1 - i2 });
    }

    pub fn fmul(&mut self) -> () {
        // For a runtime checking interpreter - type checks would go here...
        let i1 = match self.pop() {
            JvmValue::Float { val: i } => i,
            _ => panic!("Unexpected, non-double value encountered"),
        };
        let i2 = match self.pop() {
            JvmValue::Float { val: i } => i,
            _ => panic!("Unexpected, non-double value encountered"),
        };

        self.push(JvmValue::Float { val: i1 * i2 });
    }

    pub fn fdiv(&mut self) -> () {
        // For a runtime checking interpreter - type checks would go here...
        let i1 = match self.pop() {
            JvmValue::Float { val: i } => i,
            _ => panic!("Unexpected, non-double value encountered"),
        };
        let i2 = match self.pop() {
            JvmValue::Float { val: i } => i,
            _ => panic!("Unexpected, non-double value encountered"),
        };

        self.push(JvmValue::Float { val: i1 / i2 });
    }

    pub fn fconst(&mut self, v: f32) -> () {
        self.push(JvmValue::Float { val: v });
    }

    pub fn fneg(&mut self) -> () {
        let d = match self.pop() {
            JvmValue::Float { val: d } => d,
            _ => panic!("Unexpected, non-float value encountered"),
        };
        self.push(JvmValue::Float { val: -d });
    }

    pub fn fcmpg(&mut self) {
        let v2 = match self.pop() {
            JvmValue::Float { val: v } => v,
            _ => panic!("Non-double seen on stack during FCMPG"),
        };
        let v1 = match self.pop() {
            JvmValue::Float { val: v } => v,
            _ => panic!("Non-double seen on stack during FCMPG"),
        };
        if v1.is_nan() || v2.is_nan() {
            self.push(JvmValue::Int { val: 1 });
        } else {
            let mut out = JvmValue::Int { val: 0 };
            if v1 > v2 {
                out = JvmValue::Int { val: 1 };
            }
            if v1 < v2 {
                out = JvmValue::Int { val: -1 };
            }
//            dbg!(out, v1, v2);
            self.push(out);
        }

    }

    pub fn fcmpl(&mut self) {
        let v2 = match self.pop() {
            JvmValue::Float { val: v } => v,
            _ => panic!("Non-double seen on stack during FCMPL"),
        };
        let v1 = match self.pop() {
            JvmValue::Float { val: v } => v,
            _ => panic!("Non-double seen on stack during FCMPL"),
        };
        if v1.is_nan() || v2.is_nan() {
            self.push(JvmValue::Int { val: -1 });
        } else {
            let mut out = JvmValue::Int { val: 0 };
            if v1 > v2 {
                out = JvmValue::Int { val: 1 };
            }
            if v1 < v2 {
                out = JvmValue::Int { val: -1 };
            }
//            dbg!(out, v1, v2);
            self.push(out);
        }
    }

    //
    // D opcodes - double
    //

    pub fn dadd(&mut self) -> () {
        // For a runtime checking interpreter - type checks would go here...
        let i1 = match self.pop() {
            JvmValue::Double { val: i } => i,
            _ => panic!("Unexpected, non-double value encountered"),
        };
        let i2 = match self.pop() {
            JvmValue::Double { val: i } => i,
            _ => panic!("Unexpected, non-double value encountered"),
        };

        self.push(JvmValue::Double { val: i1 + i2 });
    }

    pub fn dsub(&mut self) -> () {
        // For a runtime checking interpreter - type checks would go here...
        let i1 = match self.pop() {
            JvmValue::Double { val: i } => i,
            _ => panic!("Unexpected, non-double value encountered"),
        };
        let i2 = match self.pop() {
            JvmValue::Double { val: i } => i,
            _ => panic!("Unexpected, non-double value encountered"),
        };

        self.push(JvmValue::Double { val: i1 - i2 });
    }

    pub fn dmul(&mut self) -> () {
        // For a runtime checking interpreter - type checks would go here...
        let i1 = match self.pop() {
            JvmValue::Double { val: i } => i,
            _ => panic!("Unexpected, non-double value encountered"),
        };
        let i2 = match self.pop() {
            JvmValue::Double { val: i } => i,
            _ => panic!("Unexpected, non-double value encountered"),
        };

        self.push(JvmValue::Double { val: i1 * i2 });
    }

    pub fn drem(&mut self) -> () {
        // For a runtime checking interpreter - type checks would go here...
        let i1 = match self.pop() {
            JvmValue::Double { val: i } => i,
            _ => panic!("Unexpected, non-integer value encountered"),
        };
        let i2 = match self.pop() {
            JvmValue::Double { val: i } => i,
            _ => panic!("Unexpected, non-integer value encountered"),
        };

        self.push(JvmValue::Double { val: i2.rem_euclid(i1) });
    }

    pub fn ddiv(&mut self) -> () {
        // For a runtime checking interpreter - type checks would go here...
        let i1 = match self.pop() {
            JvmValue::Double { val: i } => i,
            _ => panic!("Unexpected, non-double value encountered"),
        };
        let i2 = match self.pop() {
            JvmValue::Double { val: i } => i,
            _ => panic!("Unexpected, non-double value encountered"),
        };

        self.push(JvmValue::Double { val: i2 / i1 });
    }

    pub fn dneg(&mut self) -> () {
        let d = match self.pop() {
            JvmValue::Double { val: d } => d,
            _ => panic!("Unexpected, non-integer value encountered"),
        };
        self.push(JvmValue::Double { val: -d });
    }

    pub fn dconst(&mut self, v: f64) -> () {
        self.push(JvmValue::Double { val: v });
    }

    pub fn dcmpg(&mut self) {
        let v2 = match self.pop() {
            JvmValue::Double { val: v } => v,
            _ => panic!("Non-double seen on stack during DCMPG"),
        };
        let v1 = match self.pop() {
            JvmValue::Double { val: v } => v,
            _ => panic!("Non-double seen on stack during DCMPG"),
        };
        if v1.is_nan() || v2.is_nan() {
            self.push(JvmValue::Int { val: 1 });
        } else {
            let mut out = JvmValue::Int { val: 0 };
            if v1 > v2 {
                out = JvmValue::Int { val: 1 };
            }
            if v1 < v2 {
                out = JvmValue::Int { val: -1 };
            }
//            dbg!(out, v1, v2);
            self.push(out);
        }

    }

    pub fn dcmpl(&mut self) {
        let v2 = match self.pop() {
            JvmValue::Double { val: v } => v,
            _ => panic!("Non-double seen on stack during DCMPL"),
        };
        let v1 = match self.pop() {
            JvmValue::Double { val: v } => v,
            _ => panic!("Non-double seen on stack during DCMPL"),
        };
        if v1.is_nan() || v2.is_nan() {
            self.push(JvmValue::Int { val: -1 });
        } else {
            let mut out = JvmValue::Int { val: 0 };
            if v1 > v2 {
                out = JvmValue::Int { val: 1 };
            }
            if v1 < v2 {
                out = JvmValue::Int { val: -1 };
            }
//            dbg!(out, v1, v2);
            self.push(out);
        }
    }

    //
    //  Stack Manipulation
    //

    pub fn dup(&mut self) -> () {
        let i1 = self.pop();
        self.push(i1.to_owned());
        self.push(i1.to_owned());
    }

    pub fn dup_x1(&mut self) -> () {
        let i1 = self.pop();
        let i1c = i1.clone();
        let i2 = self.pop();
        self.push(i1);
        self.push(i2);
        self.push(i1c);
    }

    pub fn dup2(&mut self) -> () {
        let v1 = self.pop();
        // if v1 is double-width

        let v2 = self.pop();

        self.push(v2.to_owned());
        self.push(v2.to_owned());
    }

}
