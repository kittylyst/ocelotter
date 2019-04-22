use crate::JvmValue;
use crate::InterpLocalVars;

pub fn java_lang_Object__hashcode(args: &InterpLocalVars) -> Option<JvmValue> {
    println!("In java_lang_Object__hashcode ");
    Some(JvmValue::Int { val: 255 })
}
