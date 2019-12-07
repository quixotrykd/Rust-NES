use crate::memory::Memory;

pub struct Cpu<'a> {
    pub pc: u16,
    sp: u8,
    accumulator: u8,
    x: u8,
    y: u8,
    carry: bool,
    zero: bool,
    interrupt: bool,
    decimal: bool,
    brk: bool,
    overflow: bool,
    sign: bool,
    memory: &'a mut Memory,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Opcode {
    Add, And, Asl, Bcc, Bcs, Beq, Bit, Bmi, Bne, Bpl, Brk, Bvc, Bvs, Clc, Cld, Cli, Clv, Cmp, Cpx, Cpy, Dec, Dex, Dey, Eor, Inc, Inx, Iny, Jmp, Jsr, Lda, Ldx, Ldy, Lsr, Nop, Ora, Pha, Php, Pla, Plp, Rol, Ror, Rti, Rts, Sbc, Sec, Sed, Sei, Sta, Stx, Sty, Tax, Tay, Tsx, Txa, Txs, Tya,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum AddressingMode {
    ZeroPage(u8), ZeroPageX(u8), ZeroPageY(u8),
    Absolute(u16), AbsoluteX(u16), AbsoluteY(u16),
    Indirect(u16), IndirectX(u8), IndirectY(u8),
    Implicit,
    Immediate(u8),
    Relative(i8),
    Accumulator,
}

#[derive(Copy, Clone, Debug)]
pub struct Instruction {
    opcode: Opcode,
    mode: AddressingMode,
    pub cycles: u8,
    page_cross_cost: bool,
}

impl<'a> Cpu<'a> {
    pub fn new(memory: &'a mut Memory) -> Self {
        Cpu {
            pc: 0x0,
            sp: 0x0,
            accumulator: 0x0,
            x: 0x0,
            y: 0x0,
            carry: false,
            zero: false,
            interrupt: false,
            decimal: false,
            brk: false,
            overflow: false,
            sign: false,
            memory,
        }
    }

    pub fn reset(&mut self) {
        self.pc = self.memory.get_word_at(0xFFFC);
    }

    pub fn fetch_next_instruction(&mut self) -> Instruction {
        let opcode: u8 = self.memory.get_byte_at(self.pc);
        let byte_after_opcode: u8 = self.memory.get_byte_at(self.pc + 1);
        let signed_byte_after_opcode: i8 = byte_after_opcode as i8;
        let word_after_opcode: u16 = self.memory.get_word_at(self.pc + 1);

        //println!("Opcode: 0x{:02x}", opcode);
        let result = match opcode {
            /* Add */
            0x69 => Instruction {
                opcode: Opcode::Add,
                mode: AddressingMode::Immediate(byte_after_opcode),
                cycles: 2,
                page_cross_cost: false,
            },
            0x65 => Instruction {
                opcode: Opcode::Add,
                mode: AddressingMode::ZeroPage(byte_after_opcode),
                cycles: 3,
                page_cross_cost: false,
            },
            0x75 => Instruction {
                opcode: Opcode::Add,
                mode: AddressingMode::ZeroPageX(byte_after_opcode),
                cycles: 4,
                page_cross_cost: false,
            },
            0x6D => Instruction {
                opcode: Opcode::Add,
                mode: AddressingMode::Absolute(word_after_opcode),
                cycles: 4,
                page_cross_cost: false,
            },
            0x7D => Instruction {
                opcode: Opcode::Add,
                mode: AddressingMode::AbsoluteX(word_after_opcode),
                cycles: 4,
                page_cross_cost: true,
            },
            0x79 => Instruction {
                opcode: Opcode::Add,
                mode: AddressingMode::AbsoluteY(word_after_opcode),
                cycles: 4,
                page_cross_cost: true,
            },
            0x61 => Instruction {
                opcode: Opcode::Add,
                mode: AddressingMode::IndirectX(byte_after_opcode),
                cycles: 6,
                page_cross_cost: false,
            },
            0x71 => Instruction {
                opcode: Opcode::Add,
                mode: AddressingMode::IndirectY(byte_after_opcode),
                cycles: 5,
                page_cross_cost: true,
            },
            /* And */
            0x29 => Instruction {
                opcode: Opcode::And,
                mode: AddressingMode::Immediate(byte_after_opcode),
                cycles: 2,
                page_cross_cost: false,
            },
            0x25 => Instruction {
                opcode: Opcode::And,
                mode: AddressingMode::ZeroPage(byte_after_opcode),
                cycles: 3,
                page_cross_cost: false,
            },
            0x35 => Instruction {
                opcode: Opcode::And,
                mode: AddressingMode::ZeroPageX(byte_after_opcode),
                cycles: 4,
                page_cross_cost: false,
            },
            0x2D => Instruction {
                opcode: Opcode::And,
                mode: AddressingMode::Absolute(word_after_opcode),
                cycles: 4,
                page_cross_cost: false,
            },
            0x3D => Instruction {
                opcode: Opcode::And,
                mode: AddressingMode::AbsoluteX(word_after_opcode),
                cycles: 4,
                page_cross_cost: true,
            },
            0x39 => Instruction {
                opcode: Opcode::And,
                mode: AddressingMode::AbsoluteY(word_after_opcode),
                cycles: 4,
                page_cross_cost: true,
            },
            0x21 => Instruction {
                opcode: Opcode::And,
                mode: AddressingMode::IndirectX(byte_after_opcode),
                cycles: 6,
                page_cross_cost: false,
            },
            0x31 => Instruction {
                opcode: Opcode::And,
                mode: AddressingMode::IndirectY(byte_after_opcode),
                cycles: 5,
                page_cross_cost: false,
            },
            /* Asl */
            0x0A => Instruction {
                opcode: Opcode::Asl,
                mode: AddressingMode::Implicit,
                cycles: 2,
                page_cross_cost: false,
            },
            0x06 => Instruction {
                opcode: Opcode::Asl,
                mode: AddressingMode::ZeroPage(byte_after_opcode),
                cycles: 5,
                page_cross_cost: false,
            },
            0x16 => Instruction {
                opcode: Opcode::Asl,
                mode: AddressingMode::ZeroPageX(byte_after_opcode),
                cycles: 6,
                page_cross_cost: false,
            },
            0x0E => Instruction {
                opcode: Opcode::Asl,
                mode: AddressingMode::Absolute(word_after_opcode),
                cycles: 6,
                page_cross_cost: false,
            },
            0x1E => Instruction {
                opcode: Opcode::Asl,
                mode: AddressingMode::AbsoluteX(word_after_opcode),
                cycles: 7,
                page_cross_cost: false,
            },
            /* Bcc */
            0x90 => Instruction {
                opcode: Opcode::Bcc,
                mode: AddressingMode::Relative(signed_byte_after_opcode),
                cycles: 2,
                page_cross_cost: true,
            },
            /* Bcs */
            0xB0 => Instruction {
                opcode: Opcode::Bcs,
                mode: AddressingMode::Relative(signed_byte_after_opcode),
                cycles: 2,
                page_cross_cost: true,
            },
            /* Beq */
            0xF0 => Instruction {
                opcode: Opcode::Beq,
                mode: AddressingMode::Relative(signed_byte_after_opcode),
                cycles: 2,
                page_cross_cost: true,
            },
            /* Bit */
            0x24 => Instruction {
                opcode: Opcode::Bit,
                mode: AddressingMode::ZeroPage(byte_after_opcode),
                cycles: 3,
                page_cross_cost: false,
            },
            0x2C => Instruction {
                opcode: Opcode::Bit,
                mode: AddressingMode::Absolute(word_after_opcode),
                cycles: 4,
                page_cross_cost: false,
            },
            /* Bmi */
            0x30 => Instruction {
                opcode: Opcode::Bmi,
                mode: AddressingMode::Relative(signed_byte_after_opcode),
                cycles: 2,
                page_cross_cost: false, /* Is this right? */
            },
            /* Bne */
            0xD0 => Instruction {
                opcode: Opcode::Bne,
                mode: AddressingMode::Relative(signed_byte_after_opcode),
                cycles: 2,
                page_cross_cost: true,
            },
            /* Bpl */
            0x10 => Instruction {
                opcode: Opcode::Bpl,
                mode: AddressingMode::Relative(signed_byte_after_opcode),
                cycles: 2,
                page_cross_cost: true,
            },
            /* Brk */
            0x00 => Instruction {
                opcode: Opcode::Brk,
                mode: AddressingMode::Implicit,
                cycles: 7,
                page_cross_cost: false,
            },
            /* Bvc */
            0x50 => Instruction {
                opcode: Opcode::Bvc,
                mode: AddressingMode::Relative(signed_byte_after_opcode),
                cycles: 2,
                page_cross_cost: true,
            },
            /* Bvs */
            0x70 => Instruction {
                opcode: Opcode::Bvs,
                mode: AddressingMode::Relative(signed_byte_after_opcode),
                cycles: 2,
                page_cross_cost: true,
            },
            /* Clc */
            0x18 => Instruction {
                opcode: Opcode::Clc,
                mode: AddressingMode::Implicit,
                cycles: 2,
                page_cross_cost: false,
            },
            /* Cld */
            0xD8 => Instruction {
                opcode: Opcode::Cld,
                mode: AddressingMode::Implicit,
                cycles: 2,
                page_cross_cost: false,
            },
            /* Cli */
            0x58 => Instruction {
                opcode: Opcode::Cli,
                mode: AddressingMode::Implicit,
                cycles: 2,
                page_cross_cost: false,
            },
            /* Clv */
            0xB8 => Instruction {
                opcode: Opcode::Clv,
                mode: AddressingMode::Implicit,
                cycles: 2,
                page_cross_cost: false,
            },
            /* Cmp */
            0xC9 => Instruction {
                opcode: Opcode::Cmp,
                mode: AddressingMode::Immediate(byte_after_opcode),
                cycles: 2,
                page_cross_cost: false,
            },
            0xC5 => Instruction {
                opcode: Opcode::Cmp,
                mode: AddressingMode::ZeroPage(byte_after_opcode),
                cycles: 2,
                page_cross_cost: false,
            },
            0xD5 => Instruction {
                opcode: Opcode::Cmp,
                mode: AddressingMode::ZeroPageX(byte_after_opcode),
                cycles: 2,
                page_cross_cost: false,
            },
            0xCD => Instruction {
                opcode: Opcode::Cmp,
                mode: AddressingMode::Absolute(word_after_opcode),
                cycles: 4,
                page_cross_cost: false,
            },
            0xDD => Instruction {
                opcode: Opcode::Cmp,
                mode: AddressingMode::AbsoluteX(word_after_opcode),
                cycles: 4,
                page_cross_cost: true,
            },
            0xD9 => Instruction {
                opcode: Opcode::Cmp,
                mode: AddressingMode::AbsoluteY(word_after_opcode),
                cycles: 4,
                page_cross_cost: true,
            },
            0xC1 => Instruction {
                opcode: Opcode::Cmp,
                mode: AddressingMode::IndirectX(byte_after_opcode),
                cycles: 6,
                page_cross_cost: false,
            },
            0xD1 => Instruction {
                opcode: Opcode::Cmp,
                mode: AddressingMode::IndirectY(byte_after_opcode),
                cycles: 5,
                page_cross_cost: true,
            },
            /* Cpx */
            0xE0 => Instruction {
                opcode: Opcode::Cpx,
                mode: AddressingMode::Immediate(byte_after_opcode),
                cycles: 2,
                page_cross_cost: false,
            },
            0xE4 => Instruction {
                opcode: Opcode::Cpx,
                mode: AddressingMode::ZeroPage(byte_after_opcode),
                cycles: 3,
                page_cross_cost: false,
            },
            0xEC => Instruction {
                opcode: Opcode::Cpx,
                mode: AddressingMode::Absolute(word_after_opcode),
                cycles: 4,
                page_cross_cost: false,
            },
            /* Cpy */
            0xC0 => Instruction {
                opcode: Opcode::Cpy,
                mode: AddressingMode::Immediate(byte_after_opcode),
                cycles: 2,
                page_cross_cost: false,
            },
            0xC4 => Instruction {
                opcode: Opcode::Cpy,
                mode: AddressingMode::ZeroPage(byte_after_opcode),
                cycles: 3,
                page_cross_cost: false,
            },
            0xCC => Instruction {
                opcode: Opcode::Cpy,
                mode: AddressingMode::Absolute(word_after_opcode),
                cycles: 4,
                page_cross_cost: false,
            },
            /* Dec */
            0xC6 => Instruction {
                opcode: Opcode::Dec,
                mode: AddressingMode::ZeroPage(byte_after_opcode),
                cycles: 5,
                page_cross_cost: false,
            },
            0xD6 => Instruction {
                opcode: Opcode::Dec,
                mode: AddressingMode::ZeroPageX(byte_after_opcode),
                cycles: 6,
                page_cross_cost: false,
            },
            0xCE => Instruction {
                opcode: Opcode::Dec,
                mode: AddressingMode::Absolute(word_after_opcode),
                cycles: 6,
                page_cross_cost: false,
            },
            0xDE => Instruction {
                opcode: Opcode::Dec,
                mode: AddressingMode::AbsoluteX(word_after_opcode),
                cycles: 7,
                page_cross_cost: false,
            },
            /* Dex */
            0xCA => Instruction {
                opcode: Opcode::Dex,
                mode: AddressingMode::Implicit,
                cycles: 2,
                page_cross_cost: false,
            },
            /* Dey */
            0x88 => Instruction {
                opcode: Opcode::Dey,
                mode: AddressingMode::Implicit,
                cycles: 2,
                page_cross_cost: false,
            },
            /* Eor */
            0x49 => Instruction {
                opcode: Opcode::Eor,
                mode: AddressingMode::Immediate(byte_after_opcode),
                cycles: 2,
                page_cross_cost: false,
            },
            0x45 => Instruction {
                opcode: Opcode::Eor,
                mode: AddressingMode::ZeroPage(byte_after_opcode),
                cycles: 3,
                page_cross_cost: false,
            },
            0x55 => Instruction {
                opcode: Opcode::Eor,
                mode: AddressingMode::ZeroPageX(byte_after_opcode),
                cycles: 4,
                page_cross_cost: false,
            },
            0x4D => Instruction {
                opcode: Opcode::Eor,
                mode: AddressingMode::Absolute(word_after_opcode),
                cycles: 4,
                page_cross_cost: false,
            },
            0x5D => Instruction {
                opcode: Opcode::Eor,
                mode: AddressingMode::AbsoluteX(word_after_opcode),
                cycles: 4,
                page_cross_cost: true,
            },
            0x59 => Instruction {
                opcode: Opcode::Eor,
                mode: AddressingMode::AbsoluteY(word_after_opcode),
                cycles: 5,
                page_cross_cost: true,
            },
            0x41 => Instruction {
                opcode: Opcode::Eor,
                mode: AddressingMode::IndirectX(byte_after_opcode),
                cycles: 6,
                page_cross_cost: false,
            },
            0x51 => Instruction {
                opcode: Opcode::Eor,
                mode: AddressingMode::IndirectY(byte_after_opcode),
                cycles: 5,
                page_cross_cost: true,
            },
            /* Inc */
            0xE6 => Instruction {
                opcode: Opcode::Inc,
                mode: AddressingMode::ZeroPage(byte_after_opcode),
                cycles: 5,
                page_cross_cost: false,
            },
            0xF6 => Instruction {
                opcode: Opcode::Inc,
                mode: AddressingMode::ZeroPageX(byte_after_opcode),
                cycles: 6,
                page_cross_cost: false,
            },
            0xEE => Instruction {
                opcode: Opcode::Inc,
                mode: AddressingMode::Absolute(word_after_opcode),
                cycles: 6,
                page_cross_cost: false,
            },
            0xFE => Instruction {
                opcode: Opcode::Inc,
                mode: AddressingMode::AbsoluteX(word_after_opcode),
                cycles: 7,
                page_cross_cost: false,
            },
            /* Inx */
            0xE8 => Instruction {
                opcode: Opcode::Inx,
                mode: AddressingMode::Implicit,
                cycles: 2,
                page_cross_cost: false,
            },
            /* Iny */
            0xC8 => Instruction {
                opcode: Opcode::Iny,
                mode: AddressingMode::Implicit,
                cycles: 2,
                page_cross_cost: false,
            },
            /* Jmp */
            0x4C => Instruction {
                opcode: Opcode::Jmp,
                mode: AddressingMode::Absolute(word_after_opcode),
                cycles: 3,
                page_cross_cost: false,
            },
            0x6C => Instruction {
                opcode: Opcode::Jmp,
                mode: AddressingMode::Indirect(word_after_opcode),
                cycles: 5,
                page_cross_cost: false,
            },
            /* Jsr */
            0x20 => Instruction {
                opcode: Opcode::Jsr,
                mode: AddressingMode::Absolute(word_after_opcode),
                cycles: 6,
                page_cross_cost: false,
            },
            /* Lda */
            0xA9 => Instruction {
                opcode: Opcode::Lda,
                mode: AddressingMode::Immediate(byte_after_opcode),
                cycles: 2,
                page_cross_cost: false,
            },
            0xA5 => Instruction {
                opcode: Opcode::Lda,
                mode: AddressingMode::ZeroPage(byte_after_opcode),
                cycles: 3,
                page_cross_cost: false,
            },
            0xB5 => Instruction {
                opcode: Opcode::Lda,
                mode: AddressingMode::ZeroPageX(byte_after_opcode),
                cycles: 4,
                page_cross_cost: false,
            },
            0xAD => Instruction {
                opcode: Opcode::Lda,
                mode: AddressingMode::Absolute(word_after_opcode),
                cycles: 4,
                page_cross_cost: false,
            },
            0xBD => Instruction {
                opcode: Opcode::Lda,
                mode: AddressingMode::AbsoluteX(word_after_opcode),
                cycles: 4,
                page_cross_cost: true,
            },
            0xB9 => Instruction {
                opcode: Opcode::Lda,
                mode: AddressingMode::AbsoluteY(word_after_opcode),
                cycles: 4,
                page_cross_cost: true,
            },
            0xA1 => Instruction {
                opcode: Opcode::Lda,
                mode: AddressingMode::IndirectX(byte_after_opcode),
                cycles: 6,
                page_cross_cost: false,
            },
            0xB1 => Instruction {
                opcode: Opcode::Lda,
                mode: AddressingMode::IndirectY(byte_after_opcode),
                cycles: 5,
                page_cross_cost: true,
            },
            /* Ldx */
            0xA2 => Instruction {
                opcode: Opcode::Ldx,
                mode: AddressingMode::Immediate(byte_after_opcode),
                cycles: 2,
                page_cross_cost: false,
            },
            0xA6 => Instruction {
                opcode: Opcode::Ldx,
                mode: AddressingMode::ZeroPage(byte_after_opcode),
                cycles: 3,
                page_cross_cost: false,
            },
            0xB6 => Instruction {
                opcode: Opcode::Ldx,
                mode: AddressingMode::ZeroPageY(byte_after_opcode),
                cycles: 4,
                page_cross_cost: false,
            },
            0xAE => Instruction {
                opcode: Opcode::Ldx,
                mode: AddressingMode::Absolute(word_after_opcode),
                cycles: 4,
                page_cross_cost: false,
            },
            0xBE => Instruction {
                opcode: Opcode::Ldx,
                mode: AddressingMode::AbsoluteY(word_after_opcode),
                cycles: 4,
                page_cross_cost: true,
            },
            /* Ldy */
            0xA0 => Instruction {
                opcode: Opcode::Ldy,
                mode: AddressingMode::Immediate(byte_after_opcode),
                cycles: 2,
                page_cross_cost: false,
            },
            0xA4 => Instruction {
                opcode: Opcode::Ldy,
                mode: AddressingMode::ZeroPage(byte_after_opcode),
                cycles: 3,
                page_cross_cost: false,
            },
            0xB4 => Instruction {
                opcode: Opcode::Ldy,
                mode: AddressingMode::ZeroPageX(byte_after_opcode),
                cycles: 4,
                page_cross_cost: false,
            },
            0xAC => Instruction {
                opcode: Opcode::Ldy,
                mode: AddressingMode::Absolute(word_after_opcode),
                cycles: 4,
                page_cross_cost: false,
            },
            0xBC => Instruction {
                opcode: Opcode::Ldy,
                mode: AddressingMode::AbsoluteX(word_after_opcode),
                cycles: 4,
                page_cross_cost: true,
            },
            /* Lsr */
            0x4A => Instruction {
                opcode: Opcode::Lsr,
                mode: AddressingMode::Implicit,
                cycles: 2,
                page_cross_cost: false,
            },
            0x46 => Instruction {
                opcode: Opcode::Lsr,
                mode: AddressingMode::ZeroPage(byte_after_opcode),
                cycles: 5,
                page_cross_cost: false,
            },
            0x56 => Instruction {
                opcode: Opcode::Lsr,
                mode: AddressingMode::ZeroPageX(byte_after_opcode),
                cycles: 6,
                page_cross_cost: false,
            },
            0x4E => Instruction {
                opcode: Opcode::Lsr,
                mode: AddressingMode::Absolute(word_after_opcode),
                cycles: 6,
                page_cross_cost: false,
            },
            0x5E => Instruction {
                opcode: Opcode::Lsr,
                mode: AddressingMode::AbsoluteX(word_after_opcode),
                cycles: 7,
                page_cross_cost: false,
            },
            /* Nop */
            0xEA => Instruction {
                opcode: Opcode::Nop,
                mode: AddressingMode::Implicit,
                cycles: 2,
                page_cross_cost: false,
            },
            /* Ora */
            0x09 => Instruction {
                opcode: Opcode::Ora,
                mode: AddressingMode::Immediate(byte_after_opcode),
                cycles: 2,
                page_cross_cost: false,
            },
            0x05 => Instruction {
                opcode: Opcode::Ora,
                mode: AddressingMode::ZeroPage(byte_after_opcode),
                cycles: 3,
                page_cross_cost: false,
            },
            0x15 => Instruction {
                opcode: Opcode::Ora,
                mode: AddressingMode::ZeroPageX(byte_after_opcode),
                cycles: 4,
                page_cross_cost: false,
            },
            0x0D => Instruction {
                opcode: Opcode::Ora,
                mode: AddressingMode::Absolute(word_after_opcode),
                cycles: 4,
                page_cross_cost: false,
            },
            0x1D => Instruction {
                opcode: Opcode::Ora,
                mode: AddressingMode::AbsoluteX(word_after_opcode),
                cycles: 4,
                page_cross_cost: true,
            },
            0x19 => Instruction {
                opcode: Opcode::Ora,
                mode: AddressingMode::AbsoluteY(word_after_opcode),
                cycles: 2,
                page_cross_cost: true,
            },
            0x01 => Instruction {
                opcode: Opcode::Ora,
                mode: AddressingMode::IndirectX(byte_after_opcode),
                cycles: 6,
                page_cross_cost: false,
            },
            0x11 => Instruction {
                opcode: Opcode::Ora,
                mode: AddressingMode::IndirectY(byte_after_opcode),
                cycles: 5,
                page_cross_cost: false,
            },
            /* Pha */
            0x48 => Instruction {
                opcode: Opcode::Pha,
                mode: AddressingMode::Implicit,
                cycles: 3,
                page_cross_cost: false,
            },
            /* Php */
            0x08 => Instruction {
                opcode: Opcode::Php,
                mode: AddressingMode::Implicit,
                cycles: 3,
                page_cross_cost: false,
            },
            /* Pla */
            0x68 => Instruction {
                opcode: Opcode::Pla,
                mode: AddressingMode::Implicit,
                cycles: 4,
                page_cross_cost: false,
            },
            /* Plp */
            0x28 => Instruction {
                opcode: Opcode::Plp,
                mode: AddressingMode::Implicit,
                cycles: 4,
                page_cross_cost: false,
            },
            /* Rol */
            0x2A => Instruction {
                opcode: Opcode::Rol,
                mode: AddressingMode::Implicit,
                cycles: 2,
                page_cross_cost: false,
            },
            0x26 => Instruction {
                opcode: Opcode::Rol,
                mode: AddressingMode::ZeroPage(byte_after_opcode),
                cycles: 5,
                page_cross_cost: false,
            },
            0x36 => Instruction {
                opcode: Opcode::Rol,
                mode: AddressingMode::ZeroPageX(byte_after_opcode),
                cycles: 6,
                page_cross_cost: false,
            },
            0x2E => Instruction {
                opcode: Opcode::Rol,
                mode: AddressingMode::Absolute(word_after_opcode),
                cycles: 6,
                page_cross_cost: false,
            },
            0x3E => Instruction {
                opcode: Opcode::Rol,
                mode: AddressingMode::AbsoluteX(word_after_opcode),
                cycles: 7,
                page_cross_cost: false,
            },
            /* Ror */
            0x6A => Instruction {
                opcode: Opcode::Ror,
                mode: AddressingMode::Implicit,
                cycles: 2,
                page_cross_cost: false,
            },
            0x66 => Instruction {
                opcode: Opcode::Ror,
                mode: AddressingMode::ZeroPage(byte_after_opcode),
                cycles: 5,
                page_cross_cost: false,
            },
            0x76 => Instruction {
                opcode: Opcode::Ror,
                mode: AddressingMode::ZeroPageX(byte_after_opcode),
                cycles: 6,
                page_cross_cost: false,
            },
            0x6E => Instruction {
                opcode: Opcode::Ror,
                mode: AddressingMode::Absolute(word_after_opcode),
                cycles: 6,
                page_cross_cost: false,
            },
            0x7E => Instruction {
                opcode: Opcode::Ror,
                mode: AddressingMode::AbsoluteX(word_after_opcode),
                cycles: 7,
                page_cross_cost: false,
            },
            /* Rti */
            0x40 => Instruction {
                opcode: Opcode::Rti,
                mode: AddressingMode::Implicit,
                cycles: 6,
                page_cross_cost: false,
            },
            /* Rts */
            0x60 => Instruction {
                opcode: Opcode::Rts,
                mode: AddressingMode::Implicit,
                cycles: 6,
                page_cross_cost: false,
            },
            /* Sbc */
            0xE9 => Instruction {
                opcode: Opcode::Sbc,
                mode: AddressingMode::Immediate(byte_after_opcode),
                cycles: 2,
                page_cross_cost: false,
            },
            0xE5 => Instruction {
                opcode: Opcode::Sbc,
                mode: AddressingMode::ZeroPage(byte_after_opcode),
                cycles: 3,
                page_cross_cost: false,
            },
            0xF5 => Instruction {
                opcode: Opcode::Sbc,
                mode: AddressingMode::ZeroPageX(byte_after_opcode),
                cycles: 4,
                page_cross_cost: false,
            },
            0xED => Instruction {
                opcode: Opcode::Sbc,
                mode: AddressingMode::Absolute(word_after_opcode),
                cycles: 4,
                page_cross_cost: false,
            },
            0xFD => Instruction {
                opcode: Opcode::Sbc,
                mode: AddressingMode::AbsoluteX(word_after_opcode),
                cycles: 4,
                page_cross_cost: true,
            },
            0xF9 => Instruction {
                opcode: Opcode::Sbc,
                mode: AddressingMode::AbsoluteY(word_after_opcode),
                cycles: 4,
                page_cross_cost: true,
            },
            0xE1 => Instruction {
                opcode: Opcode::Sbc,
                mode: AddressingMode::IndirectX(byte_after_opcode),
                cycles: 6,
                page_cross_cost: false,
            },
            0xF1 => Instruction {
                opcode: Opcode::Sbc,
                mode: AddressingMode::IndirectY(byte_after_opcode),
                cycles: 5,
                page_cross_cost: false,
            },
            /* Sec */
            0x38 => Instruction {
                opcode: Opcode::Sec,
                mode: AddressingMode::Implicit,
                cycles: 2,
                page_cross_cost: false,
            },
            /* Sed */
            0xF8 => Instruction {
                opcode: Opcode::Sed,
                mode: AddressingMode::Implicit,
                cycles: 2,
                page_cross_cost: false,
            },
            /* Sei */
            0x78 => Instruction {
                opcode: Opcode::Sei,
                mode: AddressingMode::Implicit,
                cycles: 2,
                page_cross_cost: false,
            },
            /* Sta */
            0x85 => Instruction {
                opcode: Opcode::Sta,
                mode: AddressingMode::ZeroPage(byte_after_opcode),
                cycles: 3,
                page_cross_cost: false,
            },
            0x95 => Instruction {
                opcode: Opcode::Sta,
                mode: AddressingMode::ZeroPageX(byte_after_opcode),
                cycles: 4,
                page_cross_cost: false,
            },
            0x8D => Instruction {
                opcode: Opcode::Sta,
                mode: AddressingMode::Absolute(word_after_opcode),
                cycles: 4,
                page_cross_cost: false,
            },
            0x9D => Instruction {
                opcode: Opcode::Sta,
                mode: AddressingMode::AbsoluteX(word_after_opcode),
                cycles: 5,
                page_cross_cost: false,
            },
            0x99 => Instruction {
                opcode: Opcode::Sta,
                mode: AddressingMode::AbsoluteY(word_after_opcode),
                cycles: 5,
                page_cross_cost: false,
            },
            0x81 => Instruction {
                opcode: Opcode::Sta,
                mode: AddressingMode::IndirectX(byte_after_opcode),
                cycles: 6,
                page_cross_cost: false,
            },
            0x91 => Instruction {
                opcode: Opcode::Sta,
                mode: AddressingMode::IndirectY(byte_after_opcode),
                cycles: 6,
                page_cross_cost: false,
            },
            /* Stx */
            0x86 => Instruction {
                opcode: Opcode::Stx,
                mode: AddressingMode::ZeroPage(byte_after_opcode),
                cycles: 3,
                page_cross_cost: false,
            },
            0x96 => Instruction {
                opcode: Opcode::Stx,
                mode: AddressingMode::ZeroPageY(byte_after_opcode),
                cycles: 4,
                page_cross_cost: false,
            },
            0x8E => Instruction {
                opcode: Opcode::Stx,
                mode: AddressingMode::Absolute(word_after_opcode),
                cycles: 4,
                page_cross_cost: false,
            },
            /* Sty */
            0x84 => Instruction {
                opcode: Opcode::Sty,
                mode: AddressingMode::ZeroPage(byte_after_opcode),
                cycles: 3,
                page_cross_cost: false,
            },
            0x94 => Instruction {
                opcode: Opcode::Sty,
                mode: AddressingMode::ZeroPageX(byte_after_opcode),
                cycles: 4,
                page_cross_cost: false,
            },
            0x8C => Instruction {
                opcode: Opcode::Sty,
                mode: AddressingMode::Absolute(word_after_opcode),
                cycles: 4,
                page_cross_cost: false,
            },
            /* Tax */
            0xAA => Instruction {
                opcode: Opcode::Tax,
                mode: AddressingMode::Implicit,
                cycles: 2,
                page_cross_cost: false,
            },
            /* Tay */
            0xA8 => Instruction {
                opcode: Opcode::Tay,
                mode: AddressingMode::Implicit,
                cycles: 2,
                page_cross_cost: false,
            },
            /* Tsx */
            0xBA => Instruction {
                opcode: Opcode::Tsx,
                mode: AddressingMode::Implicit,
                cycles: 2,
                page_cross_cost: false,
            },
            /* Txa */
            0x8A => Instruction {
                opcode: Opcode::Txa,
                mode: AddressingMode::Implicit,
                cycles: 2,
                page_cross_cost: false,
            },
            /* Txs */
            0x9A => Instruction {
                opcode: Opcode::Txs,
                mode: AddressingMode::Implicit,
                cycles: 2,
                page_cross_cost: false,
            },
            /* Tya */
            0x98 => Instruction {
                opcode: Opcode::Tya,
                mode: AddressingMode::Implicit,
                cycles: 2,
                page_cross_cost: false,
            },
            _ => Instruction {
                opcode: Opcode::Nop,
                mode: AddressingMode::Implicit,
                cycles: 0,
                page_cross_cost: false,
            },
        };

        let instruction_length = 1 + match result.mode {
            AddressingMode::ZeroPage(_) => 1,
            AddressingMode::ZeroPageX(_) => 1,
            AddressingMode::ZeroPageY(_) => 1,
            AddressingMode::IndirectX(_) => 1,
            AddressingMode::IndirectY(_) => 1,
            AddressingMode::Immediate(_) => 1,
            AddressingMode::Relative(_) => 1,
            AddressingMode::Absolute(_) => 2,
            AddressingMode::AbsoluteX(_) => 2,
            AddressingMode::AbsoluteY(_) => 2,
            AddressingMode::Indirect(_) => 2,
            AddressingMode::Implicit => 0,
            AddressingMode::Accumulator => 0,
        };

        self.pc += instruction_length;
        result
    }

    pub fn execute_instruction(&mut self, instruction: Instruction) -> u8 {
        match instruction.opcode {
            Opcode::Add => self.adc(instruction.mode),
            Opcode::And => self.and(instruction.mode),
            Opcode::Asl => self.asl(instruction.mode),
            Opcode::Bcc => self.bcc(instruction.mode),
            Opcode::Bcs => self.bcs(instruction.mode),
            Opcode::Beq => self.beq(instruction.mode),
            Opcode::Bit => self.bit(instruction.mode),
            Opcode::Bmi => self.bmi(instruction.mode),
            Opcode::Bne => self.bne(instruction.mode),
            Opcode::Bpl => self.bpl(instruction.mode),
            Opcode::Brk => self.brk(instruction.mode),
            Opcode::Bvc => self.bvc(instruction.mode),
            Opcode::Bvs => self.bvs(instruction.mode),
            Opcode::Clc => self.clc(),
            Opcode::Cld => self.cld(),
            Opcode::Cli => self.cli(),
            Opcode::Clv => self.clv(),
            Opcode::Cmp => self.cmp(instruction.mode),
            Opcode::Cpx => self.cpx(instruction.mode),
            Opcode::Cpy => self.cpy(instruction.mode),
            Opcode::Dec => self.dec(instruction.mode),
            Opcode::Dex => self.dex(),
            Opcode::Dey => self.dey(),
            Opcode::Eor => self.eor(instruction.mode),
            Opcode::Inc => self.inc(instruction.mode),
            Opcode::Inx => self.inx(),
            Opcode::Iny => self.iny(),
            Opcode::Jmp => self.jmp(instruction.mode),
            Opcode::Jsr => self.jsr(instruction.mode),
            //_ => panic!("{:?} unimplemented", instruction),
            _ => { },
        }
        0x0
    }

    fn read_with_addressing_mode(&self, mode: AddressingMode) -> u8 {
        match mode {
            AddressingMode::ZeroPage(val) => self.memory.get_byte_at(u16::from(val)),
            AddressingMode::ZeroPageX(val) => self.memory.get_byte_at(u16::from(val.wrapping_add(self.x))),
            AddressingMode::ZeroPageY(val) => self.memory.get_byte_at(u16::from(val.wrapping_add(self.y))),
            AddressingMode::Absolute(addr) => self.memory.get_byte_at(addr),
            AddressingMode::AbsoluteX(val) => self.memory.get_byte_at(val + u16::from(self.x)),
            AddressingMode::AbsoluteY(val) => self.memory.get_byte_at(val + u16::from(self.y)),
            AddressingMode::IndirectX(val) => self.memory.get_byte_at(self.memory.get_word_at(u16::from(val) + u16::from(self.x))),
            AddressingMode::IndirectY(val) => self.memory.get_byte_at(self.memory.get_word_at(u16::from(val)) + u16::from(self.y)),
            AddressingMode::Immediate(val) => val,
            _ => panic!("Attempted to resolve address value of {:?} illegally", mode),
        }
    }

    fn write_with_addressing_mode(&mut self, mode: AddressingMode, assigned_val: u8) {
        match mode {
            AddressingMode::ZeroPage(val) => self.memory.set_byte_at(u16::from(val), assigned_val),
            AddressingMode::ZeroPageX(val) => self.memory.set_byte_at(u16::from(val.wrapping_add(self.x)), assigned_val),
            AddressingMode::ZeroPageY(val) => self.memory.set_byte_at(u16::from(val.wrapping_add(self.y)), assigned_val),
            AddressingMode::Absolute(addr) => self.memory.set_byte_at(addr, assigned_val),
            AddressingMode::AbsoluteX(val) => self.memory.set_byte_at(val + u16::from(self.x), assigned_val),
            AddressingMode::AbsoluteY(val) => self.memory.set_byte_at(val + u16::from(self.y), assigned_val),
            AddressingMode::IndirectX(val) => self.memory.set_byte_at(self.memory.get_word_at(u16::from(val) + u16::from(self.x)), assigned_val),
            AddressingMode::IndirectY(val) => self.memory.set_byte_at(self.memory.get_word_at(u16::from(val)) + u16::from(self.y), assigned_val),
            _ => panic!("Attempted to resolve address value of {:?} illegally", mode),
        }
    }

    fn adc(&mut self, mode: AddressingMode) {
        let to_be_added = self.read_with_addressing_mode(mode);

        let (first_add, first_carry) = self.accumulator.overflowing_add(to_be_added);
        let (result, second_carry) = first_add.overflowing_add(u8::from(self.carry));

        self.accumulator = result;
        self.sign = (result as i8) < 0;
        self.zero = (result == 0);
        self.carry = first_carry | second_carry;
        self.overflow = ((to_be_added ^ result) & (self.accumulator ^ result) & 0x80) != 0;
    }

    fn and(&mut self, mode: AddressingMode) {
        let to_be_anded = self.read_with_addressing_mode(mode);
        let result = to_be_anded & self.accumulator;

        self.accumulator = result;
        self.sign = (result as i8) < 0;
        self.zero = (result == 0);
    }

    fn asl(&mut self, mode: AddressingMode) {
        let to_be_asled = self.read_with_addressing_mode(mode);
        let result = to_be_asled << 1;

        self.write_with_addressing_mode(mode, result);
        self.sign = (result as i8) < 0;
        self.zero = (result == 0);
        self.carry = (to_be_asled & (1 << 7)) != 0;
    }

    fn branch(&mut self, condition: bool, offset: i8) {
        if condition {
            self.pc = (self.pc as i32 + offset as i32) as u16;
        }
    }

    fn bcc(&mut self, mode: AddressingMode) {
        match mode {
            AddressingMode::Relative(offset) => self.branch(!self.carry, offset),
            _ => panic!("Cannot branch using {:?}", mode),
        }
    }

    fn bcs(&mut self, mode: AddressingMode) {
        match mode {
            AddressingMode::Relative(offset) => self.branch(self.carry, offset),
            _ => panic!("Cannot branch using {:?}", mode),
        }
    }

    fn beq(&mut self, mode: AddressingMode) {
        match mode {
            AddressingMode::Relative(offset) => self.branch(self.zero, offset),
            _ => panic!("Cannot branch using {:?}", mode),
        }
    }

    fn bit(&mut self, mode: AddressingMode) {
        panic!("TODO");
    }

    fn bmi(&mut self, mode: AddressingMode) {
        match mode {
            AddressingMode::Relative(offset) => self.branch(self.sign, offset),
            _ => panic!("Cannot branch using {:?}", mode),
        }
    }

    fn bne(&mut self, mode: AddressingMode) {
        match mode {
            AddressingMode::Relative(offset) => self.branch(!self.zero, offset),
            _ => panic!("Cannot branch using {:?}", mode),
        }
    }

    fn bpl(&mut self, mode: AddressingMode) {
        match mode {
            AddressingMode::Relative(offset) => self.branch(!self.sign, offset),
            _ => panic!("Cannot branch using {:?}", mode),
        }
    }

    fn brk(&mut self, mode: AddressingMode) {
        //panic!("TODO");
    }

    fn bvc(&mut self, mode: AddressingMode) {
        match mode {
            AddressingMode::Relative(offset) => self.branch(!self.overflow, offset),
            _ => panic!("Cannot branch using {:?}", mode),
        }
    }

    fn bvs(&mut self, mode: AddressingMode) {
        match mode {
            AddressingMode::Relative(offset) => self.branch(self.overflow, offset),
            _ => panic!("Cannot branch using {:?}", mode),
        }
    }

    fn clc(&mut self) {
        self.carry = false;
    }

    fn cld(&mut self) {
        self.decimal = false;
    }

    fn cli(&mut self) {
        self.brk = false;
    }

    fn clv(&mut self) {
        self.overflow = false;
    }

    /* Maybe not quite right? */
    fn cmp(&mut self, mode: AddressingMode) {
        let to_compare = self.read_with_addressing_mode(mode);

        self.sign = self.accumulator < to_compare;
        self.zero = self.accumulator == to_compare;
        self.carry = self.accumulator >= to_compare;
    }

    fn cpx(&mut self, mode: AddressingMode) {
        let to_compare = self.read_with_addressing_mode(mode);

        self.sign = self.x < to_compare;
        self.zero = self.x == to_compare;
        self.carry = self.x >= to_compare;
    }

    fn cpy(&mut self, mode: AddressingMode) {
        let to_compare = self.read_with_addressing_mode(mode);

        self.sign = self.y < to_compare;
        self.zero = self.y == to_compare;
        self.carry = self.y >= to_compare;
    }

    fn dec(&mut self, mode: AddressingMode) {
        let old_val = self.read_with_addressing_mode(mode);
        let result = self.accumulator.wrapping_sub(1);
        self.write_with_addressing_mode(mode, result);

        self.sign = (result as i8) < 0;
        self.zero = (result == 0);
    }

    fn dex(&mut self) {
        self.x = self.x.wrapping_sub(1);

        self.sign = (self.x as i8) < 0;
        self.zero = (self.x == 0);
    }

    fn dey(&mut self) {
        self.y = self.y.wrapping_sub(1);

        self.sign = (self.y as i8) < 0;
        self.zero = (self.y == 0);
    }

    fn eor(&mut self, mode: AddressingMode) {
        let other_val = self.read_with_addressing_mode(mode);
        self.accumulator ^= other_val;

        self.sign = (self.accumulator as i8) < 0;
        self.zero = (self.accumulator == 0);
    }

    fn inc(&mut self, mode: AddressingMode) {
        let old_val = self.read_with_addressing_mode(mode);
        let result = self.accumulator.wrapping_add(1);
        self.write_with_addressing_mode(mode, result);

        self.sign = (result as i8) < 0;
        self.zero = (result == 0);
    }

    fn inx(&mut self) {
        self.x = self.x.wrapping_add(1);

        self.sign = (self.x as i8) < 0;
        self.zero = (self.x == 0);
    }

    fn iny(&mut self) {
        self.y = self.y.wrapping_add(1);

        self.sign = (self.y as i8) < 0;
        self.zero = (self.y == 0);
    }

    fn jmp(&mut self, mode: AddressingMode) {
        match mode {
            AddressingMode::Absolute(addr) => {
                self.pc = addr;
            },
            AddressingMode::Indirect(addr) => {
                self.pc = self.memory.get_word_at(addr);
            },
            _ => panic!("Cannot jmp using {:?}", mode),
        }
    }

    fn jsr(&mut self, mode: AddressingMode) {
        match mode {
            AddressingMode::Absolute(addr) => {
                self.sp = self.sp.wrapping_sub(2);
                self.memory.set_word_at(0x100 + u16::from(self.sp), self.pc + 2);
                self.pc = addr;
            },
            _ => panic!("Cannot jsr using {:?}", mode),
        }
    }
}
