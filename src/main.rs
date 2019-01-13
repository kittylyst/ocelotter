mod opcode;
mod runtime;

// static heap: runtime::SimpleLinkedJVMHeap = runtime::SimpleLinkedJVMHeap {};
// static repo: runtime::ClassRepository = runtime::ClassRepository {};

fn main() {
    // let first_test = vec![
    //     opcode::Opcode::ICONST_1,
    //     opcode::Opcode::ICONST_1,
    //     opcode::Opcode::IADD,
    //     opcode::Opcode::IRETURN,
    // ];
    // let lvt = runtime::LocalVariableTable {};
    // let opt_ret = exec_method(
    //     "DUMMY".to_string(),
    //     "DUMMY_DESC".to_string(),
    //     &first_test,
    //     &lvt,
    // );
    // match opt_ret {
    //     Some(value) => println!("Method returns: {}", value),
    //     None => println!("Method has void return"),
    // };
}
