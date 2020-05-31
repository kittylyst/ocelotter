// AALOAD 0x32
// AASTORE 0x53
pub const ACONST_NULL: u8 = 0x01;
pub const ALOAD: u8 = 0x19;
pub const ALOAD_0: u8 = 0x2a;
pub const ALOAD_1: u8 = 0x2b;
pub const ALOAD_2: u8 = 0x2c;
pub const ALOAD_3: u8 = 0x2d;
// ANEWARRAY 0xbd
pub const ARETURN: u8 = 0xb0;
// ARRAYLENGTH 0xbe
pub const ASTORE: u8 = 0x53;
pub const ASTORE_0: u8 = 0x4b;
pub const ASTORE_1: u8 = 0x4c;
pub const ASTORE_2: u8 = 0x4d;
pub const ASTORE_3: u8 = 0x4e;
// ATHROW 0xbf
// BALOAD 0x33
// BASTORE 0x54
pub const BIPUSH: u8 = 0x10;
pub const BREAKPOINT: u8 = 0xca;
// CALOAD 0x34
// CASTORE 0x55
// CHECKCAST 0xc0
pub const D2F: u8 = 0x90;
pub const D2I: u8 = 0x8e;
pub const D2L: u8 = 0x8f;
pub const DADD: u8 = 0x63;
// DALOAD 0x31
// DASTORE 0x52
pub const DCMPG: u8 = 0x98;
pub const DCMPL: u8 = 0x97;
pub const DCONST_0: u8 = 0x0e;
pub const DCONST_1: u8 = 0x0f;
pub const DDIV: u8 = 0x6f;
pub const DLOAD: u8 = 0x18;
pub const DLOAD_0: u8 = 0x26;
pub const DLOAD_1: u8 = 0x27;
pub const DLOAD_2: u8 = 0x28;
pub const DLOAD_3: u8 = 0x29;
pub const DMUL: u8 = 0x6b;
pub const DNEG: u8 = 0x77;
// DREM 0x73
pub const DRETURN: u8 = 0xaf;
pub const DSTORE: u8 = 0x39;
pub const DSTORE_0: u8 = 0x47;
pub const DSTORE_1: u8 = 0x48;
pub const DSTORE_2: u8 = 0x49;
pub const DSTORE_3: u8 = 0x4a;
pub const DSUB: u8 = 0x67;
pub const DUP: u8 = 0x59;
pub const DUP_X1: u8 = 0x5a;
// DUP_X2 0x5b
// DUP2 0x5c
// DUP2_X1 0x5d
// DUP2_X2 0x5e
pub const F2D: u8 = 0x8d;
// F2I 0x8b
// F2L 0x8c
pub const FADD: u8 = 0x62;
// FALOAD 0x30
// FASTORE 0x51
pub const FCMPG: u8 = 0x96;
pub const FCMPL: u8 = 0x95;
pub const FCONST_0: u8 = 0x0b;
pub const FCONST_1: u8 = 0x0c;
pub const FCONST_2: u8 = 0x0d;
pub const FDIV: u8 = 0x6e;
pub const FLOAD: u8 = 0x17;
pub const FLOAD_0: u8 = 0x22;
pub const FLOAD_1: u8 = 0x23;
pub const FLOAD_2: u8 = 0x24;
pub const FLOAD_3: u8 = 0x25;
pub const FMUL: u8 = 0x6a;
pub const FNEG: u8 = 0x76;
// FREM 0x72
pub const FRETURN: u8 = 0xae;
pub const FSTORE: u8 = 0x38;
pub const FSTORE_0: u8 = 0x43;
pub const FSTORE_1: u8 = 0x44;
pub const FSTORE_2: u8 = 0x45;
pub const FSTORE_3: u8 = 0x46;
pub const FSUB: u8 = 0x66;
pub const GETFIELD: u8 = 0xb4;
pub const GETSTATIC: u8 = 0xb2;
pub const GOTO: u8 = 0xa7;
pub const GOTO_W: u8 = 0xc8;
// I2B 0x91
// I2C 0x92
pub const I2D: u8 = 0x87;
// I2F 0x86
pub const I2L: u8 = 0x85;
// I2S 0x93
pub const IADD: u8 = 0x60;
pub const IALOAD: u8 = 0x2e;
pub const IAND: u8 = 0x7e;
pub const IASTORE: u8 = 0x4f;
pub const ICONST_M1: u8 = 0x02;
pub const ICONST_0: u8 = 0x03;
pub const ICONST_1: u8 = 0x04;
pub const ICONST_2: u8 = 0x05;
pub const ICONST_3: u8 = 0x06;
pub const ICONST_4: u8 = 0x07;
pub const ICONST_5: u8 = 0x08;
pub const IDIV: u8 = 0x6c;
// IF_ACMPEQ 0xa5
// IF_ACMPNE 0xa6
pub const IF_ICMPEQ: u8 = 0x9f;
pub const IF_ICMPGE: u8 = 0xa2;
pub const IF_ICMPGT: u8 = 0xa3;
pub const IF_ICMPLE: u8 = 0xa4;
pub const IF_ICMPLT: u8 = 0xa1;
pub const IF_ICMPNE: u8 = 0xa0;
pub const IFEQ: u8 = 0x99;
pub const IFGE: u8 = 0x9c;
pub const IFGT: u8 = 0x9d;
pub const IFLE: u8 = 0x9e;
pub const IFLT: u8 = 0x9b;
pub const IFNE: u8 = 0x9a;
pub const IFNONNULL: u8 = 0xc7;
pub const IFNULL: u8 = 0xc6;
pub const IINC: u8 = 0x84;
pub const ILOAD: u8 = 0x15;
pub const ILOAD_0: u8 = 0x1a;
pub const ILOAD_1: u8 = 0x1b;
pub const ILOAD_2: u8 = 0x1c;
pub const ILOAD_3: u8 = 0x1d;
pub const IMPDEP1: u8 = 0xfe;
pub const IMPDEP2: u8 = 0xff;
pub const IMUL: u8 = 0x68;
pub const INEG: u8 = 0x74;
// INSTANCEOF 0xc1
// INVOKEDYNAMIC 0xba
// INVOKEINTERFACE 0xb9
pub const INVOKESPECIAL: u8 = 0xb7;
pub const INVOKESTATIC: u8 = 0xb8;
pub const INVOKEVIRTUAL: u8 = 0xb6;
pub const IOR: u8 = 0x80;
pub const IREM: u8 = 0x70;
pub const IRETURN: u8 = 0xac;
pub const ISHL: u8 = 0x78;
pub const ISHR: u8 = 0x7a;
pub const ISTORE: u8 = 0x36;
pub const ISTORE_0: u8 = 0x3b;
pub const ISTORE_1: u8 = 0x3c;
pub const ISTORE_2: u8 = 0x3d;
pub const ISTORE_3: u8 = 0x3e;
pub const ISUB: u8 = 0x64;
// IUSHR 0x7c
pub const IXOR: u8 = 0x82;
pub const JSR: u8 = 0xa8;
pub const JSR_W: u8 = 0xc9;
// L2D 0x8a
// L2F 0x89
pub const L2I: u8 = 0x88;
pub const LADD: u8 = 0x61;
// LALOAD 0x2f
pub const LAND: u8 = 0x7f;
// LASTORE 0x50
pub const LCMP: u8 = 0x94;
pub const LCONST_0: u8 = 0x09;
pub const LCONST_1: u8 = 0x0a;
pub const LDC: u8 = 0x12;
pub const LDC2_W: u8 = 0x14;
pub const LDIV: u8 = 0x6d;
pub const LLOAD: u8 = 0x16;
pub const LLOAD_0: u8 = 0x1e;
pub const LLOAD_1: u8 = 0x1f;
pub const LLOAD_2: u8 = 0x20;
pub const LLOAD_3: u8 = 0x21;
pub const LMUL: u8 = 0x69;
pub const LNEG: u8 = 0x75;
// LOOKUPSWITCH 0xab
pub const LOR: u8 = 0x81;
// LREM 0x71
pub const LRETURN: u8 = 0xad;
pub const LSHL: u8 = 0x79;
pub const LSHR: u8 = 0x7b;
pub const LSTORE: u8 = 0x37;
pub const LSTORE_0: u8 = 0x3f;
pub const LSTORE_1: u8 = 0x40;
pub const LSTORE_2: u8 = 0x41;
pub const LSTORE_3: u8 = 0x42;
pub const LSUB: u8 = 0x65;
// LUSHR 0x7d
pub const LXOR: u8 = 0x83;
pub const MONITORENTER: u8 = 0xc2;
pub const MONITOREXIT: u8 = 0xc3;
// MULTINEWARRAY 0xc5
pub const NEW: u8 = 0xbb;
pub const NEWARRAY: u8 = 0xbc;
pub const NOP: u8 = 0x00;
pub const POP: u8 = 0x57;
pub const POP2: u8 = 0x58;
pub const PUTFIELD: u8 = 0xb5;
pub const PUTSTATIC: u8 = 0xb3;
pub const RET: u8 = 0xa9;
pub const RETURN: u8 = 0xb1;
// SALOAD 0x35
// SASTORE 0x56
pub const SIPUSH: u8 = 0x11;
pub const SWAP: u8 = 0x5f;
// TABLESWITCH 0xaa
// WIDE 0xc4

// [UNUSED] 0cb - 0xfd

fn num_params(c: u8) -> u8 {
    match c {
        ALOAD => 1,
        ASTORE => 1,
        BIPUSH => 1,
        DLOAD => 1,
        DSTORE => 1,
        FLOAD => 1,
        FSTORE => 1,
        GETFIELD => 2,
        GETSTATIC => 2,
        GOTO => 2,
        IF_ICMPEQ => 2,
        IFEQ => 2,
        IFGE => 2,
        IFGT => 2,
        IFLE => 2,
        IFLT => 2,
        IFNE => 2,
        IFNONNULL => 2,
        IFNULL => 2,
        IINC => 2,
        ILOAD => 1,
        INVOKESPECIAL => 2,
        INVOKESTATIC => 2,
        INVOKEVIRTUAL => 2,
        ISTORE => 1,
        LLOAD => 1,
        LSTORE => 1,
        NEW => 2,
        NEWARRAY => 1,
        JSR => 2,
        JSR_W => 2,
        LDC => 1,
        PUTFIELD => 2,
        PUTSTATIC => 2,
        RET => 1,
        SIPUSH => 2,
        _ => 0,
    }
}
