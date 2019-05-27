pub mod Opcode {

    pub const BIPUSH: u8 = 0x10;
    pub const BREAKPOINT: u8 = 0xca;
    pub const DUP: u8 = 0x59;
    pub const DUP_X1: u8 = 0x5a;
    pub const IADD: u8 = 0x60;
    pub const IAND: u8 = 0x7e;
    pub const ICONST_M1: u8 = 0x02;
    pub const ICONST_0: u8 = 0x03;
    pub const ICONST_1: u8 = 0x04;
    pub const ICONST_2: u8 = 0x05;
    pub const ICONST_3: u8 = 0x06;
    pub const ICONST_4: u8 = 0x07;
    pub const ICONST_5: u8 = 0x08;
    pub const IDIV: u8 = 0x6c;
    pub const IINC: u8 = 0x84;
    pub const IMPDEP1: u8 = 0xfe;
    pub const IMPDEP2: u8 = 0xff;
    pub const IMUL: u8 = 0x68;
    pub const INEG: u8 = 0x74;
    pub const IOR: u8 = 0x80;
    pub const IREM: u8 = 0x70;
    pub const IRETURN: u8 = 0xac;
    pub const ISUB: u8 = 0x64;
    pub const JSR: u8 = 0xa8;
    pub const JSR_W: u8 = 0xc9;
    pub const NOP: u8 = 0x00;
    pub const POP: u8 = 0x57;
    pub const POP2: u8 = 0x58;
    pub const RET: u8 = 0xa9;
    pub const RETURN: u8 = 0xb1;
    pub const SIPUSH: u8 = 0x11;
    pub const SWAP: u8 = 0x5f;

    fn num_params(c: u8) -> u8 {
        match c {
            BIPUSH => 1,
            IINC => 2,
            RET => 1,
            SIPUSH => 2,
            _ => 0,
        }
    }
}
