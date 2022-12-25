use std::time::SystemTime;

use crate::InterpLocalVars;
use crate::JvmValue;

////////////////////////////////////////////
// java.lang.Object

// getClass()

pub fn java_lang_Object__hashcode(args: &InterpLocalVars) -> Option<JvmValue> {
    // FIXME Proper hashCode algorithm
    Some(JvmValue::Int(255))
}

// clone()

pub fn java_lang_Object__notify(args: &InterpLocalVars) -> Option<JvmValue> {
    // NO-OP for now
    None
}

pub fn java_lang_Object__notifyAll(args: &InterpLocalVars) -> Option<JvmValue> {
    // NO-OP for now
    None
}

pub fn java_lang_Object__wait(args: &InterpLocalVars) -> Option<JvmValue> {
    // NO-OP for now
    None
}

////////////////////////////////////////////
// java.lang.Class

pub fn java_lang_Class__getName(args: &InterpLocalVars) -> Option<JvmValue> {
    let obj = match args.load(0) {
        JvmValue::ObjRef(v) => v,
        x => panic!(
            "Non-object value {} of type {} encountered in Class.getName()",
            x,
            x.name()
        ),
    };
    // Lookup object in heap...

    // FIXME Currently returns null
    Some(JvmValue::ObjRef(0))
}

////////////////////////////////////////////
// java.lang.Compiler

pub fn java_lang_Compiler__compileClass(args: &InterpLocalVars) -> Option<JvmValue> {
    Some(JvmValue::Boolean(true))
}

pub fn java_lang_Compiler__compileClasses(args: &InterpLocalVars) -> Option<JvmValue> {
    Some(JvmValue::Boolean(true))
}

pub fn java_lang_Compiler__enable(args: &InterpLocalVars) -> Option<JvmValue> {
    // DUMMY
    None
}

pub fn java_lang_Compiler__disable(args: &InterpLocalVars) -> Option<JvmValue> {
    // DUMMY
    None
}

////////////////////////////////////////////
// java.lang.Runtime

pub fn java_lang_Runtime__freeMemory(args: &InterpLocalVars) -> Option<JvmValue> {
    // DUMMY
    Some(JvmValue::Long(64 * 1024 * 1024))
}

pub fn java_lang_Runtime__totalMemory(args: &InterpLocalVars) -> Option<JvmValue> {
    // DUMMY
    Some(JvmValue::Long(64 * 1024 * 1024))
}

pub fn java_lang_Runtime__gc(args: &InterpLocalVars) -> Option<JvmValue> {
    // DUMMY
    None
}

pub fn java_lang_Runtime__runFinalization(args: &InterpLocalVars) -> Option<JvmValue> {
    // DUMMY
    None
}

pub fn java_lang_Runtime__traceInstructions(args: &InterpLocalVars) -> Option<JvmValue> {
    // DUMMY
    None
}

pub fn java_lang_Runtime__traceMethodCalls(args: &InterpLocalVars) -> Option<JvmValue> {
    // DUMMY
    None
}

////////////////////////////////////////////
// java.lang.System

pub fn java_lang_System__currentTimeMillis(args: &InterpLocalVars) -> Option<JvmValue> {
    let millis = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .expect("SystemTime before UNIX EPOCH!")
        .as_millis();
    Some(JvmValue::Long(millis as i64))
}

pub fn java_lang_System__arraycopy(args: &InterpLocalVars) -> Option<JvmValue> {
    // NO-OP for now
    None
}

////////////////////////////////////////////
// java.lang.Math simple maths methods

pub fn java_lang_Math__sin(args: &InterpLocalVars) -> Option<JvmValue> {
    let d = match args.load(0) {
        JvmValue::Double(v) => v,
        x => panic!(
            "Non-double value {} of type {} encountered in Math.sin",
            x,
            x.name()
        ),
    };
    Some(JvmValue::Double(d.sin()))
}

pub fn java_lang_Math__cos(args: &InterpLocalVars) -> Option<JvmValue> {
    let d = match args.load(0) {
        JvmValue::Double(v) => v,
        x => panic!(
            "Non-double value {} of type {} encountered in Math.cos",
            x,
            x.name()
        ),
    };
    Some(JvmValue::Double(d.cos()))
}

pub fn java_lang_Math__tan(args: &InterpLocalVars) -> Option<JvmValue> {
    let d = match args.load(0) {
        JvmValue::Double(v) => v,
        x => panic!(
            "Non-double value {} of type {} encountered in Math.tan",
            x,
            x.name()
        ),
    };
    Some(JvmValue::Double(d.tan()))
}

pub fn java_lang_Math__asin(args: &InterpLocalVars) -> Option<JvmValue> {
    let d = match args.load(0) {
        JvmValue::Double(v) => v,
        x => panic!(
            "Non-double value {} of type {} encountered in Math.asin",
            x,
            x.name()
        ),
    };
    Some(JvmValue::Double(d.asin()))
}

pub fn java_lang_Math__acos(args: &InterpLocalVars) -> Option<JvmValue> {
    let d = match args.load(0) {
        JvmValue::Double(v) => v,
        x => panic!(
            "Non-double value {} of type {} encountered in Math.acos",
            x,
            x.name()
        ),
    };

    Some(JvmValue::Double(d.acos()))
}

pub fn java_lang_Math__atan(args: &InterpLocalVars) -> Option<JvmValue> {
    let d = match args.load(0) {
        JvmValue::Double(v) => v,
        x => panic!(
            "Non-double value {} of type {} encountered in Math.atan",
            x,
            x.name()
        ),
    };

    Some(JvmValue::Double(d.atan()))
}

pub fn java_lang_Math__exp(args: &InterpLocalVars) -> Option<JvmValue> {
    let d = match args.load(0) {
        JvmValue::Double(v) => v,
        x => panic!(
            "Non-double value {} of type {} encountered in Math.exp",
            x,
            x.name()
        ),
    };

    Some(JvmValue::Double(d.exp()))
}

pub fn java_lang_Math__log(args: &InterpLocalVars) -> Option<JvmValue> {
    let d = match args.load(0) {
        JvmValue::Double(v) => v,
        x => panic!(
            "Non-double value {} of type {} encountered in Math.log",
            x,
            x.name()
        ),
    };

    Some(JvmValue::Double(d.ln()))
}

pub fn java_lang_Math__sqrt(args: &InterpLocalVars) -> Option<JvmValue> {
    let d = match args.load(0) {
        JvmValue::Double(v) => v,
        x => panic!(
            "Non-double value {} of type {} encountered in Math.sqrt",
            x,
            x.name()
        ),
    };

    Some(JvmValue::Double(d.sqrt()))
}

pub fn java_lang_Math__ceil(args: &InterpLocalVars) -> Option<JvmValue> {
    let d = match args.load(0) {
        JvmValue::Double(v) => v,
        x => panic!(
            "Non-double value {} of type {} encountered in Math.ceil",
            x,
            x.name()
        ),
    };

    Some(JvmValue::Double(d.ceil()))
}

pub fn java_lang_Math__floor(args: &InterpLocalVars) -> Option<JvmValue> {
    let d = match args.load(0) {
        JvmValue::Double(v) => v,
        x => panic!(
            "Non-double value {} of type {} encountered in Math.floor",
            x,
            x.name()
        ),
    };

    Some(JvmValue::Double(d.floor()))
}

//public static final native double IEEEremainder(double, double);

//public static final native double rint(double);

pub fn java_lang_Math__atan2(args: &InterpLocalVars) -> Option<JvmValue> {
    let base = match args.load(0) {
        JvmValue::Double(v) => v,
        x => panic!(
            "Non-double value {} of type {} encountered in Math.atan2",
            x,
            x.name()
        ),
    };

    let other = match args.load(0) {
        JvmValue::Double(v) => v,
        x => panic!(
            "Non-double value {} of type {} encountered in Math.atan2",
            x,
            x.name()
        ),
    };

    Some(JvmValue::Double(base.atan2(other)))
}

pub fn java_lang_Math__pow(args: &InterpLocalVars) -> Option<JvmValue> {
    let base = match args.load(0) {
        JvmValue::Double(v) => v,
        x => panic!(
            "Non-double value {} of type {} encountered in Math.pow",
            x,
            x.name()
        ),
    };

    let raise = match args.load(0) {
        JvmValue::Double(v) => v,
        x => panic!(
            "Non-double value {} of type {} encountered in Math.pow",
            x,
            x.name()
        ),
    };

    Some(JvmValue::Double(base.powf(raise)))
}

//public static final native double IEEEremainder(double, double);
//public static final native double ceil(double);
//public static final native double floor(double);
//public static final native double rint(double);
//public static final native double atan2(double, double);

////////////////////////////////////////////

// FIXME System -> Runtime -> Shutdown
pub fn java_lang_Shutdown__exit(args: &InterpLocalVars) -> Option<JvmValue> {
    Some(JvmValue::Int(255))
}

pub fn java_io_FileDescriptor__initSystemFD(args: &InterpLocalVars) -> Option<JvmValue> {
    let obj = args.load(0);
    let fd = args.load(1);

    // Fix up actual system FD with fd and return obj

    Some(obj)
}

// pub fn java_lang_System__nanoTime(args: &InterpLocalVars) -> Option<JvmValue> {
//     let millis = match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
//         Ok(n) => n.as_millis(),
//         Err(_) => panic!("SystemTime before UNIX EPOCH!"),
//     };
//     Some(JvmValue::Long { val: millis as i64})
// }
