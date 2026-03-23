// Unit tests for comparison and branch instructions

use cpu_6502_emulator::cpu::Cpu;
use cpu_6502_emulator::memory::Memory;

// ============================================================================
// CMP (Compare Accumulator) Tests
// ============================================================================

#[test]
fn test_cmp_equal() {
    let mut memory = Memory::new();
    memory.write(0x1000, 0xC9);  // CMP #$42 opcode
    memory.write(0x1001, 0x42);
    
    let mut cpu = Cpu::new(memory, 0x1000);
    cpu.state.a = 0x42;
    
    cpu.step().unwrap();
    
    assert!(cpu.state.flag_zero);    // A == M
    assert!(cpu.state.flag_carry);   // A >= M
    assert!(!cpu.state.flag_negative);
}

#[test]
fn test_cmp_greater_than() {
    let mut memory = Memory::new();
    memory.write(0x1000, 0xC9);  // CMP #$30 opcode
    memory.write(0x1001, 0x30);
    
    let mut cpu = Cpu::new(memory, 0x1000);
    cpu.state.a = 0x50;
    
    cpu.step().unwrap();
    
    assert!(!cpu.state.flag_zero);   // A != M
    assert!(cpu.state.flag_carry);   // A >= M
    assert!(!cpu.state.flag_negative);
}

#[test]
fn test_cmp_less_than() {
    let mut memory = Memory::new();
    memory.write(0x1000, 0xC9);  // CMP #$60 opcode
    memory.write(0x1001, 0x60);
    
    let mut cpu = Cpu::new(memory, 0x1000);
    cpu.state.a = 0x50;
    
    cpu.step().unwrap();
    
    assert!(!cpu.state.flag_zero);   // A != M
    assert!(!cpu.state.flag_carry);  // A < M
    assert!(cpu.state.flag_negative); // Result is negative
}

// ============================================================================
// CPX (Compare X Register) Tests
// ============================================================================

#[test]
fn test_cpx_equal() {
    let mut memory = Memory::new();
    memory.write(0x1000, 0xE0);  // CPX #$42 opcode
    memory.write(0x1001, 0x42);
    
    let mut cpu = Cpu::new(memory, 0x1000);
    cpu.state.x = 0x42;
    
    cpu.step().unwrap();
    
    assert!(cpu.state.flag_zero);
    assert!(cpu.state.flag_carry);
}

#[test]
fn test_cpx_greater_than() {
    let mut memory = Memory::new();
    memory.write(0x1000, 0xE0);  // CPX #$30 opcode
    memory.write(0x1001, 0x30);
    
    let mut cpu = Cpu::new(memory, 0x1000);
    cpu.state.x = 0x50;
    
    cpu.step().unwrap();
    
    assert!(!cpu.state.flag_zero);
    assert!(cpu.state.flag_carry);
}

// ============================================================================
// CPY (Compare Y Register) Tests
// ============================================================================

#[test]
fn test_cpy_equal() {
    let mut memory = Memory::new();
    memory.write(0x1000, 0xC0);  // CPY #$42 opcode
    memory.write(0x1001, 0x42);
    
    let mut cpu = Cpu::new(memory, 0x1000);
    cpu.state.y = 0x42;
    
    cpu.step().unwrap();
    
    assert!(cpu.state.flag_zero);
    assert!(cpu.state.flag_carry);
}

#[test]
fn test_cpy_less_than() {
    let mut memory = Memory::new();
    memory.write(0x1000, 0xC0);  // CPY #$60 opcode
    memory.write(0x1001, 0x60);
    
    let mut cpu = Cpu::new(memory, 0x1000);
    cpu.state.y = 0x50;
    
    cpu.step().unwrap();
    
    assert!(!cpu.state.flag_zero);
    assert!(!cpu.state.flag_carry);
}

// ============================================================================
// Branch Instructions Tests
// ============================================================================

#[test]
fn test_bcc_branch_taken() {
    // BCC (Branch if Carry Clear) - should branch when carry is clear
    let mut memory = Memory::new();
    memory.write(0x1000, 0x90);  // BCC opcode
    memory.write(0x1001, 0x10);  // Offset +16
    
    let mut cpu = Cpu::new(memory, 0x1000);
    cpu.state.flag_carry = false;
    
    cpu.step().unwrap();
    
    assert_eq!(cpu.state.pc, 0x1012);  // 0x1000 + 2 + 0x10
}

#[test]
fn test_bcc_branch_not_taken() {
    // BCC should not branch when carry is set
    let mut memory = Memory::new();
    memory.write(0x1000, 0x90);  // BCC opcode
    memory.write(0x1001, 0x10);
    
    let mut cpu = Cpu::new(memory, 0x1000);
    cpu.state.flag_carry = true;
    
    cpu.step().unwrap();
    
    assert_eq!(cpu.state.pc, 0x1002);  // Just advance past instruction
}

#[test]
fn test_bcs_branch_taken() {
    // BCS (Branch if Carry Set) - should branch when carry is set
    let mut memory = Memory::new();
    memory.write(0x1000, 0xB0);  // BCS opcode
    memory.write(0x1001, 0x10);
    
    let mut cpu = Cpu::new(memory, 0x1000);
    cpu.state.flag_carry = true;
    
    cpu.step().unwrap();
    
    assert_eq!(cpu.state.pc, 0x1012);
}

#[test]
fn test_beq_branch_taken() {
    // BEQ (Branch if Equal/Zero Set) - should branch when zero is set
    let mut memory = Memory::new();
    memory.write(0x1000, 0xF0);  // BEQ opcode
    memory.write(0x1001, 0x10);
    
    let mut cpu = Cpu::new(memory, 0x1000);
    cpu.state.flag_zero = true;
    
    cpu.step().unwrap();
    
    assert_eq!(cpu.state.pc, 0x1012);
}

#[test]
fn test_bne_branch_taken() {
    // BNE (Branch if Not Equal/Zero Clear) - should branch when zero is clear
    let mut memory = Memory::new();
    memory.write(0x1000, 0xD0);  // BNE opcode
    memory.write(0x1001, 0x10);
    
    let mut cpu = Cpu::new(memory, 0x1000);
    cpu.state.flag_zero = false;
    
    cpu.step().unwrap();
    
    assert_eq!(cpu.state.pc, 0x1012);
}

#[test]
fn test_bmi_branch_taken() {
    // BMI (Branch if Minus/Negative Set) - should branch when negative is set
    let mut memory = Memory::new();
    memory.write(0x1000, 0x30);  // BMI opcode
    memory.write(0x1001, 0x10);
    
    let mut cpu = Cpu::new(memory, 0x1000);
    cpu.state.flag_negative = true;
    
    cpu.step().unwrap();
    
    assert_eq!(cpu.state.pc, 0x1012);
}

#[test]
fn test_bpl_branch_taken() {
    // BPL (Branch if Plus/Negative Clear) - should branch when negative is clear
    let mut memory = Memory::new();
    memory.write(0x1000, 0x10);  // BPL opcode
    memory.write(0x1001, 0x10);
    
    let mut cpu = Cpu::new(memory, 0x1000);
    cpu.state.flag_negative = false;
    
    cpu.step().unwrap();
    
    assert_eq!(cpu.state.pc, 0x1012);
}

#[test]
fn test_bvc_branch_taken() {
    // BVC (Branch if Overflow Clear) - should branch when overflow is clear
    let mut memory = Memory::new();
    memory.write(0x1000, 0x50);  // BVC opcode
    memory.write(0x1001, 0x10);
    
    let mut cpu = Cpu::new(memory, 0x1000);
    cpu.state.flag_overflow = false;
    
    cpu.step().unwrap();
    
    assert_eq!(cpu.state.pc, 0x1012);
}

#[test]
fn test_bvs_branch_taken() {
    // BVS (Branch if Overflow Set) - should branch when overflow is set
    let mut memory = Memory::new();
    memory.write(0x1000, 0x70);  // BVS opcode
    memory.write(0x1001, 0x10);
    
    let mut cpu = Cpu::new(memory, 0x1000);
    cpu.state.flag_overflow = true;
    
    cpu.step().unwrap();
    
    assert_eq!(cpu.state.pc, 0x1012);
}

#[test]
fn test_branch_backward() {
    // Test branching backward with negative offset
    let mut memory = Memory::new();
    memory.write(0x1000, 0xD0);  // BNE opcode
    memory.write(0x1001, 0xFE);  // Offset -2 (signed)
    
    let mut cpu = Cpu::new(memory, 0x1000);
    cpu.state.flag_zero = false;
    
    cpu.step().unwrap();
    
    assert_eq!(cpu.state.pc, 0x1000);  // 0x1000 + 2 - 2
}

#[test]
fn test_branch_doesnt_affect_flags() {
    // Branches should not modify any flags
    let mut memory = Memory::new();
    memory.write(0x1000, 0xD0);  // BNE opcode
    memory.write(0x1001, 0x10);
    
    let mut cpu = Cpu::new(memory, 0x1000);
    cpu.state.flag_zero = false;
    cpu.state.flag_carry = true;
    cpu.state.flag_negative = true;
    
    cpu.step().unwrap();
    
    // All flags should remain unchanged
    assert!(!cpu.state.flag_zero);
    assert!(cpu.state.flag_carry);
    assert!(cpu.state.flag_negative);
}
