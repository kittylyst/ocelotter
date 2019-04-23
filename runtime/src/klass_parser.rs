#![deny(unreachable_patterns)]

use byteorder::{BigEndian, ByteOrder};
use std::io::Read;
use std::str;

use crate::constant_pool::*;

use crate::OtField;
use crate::OtKlass;
use crate::OtMethod;

pub struct OtKlassParser {
    clz_read: Vec<u8>,
    filename: String,
    current: usize,
    major: u16,
    minor: u16,

    pool_item_count: u16,
    flags: u16,
    cp_index_this: u16,
    cp_index_super: u16,
    cp_entries: Vec<CpEntry>,
    interfaces: Vec<u16>,
    fields: Vec<OtField>,
    methods: Vec<OtMethod>,
    // attributes: Vec<CpAttr>,
}

impl OtKlassParser {
    pub fn of(buf: Vec<u8>, fname: String) -> OtKlassParser {
        OtKlassParser {
            clz_read: buf,
            filename: fname,
            current: 0,
            major: 0,
            minor: 0,
            pool_item_count: 0,
            flags: 0,
            cp_index_this: 0,
            cp_index_super: 0,
            cp_entries: Vec::new(),
            interfaces: Vec::new(),
            fields: Vec::new(),
            methods: Vec::new(),
        }
    }

    pub fn klass(&mut self) -> OtKlass {
        OtKlass::of(
            self.klass_name().to_string(),
            self.super_name().to_string(),
            self.flags,
            &self.cp_entries,
            &self.methods,
            &self.fields,
        )
    }

    fn klass_name(&self) -> &String {
        // Lookup the name in the CP - note that CP indices are 1-indexed
        match self.cp_entries[self.cp_index_this as usize] {
            CpEntry::class { idx: icl } => match &self.cp_entries[icl as usize] {
                CpEntry::utf8 { val: s } => s,
                _ => panic!(
                    "Class index {} does not point at utf8 string in constant pool",
                    icl
                ),
            },
            _ => panic!(
                "Self-index {} does not point at class element in constant pool",
                self.cp_index_this
            ),
        }
    }

    fn super_name(&mut self) -> &String {
        // Special-case j.l.O
        if self.klass_name() == "java/lang/Object" {
            return self.klass_name();
        }

        // Lookup the superclass name in the CP - note that CP indices are 1-indexed
        match self.cp_entries[self.cp_index_super as usize] {
            CpEntry::class { idx: scl } => match &self.cp_entries[scl as usize] {
                CpEntry::utf8 { val: s } => s,
                _ => panic!(
                    "Superclass index {} does not point at utf8 string in constant pool",
                    scl
                ),
            },
            _ => panic!(
                "Super-index {} does not point at class element in constant pool",
                self.cp_index_this
            ),
        }
    }

    fn stringref_from_cp(&mut self, idx: u16) -> &String {
        match &self.cp_entries[idx as usize] {
            CpEntry::utf8 { val: s } => s,
            _ => panic!(
                "Superclass index {} does not point at utf8 string in constant pool",
                idx
            ),
        }
    }

    pub fn parse(&mut self) -> () {
        self.parse_header();
        self.parse_constant_pool();
        self.parse_basic_type_info();
        self.parse_fields();
        self.parse_methods();
        //        self.parseAttributes();
    }

    // CP is 1-indexed
    pub fn get_pool_size(&self) -> u16 {
        self.pool_item_count - 1
    }

    // Impl methods
    fn parse_header(&mut self) -> () {
        if self.clz_read[0] != 0xca
            || self.clz_read[1] != 0xfe
            || self.clz_read[2] != 0xba
            || self.clz_read[3] != 0xbe
        {
            panic!(
                "Input file {} does not have correct magic number",
                self.filename
            );
        }

        self.minor = ((self.clz_read[4] as u16) << 8) + self.clz_read[5] as u16;
        self.major = ((self.clz_read[6] as u16) << 8) + self.clz_read[7] as u16;
        self.pool_item_count = ((self.clz_read[8] as u16) << 8) + self.clz_read[9] as u16;
    }

    fn parse_constant_pool(&mut self) -> () {
        self.current = 10;
        dbg!("Pool size:");
        dbg!(self.get_pool_size());
        self.cp_entries.resize(
            (self.pool_item_count as usize) + 1,
            CpEntry::integer { val: 0 },
        );
        let mut current_cp = 1;
        while current_cp < self.pool_item_count {
            let tag = self.clz_read[self.current];
            dbg!(tag);
            dbg!(current_cp);
            self.current += 1;
            let item = match tag {
                CP_UTF8 => {
                    dbg!("Parsing a utf8");
                    let b1 = self.clz_read[self.current];
                    let b2 = self.clz_read[self.current + 1];
                    self.current += 2;

                    let len = ((b1 as u16) << 8) + b2 as u16;

                    let mut buf = vec![];
                    let mut chunk = self.clz_read[self.current..].take(len as u64);
                    match chunk.read_to_end(&mut buf) {
                        Ok(v) => {
                            self.current += len as usize;

                            let str_c = match str::from_utf8(&buf) {
                                Ok(v) => v,
                                Err(e) => panic!("{}", e),
                            }
                            .to_owned();
                            dbg!(str_c.clone());
                            CpEntry::utf8 { val: str_c }
                        }
                        Err(e) => panic!("error parsing constant pool: {:?}", e),
                    }
                }
                CP_INTEGER => {
                    let b1 = self.clz_read[self.current];
                    let b2 = self.clz_read[self.current + 1];
                    let b3 = self.clz_read[self.current + 2];
                    let b4 = self.clz_read[self.current + 3];
                    self.current += 4;

                    let buf = &[b1, b2, b3, b4];
                    CpEntry::integer {
                        val: BigEndian::read_i32(buf),
                    }
                }
                CP_FLOAT => {
                    let b1 = self.clz_read[self.current];
                    let b2 = self.clz_read[self.current + 1];
                    let b3 = self.clz_read[self.current + 2];
                    let b4 = self.clz_read[self.current + 3];
                    self.current += 4;

                    let buf = &[b1, b2, b3, b4];
                    CpEntry::float {
                        val: BigEndian::read_f32(buf),
                    }
                }

                CP_LONG => {
                    let b1 = self.clz_read[self.current];
                    let b2 = self.clz_read[self.current + 1];
                    let b3 = self.clz_read[self.current + 2];
                    let b4 = self.clz_read[self.current + 3];
                    let b5 = self.clz_read[self.current + 4];
                    let b6 = self.clz_read[self.current + 5];
                    let b7 = self.clz_read[self.current + 6];
                    let b8 = self.clz_read[self.current + 7];
                    self.current += 8;
                    // Longs are double width
                    current_cp += 1;

                    let buf = &[b1, b2, b3, b4, b5, b6, b7, b8];
                    CpEntry::long {
                        val: BigEndian::read_i64(buf),
                    }
                }

                CP_DOUBLE => {
                    let b1 = self.clz_read[self.current];
                    let b2 = self.clz_read[self.current + 1];
                    let b3 = self.clz_read[self.current + 2];
                    let b4 = self.clz_read[self.current + 3];
                    let b5 = self.clz_read[self.current + 4];
                    let b6 = self.clz_read[self.current + 5];
                    let b7 = self.clz_read[self.current + 6];
                    let b8 = self.clz_read[self.current + 7];
                    self.current += 8;
                    // Doubles are double width
                    current_cp += 1;

                    let buf = &[b1, b2, b3, b4, b5, b6, b7, b8];
                    CpEntry::double {
                        val: BigEndian::read_f64(buf),
                    }
                }
                CP_CLASS => {
                    // println!("Parsing a class");
                    let b1 = self.clz_read[self.current];
                    let b2 = self.clz_read[self.current + 1];
                    self.current += 2;

                    CpEntry::class {
                        idx: ((b1 as u16) << 8) + b2 as u16,
                    }
                }
                CP_STRING => {
                    let b1 = self.clz_read[self.current];
                    let b2 = self.clz_read[self.current + 1];
                    self.current += 2;

                    CpEntry::string {
                        idx: ((b1 as u16) << 8) + b2 as u16,
                    }
                }
                CP_FIELDREF => {
                    // println!("Parsing a fieldref");
                    let b1 = self.clz_read[self.current];
                    let b2 = self.clz_read[self.current + 1];
                    let b3 = self.clz_read[self.current + 2];
                    let b4 = self.clz_read[self.current + 3];
                    self.current += 4;

                    CpEntry::fieldref {
                        clz_idx: ((b1 as u16) << 8) + b2 as u16,
                        nt_idx: ((b3 as u16) << 8) + b4 as u16,
                    }
                }
                CP_METHODREF => {
                    // println!("Parsing a methodref");
                    let b1 = self.clz_read[self.current];
                    let b2 = self.clz_read[self.current + 1];
                    let b3 = self.clz_read[self.current + 2];
                    let b4 = self.clz_read[self.current + 3];
                    self.current += 4;

                    CpEntry::methodref {
                        clz_idx: ((b1 as u16) << 8) + b2 as u16,
                        nt_idx: ((b3 as u16) << 8) + b4 as u16,
                    }
                }
                CP_INTERFACE_METHODREF => {
                    // println!("Parsing an interface_methodref");
                    let b1 = self.clz_read[self.current];
                    let b2 = self.clz_read[self.current + 1];
                    let b3 = self.clz_read[self.current + 2];
                    let b4 = self.clz_read[self.current + 3];
                    self.current += 4;

                    CpEntry::interface_methodref {
                        clz_idx: ((b1 as u16) << 8) + b2 as u16,
                        nt_idx: ((b3 as u16) << 8) + b4 as u16,
                    }
                }
                CP_NAMEANDTYPE => {
                    // println!("Parsing a name_and_type");
                    let b1 = self.clz_read[self.current];
                    let b2 = self.clz_read[self.current + 1];
                    let b3 = self.clz_read[self.current + 2];
                    let b4 = self.clz_read[self.current + 3];
                    self.current += 4;

                    CpEntry::name_and_type {
                        name_idx: ((b1 as u16) << 8) + b2 as u16,
                        type_idx: ((b3 as u16) << 8) + b4 as u16,
                    }
                }
                _ => panic!("Unsupported Constant Pool type {} at {}", tag, self.current),
            };
            self.cp_entries[current_cp as usize] = item;
            current_cp += 1;
        }
    }

    fn parse_basic_type_info(&mut self) -> () {
        self.flags =
            ((self.clz_read[self.current] as u16) << 8) + self.clz_read[self.current + 1] as u16;
        self.cp_index_this = ((self.clz_read[self.current + 2] as u16) << 8)
            + self.clz_read[self.current + 3] as u16;
        self.cp_index_super = ((self.clz_read[self.current + 4] as u16) << 8)
            + self.clz_read[self.current + 5] as u16;
        let count = ((self.clz_read[self.current + 6] as u16) << 8)
            + self.clz_read[self.current + 7] as u16;
        self.current += 8;

        for _i in 0..count {
            self.interfaces.push(
                ((self.clz_read[self.current] as u16) << 8)
                    + self.clz_read[self.current + 1] as u16,
            );
            self.current += 2;
        }
    }

    fn parse_fields(&mut self) -> () {
        let f_count =
            ((self.clz_read[self.current] as u16) << 8) + self.clz_read[self.current + 1] as u16;
        self.current += 2;

        for _idx in 0..f_count {
            let f_flags = ((self.clz_read[self.current] as u16) << 8)
                + self.clz_read[self.current + 1] as u16;
            let name_idx = ((self.clz_read[self.current + 2] as u16) << 8)
                + self.clz_read[self.current + 3] as u16;
            let desc_idx = ((self.clz_read[self.current + 4] as u16) << 8)
                + self.clz_read[self.current + 5] as u16;
            let attr_count = ((self.clz_read[self.current + 6] as u16) << 8)
                + self.clz_read[self.current + 7] as u16;
            self.current += 8;

            let f_name = match &self.cp_entries[name_idx as usize] {
                CpEntry::utf8 { val: s } => s,
                _ => panic!(
                    "Name index {} does not point at utf8 string in constant pool",
                    name_idx
                ),
            };
            let f_desc = match &self.cp_entries[desc_idx as usize] {
                CpEntry::utf8 { val: s } => s,
                _ => panic!(
                    "Desc index {} does not point at utf8 string in constant pool",
                    desc_idx
                ),
            };

            // NOTE: have just thrashed about to get the borrow checker to shut up here... need to revisit
            let k_name = &self.klass_name();
            let f = OtField::of(
                k_name.to_string(),
                f_name.to_string(),
                f_desc.to_string(),
                f_flags,
                name_idx,
                desc_idx,
            );
            for aidx in 0..attr_count {
                f.set_attr(aidx, self.parse_field_attribute(&f));
            }
            self.fields.push(f);
        }
    }

    fn parse_field_attribute(&mut self, field: &OtField) -> CpAttr {
        let name_idx =
            ((self.clz_read[self.current] as u16) << 8) + self.clz_read[self.current + 1] as u16;
        let b1 = self.clz_read[self.current + 2];
        let b2 = self.clz_read[self.current + 3];
        let b3 = self.clz_read[self.current + 4];
        let b4 = self.clz_read[self.current + 5];
        self.current += 6;

        let buf = &[b1, b2, b3, b4];
        // Fix me - is this actually u32 (check spec)
        let attr_len = BigEndian::read_u32(buf);
        let end_index = self.current + attr_len as usize;

        let s = self.stringref_from_cp(name_idx).as_str();

        // The attributes defined by this spec as appearing in the attributes table of a field_info structure are:
        //
        // * ConstantValue (§4.7.2),
        // * Synthetic (§4.7.8),
        // * Signature (§4.7.9),
        // * Deprecated (§4.7.15),
        // * RuntimeVisibleAnnotations (§4.7.16)
        // * RuntimeInvisibleAnnotations (§4.7.17).
        match s {
            // FIXME: Actually parse these instead of skipping
            "ConstantValue" => self.current += 2,
            "Signature" => self.current += 2,
            _ => panic!("Unsupported attribute {} seen on {}", s, field),
        }

        if self.current != end_index {
            panic!(
                "Inconsistent attribute index seen at {}, expected position {}",
                self.current, end_index
            )
        }
        CpAttr::of(name_idx)
    }

    fn parse_methods(&mut self) -> () {
        let mcount =
            ((self.clz_read[self.current] as u16) << 8) + self.clz_read[self.current + 1] as u16;
        self.current += 2;

        for _idx in 0..mcount {
            let mflags = ((self.clz_read[self.current] as u16) << 8)
                + self.clz_read[self.current + 1] as u16;
            let name_idx = ((self.clz_read[self.current + 2] as u16) << 8)
                + self.clz_read[self.current + 3] as u16;
            let desc_idx = ((self.clz_read[self.current + 4] as u16) << 8)
                + self.clz_read[self.current + 5] as u16;
            let attr_count = ((self.clz_read[self.current + 6] as u16) << 8)
                + self.clz_read[self.current + 7] as u16;
            self.current += 8;

            let m_name = match &self.cp_entries[name_idx as usize] {
                CpEntry::utf8 { val: s } => s,
                _ => panic!(
                    "Name index {} does not point at utf8 string in constant pool",
                    name_idx
                ),
            };
            let m_desc = match &self.cp_entries[desc_idx as usize] {
                CpEntry::utf8 { val: s } => s,
                _ => panic!(
                    "Desc index {} does not point at utf8 string in constant pool",
                    desc_idx
                ),
            };

            // NOTE: have just thrashed about to get the borrow checker to shut up here... need to revisit
            let k_name = &self.klass_name();
            let mut m = OtMethod::of(
                k_name.to_string(),
                m_name.to_string(),
                m_desc.to_string(),
                mflags,
                name_idx,
                desc_idx,
            );
            for aidx in 0..attr_count {
                dbg!(aidx);
                let att = self.parse_method_attribute(&mut m);
                m.set_attr(aidx, att.clone());
            }
            self.methods.push(m);
        }
    }

    fn parse_method_attribute(&mut self, method: &mut OtMethod) -> CpAttr {
        let name_idx =
            ((self.clz_read[self.current] as u16) << 8) + self.clz_read[self.current + 1] as u16;
        let b1 = self.clz_read[self.current + 2];
        let b2 = self.clz_read[self.current + 3];
        let b3 = self.clz_read[self.current + 4];
        let b4 = self.clz_read[self.current + 5];
        self.current += 6;

        let buf = &[b1, b2, b3, b4];
        // FIXME - is this actually a u32 (check spec)
        let attr_len = BigEndian::read_u32(buf);
        let end_index = self.current + attr_len as usize;
        dbg!(attr_len);

        let s = self.stringref_from_cp(name_idx);
        match s.as_str() {
            "Code" => {
                //    u2 max_stack;
                //    u2 max_locals;
                //    FIXME: Currently Don't care about stack depth or locals
                self.current += 4;
                // //    u4 code_length;
                // //    u1 code[code_length];
                let b1 = self.clz_read[self.current];
                let b2 = self.clz_read[self.current + 1];
                let b3 = self.clz_read[self.current + 2];
                let b4 = self.clz_read[self.current + 3];
                self.current += 4;

                let buf = &[b1, b2, b3, b4];
                // FIXME: Is this actually u32?
                let code_len = BigEndian::read_u32(buf);

                let mut bytecode = vec![];
                let mut chunk = self.clz_read[self.current..].take(code_len as u64);

                match chunk.read_to_end(&mut bytecode) {
                    Ok(v) => {
                        self.current += code_len as usize;
                        method.set_code(bytecode);
                    }
                    Err(e) => panic!("error parsing file: {:?}", e),
                };
            }
            "Signature" => {
                dbg!("Encountered signature in bytecode - skipping");
                ()
            }
            //    u2 exception_table_length;
            //    {   u2 start_pc;
            //        u2 end_pc;
            //        u2 handler_pc;
            //        u2 catch_type;
            //    } exception_table[exception_table_length];
            //    u2 attributes_count;
            //    attribute_info attributes[attributes_count];
            "Exceptions" => {
                dbg!("Encountered exception handlers in bytecode - skipping");
                ()
            }
            "Deprecated" => {
                dbg!("Encountered Deprecated attribute in bytecode - skipping");
                ()
            }
            "RuntimeVisibleAnnotations" => {
                dbg!("Encountered RuntimeVisibleAnnotations attribute in bytecode - skipping");
                ()
            }
            _ => panic!("Unsupported attribute {} seen on {}", s, method),
        };
        // HACK HACK FIX THIS
        self.current = end_index;

        // if self.current != end_index {
        //     panic!(
        //         "Inconsistent attribute index seen at {}, expected position {} in method",
        //         self.current, end_index
        //     )
        // }
        CpAttr::of(name_idx)
    }

    //         int nameCPIdx = ((int) clzBytes[current++] << 8) + (int) clzBytes[current++];
    //         int attrLen = ((int) clzBytes[current++] << 24) + ((int) clzBytes[current++] << 16) + ((int) clzBytes[current++] << 8) + (int) clzBytes[current++];
    //         int endIndex = current + attrLen;

    //         // Now check to see what type of attribute it is...
    //         String s = getCPEntry(nameCPIdx).getStr();

    //         // E.g. for fields....
    //         //
    // //        The attributes defined by this specification as appearing in the attributes table of a field_info structure are ConstantValue (§4.7.2), Synthetic (§4.7.8), Signature (§4.7.9), Deprecated (§4.7.15), RuntimeVisibleAnnotations (§4.7.16) and RuntimeInvisibleAnnotations (§4.7.17).
    //         // FIXME
    //         switch (s) {
    //             case "ConstantValue":
    //                 if (b instanceof CPMethod) {
    //                     CPMethod m = (CPMethod) b;
    //                     String methDesc = resolveAsString(m.nameIndex) + ":" + resolveAsString(m.descIndex);
    //                     throw new IllegalArgumentException("Method " + methDesc + " cannot be a constant");
    //                 }
    //                 // FIXME
    //                 current += 2;
    //                 break;
    //             case "Code":
    //                 if (b instanceof CPField) {
    //                     CPField f = (CPField) b;
    //                     String fieldDesc = resolveAsString(f.nameIndex) + ":" + resolveAsString(f.descIndex);
    //                     throw new IllegalArgumentException("Field " + fieldDesc + " cannot contain code");
    //                 }
    //                 final CPMethod m = (CPMethod) b;
    // //    u2 max_stack;
    // //    u2 max_locals;
    //                 // Don't care about stack depth or locals
    //                 current += 4;
    // //    u4 code_length;
    // //    u1 code[code_length];
    //                 int codeLen = ((int) clzBytes[current++] << 24) + ((int) clzBytes[current++] << 16) + ((int) clzBytes[current++] << 8) + (int) clzBytes[current++];
    //                 byte[] bytecode = Arrays.copyOfRange(clzBytes, current, current + codeLen);
    //                 m.setBytecode(bytecode);
    // //    u2 exception_table_length;
    // //    {   u2 start_pc;
    // //        u2 end_pc;
    // //        u2 handler_pc;
    // //        u2 catch_type;
    // //    } exception_table[exception_table_length];
    // //    u2 attributes_count;
    // //    attribute_info attributes[attributes_count];
    //                 break;
    //             case "Exceptions":
    //                 System.err.println("Encountered exception handlers in bytecode - skipping");
    //                 break;
    //             default:
    //                 throw new IllegalArgumentException("Input file has unhandled Attribute type: " + s);
    //         }
    //         // Skip to the end
    //         current = endIndex;

    //         return new CPAttr(nameCPIdx);
}
