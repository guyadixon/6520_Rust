// Unit tests for CPU step() method
// Validates Requirements 4.1, 4.2, 4.3, 4.4, 4.5

use cpu_6502_emulator::cpu::Cpu;
use cpu_6502_emulator::memory::Memory;

#[test]
fn test_step_executes_single_instruction() {
    // Test that step() executes exactly one instruction
    // Requirement 4.1: Fetch opcode from PC
    // Requirement 4.2: Decode instruction
    // Requirement 4.3: Fetch operands
    // Requirement 4.4: Execute instruction
    // Requirement 4.5: Increment PC by instruction length
    
    let mut memory = Memory::new();
    // LDA #$42 - Load accumulator with immediate value 0x42
    memory.write(0x8000, 0xA9); // LDA immediate opcode
    memory.write(0x8001, 0x42); // operand
    
    let mut cpu = Cpu::new(memory, 0x8000);
    
    // Initial state
    assert_eq!(cpu.state.pc, 0x8000);
    assert_eq!(cpu.state.a, 0x00);
    
    // Execute one step
    let result = cpu.step();
    assert!(result.is_ok(), "Step should execute successfully");
    
    // Verify instruction was executed
    assert_eq!(cpu.state.a, 0x42, "Accumulator should be loaded with 0x42");
    
    // Verify PC was advanced by instruction length (2 bytes for LDA immediate)
    assert_eq!(cpu.state.pc, 0x8002, "PC should advance by 2 bytes");
    
    // Verify flags were updated
    assert!(!cpu.state.flag_zero, "Zero flag should be clear");
    assert!(!cpu.state.flag_negative, "Negative flag should be clear");
}

#[test]
fn test_step_advances_pc_correctly() {
    // Test that PC advances by the correct instruction length
    // Requirement 4.5: Increment PC by instruction length
    
    let mut memory = Memory::new();
    
    // Test 1-byte instruction (INX)
    memory.write(0x8000, 0xE8); // INX opcode
    let mut cpu = Cpu::new(memory, 0x8000);
    cpu.step().unwrap();
    assert_eq!(cpu.state.pc, 0x8001, "PC should advance by 1 byte for INX");
    
    // Test 2-byte instruction (LDA immediate)
    let mut memory = Memory::new();
    memory.write(0x8000, 0xA9); // LDA immediate
    memory.write(0x8001, 0x42);
    let mut cpu = Cpu::new(memory, 0x8000);
    cpu.step().unwrap();
    assert_eq!(cpu.state.pc, 0x8002, "PC should advance by 2 bytes for LDA immediate");
    
    // Test 3-byte instruction (LDA absolute)
    let mut memory = Memory::new();
    memory.write(0x8000, 0xAD); // LDA absolute
    memory.write(0x8001, 0x00); // low byte of address
    memory.write(0x8002, 0x10); // high byte of address
    memory.write(0x1000, 0x99); // value at 0x1000
    let mut cpu = Cpu::new(memory, 0x8000);
    cpu.step().unwrap();
    assert_eq!(cpu.state.pc, 0x8003, "PC should advance by 3 bytes for LDA absolute");
    assert_eq!(cpu.state.a, 0x99, "Accumulator should be loaded with value from 0x1000");
}

#[test]
fn test_step_multiple_instructions() {
    // Test executing multiple instructions in sequence
    
    let mut memory = Memory::new();
    // Program: LDA #$42, TAX, INX
    memory.write(0x8000, 0xA9); // LDA immediate
    memory.write(0x8001, 0x42);
    memory.write(0x8002, 0xAA); // TAX
    memory.write(0x8003, 0xE8); // INX
    
    let mut cpu = Cpu::new(memory, 0x8000);
    
    // Step 1: LDA #$42
    cpu.step().unwrap();
    assert_eq!(cpu.state.a, 0x42);
    assert_eq!(cpu.state.pc, 0x8002);
    
    // Step 2: TAX
    cpu.step().unwrap();
    assert_eq!(cpu.state.x, 0x42);
    assert_eq!(cpu.state.pc, 0x8003);
    
    // Step 3: INX
    cpu.step().unwrap();
    assert_eq!(cpu.state.x, 0x43);
    assert_eq!(cpu.state.pc, 0x8004);
}

#[test]
fn test_step_with_invalid_opcode() {
    // Test that step() returns error for invalid opcodes
    // Requirement 4.6: Display error for invalid opcodes
    // Requirement 10.2: Display opcode value and PC location
    
    let mut memory = Memory::new();
    memory.write(0x8000, 0x02); // Invalid opcode
    
    let mut cpu = Cpu::new(memory, 0x8000);
    
    // Verify CPU is not halted initially
    assert!(!cpu.halted, "CPU should not be halted initially");
    
    let result = cpu.step();
    assert!(result.is_err(), "Step should return error for invalid opcode");
    
    let error_msg = result.unwrap_err();
    assert!(error_msg.contains("Invalid opcode"), 
        "Error message should mention invalid opcode");
    assert!(error_msg.contains("0x02"), 
        "Error message should include the opcode value");
    assert!(error_msg.contains("0x8000") || error_msg.contains("8000"), 
        "Error message should include the PC location, got: {}", error_msg);
    
    // Verify CPU is halted after invalid opcode
    assert!(cpu.halted, "CPU should be halted after invalid opcode");
    
    // Verify subsequent step() calls return error
    let result2 = cpu.step();
    assert!(result2.is_err(), "Step should return error when CPU is halted");
    assert!(result2.unwrap_err().contains("halted"), 
        "Error should indicate CPU is halted");
}

#[test]
fn test_step_with_halted_cpu() {
    // Test that step() returns error when CPU is halted
    
    let mut memory = Memory::new();
    memory.write(0x8000, 0xA9); // LDA immediate
    memory.write(0x8001, 0x42);
    
    let mut cpu = Cpu::new(memory, 0x8000);
    cpu.halted = true;
    
    let result = cpu.step();
    assert!(result.is_err(), "Step should return error when CPU is halted");
    
    let error_msg = result.unwrap_err();
    assert!(error_msg.contains("halted"), 
        "Error message should mention CPU is halted");
}

#[test]
fn test_step_fetch_decode_execute_cycle() {
    // Test the complete fetch-decode-execute cycle
    
    let mut memory = Memory::new();
    // LDX #$10 - Load X register with immediate value 0x10
    memory.write(0x8000, 0xA2); // LDX immediate opcode
    memory.write(0x8001, 0x10); // operand
    
    let mut cpu = Cpu::new(memory, 0x8000);
    
    // Before step
    assert_eq!(cpu.state.pc, 0x8000, "PC should point to instruction");
    assert_eq!(cpu.state.x, 0x00, "X register should be 0");
    
    // Execute step (fetch-decode-execute)
    cpu.step().unwrap();
    
    // After step
    assert_eq!(cpu.state.x, 0x10, "X register should be loaded with 0x10");
    assert_eq!(cpu.state.pc, 0x8002, "PC should advance past instruction");
    assert!(!cpu.state.flag_zero, "Zero flag should be clear");
    assert!(!cpu.state.flag_negative, "Negative flag should be clear");
}

#[test]
fn test_step_with_store_instruction() {
    // Test step() with a store instruction that modifies memory
    
    let mut memory = Memory::new();
    // STA $10 - Store accumulator to zero page address 0x10
    memory.write(0x8000, 0x85); // STA zero page opcode
    memory.write(0x8001, 0x10); // zero page address
    
    let mut cpu = Cpu::new(memory, 0x8000);
    cpu.state.a = 0x99; // Set accumulator to 0x99
    
    // Execute step
    cpu.step().unwrap();
    
    // Verify memory was written
    assert_eq!(cpu.memory.read(0x0010), 0x99, 
        "Memory at 0x10 should contain accumulator value");
    
    // Verify PC advanced
    assert_eq!(cpu.state.pc, 0x8002, "PC should advance by 2 bytes");
    
    // Verify accumulator unchanged
    assert_eq!(cpu.state.a, 0x99, "Accumulator should remain unchanged");
}

#[test]
fn test_step_with_zero_flag_update() {
    // Test that step() correctly updates the zero flag
    
    let mut memory = Memory::new();
    // LDA #$00 - Load accumulator with zero
    memory.write(0x8000, 0xA9); // LDA immediate
    memory.write(0x8001, 0x00); // zero value
    
    let mut cpu = Cpu::new(memory, 0x8000);
    
    cpu.step().unwrap();
    
    assert_eq!(cpu.state.a, 0x00, "Accumulator should be 0");
    assert!(cpu.state.flag_zero, "Zero flag should be set");
    assert!(!cpu.state.flag_negative, "Negative flag should be clear");
}

#[test]
fn test_step_with_negative_flag_update() {
    // Test that step() correctly updates the negative flag
    
    let mut memory = Memory::new();
    // LDA #$80 - Load accumulator with 0x80 (bit 7 set)
    memory.write(0x8000, 0xA9); // LDA immediate
    memory.write(0x8001, 0x80); // value with bit 7 set
    
    let mut cpu = Cpu::new(memory, 0x8000);
    
    cpu.step().unwrap();
    
    assert_eq!(cpu.state.a, 0x80, "Accumulator should be 0x80");
    assert!(!cpu.state.flag_zero, "Zero flag should be clear");
    assert!(cpu.state.flag_negative, "Negative flag should be set");
}

#[test]
fn test_step_pc_wrapping() {
    // Test that PC wraps correctly at memory boundary
    
    let mut memory = Memory::new();
    // Place a 1-byte instruction at 0xFFFF
    memory.write(0xFFFF, 0xEA); // NOP opcode
    
    let mut cpu = Cpu::new(memory, 0xFFFF);
    
    cpu.step().unwrap();
    
    // PC should wrap to 0x0000
    assert_eq!(cpu.state.pc, 0x0000, "PC should wrap to 0x0000");
}

#[test]
fn test_step_atomicity() {
    // Test that step() executes exactly one complete instruction
    // and leaves CPU in a valid state
    
    let mut memory = Memory::new();
    // Program with multiple instructions
    memory.write(0x8000, 0xA9); // LDA #$42
    memory.write(0x8001, 0x42);
    memory.write(0x8002, 0xAA); // TAX
    memory.write(0x8003, 0xE8); // INX
    
    let mut cpu = Cpu::new(memory, 0x8000);
    
    // Execute first step only
    cpu.step().unwrap();
    
    // Verify only the first instruction was executed
    assert_eq!(cpu.state.a, 0x42, "Only LDA should have executed");
    assert_eq!(cpu.state.x, 0x00, "TAX should not have executed yet");
    assert_eq!(cpu.state.pc, 0x8002, "PC should point to next instruction");
    
    // CPU should be in valid state ready for next instruction
    assert!(!cpu.halted, "CPU should not be halted");
}

#[test]
fn test_invalid_opcode_at_different_pc() {
    // Test that error message includes correct PC for invalid opcodes at different locations
    // Requirement 10.2: Display opcode value and PC location
    
    let mut memory = Memory::new();
    memory.write(0x1234, 0x03); // Invalid opcode at 0x1234
    
    let mut cpu = Cpu::new(memory, 0x1234);
    
    let result = cpu.step();
    assert!(result.is_err(), "Step should return error for invalid opcode");
    
    let error_msg = result.unwrap_err();
    assert!(error_msg.contains("0x03"), 
        "Error message should include the opcode value 0x03");
    assert!(error_msg.contains("0x1234") || error_msg.contains("1234"), 
        "Error message should include the PC location 0x1234, got: {}", error_msg);
    assert!(cpu.halted, "CPU should be halted");
}

#[test]
fn test_invalid_opcode_halts_execution() {
    // Test that invalid opcode halts execution and prevents further steps
    // Requirement 4.6: Halt execution on invalid opcode
    
    let mut memory = Memory::new();
    memory.write(0x8000, 0xA9); // LDA #$42 (valid)
    memory.write(0x8001, 0x42);
    memory.write(0x8002, 0x02); // Invalid opcode
    memory.write(0x8003, 0xAA); // TAX (should not execute)
    
    let mut cpu = Cpu::new(memory, 0x8000);
    
    // Execute first instruction successfully
    cpu.step().unwrap();
    assert_eq!(cpu.state.a, 0x42);
    assert_eq!(cpu.state.pc, 0x8002);
    assert!(!cpu.halted);
    
    // Execute second instruction - should fail with invalid opcode
    let result = cpu.step();
    assert!(result.is_err());
    assert!(cpu.halted, "CPU should be halted after invalid opcode");
    
    // Try to execute third instruction - should fail because CPU is halted
    let result2 = cpu.step();
    assert!(result2.is_err());
    assert!(result2.unwrap_err().contains("halted"));
    
    // Verify TAX was never executed
    assert_eq!(cpu.state.x, 0x00, "TAX should not have executed");
}

#[test]
fn test_error_message_format() {
    // Test that error message has the expected format
    // Requirement 10.2: Descriptive error with opcode and PC
    
    let mut memory = Memory::new();
    memory.write(0xABCD, 0xFF); // Invalid opcode 0xFF at 0xABCD
    
    let mut cpu = Cpu::new(memory, 0xABCD);
    
    let result = cpu.step();
    assert!(result.is_err());
    
    let error_msg = result.unwrap_err();
    
    // Error should contain "Invalid opcode"
    assert!(error_msg.contains("Invalid opcode"), 
        "Error should contain 'Invalid opcode', got: {}", error_msg);
    
    // Error should contain the opcode in hex format
    assert!(error_msg.contains("0xFF") || error_msg.contains("0xff"), 
        "Error should contain opcode '0xFF', got: {}", error_msg);
    
    // Error should contain "PC" or "at"
    assert!(error_msg.contains("PC") || error_msg.contains("at"), 
        "Error should indicate PC location, got: {}", error_msg);
    
    // Error should contain the PC address in hex format
    assert!(error_msg.contains("0xABCD") || error_msg.contains("0xabcd") || 
            error_msg.contains("ABCD") || error_msg.contains("abcd"), 
        "Error should contain PC address '0xABCD', got: {}", error_msg);
}
