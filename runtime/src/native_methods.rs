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

// pub fn java_lang_System__nanoTime(args: &InterpLocalVars) -> Option<JvmValue> {
//     let millis = match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
//         Ok(n) => n.as_millis(),
//         Err(_) => panic!("SystemTime before UNIX EPOCH!"),
//     };
//     Some(JvmValue::Long { val: millis as i64})
// }
