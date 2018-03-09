pub mod instruction;
mod test;
use self::instruction::*;
use std::ops::DerefMut;

fn same_sign(lhs: u8, rhs: u8) -> bool {
    lhs & 0x80 == rhs & 0x80
}

trait Address where Self: Sized {
    fn low_byte(self) -> u8;
    fn high_byte(self) -> u8;
    fn combine_low_high(low: u8, high: u8) -> Self;
    fn add_offset(self, offset: u8) -> Self;
    fn on_different_page(self, rhs: Self) -> bool {
        self.high_byte() != rhs.high_byte()
    }
}

impl Address for usize {
    fn low_byte(self) -> u8 {
        (self & 0xff) as u8
    }

    fn high_byte(self) -> u8 {
        ((self & 0xff00) >> 8) as u8
    }

    fn combine_low_high(low: u8, high: u8) -> Self {
        low as Self | ((high as Self) << 8)
    }

    fn add_offset(self, offset: u8) -> usize {
        if offset < 128 {
            self + offset as usize
        }
        else {
            self - (255 - offset) as usize
        }
    }
}

#[derive(Default)]
pub struct StatusRegister {
    carry: bool,
    zero: bool,
    irq_disabled: bool,
    decimal_mode: bool,
    overflow: bool,
    negative: bool
}

impl StatusRegister {
    fn carry_from_u8(&mut self, val: u8) {
        self.carry = val != 0;
    }

    fn carry_into_u8(&self) -> u8 {
        if self.carry {
            1
        }
        else {
            0
        }
    }
}

impl From<u8> for StatusRegister {
    fn from(val: u8) -> Self {
        Self {
            carry: val & 1 == 1,
            zero: val & 2 == 1,
            irq_disabled: val & 4 == 1,
            decimal_mode: val & 8 == 1,
            overflow: val & 64 == 1,
            negative: val & 128 == 1
        }
    }
}

fn from_bool(b: bool, val: u8) -> u8 {
    if b {
        val 
    }
    else {
        0
    }
}

impl Into<u8> for StatusRegister {
    fn into(self) -> u8 {
        0x30 | from_bool(self.carry, 1)
             | from_bool(self.zero, 2)
             | from_bool(self.irq_disabled, 4)
             | from_bool(self.decimal_mode, 8)
             | from_bool(self.overflow, 64)
             | from_bool(self.negative, 128)
    }
}

#[derive(Debug)]
pub enum Error {
    IllegalOpcode(u8)
}

#[derive(Default)]
pub struct Registers {
    p: StatusRegister,
    pc: usize,
    a: u8,
    x: u8,
    y: u8,
    s: usize
}

pub struct Cpu {
    memory: Vec<u8>,
    cycles: u64,
    registers: Registers,
    executing_read: bool
}

impl Default for Cpu {
    fn default() -> Self {
        Cpu {
            memory: vec![0; 65536],
            cycles: 0,
            registers: Default::default(),
            executing_read: false
        }
    }
}


impl Cpu {
    pub fn new(r: Registers) -> Self {
        Cpu { 
            registers: r,
            .. Default::default()
        }
    }

    pub fn fill_memory(&mut self, from: usize, with: &[u8]) {
        let end = from + with.len();
        assert!(end <= 65536, "Filling memory would exceed 64K mark");
        self.memory[from..end].clone_from_slice(with);
    }

    fn s_into_byte(&self) -> u8 {
        (self.registers.s & 0xff) as u8
    }

    fn s_from_byte(&mut self, val: u8) {
        self.registers.s = val as usize;
    }

    fn set_zn(&mut self, val: u8) {
        self.registers.p.zero = val == 0;
        self.registers.p.negative = val & 0x80 == 0x80;
    }

    fn get_x(&self) -> u8 {
        self.registers.x
    }

    fn get_y(&self) -> u8 {
        self.registers.y
    }


    fn set_a(&mut self, val: u8) {
        self.registers.a = val;
        self.set_zn(val)
    }

    fn set_x(&mut self, val: u8) {
        self.registers.x = val;
        self.set_zn(val)
    }

    fn set_y(&mut self, val: u8) {
        self.registers.y = val;
        self.set_zn(val)
    }

    pub fn get_memory(&mut self) -> &mut [u8] {
        self.memory.deref_mut()
    }

    pub fn step(&mut self) -> Result<(), Error> {
        let opcode = self.read_pc();
        self.dispatch(opcode)
    }

    fn do_asl(&mut self, val: u8) -> u8 {
        self.registers.p.carry_from_u8(self.registers.a & 0x80);
        val << 1
    }

    fn do_lsr(&mut self, val: u8) -> u8 {
        self.registers.p.carry_from_u8(self.registers.a & 0x01);
        val >> 1
    }

    fn do_rol(&mut self, val: u8) -> u8 {
        let old_c = self.registers.p.carry_into_u8();
        self.registers.p.carry_from_u8(val & 0x80);
        (val << 1) | old_c
    }

    fn do_ror(&mut self, val: u8) -> u8 {
        let old_c = self.registers.p.carry_into_u8();
        self.registers.p.carry_from_u8(val & 0x01);
        (val >> 1) | (old_c << 7)
    }

    fn mod_x<F>(&mut self, f: F)
        where F: Fn(&mut Self, u8) -> u8
    {
        let x = self.registers.x;
        let res = f(self, x);
        self.set_x(res);
    }

    fn mod_y<F>(&mut self, f: F)
        where F: Fn(&mut Self, u8) -> u8
    {
        let y = self.registers.y;
        let res = f(self, y);
        self.set_y(res);
    }

    fn mod_a<F>(&mut self, mut f: F)
        where F: FnMut(&mut Self, u8) -> u8
    {
        let a = self.registers.a;
        let res = f(self, a);
        self.set_a(res);
    }

    fn resolve_immmediate(&mut self) -> usize {
        let pc = self.registers.pc;
        self.registers.pc += 1;
        pc
    }

    fn resolve_zero_page(&mut self) -> usize {
        let val = self.read_pc();
        val as usize
    }

    fn resolve_zero_page_x(&mut self) -> usize {
        let val = self.read_pc();
        let _ = self.read_byte(val as usize);
        let x = self.get_x();
        val.wrapping_add(x) as usize
    }

    fn resolve_zero_page_y(&mut self) -> usize {
        let val = self.read_pc();
        let _ = self.read_byte(val as usize);
        let y = self.get_y();
        val.wrapping_add(y) as usize
    }

    fn resolve_absolute(&mut self) -> usize {
        let adl = self.read_pc();
        let adh = self.read_pc();
        Address::combine_low_high(adl, adh)
    }

    fn resolve_low_high(&mut self, adl: u8, adh: u8, val: u8) -> usize {
        let addr = Address::combine_low_high(adl.wrapping_add(val), adh);
        if adl <= 255 - val {
            if !self.executing_read {
                let _ = self.read_byte(addr);
            }
            addr
        }
        else {
            let _ = self.read_byte(addr);
            addr + 0x100
        }
    }

    fn resolve_absolute_add(&mut self, val: u8) -> usize {
        let adl = self.read_pc();
        let adh = self.read_pc();
        self.resolve_low_high(adl, adh, val)
    }

    fn resolve_absolute_x(&mut self) -> usize {
        let x = self.get_x();
        self.resolve_absolute_add(x)
    }

    fn resolve_absolute_y(&mut self) -> usize {
        let y = self.get_y();
        self.resolve_absolute_add(y)
    }

    fn resolve_indexed_indirect(&mut self) -> usize {
        let bal = self.read_pc();
        let _ = self.read_byte(bal as usize);
        let effective_bal = bal.wrapping_add(self.get_x());
        let adl = self.read_byte(effective_bal as usize);
        let adh = self.read_byte(effective_bal.wrapping_add(1) as usize);
        Address::combine_low_high(adl, adh)
    }


    fn resolve_indirect_indexed(&mut self) -> usize {
        let ial = self.read_pc();
        let bal = self.read_byte(ial as usize);
        let bah = self.read_byte(ial.wrapping_add(1) as usize);
        let y = self.get_y();
        self.resolve_low_high(bal, bah, y)
    }

    fn resolve_address(&mut self, am: AddressingMode) -> usize {
        match am {
            AddressingMode::Immediate => self.resolve_immmediate(),
            AddressingMode::ZeroPage => self.resolve_zero_page(),
            AddressingMode::ZeroPageX => self.resolve_zero_page_x(),
            AddressingMode::ZeroPageY => self.resolve_zero_page_y(),
            AddressingMode::Absolute => self.resolve_absolute(),
            AddressingMode::AbsoluteX => self.resolve_absolute_x(),
            AddressingMode::AbsoluteY => self.resolve_absolute_y(),
            AddressingMode::IndexedIndirect => self.resolve_indexed_indirect(),
            AddressingMode::IndirectIndexed => self.resolve_indirect_indexed()
        }
    }

    fn fetch_operand(&mut self, am: AddressingMode) -> u8 {
        let addr = self.resolve_address(am);
        self.read_byte(addr)
    }

    fn compare(&mut self, register: u8, operand: u8) {
        self.registers.p.carry = register >= operand;
        self.registers.p.zero = register == operand;
        self.registers.p.negative = ((register.wrapping_sub(operand)) & 0x80) != 0;
    }

    fn compare_a(&mut self, operand: u8) {
        let a = self.registers.a;
        self.compare(a, operand);
    }

    fn compare_x(&mut self, operand: u8) {
        let x = self.registers.x;
        self.compare(x, operand);
    }

    fn compare_y(&mut self, operand: u8) {
        let y = self.registers.y;
        self.compare(y, operand);
    }

    fn bit(&mut self, operand: u8) {
        let a = self.registers.a;
        self.registers.p.overflow = operand & 0x40 != 0;
        self.registers.p.negative = operand & 0x80 != 0;
        self.registers.p.zero = a == operand;
    }

    fn sbc(&mut self, operand: u8) {
        // [TODO]: Add support for decimal mode
        let a = self.registers.a;
        let c = if self.registers.p.carry {
            0
        } else {
            1
        };
        let effective_operand = operand.wrapping_sub(c);
        let res = a.wrapping_sub(effective_operand);
        self.registers.p.carry = a < effective_operand;
        self.registers.p.overflow = same_sign(a, operand) && !same_sign(a, res);
        self.set_a(res)
    }

    fn adc(&mut self, operand: u8) {
        // [TODO]: Add support for decimal mode
        let a = self.registers.a;
        let effective_operand = operand.wrapping_add(self.registers.p.carry_into_u8());
        let res = a.wrapping_add(effective_operand);
        self.registers.p.carry = a > 255 - effective_operand;
        self.registers.p.overflow = same_sign(a, operand) && !same_sign(a, res);
        self.set_a(res);
    }


    fn execute_single_byte(&mut self, m: SingleByteMnemonic) {
        match m {
            SingleByteMnemonic::ASL => self.mod_a(Self::do_asl),
            SingleByteMnemonic::CLC => self.registers.p.carry = false,
            SingleByteMnemonic::CLD => self.registers.p.decimal_mode = false,
            SingleByteMnemonic::CLI => self.registers.p.irq_disabled = false,
            SingleByteMnemonic::CLV => self.registers.p.overflow = false,
            SingleByteMnemonic::DEX => self.mod_x(|_, x| x.wrapping_sub(1)),
            SingleByteMnemonic::DEY => self.mod_y(|_, y| y.wrapping_sub(1)),
            SingleByteMnemonic::INX => self.mod_x(|_, x| x.wrapping_add(1)),
            SingleByteMnemonic::INY => self.mod_y(|_, y| y.wrapping_add(1)),
            SingleByteMnemonic::LSR => self.mod_a(Self::do_lsr),
            SingleByteMnemonic::NOP => {},
            SingleByteMnemonic::ROL => self.mod_a(Self::do_rol),
            SingleByteMnemonic::ROR => self.mod_a(Self::do_ror),
            SingleByteMnemonic::SEC => self.registers.p.carry = true,
            SingleByteMnemonic::SED => self.registers.p.decimal_mode = true,
            SingleByteMnemonic::SEI => self.registers.p.irq_disabled = true,
            SingleByteMnemonic::TAX => self.mod_x(|this, _| this.registers.a),
            SingleByteMnemonic::TAY => self.mod_y(|this, _| this.registers.y),
            SingleByteMnemonic::TSX => self.mod_x(|this, _| this.s_into_byte()),
            SingleByteMnemonic::TXA => self.mod_a(|this, _| this.registers.x),
            SingleByteMnemonic::TXS => {
                let x = self.registers.x;
                self.s_from_byte(x)
            },
            SingleByteMnemonic::TYA => self.mod_a(|this, _| this.registers.y)
        }
        self.full_cycle();
        self.bogus_read_pc();
    }

    fn execute_read(&mut self, m: ReadMnemonic, am: AddressingMode) {
        self.executing_read = true;
        let operand = self.fetch_operand(am);
        match m {
            ReadMnemonic::ADC => self.adc(operand),
            ReadMnemonic::AND => self.mod_a(|_, a| a & operand),
            ReadMnemonic::BIT => self.bit(operand),
            ReadMnemonic::CMP => self.compare_a(operand),
            ReadMnemonic::CPX => self.compare_x(operand),
            ReadMnemonic::CPY => self.compare_y(operand),
            ReadMnemonic::EOR => self.mod_a(|_, a| a ^ operand),
            ReadMnemonic::LDA => self.mod_a(|_, _| operand),
            ReadMnemonic::LDX => self.mod_x(|_, _| operand),
            ReadMnemonic::LDY => self.mod_y(|_, _| operand),
            ReadMnemonic::ORA => self.mod_a(|_, a| a | operand),
            ReadMnemonic::SBC => self.sbc(operand)
        }
        self.executing_read = false;
    }

    fn should_branch(&self, m: BranchMnemonic) -> bool {
        match m {
            BranchMnemonic::BCC => !self.registers.p.carry,
            BranchMnemonic::BCS => self.registers.p.carry,
            BranchMnemonic::BEQ => self.registers.p.zero,
            BranchMnemonic::BMI => self.registers.p.negative,
            BranchMnemonic::BNE => !self.registers.p.zero,
            BranchMnemonic::BPL => !self.registers.p.negative,
            BranchMnemonic::BVC => self.registers.p.overflow,
            BranchMnemonic::BVS => !self.registers.p.overflow
        }
    }

    fn execute_branch(&mut self, m: BranchMnemonic) {
        let offset = self.read_pc();
        if !self.should_branch(m) {
            return;
        }
        let pc = self.registers.pc;
        let next_pc = pc.add_offset(offset);
        if next_pc.on_different_page(pc) {
            let _ = self.read_byte(Address::combine_low_high(next_pc.low_byte(), pc.high_byte()));
        }
        let _ = self.read_byte(next_pc);
        self.registers.pc = next_pc;
    }

    fn dispatch(&mut self, opcode: u8) -> Result<(), Error> {
        let instruction = decode(opcode).ok_or(Error::IllegalOpcode(opcode))?;
        match instruction {
            Instruction::SingleByte(mnemonic) => self.execute_single_byte(mnemonic),
            Instruction::Read(mnemonic, am) => self.execute_read(mnemonic, am),
            Instruction::Branch(mnemonic) => self.execute_branch(mnemonic),
            _ => return Err(Error::IllegalOpcode(065))
        }
        Ok(())
    }

    pub fn run(&mut self, cycles: u64) {
    }

    fn full_cycle(&mut self) {
        self.cycles += 1;
    }

    fn read_pc(&mut self) -> u8 {
        let pc = self.registers.pc;
        let v = self.read_byte(pc);
        self.registers.pc += 1;
        v
    }

    fn read_byte(&mut self, address: usize) -> u8 {
        assert!(address <= 65535, "address out of bounds");
        let v = self.memory[address];
        self.full_cycle();
        v
    }

    fn bogus_read_pc(&mut self) {
        let _ = self.read_pc();
        self.registers.pc -= 1;
    }
}

