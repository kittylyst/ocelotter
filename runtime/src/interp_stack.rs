#![deny(unreachable_patterns)]
use std::cmp::Ordering;

use crate::JvmValue;

pub struct InterpEvalStack {
    stack: Vec<JvmValue>,
}

fn ordering(o: Ordering) -> JvmValue {
    JvmValue::Int(match o {
        Ordering::Less => -1,
        Ordering::Equal => 0,
        Ordering::Greater => 1,
    })
}

impl InterpEvalStack {
    pub fn of() -> InterpEvalStack {
        InterpEvalStack { stack: Vec::new() }
    }

    pub fn push(&mut self, val: JvmValue) {
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

    pub fn aconst_null(&mut self) {
        self.push(JvmValue::ObjRef(0)); // OtObj::get_null(),
    }

    //
    // I opcodes - int
    //

    pub fn iconst(&mut self, v: i32) {
        self.push(JvmValue::Int(v));
    }

    pub fn i2b(&mut self) {
        let i1 = self.pop().as_int().expect("Unexpected, non-integer value encountered");
        self.push(JvmValue::Byte(i1 as i8));
    }

    pub fn i2c(&mut self) {
        let i1 = self.pop().as_int().expect("Unexpected, non-integer value encountered");
        let c = std::char::from_u32(i1 as u32).unwrap();
        self.push(JvmValue::Char(c));
    }

    pub fn i2d(&mut self) {
        let i1 = self.pop().as_int().expect("Unexpected, non-integer value encountered");
        self.push(JvmValue::Double(i1 as f64))
    }

    pub fn i2f(&mut self) {
        let i1 = self.pop().as_int().expect("Unexpected, non-integer value encountered");
        self.push(JvmValue::Float(i1 as f32));
    }

    pub fn i2l(&mut self) {
        let i1 = self.pop().as_int().expect("Unexpected, non-integer value encountered");
        self.push(JvmValue::Long(i1 as i64));
    }

    pub fn i2s(&mut self) {
        let i1 = self.pop().as_int().expect("Unexpected, non-integer value encountered");
        self.push(JvmValue::Short(i1 as i16));
    }

    pub fn iadd(&mut self) {
        // For a runtime checking interpreter - type checks would go here...
        let i1 = self.pop().as_int().expect("Unexpected, non-integer value encountered");
        let i2 = self.pop().as_int().expect("Unexpected, non-integer value encountered");
        self.push(JvmValue::Int(i1 + i2));
    }

    pub fn isub(&mut self) {
        // For a runtime checking interpreter - type checks would go here...
        let i1 = self.pop().as_int().expect("Unexpected, non-integer value encountered");
        let i2 = self.pop().as_int().expect("Unexpected, non-integer value encountered");
        self.push(JvmValue::Int(i1 - i2));
    }

    pub fn imul(&mut self) {
        // For a runtime checking interpreter - type checks would go here...
        let i1 = self.pop().as_int().expect("Unexpected, non-integer value encountered");
        let i2 = self.pop().as_int().expect("Unexpected, non-integer value encountered");
        self.push(JvmValue::Int(i1 * i2));
    }

    pub fn irem(&mut self) {
        // For a runtime checking interpreter - type checks would go here...
        let i1 = self.pop().as_int().expect("Unexpected, non-integer value encountered");
        let i2 = self.pop().as_int().expect("Unexpected, non-integer value encountered");
        self.push(JvmValue::Int(i2 % i1));
    }

    pub fn idiv(&mut self) {
        // For a runtime checking interpreter - type checks would go here...
        let i1 = self.pop().as_int().expect("Unexpected, non-integer value encountered");
        let i2 = self.pop().as_int().expect("Unexpected, non-integer value encountered");
        self.push(JvmValue::Int(i2 / i1));
    }

    pub fn iand(&mut self) {
        let i1 = self.pop().as_int().expect("Unexpected, non-integer value encountered");
        let i2 = self.pop().as_int().expect("Unexpected, non-integer value encountered");
        self.push(JvmValue::Int(i1 & i2));
    }

    pub fn ineg(&mut self) {
        let i1 = self.pop().as_int().expect("Unexpected, non-integer value encountered");
        self.push(JvmValue::Int(-i1));
    }

    pub fn ior(&mut self) {
        let i1 = self.pop().as_int().expect("Unexpected, non-integer value encountered");
        let i2 = self.pop().as_int().expect("Unexpected, non-integer value encountered");
        self.push(JvmValue::Int(i1 | i2));
    }

    pub fn ixor(&mut self) {
        let i1 = self.pop().as_int().expect("Unexpected, non-integer value encountered");
        let i2 = self.pop().as_int().expect("Unexpected, non-integer value encountered");
        self.push(JvmValue::Int(i1 ^ i2));

    }

    pub fn ishl(&mut self) {
        let i1 = self.pop().as_int().expect("Unexpected, non-integer value encountered");
        let i2 = self.pop().as_int().expect("Unexpected, non-integer value encountered");
        self.push(JvmValue::Int(i1 << i2));
    }

    pub fn ishr(&mut self) {
        let i1 = self.pop().as_int().expect("Unexpected, non-integer value encountered");
        let i2 = self.pop().as_int().expect("Unexpected, non-integer value encountered");
        self.push(JvmValue::Int(i1 >> i2));
    }

    pub fn iushr(&mut self) {
        let i1 = self.pop().as_int().expect("Unexpected, non-integer value encountered");
        let i2 = self.pop().as_int().expect("Unexpected, non-integer value encountered");
        self.push(JvmValue::Int((i1 as u32 >> i2 as u32) as i32));
    }

    //
    // L opcodes - long
    //

    pub fn lconst(&mut self, v: i64) {
        self.push(JvmValue::Long(v));
    }

    pub fn l2i(&mut self) {
        let i1 = self.pop().as_long().expect("Unexpected, non-long value encountered");
        self.push(JvmValue::Int(i1 as i32));
    }

    pub fn l2d(&mut self) {
        let i1 = self.pop().as_long().expect("Unexpected, non-long value encountered");
        self.push(JvmValue::Double(i1 as f64));
    }

    pub fn l2f(&mut self) {
        let i1 = self.pop().as_long().expect("Unexpected, non-long value encountered");
        self.push(JvmValue::Float(i1 as f32));
    }

    pub fn ladd(&mut self) {
        // For a runtime checking interpreter - type checks would go here...
        let i1 = self.pop().as_long().expect("Unexpected, non-long value encountered");
        let i2 = self.pop().as_long().expect("Unexpected, non-long value encountered");
        self.push(JvmValue::Long(i1 + i2));
    }

    pub fn lsub(&mut self) {
        // For a runtime checking interpreter - type checks would go here...
        let i1 = self.pop().as_long().expect("Unexpected, non-long value encountered");
        let i2 = self.pop().as_long().expect("Unexpected, non-long value encountered");
        self.push(JvmValue::Long(i1 - i2));
    }

    pub fn lrem(&mut self) {
        // For a runtime checking interpreter - type checks would go here...
        let i1 = self.pop().as_long().expect("Unexpected, non-long value encountered");
        let i2 = self.pop().as_long().expect("Unexpected, non-long value encountered");
        self.push(JvmValue::Long(i2 % i1));
    }

    pub fn ldiv(&mut self) {
        // For a runtime checking interpreter - type checks would go here...
        let i1 = self.pop().as_long().expect("Unexpected, non-long value encountered");
        let i2 = self.pop().as_long().expect("Unexpected, non-long value encountered");
        self.push(JvmValue::Long(i2 / i1));
    }

    pub fn lmul(&mut self) {
        // For a runtime checking interpreter - type checks would go here...
        let i1 = self.pop().as_long().expect("Unexpected, non-long value encountered");
        let i2 = self.pop().as_long().expect("Unexpected, non-long value encountered");
        self.push(JvmValue::Long(i2 * i1));
    }

    pub fn lneg(&mut self) {
        let i1 = self.pop().as_long().expect("Unexpected, non-long value encountered");
        self.push(JvmValue::Long(-i1));
    }

    pub fn land(&mut self) {
        let i1 = self.pop().as_long().expect("Unexpected, non-long value encountered");
        let i2 = self.pop().as_long().expect("Unexpected, non-long value encountered");
        self.push(JvmValue::Long(i1 & i2));
    }

    pub fn lor(&mut self) {
        let i1 = self.pop().as_long().expect("Unexpected, non-long value encountered");
        let i2 = self.pop().as_long().expect("Unexpected, non-long value encountered");
        self.push(JvmValue::Long(i1 | i2));
    }

    pub fn lxor(&mut self) {
        let i1 = self.pop().as_long().expect("Unexpected, non-long value encountered");
        let i2 = self.pop().as_long().expect("Unexpected, non-long value encountered");
        self.push(JvmValue::Long(i1 ^ i2));
    }

    pub fn lshl(&mut self) {
        let i1 = self.pop().as_long().expect("Unexpected, non-long value encountered");
        let i2 = self.pop().as_long().expect("Unexpected, non-long value encountered");
        self.push(JvmValue::Long(i1 << i2));
    }

    pub fn lshr(&mut self) {
        let i1 = self.pop().as_long().expect("Unexpected, non-long value encountered");
        let i2 = self.pop().as_long().expect("Unexpected, non-long value encountered");
        self.push(JvmValue::Long(i1 >> i2));
    }

    pub fn lushr(&mut self) {
        let i1 = self.pop().as_long().expect("Unexpected, non-long value encountered");
        let i2 = self.pop().as_long().expect("Unexpected, non-long value encountered");
        self.push(JvmValue::Long((i1 as u64 >> i2 as u64) as i64));
    }

    pub fn lcmp(&mut self) {
        let v2 = self.pop().as_long().expect("Non-long seen on stack during LCMP");
        let v1 = self.pop().as_long().expect("Non-long seen on stack during LCMP");
        self.push(ordering(i64::cmp(&v1, &v2)))
    }


    //
    // F opcodes - float
    //

    pub fn f2d(&mut self) {
        let i1 = self.pop().as_float().expect("Unexpected, non-float value encountered");
        self.push(JvmValue::Double(i1 as f64));
    }

    pub fn f2i(&mut self) {
        let i1 = self.pop().as_float().expect("Unexpected, non-float value encountered");
        self.push(JvmValue::Int(i1 as i32));
    }

    pub fn f2l(&mut self) {
        let i1 = self.pop().as_float().expect("Unexpected, non-float value encountered");
        self.push(JvmValue::Long(i1 as i64));
    }

    pub fn fadd(&mut self) {
        // For a runtime checking interpreter - type checks would go here...
        let i1 = self.pop().as_float().expect("Unexpected, non-float value encountered");
        let i2 = self.pop().as_float().expect("Unexpected, non-float value encountered");
        self.push(JvmValue::Float(i1 + i2));
    }

    pub fn fsub(&mut self) {
        // For a runtime checking interpreter - type checks would go here...
        let i1 = self.pop().as_float().expect("Unexpected, non-float value encountered");
        let i2 = self.pop().as_float().expect("Unexpected, non-float value encountered");
        self.push(JvmValue::Float(i1 - i2));
    }

    pub fn fmul(&mut self) {
        // For a runtime checking interpreter - type checks would go here...
        let i1 = self.pop().as_float().expect("Unexpected, non-float value encountered");
        let i2 = self.pop().as_float().expect("Unexpected, non-float value encountered");
        self.push(JvmValue::Float(i1 * i2));
    }

    pub fn frem(&mut self) {
        // For a runtime checking interpreter - type checks would go here...
        let i1 = self.pop().as_float().expect("Unexpected, non-float value encountered");
        let i2 = self.pop().as_float().expect("Unexpected, non-float value encountered");
        self.push(JvmValue::Float(i2.rem_euclid(i1)));
    }

    pub fn fdiv(&mut self) {
        // For a runtime checking interpreter - type checks would go here...
        let i1 = self.pop().as_float().expect("Unexpected, non-float value encountered");
        let i2 = self.pop().as_float().expect("Unexpected, non-float value encountered");
        self.push(JvmValue::Float(i2 / i1));
    }

    pub fn fconst(&mut self, v: f32) {
        self.push(JvmValue::Float(v));
    }

    pub fn fneg(&mut self) {
        let d = self.pop().as_float().expect("Unexpected, non-float value encountered");
        self.push(JvmValue::Float(-d));
    }

    pub fn fcmpg(&mut self) {
        let v2 = self.pop().as_float().expect("Non-float seen on stack during FCMPG");
        let v1 = self.pop().as_float().expect("Non-float seen on stack during FCMPG");
        self.push(f32::partial_cmp(&v1, &v2).map(ordering).unwrap_or(JvmValue::Int(1)));
    } 

    pub fn fcmpl(&mut self) {
        let v2 = self.pop().as_float().expect("Non-float seen on stack during FCMPL");
        let v1 = self.pop().as_float().expect("Non-float seen on stack during FCMPL");
        self.push(f32::partial_cmp(&v1, &v2).map(ordering).unwrap_or(JvmValue::Int(-1)));
    }

    //
    // D opcodes - double
    //

    pub fn dadd(&mut self) {
        // For a runtime checking interpreter - type checks would go here...
        let i1 = self.pop().as_double().expect("Unexpected, non-double value encountered");
        let i2 = self.pop().as_double().expect("Unexpected, non-double value encountered");
        self.push(JvmValue::Double(i1 + i2));
    }

    pub fn dsub(&mut self) {
        // For a runtime checking interpreter - type checks would go here...
        let i1 = self.pop().as_double().expect("Unexpected, non-double value encountered");
        let i2 = self.pop().as_double().expect("Unexpected, non-double value encountered");
        self.push(JvmValue::Double(i1 - i2));
    }

    pub fn dmul(&mut self) {
        // For a runtime checking interpreter - type checks would go here...
        let i1 = self.pop().as_double().expect("Unexpected, non-double value encountered");
        let i2 = self.pop().as_double().expect("Unexpected, non-double value encountered");
        self.push(JvmValue::Double(i1 * i2));
    }

    pub fn drem(&mut self) {
        // For a runtime checking interpreter - type checks would go here...
        let i1 = self.pop().as_double().expect("Unexpected, non-double value encountered");
        let i2 = self.pop().as_double().expect("Unexpected, non-double value encountered");
        self.push(JvmValue::Double(i2.rem_euclid(i1)));
    }

    pub fn ddiv(&mut self) {
        // For a runtime checking interpreter - type checks would go here...
        let i1 = self.pop().as_double().expect("Unexpected, non-double value encountered");
        let i2 = self.pop().as_double().expect("Unexpected, non-double value encountered");
        self.push(JvmValue::Double(i2 / i1));
    }

    pub fn dneg(&mut self) {
        let d = self.pop().as_double().expect("Unexpected, non-double value encountered");
        self.push(JvmValue::Double(-d));
    }

    pub fn dconst(&mut self, v: f64) {
        self.push(JvmValue::Double(v));
    }

    pub fn dcmpg(&mut self) {
        let v2 = self.pop().as_double().expect("Non-double seen on stack during DCMPG");
        let v1 = self.pop().as_double().expect("Non-double seen on stack during DCMPG");
        self.push(f64::partial_cmp(&v1, &v2).map(ordering).unwrap_or(JvmValue::Int(1)))
    }

    pub fn dcmpl(&mut self) {
        let v2 = self.pop().as_double().expect("Non-double seen on stack during DCMPL");
        let v1 = self.pop().as_double().expect("Non-double seen on stack during DCMPL");
        self.push(f64::partial_cmp(&v1, &v2).map(ordering).unwrap_or(JvmValue::Int(-1)));
    }

    //
    //  Stack Manipulation
    //

    pub fn dup(&mut self) {
        let i1 = self.pop();
        self.push(i1.clone());
        self.push(i1.clone());
    }

    pub fn dup_x1(&mut self) {
        let i1 = self.pop();
        let i1c = i1.clone();
        let i2 = self.pop();
        self.push(i1);
        self.push(i2);
        self.push(i1c);
    }

    pub fn dup2(&mut self) {
        let v1 = self.pop();
        // if v1 is double-width
        let v2 = self.pop();
        self.push(v2.clone());
        self.push(v2.clone());
    }

}
