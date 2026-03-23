// Unit tests for load instructions (LDA, LDX, LDY)

use cpu_6502_emulator::cpu::Cpu;
use cpu_6502_emulator::memory::Memory;
use cpu_6502_emulator::instruction::{decode_opcode, Instruction};

#[test]
fn test_lda_immediate() {
    let mut memory = Memory::new();
    // LDA #$42
    memory.write(0x0000, 0xA9); // LDA immediate opcode
    memory.write(0x0001, 0x42); // operand
    
    let mut cpu = Cpu::new(memory, 0x0000);
    let decoded = decode_opcode(0xA9).unwrap();
    
    cpu.execute_instruction(decoded).unwrap();
    
    assert_eq!(cpu.state.a, 0x42);
    assert!(!cpu.state.flag_zero);
    assert!(!cpu.state.flag_negative);
}

#[test]
fn test_lda_zero_flag() {
    let mut memory = Memory::new();
    // LDA #$00
    memory.write(0x0000, 0xA9);
    memory.write(0x0001, 0x00);
    
    let mut cpu = Cpu::new(memory, 0x0000);
    let decoded = decode_opcode(0xA9).unwrap();
    
    cpu.execute_instruction(decoded).unwrap();
    
    assert_eq!(cpu.state.a, 0x00);
    assert!(cpu.state.flag_zero);
    assert!(!cpu.state.flag_negative);
}

#[test]
fn test_lda_negative_flag() {
    let mut memory = Memory::new();
    // LDA #$80 (bit 7 set)
    memory.write(0x0000, 0xA9);
    memory.write(0x0001, 0x80);
    
    let mut cpu = Cpu::new(memory, 0x0000);
    let decoded = decode_opcode(0xA9).unwrap();
    
    cpu.execute_instruction(decoded).unwrap();
    
    assert_eq!(cpu.state.a, 0x80);
    assert!(!cpu.state.flag_zero);
    assert!(cpu.state.flag_negative);
}

#[test]
fn test_lda_zero_page() {
    let mut memory = Memory::new();
    // LDA $50
    memory.write(0x0000, 0xA5); // LDA zero page opcode
    memory.write(0x0001, 0x50); // zero page address
    memory.write(0x0050, 0x33); // value at zero page address
    
    let mut cpu = Cpu::new(memory, 0x0000);
    let decoded = decode_opcode(0xA5).unwrap();
    
    cpu.execute_instruction(decoded).unwrap();
    
    assert_eq!(cpu.state.a, 0x33);
    assert!(!cpu.state.flag_zero);
    assert!(!cpu.state.flag_negative);
}

#[test]
fn test_lda_absolute() {
    let mut memory = Memory::new();
    // LDA $1234
    memory.write(0x0000, 0xAD); // LDA absolute opcode
    memory.write(0x0001, 0x34); // low byte of address
    memory.write(0x0002, 0x12); // high byte of address
    memory.write(0x1234, 0x99); // value at absolute address
    
    let mut cpu = Cpu::new(memory, 0x0000);
    let decoded = decode_opcode(0xAD).unwrap();
    
    cpu.execute_instruction(decoded).unwrap();
    
    assert_eq!(cpu.state.a, 0x99);
    assert!(!cpu.state.flag_zero);
    assert!(cpu.state.flag_negative);
}

#[test]
fn test_ldx_immediate() {
    let mut memory = Memory::new();
    // LDX #$55
    memory.write(0x0000, 0xA2); // LDX immediate opcode
    memory.write(0x0001, 0x55); // operand
    
    let mut cpu = Cpu::new(memory, 0x0000);
    let decoded = decode_opcode(0xA2).unwrap();
    
    cpu.execute_instruction(decoded).unwrap();
    
    assert_eq!(cpu.state.x, 0x55);
    assert!(!cpu.state.flag_zero);
    assert!(!cpu.state.flag_negative);
}

#[test]
fn test_ldx_zero_page() {
    let mut memory = Memory::new();
    // LDX $40
    memory.write(0x0000, 0xA6); // LDX zero page opcode
    memory.write(0x0001, 0x40); // zero page address
    memory.write(0x0040, 0x77); // value at zero page address
    
    let mut cpu = Cpu::new(memory, 0x0000);
    let decoded = decode_opcode(0xA6).unwrap();
    
    cpu.execute_instruction(decoded).unwrap();
    
    assert_eq!(cpu.state.x, 0x77);
    assert!(!cpu.state.flag_zero);
    assert!(!cpu.state.flag_negative);
}

#[test]
fn test_ldx_zero_flag() {
    let mut memory = Memory::new();
    // LDX #$00
    memory.write(0x0000, 0xA2);
    memory.write(0x0001, 0x00);
    
    let mut cpu = Cpu::new(memory, 0x0000);
    let decoded = decode_opcode(0xA2).unwrap();
    
    cpu.execute_instruction(decoded).unwrap();
    
    assert_eq!(cpu.state.x, 0x00);
    assert!(cpu.state.flag_zero);
    assert!(!cpu.state.flag_negative);
}

#[test]
fn test_ldy_immediate() {
    let mut memory = Memory::new();
    // LDY #$AA
    memory.write(0x0000, 0xA0); // LDY immediate opcode
    memory.write(0x0001, 0xAA); // operand
    
    let mut cpu = Cpu::new(memory, 0x0000);
    let decoded = decode_opcode(0xA0).unwrap();
    
    cpu.execute_instruction(decoded).unwrap();
    
    assert_eq!(cpu.state.y, 0xAA);
    assert!(!cpu.state.flag_zero);
    assert!(cpu.state.flag_negative);
}

#[test]
fn test_ldy_absolute() {
    let mut memory = Memory::new();
    // LDY $2000
    memory.write(0x0000, 0xAC); // LDY absolute opcode
    memory.write(0x0001, 0x00); // low byte of address
    memory.write(0x0002, 0x20); // high byte of address
    memory.write(0x2000, 0x11); // value at absolute address
    
    let mut cpu = Cpu::new(memory, 0x0000);
    let decoded = decode_opcode(0xAC).unwrap();
    
    cpu.execute_instruction(decoded).unwrap();
    
    assert_eq!(cpu.state.y, 0x11);
    assert!(!cpu.state.flag_zero);
    assert!(!cpu.state.flag_negative);
}

#[test]
fn test_ldy_zero_flag() {
    let mut memory = Memory::new();
    // LDY #$00
    memory.write(0x0000, 0xA0);
    memory.write(0x0001, 0x00);
    
    let mut cpu = Cpu::new(memory, 0x0000);
    let decoded = decode_opcode(0xA0).unwrap();
    
    cpu.execute_instruction(decoded).unwrap();
    
    assert_eq!(cpu.state.y, 0x00);
    assert!(cpu.state.flag_zero);
    assert!(!cpu.state.flag_negative);
}

#[test]
fn test_load_instructions_dont_affect_other_registers() {
    let mut memory = Memory::new();
    
    // Set up initial state
    let mut cpu = Cpu::new(memory, 0x0000);
    cpu.state.a = 0x11;
    cpu.state.x = 0x22;
    cpu.state.y = 0x33;
    cpu.state.flag_carry = true;
    cpu.state.flag_overflow = true;
    
    // LDA #$42
    cpu.memory.write(0x0000, 0xA9);
    cpu.memory.write(0x0001, 0x42);
    let decoded = decode_opcode(0xA9).unwrap();
    cpu.execute_instruction(decoded).unwrap();
    
    // Check that only A changed, X and Y unchanged
    assert_eq!(cpu.state.a, 0x42);
    assert_eq!(cpu.state.x, 0x22);
    assert_eq!(cpu.state.y, 0x33);
    // Carry and overflow should be unchanged
    assert!(cpu.state.flag_carry);
    assert!(cpu.state.flag_overflow);
}

#[test]
fn test_lda_indexed_indirect() {
    let mut memory = Memory::new();
    // LDA ($40,X) with X = $05
    memory.write(0x0000, 0xA1); // LDA indexed indirect opcode
    memory.write(0x0001, 0x40); // zero page base
    
    // Pointer at $45 (0x40 + 0x05) points to $1234
    memory.write(0x0045, 0x34); // low byte of target address
    memory.write(0x0046, 0x12); // high byte of target address
    
    // Value at $1234
    memory.write(0x1234, 0x88);
    
    let mut cpu = Cpu::new(memory, 0x0000);
    cpu.state.x = 0x05;
    
    let decoded = decode_opcode(0xA1).unwrap();
    cpu.execute_instruction(decoded).unwrap();
    
    assert_eq!(cpu.state.a, 0x88);
    assert!(!cpu.state.flag_zero);
    assert!(cpu.state.flag_negative);
}

#[test]
fn test_lda_indirect_indexed() {
    let mut memory = Memory::new();
    // LDA ($40),Y with Y = $10
    memory.write(0x0000, 0xB1); // LDA indirect indexed opcode
    memory.write(0x0001, 0x40); // zero page pointer
    
    // Pointer at $40 points to $2000
    memory.write(0x0040, 0x00); // low byte of base address
    memory.write(0x0041, 0x20); // high byte of base address
    
    // Value at $2010 ($2000 + $10)
    memory.write(0x2010, 0x66);
    
    let mut cpu = Cpu::new(memory, 0x0000);
    cpu.state.y = 0x10;
    
    let decoded = decode_opcode(0xB1).unwrap();
    cpu.execute_instruction(decoded).unwrap();
    
    assert_eq!(cpu.state.a, 0x66);
    assert!(!cpu.state.flag_zero);
    assert!(!cpu.state.flag_negative);
}
