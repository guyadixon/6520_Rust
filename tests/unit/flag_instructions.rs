// Unit tests for flag manipulation instructions

use cpu_6502_emulator::cpu::Cpu;
use cpu_6502_emulator::memory::Memory;

// ============================================================================
// Flag Clear Instructions Tests
// ============================================================================

#[test]
fn test_clc_clears_carry() {
    let mut memory = Memory::new();
    memory.write(0x1000, 0x18);  // CLC opcode
    
    let mut cpu = Cpu::new(memory, 0x1000);
    cpu.state.flag_carry = true;
    
    cpu.step().unwrap();
    
    assert!(!cpu.state.flag_carry);
}

#[test]
fn test_cld_clears_decimal() {
    let mut memory = Memory::new();
    memory.write(0x1000, 0xD8);  // CLD opcode
    
    let mut cpu = Cpu::new(memory, 0x1000);
    cpu.state.flag_decimal = true;
    
    cpu.step().unwrap();
    
    assert!(!cpu.state.flag_decimal);
}

#[test]
fn test_cli_clears_interrupt_disable() {
    let mut memory = Memory::new();
    memory.write(0x1000, 0x58);  // CLI opcode
    
    let mut cpu = Cpu::new(memory, 0x1000);
    cpu.state.flag_interrupt_disable = true;
    
    cpu.step().unwrap();
    
    assert!(!cpu.state.flag_interrupt_disable);
}

#[test]
fn test_clv_clears_overflow() {
    let mut memory = Memory::new();
    memory.write(0x1000, 0xB8);  // CLV opcode
    
    let mut cpu = Cpu::new(memory, 0x1000);
    cpu.state.flag_overflow = true;
    
    cpu.step().unwrap();
    
    assert!(!cpu.state.flag_overflow);
}

// ============================================================================
// Flag Set Instructions Tests
// ============================================================================

#[test]
fn test_sec_sets_carry() {
    let mut memory = Memory::new();
    memory.write(0x1000, 0x38);  // SEC opcode
    
    let mut cpu = Cpu::new(memory, 0x1000);
    cpu.state.flag_carry = false;
    
    cpu.step().unwrap();
    
    assert!(cpu.state.flag_carry);
}

#[test]
fn test_sed_sets_decimal() {
    let mut memory = Memory::new();
    memory.write(0x1000, 0xF8);  // SED opcode
    
    let mut cpu = Cpu::new(memory, 0x1000);
    cpu.state.flag_decimal = false;
    
    cpu.step().unwrap();
    
    assert!(cpu.state.flag_decimal);
}

#[test]
fn test_sei_sets_interrupt_disable() {
    let mut memory = Memory::new();
    memory.write(0x1000, 0x78);  // SEI opcode
    
    let mut cpu = Cpu::new(memory, 0x1000);
    cpu.state.flag_interrupt_disable = false;
    
    cpu.step().unwrap();
    
    assert!(cpu.state.flag_interrupt_disable);
}

// ============================================================================
// Flag Manipulation Doesn't Affect Other Flags Tests
// ============================================================================

#[test]
fn test_clc_doesnt_affect_other_flags() {
    let mut memory = Memory::new();
    memory.write(0x1000, 0x18);  // CLC opcode
    
    let mut cpu = Cpu::new(memory, 0x1000);
    cpu.state.flag_carry = true;
    cpu.state.flag_zero = true;
    cpu.state.flag_negative = true;
    cpu.state.flag_overflow = true;
    
    cpu.step().unwrap();
    
    assert!(!cpu.state.flag_carry);  // Changed
    assert!(cpu.state.flag_zero);    // Unchanged
    assert!(cpu.state.flag_negative); // Unchanged
    assert!(cpu.state.flag_overflow); // Unchanged
}

#[test]
fn test_sec_doesnt_affect_other_flags() {
    let mut memory = Memory::new();
    memory.write(0x1000, 0x38);  // SEC opcode
    
    let mut cpu = Cpu::new(memory, 0x1000);
    cpu.state.flag_carry = false;
    cpu.state.flag_zero = false;
    cpu.state.flag_negative = false;
    cpu.state.flag_overflow = false;
    
    cpu.step().unwrap();
    
    assert!(cpu.state.flag_carry);    // Changed
    assert!(!cpu.state.flag_zero);    // Unchanged
    assert!(!cpu.state.flag_negative); // Unchanged
    assert!(!cpu.state.flag_overflow); // Unchanged
}

#[test]
fn test_cli_doesnt_affect_other_flags() {
    let mut memory = Memory::new();
    memory.write(0x1000, 0x58);  // CLI opcode
    
    let mut cpu = Cpu::new(memory, 0x1000);
    cpu.state.flag_interrupt_disable = true;
    cpu.state.flag_carry = true;
    cpu.state.flag_zero = true;
    
    cpu.step().unwrap();
    
    assert!(!cpu.state.flag_interrupt_disable); // Changed
    assert!(cpu.state.flag_carry);              // Unchanged
    assert!(cpu.state.flag_zero);               // Unchanged
}

#[test]
fn test_sei_doesnt_affect_other_flags() {
    let mut memory = Memory::new();
    memory.write(0x1000, 0x78);  // SEI opcode
    
    let mut cpu = Cpu::new(memory, 0x1000);
    cpu.state.flag_interrupt_disable = false;
    cpu.state.flag_carry = false;
    cpu.state.flag_zero = false;
    
    cpu.step().unwrap();
    
    assert!(cpu.state.flag_interrupt_disable); // Changed
    assert!(!cpu.state.flag_carry);            // Unchanged
    assert!(!cpu.state.flag_zero);             // Unchanged
}

#[test]
fn test_clv_doesnt_affect_other_flags() {
    let mut memory = Memory::new();
    memory.write(0x1000, 0xB8);  // CLV opcode
    
    let mut cpu = Cpu::new(memory, 0x1000);
    cpu.state.flag_overflow = true;
    cpu.state.flag_carry = true;
    cpu.state.flag_negative = true;
    
    cpu.step().unwrap();
    
    assert!(!cpu.state.flag_overflow);  // Changed
    assert!(cpu.state.flag_carry);      // Unchanged
    assert!(cpu.state.flag_negative);   // Unchanged
}

// ============================================================================
// NOP (No Operation) Tests
// ============================================================================

#[test]
fn test_nop_does_nothing() {
    let mut memory = Memory::new();
    memory.write(0x1000, 0xEA);  // NOP opcode
    
    let mut cpu = Cpu::new(memory, 0x1000);
    cpu.state.a = 0x42;
    cpu.state.x = 0x10;
    cpu.state.y = 0x20;
    cpu.state.flag_carry = true;
    cpu.state.flag_zero = true;
    
    cpu.step().unwrap();
    
    // All registers and flags should remain unchanged
    assert_eq!(cpu.state.a, 0x42);
    assert_eq!(cpu.state.x, 0x10);
    assert_eq!(cpu.state.y, 0x20);
    assert!(cpu.state.flag_carry);
    assert!(cpu.state.flag_zero);
    assert_eq!(cpu.state.pc, 0x1001);  // PC should advance
}

#[test]
fn test_nop_preserves_all_flags() {
    let mut memory = Memory::new();
    memory.write(0x1000, 0xEA);  // NOP opcode
    
    let mut cpu = Cpu::new(memory, 0x1000);
    
    // Set all flags to specific values
    cpu.state.flag_carry = true;
    cpu.state.flag_zero = false;
    cpu.state.flag_interrupt_disable = true;
    cpu.state.flag_decimal = false;
    cpu.state.flag_break = true;
    cpu.state.flag_overflow = false;
    cpu.state.flag_negative = true;
    
    cpu.step().unwrap();
    
    // Verify all flags remain unchanged
    assert_eq!(cpu.state.flag_carry, true);
    assert_eq!(cpu.state.flag_zero, false);
    assert_eq!(cpu.state.flag_interrupt_disable, true);
    assert_eq!(cpu.state.flag_decimal, false);
    assert_eq!(cpu.state.flag_break, true);
    assert_eq!(cpu.state.flag_overflow, false);
    assert_eq!(cpu.state.flag_negative, true);
}

#[test]
fn test_nop_only_advances_pc() {
    let mut memory = Memory::new();
    memory.write(0x1000, 0xEA);  // NOP opcode
    
    let mut cpu = Cpu::new(memory, 0x1000);
    
    // Set registers and stack pointer to specific values
    cpu.state.a = 0xFF;
    cpu.state.x = 0xAA;
    cpu.state.y = 0x55;
    cpu.state.sp = 0xFD;
    
    let initial_sp = cpu.state.sp;
    
    cpu.step().unwrap();
    
    // Only PC should change
    assert_eq!(cpu.state.pc, 0x1001);
    assert_eq!(cpu.state.a, 0xFF);
    assert_eq!(cpu.state.x, 0xAA);
    assert_eq!(cpu.state.y, 0x55);
    assert_eq!(cpu.state.sp, initial_sp);
}
