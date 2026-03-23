// Unit tests for arithmetic instructions (ADC, SBC, INC, DEC, etc.)

use cpu_6502_emulator::cpu::Cpu;
use cpu_6502_emulator::memory::Memory;

#[test]
fn test_adc_immediate_no_carry() {
    // ADC #$10 with A=0x20, Carry=0 should result in A=0x30
    let mut memory = Memory::new();
    memory.write(0x0000, 0x69); // ADC Immediate
    memory.write(0x0001, 0x10); // Operand: 0x10
    
    let mut cpu = Cpu::new(memory, 0x0000);
    cpu.state.a = 0x20;
    cpu.state.flag_carry = false;
    
    cpu.step().unwrap();
    
    assert_eq!(cpu.state.a, 0x30);
    assert_eq!(cpu.state.flag_carry, false);
    assert_eq!(cpu.state.flag_zero, false);
    assert_eq!(cpu.state.flag_negative, false);
    assert_eq!(cpu.state.flag_overflow, false);
}

#[test]
fn test_adc_immediate_with_carry_in() {
    // ADC #$10 with A=0x20, Carry=1 should result in A=0x31
    let mut memory = Memory::new();
    memory.write(0x0000, 0x69); // ADC Immediate
    memory.write(0x0001, 0x10); // Operand: 0x10
    
    let mut cpu = Cpu::new(memory, 0x0000);
    cpu.state.a = 0x20;
    cpu.state.flag_carry = true;
    
    cpu.step().unwrap();
    
    assert_eq!(cpu.state.a, 0x31);
    assert_eq!(cpu.state.flag_carry, false);
    assert_eq!(cpu.state.flag_zero, false);
    assert_eq!(cpu.state.flag_negative, false);
    assert_eq!(cpu.state.flag_overflow, false);
}

#[test]
fn test_adc_sets_carry_flag() {
    // ADC #$FF with A=0x02, Carry=0 should result in A=0x01, Carry=1
    let mut memory = Memory::new();
    memory.write(0x0000, 0x69); // ADC Immediate
    memory.write(0x0001, 0xFF); // Operand: 0xFF
    
    let mut cpu = Cpu::new(memory, 0x0000);
    cpu.state.a = 0x02;
    cpu.state.flag_carry = false;
    
    cpu.step().unwrap();
    
    assert_eq!(cpu.state.a, 0x01);
    assert_eq!(cpu.state.flag_carry, true);
    assert_eq!(cpu.state.flag_zero, false);
    assert_eq!(cpu.state.flag_negative, false);
}

#[test]
fn test_adc_sets_zero_flag() {
    // ADC #$00 with A=0x00, Carry=0 should result in A=0x00, Zero=1
    let mut memory = Memory::new();
    memory.write(0x0000, 0x69); // ADC Immediate
    memory.write(0x0001, 0x00); // Operand: 0x00
    
    let mut cpu = Cpu::new(memory, 0x0000);
    cpu.state.a = 0x00;
    cpu.state.flag_carry = false;
    
    cpu.step().unwrap();
    
    assert_eq!(cpu.state.a, 0x00);
    assert_eq!(cpu.state.flag_zero, true);
    assert_eq!(cpu.state.flag_negative, false);
}

#[test]
fn test_adc_sets_negative_flag() {
    // ADC #$70 with A=0x70, Carry=0 should result in A=0xE0 (negative)
    let mut memory = Memory::new();
    memory.write(0x0000, 0x69); // ADC Immediate
    memory.write(0x0001, 0x70); // Operand: 0x70
    
    let mut cpu = Cpu::new(memory, 0x0000);
    cpu.state.a = 0x70;
    cpu.state.flag_carry = false;
    
    cpu.step().unwrap();
    
    assert_eq!(cpu.state.a, 0xE0);
    assert_eq!(cpu.state.flag_negative, true);
    assert_eq!(cpu.state.flag_zero, false);
}

#[test]
fn test_adc_sets_overflow_flag_positive_overflow() {
    // ADC #$50 with A=0x50, Carry=0
    // 0x50 (80) + 0x50 (80) = 0xA0 (160 unsigned, -96 signed)
    // Both operands positive, result negative -> overflow
    let mut memory = Memory::new();
    memory.write(0x0000, 0x69); // ADC Immediate
    memory.write(0x0001, 0x50); // Operand: 0x50
    
    let mut cpu = Cpu::new(memory, 0x0000);
    cpu.state.a = 0x50;
    cpu.state.flag_carry = false;
    
    cpu.step().unwrap();
    
    assert_eq!(cpu.state.a, 0xA0);
    assert_eq!(cpu.state.flag_overflow, true);
    assert_eq!(cpu.state.flag_negative, true);
    assert_eq!(cpu.state.flag_carry, false);
}

#[test]
fn test_adc_sets_overflow_flag_negative_overflow() {
    // ADC #$D0 with A=0x90, Carry=0
    // 0x90 (-112) + 0xD0 (-48) = 0x60 (96)
    // Both operands negative, result positive -> overflow
    let mut memory = Memory::new();
    memory.write(0x0000, 0x69); // ADC Immediate
    memory.write(0x0001, 0xD0); // Operand: 0xD0
    
    let mut cpu = Cpu::new(memory, 0x0000);
    cpu.state.a = 0x90;
    cpu.state.flag_carry = false;
    
    cpu.step().unwrap();
    
    assert_eq!(cpu.state.a, 0x60);
    assert_eq!(cpu.state.flag_overflow, true);
    assert_eq!(cpu.state.flag_negative, false);
    assert_eq!(cpu.state.flag_carry, true);
}

#[test]
fn test_adc_no_overflow_positive_negative() {
    // ADC #$80 with A=0x10, Carry=0
    // 0x10 (16) + 0x80 (-128) = 0x90 (-112)
    // Positive + Negative -> no overflow
    let mut memory = Memory::new();
    memory.write(0x0000, 0x69); // ADC Immediate
    memory.write(0x0001, 0x80); // Operand: 0x80
    
    let mut cpu = Cpu::new(memory, 0x0000);
    cpu.state.a = 0x10;
    cpu.state.flag_carry = false;
    
    cpu.step().unwrap();
    
    assert_eq!(cpu.state.a, 0x90);
    assert_eq!(cpu.state.flag_overflow, false);
    assert_eq!(cpu.state.flag_negative, true);
}

#[test]
fn test_adc_zero_page() {
    // ADC $10 with value at $10 = 0x25, A=0x15, Carry=0
    let mut memory = Memory::new();
    memory.write(0x0000, 0x65); // ADC Zero Page
    memory.write(0x0001, 0x10); // Address: $10
    memory.write(0x0010, 0x25); // Value at $10
    
    let mut cpu = Cpu::new(memory, 0x0000);
    cpu.state.a = 0x15;
    cpu.state.flag_carry = false;
    
    cpu.step().unwrap();
    
    assert_eq!(cpu.state.a, 0x3A);
    assert_eq!(cpu.state.flag_carry, false);
}

#[test]
fn test_adc_zero_page_x() {
    // ADC $10,X with X=0x05, value at $15 = 0x30, A=0x20, Carry=0
    let mut memory = Memory::new();
    memory.write(0x0000, 0x75); // ADC Zero Page,X
    memory.write(0x0001, 0x10); // Base address: $10
    memory.write(0x0015, 0x30); // Value at $15 ($10 + X)
    
    let mut cpu = Cpu::new(memory, 0x0000);
    cpu.state.a = 0x20;
    cpu.state.x = 0x05;
    cpu.state.flag_carry = false;
    
    cpu.step().unwrap();
    
    assert_eq!(cpu.state.a, 0x50);
    assert_eq!(cpu.state.flag_carry, false);
}

#[test]
fn test_adc_absolute() {
    // ADC $1234 with value at $1234 = 0x42, A=0x08, Carry=0
    let mut memory = Memory::new();
    memory.write(0x0000, 0x6D); // ADC Absolute
    memory.write(0x0001, 0x34); // Low byte of address
    memory.write(0x0002, 0x12); // High byte of address
    memory.write(0x1234, 0x42); // Value at $1234
    
    let mut cpu = Cpu::new(memory, 0x0000);
    cpu.state.a = 0x08;
    cpu.state.flag_carry = false;
    
    cpu.step().unwrap();
    
    assert_eq!(cpu.state.a, 0x4A);
    assert_eq!(cpu.state.flag_carry, false);
}

#[test]
fn test_adc_absolute_x() {
    // ADC $1230,X with X=0x04, value at $1234 = 0x11, A=0x22, Carry=0
    let mut memory = Memory::new();
    memory.write(0x0000, 0x7D); // ADC Absolute,X
    memory.write(0x0001, 0x30); // Low byte of base address
    memory.write(0x0002, 0x12); // High byte of base address
    memory.write(0x1234, 0x11); // Value at $1234 ($1230 + X)
    
    let mut cpu = Cpu::new(memory, 0x0000);
    cpu.state.a = 0x22;
    cpu.state.x = 0x04;
    cpu.state.flag_carry = false;
    
    cpu.step().unwrap();
    
    assert_eq!(cpu.state.a, 0x33);
    assert_eq!(cpu.state.flag_carry, false);
}

#[test]
fn test_adc_absolute_y() {
    // ADC $1230,Y with Y=0x08, value at $1238 = 0x55, A=0x05, Carry=0
    let mut memory = Memory::new();
    memory.write(0x0000, 0x79); // ADC Absolute,Y
    memory.write(0x0001, 0x30); // Low byte of base address
    memory.write(0x0002, 0x12); // High byte of base address
    memory.write(0x1238, 0x55); // Value at $1238 ($1230 + Y)
    
    let mut cpu = Cpu::new(memory, 0x0000);
    cpu.state.a = 0x05;
    cpu.state.y = 0x08;
    cpu.state.flag_carry = false;
    
    cpu.step().unwrap();
    
    assert_eq!(cpu.state.a, 0x5A);
    assert_eq!(cpu.state.flag_carry, false);
}

#[test]
fn test_adc_carry_propagation() {
    // Test that carry flag is properly used in multi-byte addition
    // ADC #$FF with A=0xFF, Carry=1 should result in A=0xFF, Carry=1
    let mut memory = Memory::new();
    memory.write(0x0000, 0x69); // ADC Immediate
    memory.write(0x0001, 0xFF); // Operand: 0xFF
    
    let mut cpu = Cpu::new(memory, 0x0000);
    cpu.state.a = 0xFF;
    cpu.state.flag_carry = true;
    
    cpu.step().unwrap();
    
    assert_eq!(cpu.state.a, 0xFF);
    assert_eq!(cpu.state.flag_carry, true);
}

// ============================================================================
// SBC (Subtract with Carry) Tests
// ============================================================================

#[test]
fn test_sbc_immediate_no_borrow() {
    // Test SBC with no borrow (carry set)
    let mut memory = Memory::new();
    memory.write(0x1000, 0xE9);  // SBC #$30 opcode
    memory.write(0x1001, 0x30);  // Immediate value
    
    let mut cpu = Cpu::new(memory, 0x1000);
    cpu.state.a = 0x50;
    cpu.state.flag_carry = true;  // No borrow
    
    cpu.step().unwrap();
    
    assert_eq!(cpu.state.a, 0x20);  // 0x50 - 0x30
    assert!(cpu.state.flag_carry);  // No borrow occurred
    assert!(!cpu.state.flag_zero);
    assert!(!cpu.state.flag_negative);
    assert!(!cpu.state.flag_overflow);
}

#[test]
fn test_sbc_immediate_with_borrow() {
    // Test SBC with borrow (carry clear)
    let mut memory = Memory::new();
    memory.write(0x1000, 0xE9);  // SBC #$01 opcode
    memory.write(0x1001, 0x01);  // Immediate value
    
    let mut cpu = Cpu::new(memory, 0x1000);
    cpu.state.a = 0x50;
    cpu.state.flag_carry = false;  // Borrow
    
    cpu.step().unwrap();
    
    assert_eq!(cpu.state.a, 0x4E);  // 0x50 - 0x01 - 1 (borrow)
    assert!(cpu.state.flag_carry);  // No borrow in result
}

#[test]
fn test_sbc_sets_carry_flag() {
    // Test that SBC sets carry when A >= M
    let mut memory = Memory::new();
    memory.write(0x1000, 0xE9);  // SBC #$30 opcode
    memory.write(0x1001, 0x30);
    
    let mut cpu = Cpu::new(memory, 0x1000);
    cpu.state.a = 0x50;
    cpu.state.flag_carry = true;
    
    cpu.step().unwrap();
    
    assert!(cpu.state.flag_carry);  // 0x50 >= 0x30
}

#[test]
fn test_sbc_clears_carry_flag() {
    // Test that SBC clears carry when A < M (with borrow)
    let mut memory = Memory::new();
    memory.write(0x1000, 0xE9);  // SBC #$60 opcode
    memory.write(0x1001, 0x60);
    
    let mut cpu = Cpu::new(memory, 0x1000);
    cpu.state.a = 0x50;
    cpu.state.flag_carry = true;
    
    cpu.step().unwrap();
    
    assert!(!cpu.state.flag_carry);  // 0x50 < 0x60, borrow occurred
    assert_eq!(cpu.state.a, 0xF0);  // Wraps around
}

#[test]
fn test_sbc_sets_zero_flag() {
    // Test that SBC sets zero flag when result is zero
    let mut memory = Memory::new();
    memory.write(0x1000, 0xE9);  // SBC #$42 opcode
    memory.write(0x1001, 0x42);
    
    let mut cpu = Cpu::new(memory, 0x1000);
    cpu.state.a = 0x42;
    cpu.state.flag_carry = true;
    
    cpu.step().unwrap();
    
    assert_eq!(cpu.state.a, 0x00);
    assert!(cpu.state.flag_zero);
}

#[test]
fn test_sbc_sets_negative_flag() {
    // Test that SBC sets negative flag when result is negative
    let mut memory = Memory::new();
    memory.write(0x1000, 0xE9);  // SBC #$01 opcode
    memory.write(0x1001, 0x01);
    
    let mut cpu = Cpu::new(memory, 0x1000);
    cpu.state.a = 0x00;
    cpu.state.flag_carry = true;
    
    cpu.step().unwrap();
    
    assert_eq!(cpu.state.a, 0xFF);
    assert!(cpu.state.flag_negative);
}

#[test]
fn test_sbc_sets_overflow_flag() {
    // Test overflow: subtracting positive from negative gives positive
    let mut memory = Memory::new();
    memory.write(0x1000, 0xE9);  // SBC #$01 opcode
    memory.write(0x1001, 0x01);
    
    let mut cpu = Cpu::new(memory, 0x1000);
    cpu.state.a = 0x80;  // -128 in signed
    cpu.state.flag_carry = true;
    
    cpu.step().unwrap();
    
    assert_eq!(cpu.state.a, 0x7F);  // +127 in signed
    assert!(cpu.state.flag_overflow);  // Overflow occurred
}

#[test]
fn test_sbc_zero_page() {
    let mut memory = Memory::new();
    memory.write(0x0042, 0x30);  // Value at zero page
    memory.write(0x1000, 0xE5);  // SBC $42 opcode
    memory.write(0x1001, 0x42);
    
    let mut cpu = Cpu::new(memory, 0x1000);
    cpu.state.a = 0x50;
    cpu.state.flag_carry = true;
    
    cpu.step().unwrap();
    
    assert_eq!(cpu.state.a, 0x20);
}

// ============================================================================
// INC/DEC Memory Tests
// ============================================================================

#[test]
fn test_inc_zero_page() {
    let mut memory = Memory::new();
    memory.write(0x0042, 0x10);  // Value at zero page
    memory.write(0x1000, 0xE6);  // INC $42 opcode
    memory.write(0x1001, 0x42);
    
    let mut cpu = Cpu::new(memory, 0x1000);
    
    cpu.step().unwrap();
    
    assert_eq!(cpu.memory.read(0x0042), 0x11);
    assert!(!cpu.state.flag_zero);
    assert!(!cpu.state.flag_negative);
}

#[test]
fn test_inc_sets_zero_flag() {
    let mut memory = Memory::new();
    memory.write(0x0042, 0xFF);  // Value at zero page
    memory.write(0x1000, 0xE6);  // INC $42 opcode
    memory.write(0x1001, 0x42);
    
    let mut cpu = Cpu::new(memory, 0x1000);
    
    cpu.step().unwrap();
    
    assert_eq!(cpu.memory.read(0x0042), 0x00);
    assert!(cpu.state.flag_zero);
}

#[test]
fn test_inc_sets_negative_flag() {
    let mut memory = Memory::new();
    memory.write(0x0042, 0x7F);  // Value at zero page
    memory.write(0x1000, 0xE6);  // INC $42 opcode
    memory.write(0x1001, 0x42);
    
    let mut cpu = Cpu::new(memory, 0x1000);
    
    cpu.step().unwrap();
    
    assert_eq!(cpu.memory.read(0x0042), 0x80);
    assert!(cpu.state.flag_negative);
}

#[test]
fn test_inc_absolute() {
    let mut memory = Memory::new();
    memory.write(0x1234, 0x42);  // Value at absolute address
    memory.write(0x1000, 0xEE);  // INC $1234 opcode
    memory.write(0x1001, 0x34);
    memory.write(0x1002, 0x12);
    
    let mut cpu = Cpu::new(memory, 0x1000);
    
    cpu.step().unwrap();
    
    assert_eq!(cpu.memory.read(0x1234), 0x43);
}

#[test]
fn test_dec_zero_page() {
    let mut memory = Memory::new();
    memory.write(0x0042, 0x10);  // Value at zero page
    memory.write(0x1000, 0xC6);  // DEC $42 opcode
    memory.write(0x1001, 0x42);
    
    let mut cpu = Cpu::new(memory, 0x1000);
    
    cpu.step().unwrap();
    
    assert_eq!(cpu.memory.read(0x0042), 0x0F);
    assert!(!cpu.state.flag_zero);
    assert!(!cpu.state.flag_negative);
}

#[test]
fn test_dec_sets_zero_flag() {
    let mut memory = Memory::new();
    memory.write(0x0042, 0x01);  // Value at zero page
    memory.write(0x1000, 0xC6);  // DEC $42 opcode
    memory.write(0x1001, 0x42);
    
    let mut cpu = Cpu::new(memory, 0x1000);
    
    cpu.step().unwrap();
    
    assert_eq!(cpu.memory.read(0x0042), 0x00);
    assert!(cpu.state.flag_zero);
}

#[test]
fn test_dec_wraps_to_ff() {
    let mut memory = Memory::new();
    memory.write(0x0042, 0x00);  // Value at zero page
    memory.write(0x1000, 0xC6);  // DEC $42 opcode
    memory.write(0x1001, 0x42);
    
    let mut cpu = Cpu::new(memory, 0x1000);
    
    cpu.step().unwrap();
    
    assert_eq!(cpu.memory.read(0x0042), 0xFF);
    assert!(cpu.state.flag_negative);
}
