// Unit tests for shift and rotate instructions (ASL, LSR, ROL, ROR)

use cpu_6502_emulator::cpu::Cpu;
use cpu_6502_emulator::memory::Memory;

// ============================================================================
// ASL (Arithmetic Shift Left) Tests
// ============================================================================

#[test]
fn test_asl_accumulator() {
    let mut memory = Memory::new();
    memory.write(0x1000, 0x0A);  // ASL A opcode
    
    let mut cpu = Cpu::new(memory, 0x1000);
    cpu.state.a = 0x42;
    
    cpu.step().unwrap();
    
    assert_eq!(cpu.state.a, 0x84);  // 0x42 << 1
    assert!(!cpu.state.flag_carry);
    assert!(cpu.state.flag_negative);
}

#[test]
fn test_asl_sets_carry_flag() {
    let mut memory = Memory::new();
    memory.write(0x1000, 0x0A);  // ASL A opcode
    
    let mut cpu = Cpu::new(memory, 0x1000);
    cpu.state.a = 0x80;  // Bit 7 is set
    
    cpu.step().unwrap();
    
    assert_eq!(cpu.state.a, 0x00);
    assert!(cpu.state.flag_carry);  // Bit 7 shifted into carry
    assert!(cpu.state.flag_zero);
}

#[test]
fn test_asl_zero_page() {
    let mut memory = Memory::new();
    memory.write(0x0042, 0x55);  // Value at zero page
    memory.write(0x1000, 0x06);  // ASL $42 opcode
    memory.write(0x1001, 0x42);
    
    let mut cpu = Cpu::new(memory, 0x1000);
    
    cpu.step().unwrap();
    
    assert_eq!(cpu.memory.read(0x0042), 0xAA);  // 0x55 << 1
    assert!(!cpu.state.flag_carry);
    assert!(cpu.state.flag_negative);
}

// ============================================================================
// LSR (Logical Shift Right) Tests
// ============================================================================

#[test]
fn test_lsr_accumulator() {
    let mut memory = Memory::new();
    memory.write(0x1000, 0x4A);  // LSR A opcode
    
    let mut cpu = Cpu::new(memory, 0x1000);
    cpu.state.a = 0x42;
    
    cpu.step().unwrap();
    
    assert_eq!(cpu.state.a, 0x21);  // 0x42 >> 1
    assert!(!cpu.state.flag_carry);
    assert!(!cpu.state.flag_negative);
}

#[test]
fn test_lsr_sets_carry_flag() {
    let mut memory = Memory::new();
    memory.write(0x1000, 0x4A);  // LSR A opcode
    
    let mut cpu = Cpu::new(memory, 0x1000);
    cpu.state.a = 0x01;  // Bit 0 is set
    
    cpu.step().unwrap();
    
    assert_eq!(cpu.state.a, 0x00);
    assert!(cpu.state.flag_carry);  // Bit 0 shifted into carry
    assert!(cpu.state.flag_zero);
}

#[test]
fn test_lsr_clears_negative_flag() {
    let mut memory = Memory::new();
    memory.write(0x1000, 0x4A);  // LSR A opcode
    
    let mut cpu = Cpu::new(memory, 0x1000);
    cpu.state.a = 0xFF;
    
    cpu.step().unwrap();
    
    assert_eq!(cpu.state.a, 0x7F);  // 0xFF >> 1
    assert!(!cpu.state.flag_negative);  // Always clears negative
}

#[test]
fn test_lsr_zero_page() {
    let mut memory = Memory::new();
    memory.write(0x0042, 0xAA);  // Value at zero page
    memory.write(0x1000, 0x46);  // LSR $42 opcode
    memory.write(0x1001, 0x42);
    
    let mut cpu = Cpu::new(memory, 0x1000);
    
    cpu.step().unwrap();
    
    assert_eq!(cpu.memory.read(0x0042), 0x55);  // 0xAA >> 1
    assert!(!cpu.state.flag_carry);
}

// ============================================================================
// ROL (Rotate Left) Tests
// ============================================================================

#[test]
fn test_rol_accumulator_no_carry() {
    let mut memory = Memory::new();
    memory.write(0x1000, 0x2A);  // ROL A opcode
    
    let mut cpu = Cpu::new(memory, 0x1000);
    cpu.state.a = 0x42;
    cpu.state.flag_carry = false;
    
    cpu.step().unwrap();
    
    assert_eq!(cpu.state.a, 0x84);  // 0x42 << 1, carry in = 0
    assert!(!cpu.state.flag_carry);
}

#[test]
fn test_rol_accumulator_with_carry() {
    let mut memory = Memory::new();
    memory.write(0x1000, 0x2A);  // ROL A opcode
    
    let mut cpu = Cpu::new(memory, 0x1000);
    cpu.state.a = 0x42;
    cpu.state.flag_carry = true;
    
    cpu.step().unwrap();
    
    assert_eq!(cpu.state.a, 0x85);  // 0x42 << 1 | 1
    assert!(!cpu.state.flag_carry);
}

#[test]
fn test_rol_sets_carry_flag() {
    let mut memory = Memory::new();
    memory.write(0x1000, 0x2A);  // ROL A opcode
    
    let mut cpu = Cpu::new(memory, 0x1000);
    cpu.state.a = 0x80;  // Bit 7 is set
    cpu.state.flag_carry = false;
    
    cpu.step().unwrap();
    
    assert_eq!(cpu.state.a, 0x00);
    assert!(cpu.state.flag_carry);  // Bit 7 rotated into carry
}

#[test]
fn test_rol_zero_page() {
    let mut memory = Memory::new();
    memory.write(0x0042, 0x55);  // Value at zero page
    memory.write(0x1000, 0x26);  // ROL $42 opcode
    memory.write(0x1001, 0x42);
    
    let mut cpu = Cpu::new(memory, 0x1000);
    cpu.state.flag_carry = true;
    
    cpu.step().unwrap();
    
    assert_eq!(cpu.memory.read(0x0042), 0xAB);  // 0x55 << 1 | 1
    assert!(!cpu.state.flag_carry);
}

// ============================================================================
// ROR (Rotate Right) Tests
// ============================================================================

#[test]
fn test_ror_accumulator_no_carry() {
    let mut memory = Memory::new();
    memory.write(0x1000, 0x6A);  // ROR A opcode
    
    let mut cpu = Cpu::new(memory, 0x1000);
    cpu.state.a = 0x42;
    cpu.state.flag_carry = false;
    
    cpu.step().unwrap();
    
    assert_eq!(cpu.state.a, 0x21);  // 0x42 >> 1, carry in = 0
    assert!(!cpu.state.flag_carry);
}

#[test]
fn test_ror_accumulator_with_carry() {
    let mut memory = Memory::new();
    memory.write(0x1000, 0x6A);  // ROR A opcode
    
    let mut cpu = Cpu::new(memory, 0x1000);
    cpu.state.a = 0x42;
    cpu.state.flag_carry = true;
    
    cpu.step().unwrap();
    
    assert_eq!(cpu.state.a, 0xA1);  // 0x42 >> 1 | 0x80
    assert!(!cpu.state.flag_carry);
    assert!(cpu.state.flag_negative);
}

#[test]
fn test_ror_sets_carry_flag() {
    let mut memory = Memory::new();
    memory.write(0x1000, 0x6A);  // ROR A opcode
    
    let mut cpu = Cpu::new(memory, 0x1000);
    cpu.state.a = 0x01;  // Bit 0 is set
    cpu.state.flag_carry = false;
    
    cpu.step().unwrap();
    
    assert_eq!(cpu.state.a, 0x00);
    assert!(cpu.state.flag_carry);  // Bit 0 rotated into carry
}

#[test]
fn test_ror_zero_page() {
    let mut memory = Memory::new();
    memory.write(0x0042, 0xAA);  // Value at zero page
    memory.write(0x1000, 0x66);  // ROR $42 opcode
    memory.write(0x1001, 0x42);
    
    let mut cpu = Cpu::new(memory, 0x1000);
    cpu.state.flag_carry = true;
    
    cpu.step().unwrap();
    
    assert_eq!(cpu.memory.read(0x0042), 0xD5);  // 0xAA >> 1 | 0x80
    assert!(!cpu.state.flag_carry);
}
