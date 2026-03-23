// Instruction module for the 6502 CPU emulator
// Defines instruction types, addressing modes, and opcode decoding

/// Represents all 6502 instruction types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Instruction {
    // Load/Store
    LDA, LDX, LDY, STA, STX, STY,
    // Arithmetic
    ADC, SBC, INC, INX, INY, DEC, DEX, DEY,
    // Logical
    AND, ORA, EOR,
    // Shift/Rotate
    ASL, LSR, ROL, ROR,
    // Compare
    CMP, CPX, CPY,
    // Branch
    BCC, BCS, BEQ, BMI, BNE, BPL, BVC, BVS,
    // Jump/Subroutine
    JMP, JSR, RTS, RTI,
    // Stack
    PHA, PHP, PLA, PLP, TSX, TXS,
    // Flags
    CLC, CLD, CLI, CLV, SEC, SED, SEI,
    // Transfer
    TAX, TAY, TXA, TYA,
    // Other
    NOP, BRK,
}

/// Represents all 6502 addressing modes
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AddressingMode {
    Implied,
    Accumulator,
    Immediate,
    ZeroPage,
    ZeroPageX,
    ZeroPageY,
    Relative,
    Absolute,
    AbsoluteX,
    AbsoluteY,
    Indirect,
    IndexedIndirect,  // (Indirect,X)
    IndirectIndexed,  // (Indirect),Y
}

/// Represents a decoded instruction with its addressing mode and length
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DecodedInstruction {
    pub instruction: Instruction,
    pub mode: AddressingMode,
    pub length: u8,  // Instruction length in bytes (1-3)
}

/// Decodes an opcode byte into an instruction, addressing mode, and length
/// Returns an error for invalid or unofficial opcodes
pub fn decode_opcode(opcode: u8) -> Result<DecodedInstruction, String> {
    use Instruction::*;
    use AddressingMode::*;
    
    match opcode {
        // LDA - Load Accumulator
        0xA9 => Ok(DecodedInstruction { instruction: LDA, mode: Immediate, length: 2 }),
        0xA5 => Ok(DecodedInstruction { instruction: LDA, mode: ZeroPage, length: 2 }),
        0xB5 => Ok(DecodedInstruction { instruction: LDA, mode: ZeroPageX, length: 2 }),
        0xAD => Ok(DecodedInstruction { instruction: LDA, mode: Absolute, length: 3 }),
        0xBD => Ok(DecodedInstruction { instruction: LDA, mode: AbsoluteX, length: 3 }),
        0xB9 => Ok(DecodedInstruction { instruction: LDA, mode: AbsoluteY, length: 3 }),
        0xA1 => Ok(DecodedInstruction { instruction: LDA, mode: IndexedIndirect, length: 2 }),
        0xB1 => Ok(DecodedInstruction { instruction: LDA, mode: IndirectIndexed, length: 2 }),
        
        // LDX - Load X Register
        0xA2 => Ok(DecodedInstruction { instruction: LDX, mode: Immediate, length: 2 }),
        0xA6 => Ok(DecodedInstruction { instruction: LDX, mode: ZeroPage, length: 2 }),
        0xB6 => Ok(DecodedInstruction { instruction: LDX, mode: ZeroPageY, length: 2 }),
        0xAE => Ok(DecodedInstruction { instruction: LDX, mode: Absolute, length: 3 }),
        0xBE => Ok(DecodedInstruction { instruction: LDX, mode: AbsoluteY, length: 3 }),
        
        // LDY - Load Y Register
        0xA0 => Ok(DecodedInstruction { instruction: LDY, mode: Immediate, length: 2 }),
        0xA4 => Ok(DecodedInstruction { instruction: LDY, mode: ZeroPage, length: 2 }),
        0xB4 => Ok(DecodedInstruction { instruction: LDY, mode: ZeroPageX, length: 2 }),
        0xAC => Ok(DecodedInstruction { instruction: LDY, mode: Absolute, length: 3 }),
        0xBC => Ok(DecodedInstruction { instruction: LDY, mode: AbsoluteX, length: 3 }),
        
        // STA - Store Accumulator
        0x85 => Ok(DecodedInstruction { instruction: STA, mode: ZeroPage, length: 2 }),
        0x95 => Ok(DecodedInstruction { instruction: STA, mode: ZeroPageX, length: 2 }),
        0x8D => Ok(DecodedInstruction { instruction: STA, mode: Absolute, length: 3 }),
        0x9D => Ok(DecodedInstruction { instruction: STA, mode: AbsoluteX, length: 3 }),
        0x99 => Ok(DecodedInstruction { instruction: STA, mode: AbsoluteY, length: 3 }),
        0x81 => Ok(DecodedInstruction { instruction: STA, mode: IndexedIndirect, length: 2 }),
        0x91 => Ok(DecodedInstruction { instruction: STA, mode: IndirectIndexed, length: 2 }),
        
        // STX - Store X Register
        0x86 => Ok(DecodedInstruction { instruction: STX, mode: ZeroPage, length: 2 }),
        0x96 => Ok(DecodedInstruction { instruction: STX, mode: ZeroPageY, length: 2 }),
        0x8E => Ok(DecodedInstruction { instruction: STX, mode: Absolute, length: 3 }),
        
        // STY - Store Y Register
        0x84 => Ok(DecodedInstruction { instruction: STY, mode: ZeroPage, length: 2 }),
        0x94 => Ok(DecodedInstruction { instruction: STY, mode: ZeroPageX, length: 2 }),
        0x8C => Ok(DecodedInstruction { instruction: STY, mode: Absolute, length: 3 }),
        
        // ADC - Add with Carry
        0x69 => Ok(DecodedInstruction { instruction: ADC, mode: Immediate, length: 2 }),
        0x65 => Ok(DecodedInstruction { instruction: ADC, mode: ZeroPage, length: 2 }),
        0x75 => Ok(DecodedInstruction { instruction: ADC, mode: ZeroPageX, length: 2 }),
        0x6D => Ok(DecodedInstruction { instruction: ADC, mode: Absolute, length: 3 }),
        0x7D => Ok(DecodedInstruction { instruction: ADC, mode: AbsoluteX, length: 3 }),
        0x79 => Ok(DecodedInstruction { instruction: ADC, mode: AbsoluteY, length: 3 }),
        0x61 => Ok(DecodedInstruction { instruction: ADC, mode: IndexedIndirect, length: 2 }),
        0x71 => Ok(DecodedInstruction { instruction: ADC, mode: IndirectIndexed, length: 2 }),
        
        // SBC - Subtract with Carry
        0xE9 => Ok(DecodedInstruction { instruction: SBC, mode: Immediate, length: 2 }),
        0xE5 => Ok(DecodedInstruction { instruction: SBC, mode: ZeroPage, length: 2 }),
        0xF5 => Ok(DecodedInstruction { instruction: SBC, mode: ZeroPageX, length: 2 }),
        0xED => Ok(DecodedInstruction { instruction: SBC, mode: Absolute, length: 3 }),
        0xFD => Ok(DecodedInstruction { instruction: SBC, mode: AbsoluteX, length: 3 }),
        0xF9 => Ok(DecodedInstruction { instruction: SBC, mode: AbsoluteY, length: 3 }),
        0xE1 => Ok(DecodedInstruction { instruction: SBC, mode: IndexedIndirect, length: 2 }),
        0xF1 => Ok(DecodedInstruction { instruction: SBC, mode: IndirectIndexed, length: 2 }),
        
        // INC - Increment Memory
        0xE6 => Ok(DecodedInstruction { instruction: INC, mode: ZeroPage, length: 2 }),
        0xF6 => Ok(DecodedInstruction { instruction: INC, mode: ZeroPageX, length: 2 }),
        0xEE => Ok(DecodedInstruction { instruction: INC, mode: Absolute, length: 3 }),
        0xFE => Ok(DecodedInstruction { instruction: INC, mode: AbsoluteX, length: 3 }),
        
        // INX - Increment X Register
        0xE8 => Ok(DecodedInstruction { instruction: INX, mode: Implied, length: 1 }),
        
        // INY - Increment Y Register
        0xC8 => Ok(DecodedInstruction { instruction: INY, mode: Implied, length: 1 }),
        
        // DEC - Decrement Memory
        0xC6 => Ok(DecodedInstruction { instruction: DEC, mode: ZeroPage, length: 2 }),
        0xD6 => Ok(DecodedInstruction { instruction: DEC, mode: ZeroPageX, length: 2 }),
        0xCE => Ok(DecodedInstruction { instruction: DEC, mode: Absolute, length: 3 }),
        0xDE => Ok(DecodedInstruction { instruction: DEC, mode: AbsoluteX, length: 3 }),
        
        // DEX - Decrement X Register
        0xCA => Ok(DecodedInstruction { instruction: DEX, mode: Implied, length: 1 }),
        
        // DEY - Decrement Y Register
        0x88 => Ok(DecodedInstruction { instruction: DEY, mode: Implied, length: 1 }),
        
        // AND - Logical AND
        0x29 => Ok(DecodedInstruction { instruction: AND, mode: Immediate, length: 2 }),
        0x25 => Ok(DecodedInstruction { instruction: AND, mode: ZeroPage, length: 2 }),
        0x35 => Ok(DecodedInstruction { instruction: AND, mode: ZeroPageX, length: 2 }),
        0x2D => Ok(DecodedInstruction { instruction: AND, mode: Absolute, length: 3 }),
        0x3D => Ok(DecodedInstruction { instruction: AND, mode: AbsoluteX, length: 3 }),
        0x39 => Ok(DecodedInstruction { instruction: AND, mode: AbsoluteY, length: 3 }),
        0x21 => Ok(DecodedInstruction { instruction: AND, mode: IndexedIndirect, length: 2 }),
        0x31 => Ok(DecodedInstruction { instruction: AND, mode: IndirectIndexed, length: 2 }),
        
        // ORA - Logical OR
        0x09 => Ok(DecodedInstruction { instruction: ORA, mode: Immediate, length: 2 }),
        0x05 => Ok(DecodedInstruction { instruction: ORA, mode: ZeroPage, length: 2 }),
        0x15 => Ok(DecodedInstruction { instruction: ORA, mode: ZeroPageX, length: 2 }),
        0x0D => Ok(DecodedInstruction { instruction: ORA, mode: Absolute, length: 3 }),
        0x1D => Ok(DecodedInstruction { instruction: ORA, mode: AbsoluteX, length: 3 }),
        0x19 => Ok(DecodedInstruction { instruction: ORA, mode: AbsoluteY, length: 3 }),
        0x01 => Ok(DecodedInstruction { instruction: ORA, mode: IndexedIndirect, length: 2 }),
        0x11 => Ok(DecodedInstruction { instruction: ORA, mode: IndirectIndexed, length: 2 }),
        
        // EOR - Logical Exclusive OR
        0x49 => Ok(DecodedInstruction { instruction: EOR, mode: Immediate, length: 2 }),
        0x45 => Ok(DecodedInstruction { instruction: EOR, mode: ZeroPage, length: 2 }),
        0x55 => Ok(DecodedInstruction { instruction: EOR, mode: ZeroPageX, length: 2 }),
        0x4D => Ok(DecodedInstruction { instruction: EOR, mode: Absolute, length: 3 }),
        0x5D => Ok(DecodedInstruction { instruction: EOR, mode: AbsoluteX, length: 3 }),
        0x59 => Ok(DecodedInstruction { instruction: EOR, mode: AbsoluteY, length: 3 }),
        0x41 => Ok(DecodedInstruction { instruction: EOR, mode: IndexedIndirect, length: 2 }),
        0x51 => Ok(DecodedInstruction { instruction: EOR, mode: IndirectIndexed, length: 2 }),
        
        // ASL - Arithmetic Shift Left
        0x0A => Ok(DecodedInstruction { instruction: ASL, mode: Accumulator, length: 1 }),
        0x06 => Ok(DecodedInstruction { instruction: ASL, mode: ZeroPage, length: 2 }),
        0x16 => Ok(DecodedInstruction { instruction: ASL, mode: ZeroPageX, length: 2 }),
        0x0E => Ok(DecodedInstruction { instruction: ASL, mode: Absolute, length: 3 }),
        0x1E => Ok(DecodedInstruction { instruction: ASL, mode: AbsoluteX, length: 3 }),
        
        // LSR - Logical Shift Right
        0x4A => Ok(DecodedInstruction { instruction: LSR, mode: Accumulator, length: 1 }),
        0x46 => Ok(DecodedInstruction { instruction: LSR, mode: ZeroPage, length: 2 }),
        0x56 => Ok(DecodedInstruction { instruction: LSR, mode: ZeroPageX, length: 2 }),
        0x4E => Ok(DecodedInstruction { instruction: LSR, mode: Absolute, length: 3 }),
        0x5E => Ok(DecodedInstruction { instruction: LSR, mode: AbsoluteX, length: 3 }),
        
        // ROL - Rotate Left
        0x2A => Ok(DecodedInstruction { instruction: ROL, mode: Accumulator, length: 1 }),
        0x26 => Ok(DecodedInstruction { instruction: ROL, mode: ZeroPage, length: 2 }),
        0x36 => Ok(DecodedInstruction { instruction: ROL, mode: ZeroPageX, length: 2 }),
        0x2E => Ok(DecodedInstruction { instruction: ROL, mode: Absolute, length: 3 }),
        0x3E => Ok(DecodedInstruction { instruction: ROL, mode: AbsoluteX, length: 3 }),
        
        // ROR - Rotate Right
        0x6A => Ok(DecodedInstruction { instruction: ROR, mode: Accumulator, length: 1 }),
        0x66 => Ok(DecodedInstruction { instruction: ROR, mode: ZeroPage, length: 2 }),
        0x76 => Ok(DecodedInstruction { instruction: ROR, mode: ZeroPageX, length: 2 }),
        0x6E => Ok(DecodedInstruction { instruction: ROR, mode: Absolute, length: 3 }),
        0x7E => Ok(DecodedInstruction { instruction: ROR, mode: AbsoluteX, length: 3 }),
        
        // CMP - Compare Accumulator
        0xC9 => Ok(DecodedInstruction { instruction: CMP, mode: Immediate, length: 2 }),
        0xC5 => Ok(DecodedInstruction { instruction: CMP, mode: ZeroPage, length: 2 }),
        0xD5 => Ok(DecodedInstruction { instruction: CMP, mode: ZeroPageX, length: 2 }),
        0xCD => Ok(DecodedInstruction { instruction: CMP, mode: Absolute, length: 3 }),
        0xDD => Ok(DecodedInstruction { instruction: CMP, mode: AbsoluteX, length: 3 }),
        0xD9 => Ok(DecodedInstruction { instruction: CMP, mode: AbsoluteY, length: 3 }),
        0xC1 => Ok(DecodedInstruction { instruction: CMP, mode: IndexedIndirect, length: 2 }),
        0xD1 => Ok(DecodedInstruction { instruction: CMP, mode: IndirectIndexed, length: 2 }),
        
        // CPX - Compare X Register
        0xE0 => Ok(DecodedInstruction { instruction: CPX, mode: Immediate, length: 2 }),
        0xE4 => Ok(DecodedInstruction { instruction: CPX, mode: ZeroPage, length: 2 }),
        0xEC => Ok(DecodedInstruction { instruction: CPX, mode: Absolute, length: 3 }),
        
        // CPY - Compare Y Register
        0xC0 => Ok(DecodedInstruction { instruction: CPY, mode: Immediate, length: 2 }),
        0xC4 => Ok(DecodedInstruction { instruction: CPY, mode: ZeroPage, length: 2 }),
        0xCC => Ok(DecodedInstruction { instruction: CPY, mode: Absolute, length: 3 }),
        
        // BCC - Branch if Carry Clear
        0x90 => Ok(DecodedInstruction { instruction: BCC, mode: Relative, length: 2 }),
        
        // BCS - Branch if Carry Set
        0xB0 => Ok(DecodedInstruction { instruction: BCS, mode: Relative, length: 2 }),
        
        // BEQ - Branch if Equal (Zero Set)
        0xF0 => Ok(DecodedInstruction { instruction: BEQ, mode: Relative, length: 2 }),
        
        // BMI - Branch if Minus (Negative Set)
        0x30 => Ok(DecodedInstruction { instruction: BMI, mode: Relative, length: 2 }),
        
        // BNE - Branch if Not Equal (Zero Clear)
        0xD0 => Ok(DecodedInstruction { instruction: BNE, mode: Relative, length: 2 }),
        
        // BPL - Branch if Plus (Negative Clear)
        0x10 => Ok(DecodedInstruction { instruction: BPL, mode: Relative, length: 2 }),
        
        // BVC - Branch if Overflow Clear
        0x50 => Ok(DecodedInstruction { instruction: BVC, mode: Relative, length: 2 }),
        
        // BVS - Branch if Overflow Set
        0x70 => Ok(DecodedInstruction { instruction: BVS, mode: Relative, length: 2 }),
        
        // JMP - Jump
        0x4C => Ok(DecodedInstruction { instruction: JMP, mode: Absolute, length: 3 }),
        0x6C => Ok(DecodedInstruction { instruction: JMP, mode: Indirect, length: 3 }),
        
        // JSR - Jump to Subroutine
        0x20 => Ok(DecodedInstruction { instruction: JSR, mode: Absolute, length: 3 }),
        
        // RTS - Return from Subroutine
        0x60 => Ok(DecodedInstruction { instruction: RTS, mode: Implied, length: 1 }),
        
        // RTI - Return from Interrupt
        0x40 => Ok(DecodedInstruction { instruction: RTI, mode: Implied, length: 1 }),
        
        // PHA - Push Accumulator
        0x48 => Ok(DecodedInstruction { instruction: PHA, mode: Implied, length: 1 }),
        
        // PHP - Push Processor Status
        0x08 => Ok(DecodedInstruction { instruction: PHP, mode: Implied, length: 1 }),
        
        // PLA - Pull Accumulator
        0x68 => Ok(DecodedInstruction { instruction: PLA, mode: Implied, length: 1 }),
        
        // PLP - Pull Processor Status
        0x28 => Ok(DecodedInstruction { instruction: PLP, mode: Implied, length: 1 }),
        
        // TSX - Transfer Stack Pointer to X
        0xBA => Ok(DecodedInstruction { instruction: TSX, mode: Implied, length: 1 }),
        
        // TXS - Transfer X to Stack Pointer
        0x9A => Ok(DecodedInstruction { instruction: TXS, mode: Implied, length: 1 }),
        
        // CLC - Clear Carry Flag
        0x18 => Ok(DecodedInstruction { instruction: CLC, mode: Implied, length: 1 }),
        
        // CLD - Clear Decimal Flag
        0xD8 => Ok(DecodedInstruction { instruction: CLD, mode: Implied, length: 1 }),
        
        // CLI - Clear Interrupt Disable Flag
        0x58 => Ok(DecodedInstruction { instruction: CLI, mode: Implied, length: 1 }),
        
        // CLV - Clear Overflow Flag
        0xB8 => Ok(DecodedInstruction { instruction: CLV, mode: Implied, length: 1 }),
        
        // SEC - Set Carry Flag
        0x38 => Ok(DecodedInstruction { instruction: SEC, mode: Implied, length: 1 }),
        
        // SED - Set Decimal Flag
        0xF8 => Ok(DecodedInstruction { instruction: SED, mode: Implied, length: 1 }),
        
        // SEI - Set Interrupt Disable Flag
        0x78 => Ok(DecodedInstruction { instruction: SEI, mode: Implied, length: 1 }),
        
        // TAX - Transfer Accumulator to X
        0xAA => Ok(DecodedInstruction { instruction: TAX, mode: Implied, length: 1 }),
        
        // TAY - Transfer Accumulator to Y
        0xA8 => Ok(DecodedInstruction { instruction: TAY, mode: Implied, length: 1 }),
        
        // TXA - Transfer X to Accumulator
        0x8A => Ok(DecodedInstruction { instruction: TXA, mode: Implied, length: 1 }),
        
        // TYA - Transfer Y to Accumulator
        0x98 => Ok(DecodedInstruction { instruction: TYA, mode: Implied, length: 1 }),
        
        // NOP - No Operation
        0xEA => Ok(DecodedInstruction { instruction: NOP, mode: Implied, length: 1 }),
        
        // BRK - Break
        0x00 => Ok(DecodedInstruction { instruction: BRK, mode: Implied, length: 1 }),
        
        // Invalid opcode - return error
        _ => Err(format!("Invalid opcode: 0x{:02X}", opcode)),
    }
}
