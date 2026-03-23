// Property-based tests for Program Counter advancement
// Feature: 6502-cpu-emulator, Property 7: Program Counter Advancement

use cpu_6502_emulator::cpu::Cpu;
use cpu_6502_emulator::memory::Memory;
use cpu_6502_emulator::instruction::decode_opcode;
use proptest::prelude::*;

// Complete list of all official 6502 opcodes
// Based on the MOS Technology 6502 Programming Manual
const VALID_OPCODES: &[u8] = &[
    // LDA - Load Accumulator
    0xA9, 0xA5, 0xB5, 0xAD, 0xBD, 0xB9, 0xA1, 0xB1,
    // LDX - Load X Register
    0xA2, 0xA6, 0xB6, 0xAE, 0xBE,
    // LDY - Load Y Register
    0xA0, 0xA4, 0xB4, 0xAC, 0xBC,
    // STA - Store Accumulator
    0x85, 0x95, 0x8D, 0x9D, 0x99, 0x81, 0x91,
    // STX - Store X Register
    0x86, 0x96, 0x8E,
    // STY - Store Y Register
    0x84, 0x94, 0x8C,
    // ADC - Add with Carry
    0x69, 0x65, 0x75, 0x6D, 0x7D, 0x79, 0x61, 0x71,
    // SBC - Subtract with Carry
    0xE9, 0xE5, 0xF5, 0xED, 0xFD, 0xF9, 0xE1, 0xF1,
    // INC - Increment Memory
    0xE6, 0xF6, 0xEE, 0xFE,
    // INX - Increment X Register
    0xE8,
    // INY - Increment Y Register
    0xC8,
    // DEC - Decrement Memory
    0xC6, 0xD6, 0xCE, 0xDE,
    // DEX - Decrement X Register
    0xCA,
    // DEY - Decrement Y Register
    0x88,
    // AND - Logical AND
    0x29, 0x25, 0x35, 0x2D, 0x3D, 0x39, 0x21, 0x31,
    // ORA - Logical OR
    0x09, 0x05, 0x15, 0x0D, 0x1D, 0x19, 0x01, 0x11,
    // EOR - Logical Exclusive OR
    0x49, 0x45, 0x55, 0x4D, 0x5D, 0x59, 0x41, 0x51,
    // ASL - Arithmetic Shift Left
    0x0A, 0x06, 0x16, 0x0E, 0x1E,
    // LSR - Logical Shift Right
    0x4A, 0x46, 0x56, 0x4E, 0x5E,
    // ROL - Rotate Left
    0x2A, 0x26, 0x36, 0x2E, 0x3E,
    // ROR - Rotate Right
    0x6A, 0x66, 0x76, 0x6E, 0x7E,
    // CMP - Compare Accumulator
    0xC9, 0xC5, 0xD5, 0xCD, 0xDD, 0xD9, 0xC1, 0xD1,
    // CPX - Compare X Register
    0xE0, 0xE4, 0xEC,
    // CPY - Compare Y Register
    0xC0, 0xC4, 0xCC,
    // BCC - Branch if Carry Clear
    0x90,
    // BCS - Branch if Carry Set
    0xB0,
    // BEQ - Branch if Equal (Zero Set)
    0xF0,
    // BMI - Branch if Minus (Negative Set)
    0x30,
    // BNE - Branch if Not Equal (Zero Clear)
    0xD0,
    // BPL - Branch if Plus (Negative Clear)
    0x10,
    // BVC - Branch if Overflow Clear
    0x50,
    // BVS - Branch if Overflow Set
    0x70,
    // JMP - Jump
    0x4C, 0x6C,
    // JSR - Jump to Subroutine
    0x20,
    // RTS - Return from Subroutine
    0x60,
    // RTI - Return from Interrupt
    0x40,
    // PHA - Push Accumulator
    0x48,
    // PHP - Push Processor Status
    0x08,
    // PLA - Pull Accumulator
    0x68,
    // PLP - Pull Processor Status
    0x28,
    // TSX - Transfer Stack Pointer to X
    0xBA,
    // TXS - Transfer X to Stack Pointer
    0x9A,
    // CLC - Clear Carry Flag
    0x18,
    // CLD - Clear Decimal Flag
    0xD8,
    // CLI - Clear Interrupt Disable Flag
    0x58,
    // CLV - Clear Overflow Flag
    0xB8,
    // SEC - Set Carry Flag
    0x38,
    // SED - Set Decimal Flag
    0xF8,
    // SEI - Set Interrupt Disable Flag
    0x78,
    // TAX - Transfer Accumulator to X
    0xAA,
    // TAY - Transfer Accumulator to Y
    0xA8,
    // TXA - Transfer X to Accumulator
    0x8A,
    // TYA - Transfer Y to Accumulator
    0x98,
    // NOP - No Operation
    0xEA,
    // BRK - Break
    0x00,
];

// Feature: 6502-cpu-emulator, Property 7: Program Counter Advancement
// **Validates: Requirements 4.5**
//
// For any valid instruction, after execution, the Program Counter should be
// incremented by exactly the instruction's length (1, 2, or 3 bytes).
//
// This property excludes control flow instructions (branches, jumps, JSR, RTS, RTI, BRK)
// which modify the PC in instruction-specific ways.

proptest! {
    #[test]
    fn pc_advances_by_instruction_length_for_non_control_flow_instructions(
        opcode_index in 0usize..VALID_OPCODES.len(),
        start_pc in 0x0200u16..=0xFFF0,
        memory_value in 0u8..=0xFF,
        register_value in 0u8..=0xFF
    ) {
        let opcode = VALID_OPCODES[opcode_index];
        
        // Decode the instruction to get its length
        let decoded = decode_opcode(opcode).unwrap();
        
        // Skip control flow instructions that modify PC in special ways:
        // - Branch instructions (BCC, BCS, BEQ, BMI, BNE, BPL, BVC, BVS)
        // - Jump instructions (JMP, JSR)
        // - Return instructions (RTS, RTI)
        // - Break instruction (BRK)
        let control_flow_opcodes = [
            0x90, 0xB0, 0xF0, 0x30, 0xD0, 0x10, 0x50, 0x70, // Branches
            0x4C, 0x6C, // JMP
            0x20, // JSR
            0x60, // RTS
            0x40, // RTI
            0x00, // BRK
        ];
        
        if control_flow_opcodes.contains(&opcode) {
            return Ok(());
        }
        
        // Set up memory with the instruction
        let mut memory = Memory::new();
        memory.write(start_pc, opcode);
        
        // Write operand bytes if needed (length > 1)
        if decoded.length >= 2 {
            memory.write(start_pc.wrapping_add(1), memory_value);
        }
        if decoded.length >= 3 {
            memory.write(start_pc.wrapping_add(2), memory_value);
        }
        
        // For instructions that read from memory, set up valid memory locations
        // This ensures the instruction can execute without errors
        memory.write(0x00, memory_value); // Zero page
        memory.write(0xFF, memory_value); // Zero page boundary
        memory.write(0x0100, memory_value); // Stack area
        memory.write(0x01FF, memory_value); // Stack boundary
        
        // Create CPU with the instruction at start_pc
        let mut cpu = Cpu::new(memory, start_pc);
        
        // Set up registers with test values to ensure instructions can execute
        cpu.state.a = register_value;
        cpu.state.x = register_value;
        cpu.state.y = register_value;
        
        // Execute the instruction
        let result = cpu.step();
        
        // The instruction should execute successfully
        prop_assert!(result.is_ok(),
            "Instruction 0x{:02X} at PC 0x{:04X} failed to execute: {:?}",
            opcode, start_pc, result.err());
        
        // PC should have advanced by exactly the instruction length
        let expected_pc = start_pc.wrapping_add(decoded.length as u16);
        prop_assert_eq!(cpu.state.pc, expected_pc,
            "Instruction 0x{:02X} (length {}) at PC 0x{:04X} should advance PC to 0x{:04X}, but PC is 0x{:04X}",
            opcode, decoded.length, start_pc, expected_pc, cpu.state.pc);
    }
}

// Test PC advancement for branch instructions when branch is NOT taken
// Branches should advance PC by 2 (instruction length) when condition is false
proptest! {
    #[test]
    fn pc_advances_by_two_for_branches_not_taken(
        branch_opcode_index in 0usize..8,
        start_pc in 0x0200u16..=0xFFF0,
        offset in 0i8..=127
    ) {
        // Branch opcodes: BCC, BCS, BEQ, BMI, BNE, BPL, BVC, BVS
        let branch_opcodes = [0x90, 0xB0, 0xF0, 0x30, 0xD0, 0x10, 0x50, 0x70];
        let opcode = branch_opcodes[branch_opcode_index];
        
        let mut memory = Memory::new();
        memory.write(start_pc, opcode);
        memory.write(start_pc.wrapping_add(1), offset as u8);
        
        let mut cpu = Cpu::new(memory, start_pc);
        
        // Set flags so branch is NOT taken
        match opcode {
            0x90 => cpu.state.flag_carry = true,     // BCC - branch if carry clear
            0xB0 => cpu.state.flag_carry = false,    // BCS - branch if carry set
            0xF0 => cpu.state.flag_zero = false,     // BEQ - branch if equal (zero set)
            0x30 => cpu.state.flag_negative = false, // BMI - branch if minus (negative set)
            0xD0 => cpu.state.flag_zero = true,      // BNE - branch if not equal (zero clear)
            0x10 => cpu.state.flag_negative = true,  // BPL - branch if plus (negative clear)
            0x50 => cpu.state.flag_overflow = true,  // BVC - branch if overflow clear
            0x70 => cpu.state.flag_overflow = false, // BVS - branch if overflow set
            _ => unreachable!(),
        }
        
        // Execute the branch instruction
        cpu.step().unwrap();
        
        // PC should advance by 2 (instruction length) when branch is not taken
        let expected_pc = start_pc.wrapping_add(2);
        prop_assert_eq!(cpu.state.pc, expected_pc,
            "Branch instruction 0x{:02X} at PC 0x{:04X} should advance PC by 2 to 0x{:04X} when not taken, but PC is 0x{:04X}",
            opcode, start_pc, expected_pc, cpu.state.pc);
    }
}

// Test PC advancement for JMP Absolute instruction
// JMP should set PC to the target address (not advance by instruction length)
proptest! {
    #[test]
    fn jmp_absolute_sets_pc_to_target_address(
        start_pc in 0x0200u16..=0xFFF0,
        target_addr in 0x0200u16..=0xFFFF
    ) {
        let mut memory = Memory::new();
        
        // JMP Absolute: 0x4C
        memory.write(start_pc, 0x4C);
        memory.write(start_pc.wrapping_add(1), (target_addr & 0xFF) as u8);
        memory.write(start_pc.wrapping_add(2), (target_addr >> 8) as u8);
        
        let mut cpu = Cpu::new(memory, start_pc);
        
        // Execute JMP
        cpu.step().unwrap();
        
        // PC should be set to target address (not advanced by instruction length)
        prop_assert_eq!(cpu.state.pc, target_addr,
            "JMP Absolute at PC 0x{:04X} should set PC to target 0x{:04X}, but PC is 0x{:04X}",
            start_pc, target_addr, cpu.state.pc);
    }
}

// Test that NOP advances PC by exactly 1 byte
#[test]
fn nop_advances_pc_by_one() {
    let mut memory = Memory::new();
    let start_pc = 0x0200;
    
    // NOP instruction: 0xEA
    memory.write(start_pc, 0xEA);
    
    let mut cpu = Cpu::new(memory, start_pc);
    
    // Execute NOP
    cpu.step().unwrap();
    
    // PC should advance by 1
    assert_eq!(cpu.state.pc, start_pc + 1,
        "NOP should advance PC by 1 byte");
}

// Test that multi-byte instructions advance PC correctly
#[test]
fn multi_byte_instructions_advance_pc_correctly() {
    // Test 2-byte instruction: LDA Immediate (0xA9)
    let mut memory = Memory::new();
    let start_pc = 0x0200;
    
    memory.write(start_pc, 0xA9);
    memory.write(start_pc + 1, 0x42);
    
    let mut cpu = Cpu::new(memory, start_pc);
    cpu.step().unwrap();
    
    assert_eq!(cpu.state.pc, start_pc + 2,
        "LDA Immediate (2-byte) should advance PC by 2");
    
    // Test 3-byte instruction: LDA Absolute (0xAD)
    let mut memory2 = Memory::new();
    let start_pc2 = 0x0300;
    memory2.write(start_pc2, 0xAD);
    memory2.write(start_pc2 + 1, 0x00);
    memory2.write(start_pc2 + 2, 0x04);
    memory2.write(0x0400, 0x55); // Value at target address
    
    let mut cpu2 = Cpu::new(memory2, start_pc2);
    cpu2.step().unwrap();
    
    assert_eq!(cpu2.state.pc, start_pc2 + 3,
        "LDA Absolute (3-byte) should advance PC by 3");
}
