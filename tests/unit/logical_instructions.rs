// Unit tests for logical instructions (AND, ORA, EOR)

use cpu_6502_emulator::cpu::Cpu;
use cpu_6502_emulator::memory::Memory;

// ============================================================================
// AND (Logical AND) Tests
// ============================================================================

#[test]
fn test_and_immediate() {
    let mut memory = Memory::new();
    memory.write(0x1000, 0x29);  // AND #$0F opcode
    memory.write(0x1001, 0x0F);  // Immediate value
    
    let mut cpu = Cpu::new(memory, 0x1000);
    cpu.state.a = 0xFF;
    
    cpu.step().unwrap();
    
    assert_eq!(cpu.state.a, 0x0F);  // 0xFF & 0x0F
    assert!(!cpu.state.flag_zero);
    assert!(!cpu.state.flag_negative);
}

#[test]
fn test_and_sets_zero_flag() {
    let mut memory = Memory::new();
    memory.write(0x1000, 0x29);  // AND #$00 opcode
    memory.write(0x1001, 0x00);
    
    let mut cpu = Cpu::new(memory, 0x1000);
    cpu.state.a = 0xFF;
    
    cpu.step().unwrap();
    
    assert_eq!(cpu.state.a, 0x00);
    assert!(cpu.state.flag_zero);
}

#[test]
fn test_and_sets_negative_flag() {
    let mut memory = Memory::new();
    memory.write(0x1000, 0x29);  // AND #$80 opcode
    memory.write(0x1001, 0x80);
    
    let mut cpu = Cpu::new(memory, 0x1000);
    cpu.state.a = 0xFF;
    
    cpu.step().unwrap();
    
    assert_eq!(cpu.state.a, 0x80);
    assert!(cpu.state.flag_negative);
}

#[test]
fn test_and_zero_page() {
    let mut memory = Memory::new();
    memory.write(0x0042, 0xF0);  // Value at zero page
    memory.write(0x1000, 0x25);  // AND $42 opcode
    memory.write(0x1001, 0x42);
    
    let mut cpu = Cpu::new(memory, 0x1000);
    cpu.state.a = 0x0F;
    
    cpu.step().unwrap();
    
    assert_eq!(cpu.state.a, 0x00);  // 0x0F & 0xF0
    assert!(cpu.state.flag_zero);
}

// ============================================================================
// ORA (Logical OR) Tests
// ============================================================================

#[test]
fn test_ora_immediate() {
    let mut memory = Memory::new();
    memory.write(0x1000, 0x09);  // ORA #$0F opcode
    memory.write(0x1001, 0x0F);
    
    let mut cpu = Cpu::new(memory, 0x1000);
    cpu.state.a = 0xF0;
    
    cpu.step().unwrap();
    
    assert_eq!(cpu.state.a, 0xFF);  // 0xF0 | 0x0F
    assert!(!cpu.state.flag_zero);
    assert!(cpu.state.flag_negative);
}

#[test]
fn test_ora_sets_zero_flag() {
    let mut memory = Memory::new();
    memory.write(0x1000, 0x09);  // ORA #$00 opcode
    memory.write(0x1001, 0x00);
    
    let mut cpu = Cpu::new(memory, 0x1000);
    cpu.state.a = 0x00;
    
    cpu.step().unwrap();
    
    assert_eq!(cpu.state.a, 0x00);
    assert!(cpu.state.flag_zero);
}

#[test]
fn test_ora_sets_negative_flag() {
    let mut memory = Memory::new();
    memory.write(0x1000, 0x09);  // ORA #$80 opcode
    memory.write(0x1001, 0x80);
    
    let mut cpu = Cpu::new(memory, 0x1000);
    cpu.state.a = 0x00;
    
    cpu.step().unwrap();
    
    assert_eq!(cpu.state.a, 0x80);
    assert!(cpu.state.flag_negative);
}

#[test]
fn test_ora_zero_page() {
    let mut memory = Memory::new();
    memory.write(0x0042, 0x0F);  // Value at zero page
    memory.write(0x1000, 0x05);  // ORA $42 opcode
    memory.write(0x1001, 0x42);
    
    let mut cpu = Cpu::new(memory, 0x1000);
    cpu.state.a = 0xF0;
    
    cpu.step().unwrap();
    
    assert_eq!(cpu.state.a, 0xFF);  // 0xF0 | 0x0F
}

// ============================================================================
// EOR (Logical Exclusive OR) Tests
// ============================================================================

#[test]
fn test_eor_immediate() {
    let mut memory = Memory::new();
    memory.write(0x1000, 0x49);  // EOR #$FF opcode
    memory.write(0x1001, 0xFF);
    
    let mut cpu = Cpu::new(memory, 0x1000);
    cpu.state.a = 0xAA;
    
    cpu.step().unwrap();
    
    assert_eq!(cpu.state.a, 0x55);  // 0xAA ^ 0xFF
    assert!(!cpu.state.flag_zero);
    assert!(!cpu.state.flag_negative);
}

#[test]
fn test_eor_sets_zero_flag() {
    let mut memory = Memory::new();
    memory.write(0x1000, 0x49);  // EOR #$FF opcode
    memory.write(0x1001, 0xFF);
    
    let mut cpu = Cpu::new(memory, 0x1000);
    cpu.state.a = 0xFF;
    
    cpu.step().unwrap();
    
    assert_eq!(cpu.state.a, 0x00);  // 0xFF ^ 0xFF
    assert!(cpu.state.flag_zero);
}

#[test]
fn test_eor_sets_negative_flag() {
    let mut memory = Memory::new();
    memory.write(0x1000, 0x49);  // EOR #$7F opcode
    memory.write(0x1001, 0x7F);
    
    let mut cpu = Cpu::new(memory, 0x1000);
    cpu.state.a = 0xFF;
    
    cpu.step().unwrap();
    
    assert_eq!(cpu.state.a, 0x80);  // 0xFF ^ 0x7F
    assert!(cpu.state.flag_negative);
}

#[test]
fn test_eor_zero_page() {
    let mut memory = Memory::new();
    memory.write(0x0042, 0x55);  // Value at zero page
    memory.write(0x1000, 0x45);  // EOR $42 opcode
    memory.write(0x1001, 0x42);
    
    let mut cpu = Cpu::new(memory, 0x1000);
    cpu.state.a = 0xAA;
    
    cpu.step().unwrap();
    
    assert_eq!(cpu.state.a, 0xFF);  // 0xAA ^ 0x55
}

#[test]
fn test_logical_operations_dont_affect_carry() {
    // Test that AND, ORA, EOR don't modify carry flag
    let mut memory = Memory::new();
    memory.write(0x1000, 0x29);  // AND #$FF
    memory.write(0x1001, 0xFF);
    
    let mut cpu = Cpu::new(memory, 0x1000);
    cpu.state.a = 0xFF;
    cpu.state.flag_carry = true;
    
    cpu.step().unwrap();
    
    assert!(cpu.state.flag_carry);  // Carry should remain set
}
