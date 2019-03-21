use super::*;

use std::path::Path;

#[test]
fn test_read_header() {
    let bytes = match file_to_bytes(Path::new("../resources/test/Foo.class")) {
        Ok(buf) => buf,
        _ => panic!("Error reading Foo"),
    };
    let mut parser = klass_parser::OtKlassParser::of(bytes, "Foo.class".to_string());
    parser.parse();
    assert_eq!(16, parser.get_pool_size());
    let k = parser.klass();
    assert_eq!("Foo", k.get_name());
    assert_eq!("java/lang/Object", k.get_super_name());
}

#[test]
fn test_read_simple_class() {
    let bytes = match file_to_bytes(Path::new("../resources/test/Foo2.class")) {
        Ok(buf) => buf,
        _ => panic!("Error reading Foo2"),
    };
    let mut parser = klass_parser::OtKlassParser::of(bytes, "Foo2.class".to_string());
    parser.parse();
    assert_eq!(30, parser.get_pool_size());
    let k = parser.klass();
    assert_eq!("Foo2", k.get_name());
    assert_eq!("java/lang/Object", k.get_super_name());
    assert_eq!(2, k.get_methods().len());
}

#[test]
fn check_simple_fields_methods() {
    let bytes = match file_to_bytes(Path::new("../resources/test/octest/SimpleFieldsAndMethods.class")) {
        Ok(buf) => buf,
        _ => panic!("Error reading SimpleFieldsAndMethods"),
    };
    let mut parser = klass_parser::OtKlassParser::of(bytes, "octest/SimpleFieldsAndMethods.class".to_string());
    parser.parse();
    assert_eq!(23, parser.get_pool_size());
    let k = parser.klass();
    assert_eq!("octest/SimpleFieldsAndMethods", k.get_name());
    assert_eq!("java/lang/Object", k.get_super_name());
    assert_eq!(2, k.get_methods().len());
    // assert_eq!(1, k.get_fields().len());
}
