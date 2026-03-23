// Unit tests for opcode decoding
// Tests specific opcodes and edge cases for the decode_opcode function

use cpu_6502_emulator::instruction::{decode_opcode, Instruction, AddressingMode};

#[test]
fn test_lda_opcodes() {
    // Test all LDA addressing modes
    let result = decode_opcode(0xA9).unwrap();
    assert_eq!(result.instruction, Instruction::LDA);
    assert_eq!(result.mode, AddressingMode::Immediate);
    assert_eq!(result.length, 2);
    
    let result = decode_opcode(0xA5).unwrap();
    assert_eq!(result.instruction, Instruction::LDA);
    assert_eq!(result.mode, AddressingMode::ZeroPage);
    assert_eq!(result.length, 2);
    
    let result = decode_opcode(0xB5).unwrap();
    assert_eq!(result.instruction, Instruction::LDA);
    assert_eq!(result.mode, AddressingMode::ZeroPageX);
    assert_eq!(result.length, 2);
    
    let result = decode_opcode(0xAD).unwrap();
    assert_eq!(result.instruction, Instruction::LDA);
    assert_eq!(result.mode, AddressingMode::Absolute);
    assert_eq!(result.length, 3);
    
    let result = decode_opcode(0xBD).unwrap();
    assert_eq!(result.instruction, Instruction::LDA);
    assert_eq!(result.mode, AddressingMode::AbsoluteX);
    assert_eq!(result.length, 3);
    
    let result = decode_opcode(0xB9).unwrap();
    assert_eq!(result.instruction, Instruction::LDA);
    assert_eq!(result.mode, AddressingMode::AbsoluteY);
    assert_eq!(result.length, 3);
    
    let result = decode_opcode(0xA1).unwrap();
    assert_eq!(result.instruction, Instruction::LDA);
    assert_eq!(result.mode, AddressingMode::IndexedIndirect);
    assert_eq!(result.length, 2);
    
    let result = decode_opcode(0xB1).unwrap();
    assert_eq!(result.instruction, Instruction::LDA);
    assert_eq!(result.mode, AddressingMode::IndirectIndexed);
    assert_eq!(result.length, 2);
}

#[test]
fn test_store_opcodes() {
    // Test STA
    let result = decode_opcode(0x85).unwrap();
    assert_eq!(result.instruction, Instruction::STA);
    assert_eq!(result.mode, AddressingMode::ZeroPage);
    
    // Test STX
    let result = decode_opcode(0x86).unwrap();
    assert_eq!(result.instruction, Instruction::STX);
    assert_eq!(result.mode, AddressingMode::ZeroPage);
    
    // Test STY
    let result = decode_opcode(0x84).unwrap();
    assert_eq!(result.instruction, Instruction::STY);
    assert_eq!(result.mode, AddressingMode::ZeroPage);
}

#[test]
fn test_arithmetic_opcodes() {
    // Test ADC
    let result = decode_opcode(0x69).unwrap();
    assert_eq!(result.instruction, Instruction::ADC);
    assert_eq!(result.mode, AddressingMode::Immediate);
    
    // Test SBC
    let result = decode_opcode(0xE9).unwrap();
    assert_eq!(result.instruction, Instruction::SBC);
    assert_eq!(result.mode, AddressingMode::Immediate);
    
    // Test INC
    let result = decode_opcode(0xE6).unwrap();
    assert_eq!(result.instruction, Instruction::INC);
    assert_eq!(result.mode, AddressingMode::ZeroPage);
    
    // Test INX
    let result = decode_opcode(0xE8).unwrap();
    assert_eq!(result.instruction, Instruction::INX);
    assert_eq!(result.mode, AddressingMode::Implied);
    assert_eq!(result.length, 1);
    
    // Test DEC
    let result = decode_opcode(0xC6).unwrap();
    assert_eq!(result.instruction, Instruction::DEC);
    assert_eq!(result.mode, AddressingMode::ZeroPage);
    
    // Test DEX
    let result = decode_opcode(0xCA).unwrap();
    assert_eq!(result.instruction, Instruction::DEX);
    assert_eq!(result.mode, AddressingMode::Implied);
}

#[test]
fn test_logical_opcodes() {
    // Test AND
    let result = decode_opcode(0x29).unwrap();
    assert_eq!(result.instruction, Instruction::AND);
    assert_eq!(result.mode, AddressingMode::Immediate);
    
    // Test ORA
    let result = decode_opcode(0x09).unwrap();
    assert_eq!(result.instruction, Instruction::ORA);
    assert_eq!(result.mode, AddressingMode::Immediate);
    
    // Test EOR
    let result = decode_opcode(0x49).unwrap();
    assert_eq!(result.instruction, Instruction::EOR);
    assert_eq!(result.mode, AddressingMode::Immediate);
}

#[test]
fn test_shift_rotate_opcodes() {
    // Test ASL accumulator mode
    let result = decode_opcode(0x0A).unwrap();
    assert_eq!(result.instruction, Instruction::ASL);
    assert_eq!(result.mode, AddressingMode::Accumulator);
    assert_eq!(result.length, 1);
    
    // Test ASL zero page mode
    let result = decode_opcode(0x06).unwrap();
    assert_eq!(result.instruction, Instruction::ASL);
    assert_eq!(result.mode, AddressingMode::ZeroPage);
    assert_eq!(result.length, 2);
    
    // Test LSR
    let result = decode_opcode(0x4A).unwrap();
    assert_eq!(result.instruction, Instruction::LSR);
    assert_eq!(result.mode, AddressingMode::Accumulator);
    
    // Test ROL
    let result = decode_opcode(0x2A).unwrap();
    assert_eq!(result.instruction, Instruction::ROL);
    assert_eq!(result.mode, AddressingMode::Accumulator);
    
    // Test ROR
    let result = decode_opcode(0x6A).unwrap();
    assert_eq!(result.instruction, Instruction::ROR);
    assert_eq!(result.mode, AddressingMode::Accumulator);
}

#[test]
fn test_comparison_opcodes() {
    // Test CMP
    let result = decode_opcode(0xC9).unwrap();
    assert_eq!(result.instruction, Instruction::CMP);
    assert_eq!(result.mode, AddressingMode::Immediate);
    
    // Test CPX
    let result = decode_opcode(0xE0).unwrap();
    assert_eq!(result.instruction, Instruction::CPX);
    assert_eq!(result.mode, AddressingMode::Immediate);
    
    // Test CPY
    let result = decode_opcode(0xC0).unwrap();
    assert_eq!(result.instruction, Instruction::CPY);
    assert_eq!(result.mode, AddressingMode::Immediate);
}

#[test]
fn test_branch_opcodes() {
    // All branch instructions use Relative addressing and are 2 bytes
    let branches = [
        (0x90, Instruction::BCC),
        (0xB0, Instruction::BCS),
        (0xF0, Instruction::BEQ),
        (0x30, Instruction::BMI),
        (0xD0, Instruction::BNE),
        (0x10, Instruction::BPL),
        (0x50, Instruction::BVC),
        (0x70, Instruction::BVS),
    ];
    
    for (opcode, expected_instruction) in branches {
        let result = decode_opcode(opcode).unwrap();
        assert_eq!(result.instruction, expected_instruction);
        assert_eq!(result.mode, AddressingMode::Relative);
        assert_eq!(result.length, 2);
    }
}

#[test]
fn test_jump_subroutine_opcodes() {
    // Test JMP absolute
    let result = decode_opcode(0x4C).unwrap();
    assert_eq!(result.instruction, Instruction::JMP);
    assert_eq!(result.mode, AddressingMode::Absolute);
    assert_eq!(result.length, 3);
    
    // Test JMP indirect
    let result = decode_opcode(0x6C).unwrap();
    assert_eq!(result.instruction, Instruction::JMP);
    assert_eq!(result.mode, AddressingMode::Indirect);
    assert_eq!(result.length, 3);
    
    // Test JSR
    let result = decode_opcode(0x20).unwrap();
    assert_eq!(result.instruction, Instruction::JSR);
    assert_eq!(result.mode, AddressingMode::Absolute);
    assert_eq!(result.length, 3);
    
    // Test RTS
    let result = decode_opcode(0x60).unwrap();
    assert_eq!(result.instruction, Instruction::RTS);
    assert_eq!(result.mode, AddressingMode::Implied);
    assert_eq!(result.length, 1);
    
    // Test RTI
    let result = decode_opcode(0x40).unwrap();
    assert_eq!(result.instruction, Instruction::RTI);
    assert_eq!(result.mode, AddressingMode::Implied);
    assert_eq!(result.length, 1);
}

#[test]
fn test_stack_opcodes() {
    // Test PHA
    let result = decode_opcode(0x48).unwrap();
    assert_eq!(result.instruction, Instruction::PHA);
    assert_eq!(result.mode, AddressingMode::Implied);
    
    // Test PHP
    let result = decode_opcode(0x08).unwrap();
    assert_eq!(result.instruction, Instruction::PHP);
    assert_eq!(result.mode, AddressingMode::Implied);
    
    // Test PLA
    let result = decode_opcode(0x68).unwrap();
    assert_eq!(result.instruction, Instruction::PLA);
    assert_eq!(result.mode, AddressingMode::Implied);
    
    // Test PLP
    let result = decode_opcode(0x28).unwrap();
    assert_eq!(result.instruction, Instruction::PLP);
    assert_eq!(result.mode, AddressingMode::Implied);
    
    // Test TSX
    let result = decode_opcode(0xBA).unwrap();
    assert_eq!(result.instruction, Instruction::TSX);
    assert_eq!(result.mode, AddressingMode::Implied);
    
    // Test TXS
    let result = decode_opcode(0x9A).unwrap();
    assert_eq!(result.instruction, Instruction::TXS);
    assert_eq!(result.mode, AddressingMode::Implied);
}

#[test]
fn test_flag_opcodes() {
    // Test flag clear instructions
    let result = decode_opcode(0x18).unwrap();
    assert_eq!(result.instruction, Instruction::CLC);
    assert_eq!(result.mode, AddressingMode::Implied);
    
    let result = decode_opcode(0xD8).unwrap();
    assert_eq!(result.instruction, Instruction::CLD);
    
    let result = decode_opcode(0x58).unwrap();
    assert_eq!(result.instruction, Instruction::CLI);
    
    let result = decode_opcode(0xB8).unwrap();
    assert_eq!(result.instruction, Instruction::CLV);
    
    // Test flag set instructions
    let result = decode_opcode(0x38).unwrap();
    assert_eq!(result.instruction, Instruction::SEC);
    
    let result = decode_opcode(0xF8).unwrap();
    assert_eq!(result.instruction, Instruction::SED);
    
    let result = decode_opcode(0x78).unwrap();
    assert_eq!(result.instruction, Instruction::SEI);
}

#[test]
fn test_transfer_opcodes() {
    // Test TAX
    let result = decode_opcode(0xAA).unwrap();
    assert_eq!(result.instruction, Instruction::TAX);
    assert_eq!(result.mode, AddressingMode::Implied);
    
    // Test TAY
    let result = decode_opcode(0xA8).unwrap();
    assert_eq!(result.instruction, Instruction::TAY);
    
    // Test TXA
    let result = decode_opcode(0x8A).unwrap();
    assert_eq!(result.instruction, Instruction::TXA);
    
    // Test TYA
    let result = decode_opcode(0x98).unwrap();
    assert_eq!(result.instruction, Instruction::TYA);
}

#[test]
fn test_miscellaneous_opcodes() {
    // Test NOP
    let result = decode_opcode(0xEA).unwrap();
    assert_eq!(result.instruction, Instruction::NOP);
    assert_eq!(result.mode, AddressingMode::Implied);
    assert_eq!(result.length, 1);
    
    // Test BRK
    let result = decode_opcode(0x00).unwrap();
    assert_eq!(result.instruction, Instruction::BRK);
    assert_eq!(result.mode, AddressingMode::Implied);
    assert_eq!(result.length, 1);
}

#[test]
fn test_invalid_opcodes() {
    // Test some known invalid opcodes (unofficial/undocumented)
    let invalid_opcodes = [0x02, 0x03, 0x04, 0x07, 0x0B, 0x0C, 0x0F];
    
    for opcode in invalid_opcodes {
        let result = decode_opcode(opcode);
        assert!(result.is_err(), "Opcode 0x{:02X} should be invalid", opcode);
        assert!(result.unwrap_err().contains("Invalid opcode"));
    }
}

#[test]
fn test_instruction_lengths() {
    // Test that instruction lengths are correct
    // 1-byte instructions (Implied/Accumulator)
    assert_eq!(decode_opcode(0xEA).unwrap().length, 1); // NOP
    assert_eq!(decode_opcode(0x0A).unwrap().length, 1); // ASL A
    assert_eq!(decode_opcode(0xE8).unwrap().length, 1); // INX
    
    // 2-byte instructions (Immediate, ZeroPage, Relative)
    assert_eq!(decode_opcode(0xA9).unwrap().length, 2); // LDA #
    assert_eq!(decode_opcode(0xA5).unwrap().length, 2); // LDA ZP
    assert_eq!(decode_opcode(0x90).unwrap().length, 2); // BCC
    
    // 3-byte instructions (Absolute)
    assert_eq!(decode_opcode(0xAD).unwrap().length, 3); // LDA abs
    assert_eq!(decode_opcode(0x4C).unwrap().length, 3); // JMP abs
    assert_eq!(decode_opcode(0x20).unwrap().length, 3); // JSR
}

#[test]
fn test_all_load_store_variants() {
    // Verify we have all addressing modes for load instructions
    // LDA: 8 modes
    assert!(decode_opcode(0xA9).is_ok()); // Immediate
    assert!(decode_opcode(0xA5).is_ok()); // ZeroPage
    assert!(decode_opcode(0xB5).is_ok()); // ZeroPageX
    assert!(decode_opcode(0xAD).is_ok()); // Absolute
    assert!(decode_opcode(0xBD).is_ok()); // AbsoluteX
    assert!(decode_opcode(0xB9).is_ok()); // AbsoluteY
    assert!(decode_opcode(0xA1).is_ok()); // IndexedIndirect
    assert!(decode_opcode(0xB1).is_ok()); // IndirectIndexed
    
    // LDX: 5 modes
    assert!(decode_opcode(0xA2).is_ok()); // Immediate
    assert!(decode_opcode(0xA6).is_ok()); // ZeroPage
    assert!(decode_opcode(0xB6).is_ok()); // ZeroPageY
    assert!(decode_opcode(0xAE).is_ok()); // Absolute
    assert!(decode_opcode(0xBE).is_ok()); // AbsoluteY
    
    // LDY: 5 modes
    assert!(decode_opcode(0xA0).is_ok()); // Immediate
    assert!(decode_opcode(0xA4).is_ok()); // ZeroPage
    assert!(decode_opcode(0xB4).is_ok()); // ZeroPageX
    assert!(decode_opcode(0xAC).is_ok()); // Absolute
    assert!(decode_opcode(0xBC).is_ok()); // AbsoluteX
}
