pub enum AddressingMode {
  Accumulator,
  Immediate,
  ZeroPage,
  ZeroPageX,
  ZeroPageY,
  Absolute,
  AbsoluteX,
  AbsoluteY,
  Implied,
  Relative,
  IndexedIndirect,
  IndirectIndexed,
  Indirect
}

pub enum SingleByteMnemonic {
    ASL, CLC, CLD, CLI, CLV,
    DEX, DEY, INX, INY, LSR,
    NOP, ROL, ROR, SEC, SED,
    SEI, TAX, TAY, TSX, TXA,
    TXS, TYA
}

pub enum ReadMnemonic {
    ADC, AND, BIT, CMP, CPX,
    CPY, EOR, LDA, LDX, LDY,
    ORA, SBC
}

pub enum StoreMnemonic {
    STA, STX, STY
}

pub enum RMWMnemonic {
    ASL, DEC, INC, LSR,
    ROL, ROR
}

pub enum PushMnemonic {
    PHA, PHP
}

pub enum PullMnemonic {
    PLA, PLP
}

pub enum BranchMnemonic {
    BCC, BCS, BEQ, BMI, BNE,
    BPL, BVC, BVS
}

pub enum MiscMnemonic {
    JSR, BRK, RTI, RTS
}


pub enum Instruction {
    SingleByte(SingleByteMnemonic),
    Read(ReadMnemonic, AddressingMode),
    Store(StoreMnemonic, AddressingMode),
    ReadModifyWrite(RMWMnemonic, AddressingMode),
    Push(PushMnemonic),
    Pull(PullMnemonic),
    Branch(BranchMnemonic),
    Jump(AddressingMode),
    Misc(MiscMnemonic)
}

pub fn decode(opcode: u8) -> Option<Instruction> {
    match opcode {
        0x69 => Some(Instruction::Read(ReadMnemonic::ADC, AddressingMode::Immediate)),
        0x65 => Some(Instruction::Read(ReadMnemonic::ADC, AddressingMode::ZeroPage)),
        0x75 => Some(Instruction::Read(ReadMnemonic::ADC, AddressingMode::ZeroPageX)),
        0x6d => Some(Instruction::Read(ReadMnemonic::ADC, AddressingMode::Absolute)),
        0x7d => Some(Instruction::Read(ReadMnemonic::ADC, AddressingMode::AbsoluteX)),
        0x79 => Some(Instruction::Read(ReadMnemonic::ADC, AddressingMode::AbsoluteY)),
        0x61 => Some(Instruction::Read(ReadMnemonic::ADC, AddressingMode::IndexedIndirect)),
        0x71 => Some(Instruction::Read(ReadMnemonic::ADC, AddressingMode::IndirectIndexed)),
        0x29 => Some(Instruction::Read(ReadMnemonic::AND, AddressingMode::Immediate)),
        0x25 => Some(Instruction::Read(ReadMnemonic::AND, AddressingMode::ZeroPage)),
        0x35 => Some(Instruction::Read(ReadMnemonic::AND, AddressingMode::ZeroPageX)),
        0x2d => Some(Instruction::Read(ReadMnemonic::AND, AddressingMode::Absolute)),
        0x3d => Some(Instruction::Read(ReadMnemonic::AND, AddressingMode::AbsoluteX)),
        0x39 => Some(Instruction::Read(ReadMnemonic::AND, AddressingMode::AbsoluteY)),
        0x21 => Some(Instruction::Read(ReadMnemonic::AND, AddressingMode::IndexedIndirect)),
        0x31 => Some(Instruction::Read(ReadMnemonic::AND, AddressingMode::IndirectIndexed)),
        0x0a => Some(Instruction::ReadModifyWrite(RMWMnemonic::ASL, AddressingMode::Accumulator)),
        0x06 => Some(Instruction::ReadModifyWrite(RMWMnemonic::ASL, AddressingMode::ZeroPage)),
        0x16 => Some(Instruction::ReadModifyWrite(RMWMnemonic::ASL, AddressingMode::ZeroPageX)),
        0x0e => Some(Instruction::ReadModifyWrite(RMWMnemonic::ASL, AddressingMode::Absolute)),
        0x1e => Some(Instruction::ReadModifyWrite(RMWMnemonic::ASL, AddressingMode::AbsoluteX)),
        0x24 => Some(Instruction::Read(ReadMnemonic::BIT, AddressingMode::ZeroPage)),
        0x2c => Some(Instruction::Read(ReadMnemonic::BIT, AddressingMode::Absolute)),
        0x90 => Some(Instruction::Branch(BranchMnemonic::BCC)),
        0xd0 => Some(Instruction::Branch(BranchMnemonic::BNE)),
        0x10 => Some(Instruction::Branch(BranchMnemonic::BPL)),
        0x00 => Some(Instruction::Misc(MiscMnemonic::BRK)),
        0x50 => Some(Instruction::Branch(BranchMnemonic::BVC)),
        0xb0 => Some(Instruction::Branch(BranchMnemonic::BCS)),
        0xf0 => Some(Instruction::Branch(BranchMnemonic::BEQ)),
        0x30 => Some(Instruction::Branch(BranchMnemonic::BMI)),
        0x70 => Some(Instruction::Branch(BranchMnemonic::BVS)),
        0x18 => Some(Instruction::SingleByte(SingleByteMnemonic::CLC)),
        0xd8 => Some(Instruction::SingleByte(SingleByteMnemonic::CLD)),
        0x58 => Some(Instruction::SingleByte(SingleByteMnemonic::CLI)),
        0xb8 => Some(Instruction::SingleByte(SingleByteMnemonic::CLV)),
        0xc9 => Some(Instruction::Read(ReadMnemonic::CMP, AddressingMode::Immediate)),
        0xc5 => Some(Instruction::Read(ReadMnemonic::CMP, AddressingMode::ZeroPage)),
        0xd5 => Some(Instruction::Read(ReadMnemonic::CMP, AddressingMode::ZeroPageX)),
        0xcd => Some(Instruction::Read(ReadMnemonic::CMP, AddressingMode::Absolute)),
        0xdd => Some(Instruction::Read(ReadMnemonic::CMP, AddressingMode::AbsoluteX)),
        0xd9 => Some(Instruction::Read(ReadMnemonic::CMP, AddressingMode::AbsoluteY)),
        0xc1 => Some(Instruction::Read(ReadMnemonic::CMP, AddressingMode::IndexedIndirect)),
        0xd1 => Some(Instruction::Read(ReadMnemonic::CMP, AddressingMode::IndirectIndexed)),
        0xe0 => Some(Instruction::Read(ReadMnemonic::CPX, AddressingMode::Immediate)),
        0xe4 => Some(Instruction::Read(ReadMnemonic::CPX, AddressingMode::ZeroPage)),
        0xec => Some(Instruction::Read(ReadMnemonic::CPX, AddressingMode::Absolute)),
        0xc0 => Some(Instruction::Read(ReadMnemonic::CPY, AddressingMode::Immediate)),
        0xc4 => Some(Instruction::Read(ReadMnemonic::CPY, AddressingMode::ZeroPage)),
        0xcc => Some(Instruction::Read(ReadMnemonic::CPY, AddressingMode::Absolute)),
        0xc6 => Some(Instruction::ReadModifyWrite(RMWMnemonic::DEC, AddressingMode::ZeroPage)),
        0xd6 => Some(Instruction::ReadModifyWrite(RMWMnemonic::DEC, AddressingMode::ZeroPageX)),
        0xce => Some(Instruction::ReadModifyWrite(RMWMnemonic::DEC, AddressingMode::Absolute)),
        0xde => Some(Instruction::ReadModifyWrite(RMWMnemonic::DEC, AddressingMode::AbsoluteX)),
        0xca => Some(Instruction::SingleByte(SingleByteMnemonic::DEX)),
        0x88 => Some(Instruction::SingleByte(SingleByteMnemonic::DEY)),
        0x41 => Some(Instruction::Read(ReadMnemonic::EOR, AddressingMode::IndexedIndirect)),
        0x45 => Some(Instruction::Read(ReadMnemonic::EOR, AddressingMode::ZeroPage)),
        0x49 => Some(Instruction::Read(ReadMnemonic::EOR, AddressingMode::Immediate)),
        0x4d => Some(Instruction::Read(ReadMnemonic::EOR, AddressingMode::Absolute)),
        0x51 => Some(Instruction::Read(ReadMnemonic::EOR, AddressingMode::IndirectIndexed)),
        0x55 => Some(Instruction::Read(ReadMnemonic::EOR, AddressingMode::ZeroPageX)),
        0x5d => Some(Instruction::Read(ReadMnemonic::EOR, AddressingMode::AbsoluteX)),
        0x59 => Some(Instruction::Read(ReadMnemonic::EOR, AddressingMode::AbsoluteY)),
        0xe6 => Some(Instruction::ReadModifyWrite(RMWMnemonic::INC, AddressingMode::ZeroPage)),
        0xf6 => Some(Instruction::ReadModifyWrite(RMWMnemonic::INC, AddressingMode::ZeroPageX)),
        0xee => Some(Instruction::ReadModifyWrite(RMWMnemonic::INC, AddressingMode::Absolute)),
        0xfe => Some(Instruction::ReadModifyWrite(RMWMnemonic::INC, AddressingMode::AbsoluteX)),
        0xe8 => Some(Instruction::SingleByte(SingleByteMnemonic::INX)),
        0xc8 => Some(Instruction::SingleByte(SingleByteMnemonic::INY)),
        0x4c => Some(Instruction::Jump(AddressingMode::Absolute)),
        0x6c => Some(Instruction::Jump(AddressingMode::Indirect)),
        0x20 => Some(Instruction::Misc(MiscMnemonic::JSR)),
        0xa1 => Some(Instruction::Read(ReadMnemonic::LDA, AddressingMode::IndexedIndirect)),
        0xa5 => Some(Instruction::Read(ReadMnemonic::LDA, AddressingMode::ZeroPage)),
        0xa9 => Some(Instruction::Read(ReadMnemonic::LDA, AddressingMode::Immediate)),
        0xad => Some(Instruction::Read(ReadMnemonic::LDA, AddressingMode::Absolute)),
        0xb1 => Some(Instruction::Read(ReadMnemonic::LDA, AddressingMode::IndirectIndexed)),
        0xb5 => Some(Instruction::Read(ReadMnemonic::LDA, AddressingMode::ZeroPageX)),
        0xbd => Some(Instruction::Read(ReadMnemonic::LDA, AddressingMode::AbsoluteX)),
        0xb9 => Some(Instruction::Read(ReadMnemonic::LDA, AddressingMode::AbsoluteY)),
        0xa2 => Some(Instruction::Read(ReadMnemonic::LDX, AddressingMode::Immediate)),
        0xa6 => Some(Instruction::Read(ReadMnemonic::LDX, AddressingMode::ZeroPage)),
        0xb6 => Some(Instruction::Read(ReadMnemonic::LDX, AddressingMode::ZeroPageY)),
        0xae => Some(Instruction::Read(ReadMnemonic::LDX, AddressingMode::Absolute)),
        0xbe => Some(Instruction::Read(ReadMnemonic::LDX, AddressingMode::AbsoluteY)),
        0xa0 => Some(Instruction::Read(ReadMnemonic::LDY, AddressingMode::Immediate)),
        0xa4 => Some(Instruction::Read(ReadMnemonic::LDY, AddressingMode::ZeroPage)),
        0xb4 => Some(Instruction::Read(ReadMnemonic::LDY, AddressingMode::ZeroPageX)),
        0xac => Some(Instruction::Read(ReadMnemonic::LDY, AddressingMode::Absolute)),
        0xbc => Some(Instruction::Read(ReadMnemonic::LDY, AddressingMode::AbsoluteX)),
        0x4a => Some(Instruction::ReadModifyWrite(RMWMnemonic::LSR, AddressingMode::Accumulator)),
        0x46 => Some(Instruction::ReadModifyWrite(RMWMnemonic::LSR, AddressingMode::ZeroPage)),
        0x56 => Some(Instruction::ReadModifyWrite(RMWMnemonic::LSR, AddressingMode::ZeroPageX)),
        0x4e => Some(Instruction::ReadModifyWrite(RMWMnemonic::LSR, AddressingMode::Absolute)),
        0x5e => Some(Instruction::ReadModifyWrite(RMWMnemonic::LSR, AddressingMode::AbsoluteX)),
        0xea => Some(Instruction::SingleByte(SingleByteMnemonic::NOP)),
        0x09 => Some(Instruction::Read(ReadMnemonic::ORA, AddressingMode::Immediate)),
        0x05 => Some(Instruction::Read(ReadMnemonic::ORA, AddressingMode::ZeroPage)),
        0x15 => Some(Instruction::Read(ReadMnemonic::ORA, AddressingMode::ZeroPageX)),
        0x0d => Some(Instruction::Read(ReadMnemonic::ORA, AddressingMode::Absolute)),
        0x1d => Some(Instruction::Read(ReadMnemonic::ORA, AddressingMode::AbsoluteX)),
        0x19 => Some(Instruction::Read(ReadMnemonic::ORA, AddressingMode::AbsoluteY)),
        0x01 => Some(Instruction::Read(ReadMnemonic::ORA, AddressingMode::IndexedIndirect)),
        0x11 => Some(Instruction::Read(ReadMnemonic::ORA, AddressingMode::IndirectIndexed)),
        0x48 => Some(Instruction::Push(PushMnemonic::PHA)),
        0x08 => Some(Instruction::Push(PushMnemonic::PHP)),
        0x68 => Some(Instruction::Pull(PullMnemonic::PLA)),
        0x28 => Some(Instruction::Pull(PullMnemonic::PLP)),
        0x2a => Some(Instruction::ReadModifyWrite(RMWMnemonic::ROL, AddressingMode::Accumulator)),
        0x26 => Some(Instruction::ReadModifyWrite(RMWMnemonic::ROL, AddressingMode::ZeroPage)),
        0x36 => Some(Instruction::ReadModifyWrite(RMWMnemonic::ROL, AddressingMode::ZeroPageX)),
        0x2e => Some(Instruction::ReadModifyWrite(RMWMnemonic::ROL, AddressingMode::Absolute)),
        0x3e => Some(Instruction::ReadModifyWrite(RMWMnemonic::ROL, AddressingMode::AbsoluteX)),
        0x6a => Some(Instruction::SingleByte(SingleByteMnemonic::ROR)),
        0x66 => Some(Instruction::ReadModifyWrite(RMWMnemonic::ROR, AddressingMode::ZeroPage)),
        0x76 => Some(Instruction::ReadModifyWrite(RMWMnemonic::ROR, AddressingMode::ZeroPageX)),
        0x6e => Some(Instruction::ReadModifyWrite(RMWMnemonic::ROR, AddressingMode::Absolute)),
        0x7e => Some(Instruction::ReadModifyWrite(RMWMnemonic::ROR, AddressingMode::AbsoluteX)),
        0x40 => Some(Instruction::Misc(MiscMnemonic::RTI)),
        0x60 => Some(Instruction::Misc(MiscMnemonic::RTS)),
        0xe9 => Some(Instruction::Read(ReadMnemonic::SBC, AddressingMode::Immediate)),
        0xe5 => Some(Instruction::Read(ReadMnemonic::SBC, AddressingMode::ZeroPage)),
        0xf5 => Some(Instruction::Read(ReadMnemonic::SBC, AddressingMode::ZeroPageX)),
        0xed => Some(Instruction::Read(ReadMnemonic::SBC, AddressingMode::Absolute)),
        0xfd => Some(Instruction::Read(ReadMnemonic::SBC, AddressingMode::AbsoluteX)),
        0xf9 => Some(Instruction::Read(ReadMnemonic::SBC, AddressingMode::AbsoluteY)),
        0xe1 => Some(Instruction::Read(ReadMnemonic::SBC, AddressingMode::IndexedIndirect)),
        0xf1 => Some(Instruction::Read(ReadMnemonic::SBC, AddressingMode::IndirectIndexed)),
        0x38 => Some(Instruction::SingleByte(SingleByteMnemonic::SEC)),
        0xf8 => Some(Instruction::SingleByte(SingleByteMnemonic::SED)),
        0x78 => Some(Instruction::SingleByte(SingleByteMnemonic::SEI)),
        0x85 => Some(Instruction::Store(StoreMnemonic::STA, AddressingMode::ZeroPage)),
        0x95 => Some(Instruction::Store(StoreMnemonic::STA, AddressingMode::ZeroPageX)),
        0x8d => Some(Instruction::Store(StoreMnemonic::STA, AddressingMode::Absolute)),
        0x9d => Some(Instruction::Store(StoreMnemonic::STA, AddressingMode::AbsoluteX)),
        0x99 => Some(Instruction::Store(StoreMnemonic::STA, AddressingMode::AbsoluteY)),
        0x81 => Some(Instruction::Store(StoreMnemonic::STA, AddressingMode::IndexedIndirect)),
        0x91 => Some(Instruction::Store(StoreMnemonic::STA, AddressingMode::IndirectIndexed)),
        0x86 => Some(Instruction::Store(StoreMnemonic::STX, AddressingMode::ZeroPage)),
        0x97 => Some(Instruction::Store(StoreMnemonic::STX, AddressingMode::ZeroPageY)),
        0x8e => Some(Instruction::Store(StoreMnemonic::STX, AddressingMode::Absolute)),
        0x84 => Some(Instruction::Store(StoreMnemonic::STY, AddressingMode::ZeroPage)),
        0x94 => Some(Instruction::Store(StoreMnemonic::STY, AddressingMode::ZeroPageX)),
        0x8c => Some(Instruction::Store(StoreMnemonic::STY, AddressingMode::Absolute)),
        0xaa => Some(Instruction::SingleByte(SingleByteMnemonic::TAX)),
        0xa8 => Some(Instruction::SingleByte(SingleByteMnemonic::TAY)),
        0x98 => Some(Instruction::SingleByte(SingleByteMnemonic::TYA)),
        0xba => Some(Instruction::SingleByte(SingleByteMnemonic::TSX)),
        0x8a => Some(Instruction::SingleByte(SingleByteMnemonic::TXA)),
        0x9a => Some(Instruction::SingleByte(SingleByteMnemonic::TXS)),
        _ => None
    }
}
