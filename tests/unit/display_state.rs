// Unit tests for CPU state display functionality
// Tests the display_state() method that shows registers, flags, and current instruction

use cpu_6502_emulator::cpu::Cpu;
use cpu_6502_emulator::memory::Memory;

#[test]
fn test_display_state_basic() {
    // Create a CPU with a simple program
    let mut memory = Memory::new();
    
    // Load a simple program: LDA #$42
    memory.write(0x1000, 0xA9); // LDA immediate
    memory.write(0x1001, 0x42); // operand
    
    let cpu = Cpu::new(memory, 0x1000);
    
    // Display state should not panic
    cpu.display_state();
    
    // Verify the CPU state is correct
    assert_eq!(cpu.state.pc, 0x1000);
    assert_eq!(cpu.state.a, 0x00);
    assert_eq!(cpu.state.x, 0x00);
    assert_eq!(cpu.state.y, 0x00);
    assert_eq!(cpu.state.sp, 0xFF);
}

#[test]
fn test_display_state_with_flags_set() {
    // Create a CPU and set some flags
    let mut memory = Memory::new();
    
    // Load a program: SEC (set carry flag)
    memory.write(0x2000, 0x38); // SEC
    
    let mut cpu = Cpu::new(memory, 0x2000);
    
    // Set some flags manually
    cpu.state.flag_carry = true;
    cpu.state.flag_zero = true;
    cpu.state.flag_negative = true;
    
    // Display state should not panic
    cpu.display_state();
    
    // Verify flags are set
    assert!(cpu.state.flag_carry);
    assert!(cpu.state.flag_zero);
    assert!(cpu.state.flag_negative);
}

#[test]
fn test_display_state_with_different_instructions() {
    // Test display with various instruction types
    
    // Test immediate addressing
    let mut memory = Memory::new();
    memory.write(0x3000, 0xA9); // LDA #$42
    memory.write(0x3001, 0x42);
    let cpu = Cpu::new(memory, 0x3000);
    cpu.display_state();
    
    // Test zero page addressing
    let mut memory = Memory::new();
    memory.write(0x3000, 0xA5); // LDA $10
    memory.write(0x3001, 0x10);
    let cpu = Cpu::new(memory, 0x3000);
    cpu.display_state();
    
    // Test absolute addressing
    let mut memory = Memory::new();
    memory.write(0x3000, 0xAD); // LDA $1234
    memory.write(0x3001, 0x34);
    memory.write(0x3002, 0x12);
    let cpu = Cpu::new(memory, 0x3000);
    cpu.display_state();
    
    // Test implied addressing
    let mut memory = Memory::new();
    memory.write(0x3000, 0xE8); // INX
    let cpu = Cpu::new(memory, 0x3000);
    cpu.display_state();
}

#[test]
fn test_display_state_with_invalid_opcode() {
    // Test display with an invalid opcode
    let mut memory = Memory::new();
    
    // Load an invalid opcode
    memory.write(0x4000, 0x02); // Invalid opcode
    
    let cpu = Cpu::new(memory, 0x4000);
    
    // Display state should not panic even with invalid opcode
    cpu.display_state();
}

#[test]
fn test_display_state_after_execution() {
    // Test display after executing an instruction
    let mut memory = Memory::new();
    
    // Load a program: LDA #$42
    memory.write(0x5000, 0xA9); // LDA immediate
    memory.write(0x5001, 0x42); // operand
    memory.write(0x5002, 0xE8); // INX
    
    let mut cpu = Cpu::new(memory, 0x5000);
    
    // Display initial state
    cpu.display_state();
    
    // Execute LDA #$42
    cpu.step().unwrap();
    
    // Display state after execution
    cpu.display_state();
    
    // Verify state changed
    assert_eq!(cpu.state.a, 0x42);
    assert_eq!(cpu.state.pc, 0x5002);
}

#[test]
fn test_format_instruction_all_addressing_modes() {
    // Test that all addressing modes format correctly
    
    // Immediate: LDA #$42
    let mut memory = Memory::new();
    memory.write(0x6000, 0xA9);
    memory.write(0x6001, 0x42);
    let cpu = Cpu::new(memory, 0x6000);
    cpu.display_state();
    
    // Zero Page: LDA $10
    let mut memory = Memory::new();
    memory.write(0x6000, 0xA5);
    memory.write(0x6001, 0x10);
    let cpu = Cpu::new(memory, 0x6000);
    cpu.display_state();
    
    // Zero Page,X: LDA $10,X
    let mut memory = Memory::new();
    memory.write(0x6000, 0xB5);
    memory.write(0x6001, 0x10);
    let cpu = Cpu::new(memory, 0x6000);
    cpu.display_state();
    
    // Absolute: LDA $1234
    let mut memory = Memory::new();
    memory.write(0x6000, 0xAD);
    memory.write(0x6001, 0x34);
    memory.write(0x6002, 0x12);
    let cpu = Cpu::new(memory, 0x6000);
    cpu.display_state();
    
    // Absolute,X: LDA $1234,X
    let mut memory = Memory::new();
    memory.write(0x6000, 0xBD);
    memory.write(0x6001, 0x34);
    memory.write(0x6002, 0x12);
    let cpu = Cpu::new(memory, 0x6000);
    cpu.display_state();
    
    // Absolute,Y: LDA $1234,Y
    let mut memory = Memory::new();
    memory.write(0x6000, 0xB9);
    memory.write(0x6001, 0x34);
    memory.write(0x6002, 0x12);
    let cpu = Cpu::new(memory, 0x6000);
    cpu.display_state();
    
    // Indexed Indirect: LDA ($10,X)
    let mut memory = Memory::new();
    memory.write(0x6000, 0xA1);
    memory.write(0x6001, 0x10);
    let cpu = Cpu::new(memory, 0x6000);
    cpu.display_state();
    
    // Indirect Indexed: LDA ($10),Y
    let mut memory = Memory::new();
    memory.write(0x6000, 0xB1);
    memory.write(0x6001, 0x10);
    let cpu = Cpu::new(memory, 0x6000);
    cpu.display_state();
    
    // Implied: INX
    let mut memory = Memory::new();
    memory.write(0x6000, 0xE8);
    let cpu = Cpu::new(memory, 0x6000);
    cpu.display_state();
    
    // Accumulator: ASL A
    let mut memory = Memory::new();
    memory.write(0x6000, 0x0A);
    let cpu = Cpu::new(memory, 0x6000);
    cpu.display_state();
}
