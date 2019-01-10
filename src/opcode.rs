pub mod Opcode {

    const ACONST_NULL: u8 = 0x01;
    const ALOAD: u8 = 0x19;
    const ALOAD_0: u8 = 0x2a;
    const ALOAD_1: u8 = 0x2b;
    const ARETURN: u8 = 0xb0;
    const ASTORE: u8 = 0x53;
    const ASTORE_0: u8 = 0x4b;
    const ASTORE_1: u8 = 0x4c;
    const BIPUSH: u8 = 0x10;
    const BREAKPOINT: u8 = 0xca;
    const DADD: u8 = 0x63;
    const DCONST_0: u8 = 0x0e;
    const DCONST_1: u8 = 0x0f;
    const DLOAD: u8 = 0x18;
    const DLOAD_0: u8 = 0x26;
    const DLOAD_1: u8 = 0x27;
    const DLOAD_2: u8 = 0x28;
    const DLOAD_3: u8 = 0x29;
    const DRETURN: u8 = 0xaf;
    const DSTORE: u8 = 0x39;
    const DSTORE_0: u8 = 0x47;
    const DSTORE_1: u8 = 0x48;
    const DSTORE_2: u8 = 0x49;
    const DSTORE_3: u8 = 0x4a;
    const DSUB: u8 = 0x67;
    const DUP: u8 = 0x59;
    const DUP_X1: u8 = 0x5a;
    const GETFIELD: u8 = 0xb4;
    const GETSTATIC: u8 = 0xb2;
    const GOTO: u8 = 0xa7;
    const I2D: u8 = 0x87;
    const IADD: u8 = 0x60;
    const IAND: u8 = 0x7e;
    const ICONST_M1: u8 = 0x02;
    const ICONST_0: u8 = 0x03;
    const ICONST_1: u8 = 0x04;
    const ICONST_2: u8 = 0x05;
    const ICONST_3: u8 = 0x06;
    const ICONST_4: u8 = 0x07;
    const ICONST_5: u8 = 0x08;
    const IDIV: u8 = 0x6c;
    const IF_ICMPEQ: u8 = 0x9f;
    const IFEQ: u8 = 0x99;
    const IFGE: u8 = 0x9c;
    const IFGT: u8 = 0x9d;
    const IFLE: u8 = 0x9e;
    const IFLT: u8 = 0x9b;
    const IFNE: u8 = 0x9a;
    const IFNONNULL: u8 = 0xc7;
    const IFNULL: u8 = 0xc6;
    const IINC: u8 = 0x84;
    const ILOAD: u8 = 0x15;
    const ILOAD_0: u8 = 0x1a;
    const ILOAD_1: u8 = 0x1b;
    const ILOAD_2: u8 = 0x1c;
    const ILOAD_3: u8 = 0x1d;
    const IMPDEP1: u8 = 0xfe;
    const IMPDEP2: u8 = 0xff;
    const IMUL: u8 = 0x68;
    const INEG: u8 = 0x74;
    const INVOKESPECIAL: u8 = 0xb7;
    const INVOKESTATIC: u8 = 0xb8;
    const INVOKEVIRTUAL: u8 = 0xb6;
    const IOR: u8 = 0x80;
    const IREM: u8 = 0x70;
    const IRETURN: u8 = 0xac;
    const ISTORE: u8 = 0x36;
    const ISTORE_0: u8 = 0x3b;
    const ISTORE_1: u8 = 0x3c;
    const ISTORE_2: u8 = 0x3d;
    const ISTORE_3: u8 = 0x3e;
    const ISUB: u8 = 0x64;
    const MONITORENTER: u8 = 0xc2;
    const MONITOREXIT: u8 = 0xc3;
    const NEW: u8 = 0xbb;
    const JSR: u8 = 0xa8;
    const JSR_W: u8 = 0xc9;
    const LDC: u8 = 0x12;
    const NOP: u8 = 0x00;
    const POP: u8 = 0x57;
    const POP2: u8 = 0x58;
    const PUTFIELD: u8 = 0xb5;
    const PUTSTATIC: u8 = 0xb3;
    const RET: u8 = 0xa9;
    const RETURN: u8 = 0xb1;
    const SIPUSH: u8 = 0x11;
    const SWAP: u8 = 0x5f;

    impl Opcode {
        fn b(&self) -> u8 {
            match *self {
                Opcode::ACONST_NULL => 0x01,
                Opcode::ALOAD => 0x19,
                Opcode::ALOAD_0 => 0x2a,
                Opcode::ALOAD_1 => 0x2b,
                Opcode::ARETURN => 0xb0,
                Opcode::ASTORE => 0x53,
                Opcode::ASTORE_0 => 0x4b,
                Opcode::ASTORE_1 => 0x4c,
                Opcode::BIPUSH => 0x10,
                Opcode::BREAKPOINT => 0xca,
                Opcode::DADD => 0x63,
                Opcode::DCONST_0 => 0x0e,
                Opcode::DCONST_1 => 0x0f,
                Opcode::DLOAD => 0x18,
                Opcode::DLOAD_0 => 0x26,
                Opcode::DLOAD_1 => 0x27,
                Opcode::DLOAD_2 => 0x28,
                Opcode::DLOAD_3 => 0x29,
                Opcode::DRETURN => 0xaf,
                Opcode::DSTORE => 0x39,
                Opcode::DSTORE_0 => 0x47,
                Opcode::DSTORE_1 => 0x48,
                Opcode::DSTORE_2 => 0x49,
                Opcode::DSTORE_3 => 0x4a,
                Opcode::DSUB => 0x67,
                Opcode::DUP => 0x59,
                Opcode::DUP_X1 => 0x5a,
                Opcode::GETFIELD => 0xb4,
                Opcode::GETSTATIC => 0xb2,
                Opcode::GOTO => 0xa7,
                Opcode::I2D => 0x87,
                Opcode::IADD => 0x60,
                Opcode::IAND => 0x7e,
                Opcode::ICONST_M1 => 0x02,
                Opcode::ICONST_0 => 0x03,
                Opcode::ICONST_1 => 0x04,
                Opcode::ICONST_2 => 0x05,
                Opcode::ICONST_3 => 0x06,
                Opcode::ICONST_4 => 0x07,
                Opcode::ICONST_5 => 0x08,
                Opcode::IDIV => 0x6c,
                Opcode::IF_ICMPEQ => 0x9f,
                Opcode::IFEQ => 0x99,
                Opcode::IFGE => 0x9c,
                Opcode::IFGT => 0x9d,
                Opcode::IFLE => 0x9e,
                Opcode::IFLT => 0x9b,
                Opcode::IFNE => 0x9a,
                Opcode::IFNONNULL => 0xc7,
                Opcode::IFNULL => 0xc6,
                Opcode::IINC => 0x84,
                Opcode::ILOAD => 0x15,
                Opcode::ILOAD_0 => 0x1a,
                Opcode::ILOAD_1 => 0x1b,
                Opcode::ILOAD_2 => 0x1c,
                Opcode::ILOAD_3 => 0x1d,
                Opcode::IMPDEP1 => 0xfe,
                Opcode::IMPDEP2 => 0xff,
                Opcode::IMUL => 0x68,
                Opcode::INEG => 0x74,
                Opcode::INVOKESPECIAL => 0xb7,
                Opcode::INVOKESTATIC => 0xb8,
                Opcode::INVOKEVIRTUAL => 0xb6,
                Opcode::IOR => 0x80,
                Opcode::IREM => 0x70,
                Opcode::IRETURN => 0xac,
                Opcode::ISTORE => 0x36,
                Opcode::ISTORE_0 => 0x3b,
                Opcode::ISTORE_1 => 0x3c,
                Opcode::ISTORE_2 => 0x3d,
                Opcode::ISTORE_3 => 0x3e,
                Opcode::ISUB => 0x64,
                Opcode::MONITORENTER => 0xc2,
                Opcode::MONITOREXIT => 0xc3,
                Opcode::NEW => 0xbb,
                Opcode::JSR => 0xa8,
                Opcode::JSR_W => 0xc9,
                Opcode::LDC => 0x12,
                Opcode::NOP => 0x00,
                Opcode::POP => 0x57,
                Opcode::POP2 => 0x58,
                Opcode::PUTFIELD => 0xb5,
                Opcode::PUTSTATIC => 0xb3,
                Opcode::RET => 0xa9,
                Opcode::RETURN => 0xb1,
                Opcode::SIPUSH => 0x11,
                Opcode::SWAP => 0x5f,
            }
        }

        fn num_params(&self) -> u8 {
            match *self {
                Opcode::ALOAD => 1,
                Opcode::ASTORE => 1,
                Opcode::BIPUSH => 1,
                Opcode::DLOAD => 1,
                Opcode::DSTORE => 1,
                Opcode::GETFIELD => 2,
                Opcode::GETSTATIC => 2,
                Opcode::GOTO => 2,
                Opcode::IF_ICMPEQ => 2,
                Opcode::IFEQ => 2,
                Opcode::IFGE => 2,
                Opcode::IFGT => 2,
                Opcode::IFLE => 2,
                Opcode::IFLT => 2,
                Opcode::IFNE => 2,
                Opcode::IFNONNULL => 2,
                Opcode::IFNULL => 2,
                Opcode::IINC => 2,
                Opcode::ILOAD => 1,
                Opcode::INVOKESPECIAL => 2,
                Opcode::INVOKESTATIC => 2,
                Opcode::INVOKEVIRTUAL => 2,
                Opcode::ISTORE => 1,
                Opcode::NEW => 2,
                Opcode::JSR => 2,
                Opcode::JSR_W => 2,
                Opcode::LDC => 1,
                Opcode::PUTFIELD => 2,
                Opcode::PUTSTATIC => 2,
                Opcode::RET => 1,
                Opcode::SIPUSH => 2,
                _ => 0,
            }
        }
    }
}

pub enum JVMValue {
    Boolean { val: bool },
    Byte { val: i8 },
    Short { val: i16 },
    Int { val: i32 },
    Long { val: i64 },
    Float { val: f32 },
    Double { val: f64 },
    Char,
    ObjRef,
}

impl JVMValue {
    fn name(&self) -> char {
        match *self {
            JVMValue::Boolean { val } => 'Z',
            JVMValue::Byte { val } => 'B',
            JVMValue::Short { val } => 'S',
            JVMValue::Int { val } => 'I',
            JVMValue::Long { val } => 'J',
            JVMValue::Float { val } => 'F',
            JVMValue::Double { val } => 'D',
            JVMValue::Char => 'C',
            JVMValue::ObjRef => 'A',
        }
    }
}
