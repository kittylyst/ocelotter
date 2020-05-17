use std::time::SystemTime;

use crate::InterpLocalVars;
use crate::JvmValue;

pub fn java_lang_Object__hashcode(args: &InterpLocalVars) -> Option<JvmValue> {
    // FIXME Proper hashCode algorithm
    Some(JvmValue::Int { val: 255 })
}

pub fn java_lang_Object__registerNatives(args: &InterpLocalVars) -> Option<JvmValue> {
    // NO-OP for now - this is needed so <clinit> will run
    None
}

////////////////////////////////////////////
// java.lang.Math simple maths methods

pub fn java_lang_Math__sin(args: &InterpLocalVars) -> Option<JvmValue> {
    let d = match args.load(0) {
        JvmValue::Double { val: v } => v,
        x => panic!("Non-double value {} of type {} encountered in Math.sin", x, x.name())
    };

    Some(JvmValue::Double {val: d.sin()})
}

pub fn java_lang_Math__cos(args: &InterpLocalVars) -> Option<JvmValue> {
    let d = match args.load(0) {
        JvmValue::Double { val: v } => v,
        x => panic!("Non-double value {} of type {} encountered in Math.cos", x, x.name())
    };

    Some(JvmValue::Double {val: d.cos()})
}

pub fn java_lang_Math__tan(args: &InterpLocalVars) -> Option<JvmValue> {
    let d = match args.load(0) {
        JvmValue::Double { val: v } => v,
        x => panic!("Non-double value {} of type {} encountered in Math.tan", x, x.name())
    };

    Some(JvmValue::Double {val: d.tan()})
}

pub fn java_lang_Math__asin(args: &InterpLocalVars) -> Option<JvmValue> {
    let d = match args.load(0) {
        JvmValue::Double { val: v } => v,
        x => panic!("Non-double value {} of type {} encountered in Math.asin", x, x.name())
    };

    Some(JvmValue::Double {val: d.asin()})
}

pub fn java_lang_Math__acos(args: &InterpLocalVars) -> Option<JvmValue> {
    let d = match args.load(0) {
        JvmValue::Double { val: v } => v,
        x => panic!("Non-double value {} of type {} encountered in Math.acos", x, x.name())
    };

    Some(JvmValue::Double {val: d.acos()})
}

pub fn java_lang_Math__atan(args: &InterpLocalVars) -> Option<JvmValue> {
    let d = match args.load(0) {
        JvmValue::Double { val: v } => v,
        x => panic!("Non-double value {} of type {} encountered in Math.atan", x, x.name())
    };

    Some(JvmValue::Double {val: d.atan()})
}

pub fn java_lang_Math__exp(args: &InterpLocalVars) -> Option<JvmValue> {
    let d = match args.load(0) {
        JvmValue::Double { val: v } => v,
        x => panic!("Non-double value {} of type {} encountered in Math.exp", x, x.name())
    };

    Some(JvmValue::Double {val: d.exp()})
}

pub fn java_lang_Math__log(args: &InterpLocalVars) -> Option<JvmValue> {
    let d = match args.load(0) {
        JvmValue::Double { val: v } => v,
        x => panic!("Non-double value {} of type {} encountered in Math.log", x, x.name())
    };

    Some(JvmValue::Double {val: d.ln()})
}

pub fn java_lang_Math__sqrt(args: &InterpLocalVars) -> Option<JvmValue> {
    let d = match args.load(0) {
        JvmValue::Double { val: v } => v,
        x => panic!("Non-double value {} of type {} encountered in Math.sqrt", x, x.name())
    };

    Some(JvmValue::Double {val: d.sqrt()})
}

//public static final native double pow(double, double);

pub fn java_lang_Math__pow(args: &InterpLocalVars) -> Option<JvmValue> {
    let base = match args.load(0) {
        JvmValue::Double { val: v } => v,
        x => panic!("Non-double value {} of type {} encountered in Math.pow", x, x.name())
    };

    let raise = match args.load(0) {
        JvmValue::Double { val: v } => v,
        x => panic!("Non-double value {} of type {} encountered in Math.pow", x, x.name())
    };


    Some(JvmValue::Double {val: base.powf(raise)})
}


//public static final native double IEEEremainder(double, double);
//public static final native double ceil(double);
//public static final native double floor(double);
//public static final native double rint(double);
//public static final native double atan2(double, double);


////////////////////////////////////////////

// FIXME System -> Runtime -> Shutdown
pub fn java_lang_Shutdown__exit(args: &InterpLocalVars) -> Option<JvmValue> {
    Some(JvmValue::Int { val: 255 })
}

pub fn java_lang_System__currentTimeMillis(args: &InterpLocalVars) -> Option<JvmValue> {
    let millis = match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
        Ok(n) => n.as_millis(),
        Err(_) => panic!("SystemTime before UNIX EPOCH!"),
    };
    Some(JvmValue::Long { val: millis as i64 })
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
