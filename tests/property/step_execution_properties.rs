// Property-based tests for step execution atomicity
// Feature: 6502-cpu-emulator, Property 21: Step Execution Atomicity

use cpu_6502_emulator::cpu::Cpu;
use cpu_6502_emulator::memory::Memory;
use cpu_6502_emulator::instruction::decode_opcode;
use proptest::prelude::*;

// Complete list of all official 6502 opcodes
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

// Feature: 6502-cpu-emulator, Property 21: Step Execution Atomicity
// **Validates: Requirements 6.3**
//
// For any CPU state, executing a step command should execute exactly one
// complete instruction and leave the CPU in a valid state ready for the
// next instruction.
//
// This property verifies that:
// 1. Exactly one instruction is executed (PC advances correctly)
// 2. The CPU remains in a valid state (not halted unless error)
// 3. The instruction completes fully before returning
// 4. No partial execution occurs

proptest! {
    #[test]
    fn step_executes_exactly_one_instruction(
        opcode_index in 0usize..VALID_OPCODES.len(),
        start_pc in 0x0200u16..=0xFFF0,
        a_value in 0u8..=0xFF,
        x_value in 0u8..=0xFF,
        y_value in 0u8..=0xFF,
        sp_value in 0x02u8..=0xFF,
        carry_flag in any::<bool>(),
        zero_flag in any::<bool>(),
        interrupt_flag in any::<bool>(),
        decimal_flag in any::<bool>(),
        overflow_flag in any::<bool>(),
        negative_flag in any::<bool>(),
        memory_value in 0u8..=0xFF
    ) {
        let opcode = VALID_OPCODES[opcode_index];
        
        // Decode the instruction to get its length
        let decoded = decode_opcode(opcode).unwrap();
        
        // Set up memory with the instruction and operands
        let mut memory = Memory::new();
        memory.write(start_pc, opcode);
        
        // Write operand bytes if needed
        if decoded.length >= 2 {
            memory.write(start_pc.wrapping_add(1), memory_value);
        }
        if decoded.length >= 3 {
            memory.write(start_pc.wrapping_add(2), memory_value);
        }
        
        // Set up valid memory locations for instructions that read/write memory
        // Zero page
        for addr in 0x00..=0xFF {
            memory.write(addr, memory_value);
        }
        // Stack area
        for addr in 0x0100..=0x01FF {
            memory.write(addr, memory_value);
        }
        // IRQ vector for BRK instruction
        memory.write(0xFFFE, 0x00);
        memory.write(0xFFFF, 0x02);
        
        // Create CPU with the instruction at start_pc
        let mut cpu = Cpu::new(memory, start_pc);
        
        // Set up CPU state with random values
        cpu.state.a = a_value;
        cpu.state.x = x_value;
        cpu.state.y = y_value;
        cpu.state.sp = sp_value;
        cpu.state.flag_carry = carry_flag;
        cpu.state.flag_zero = zero_flag;
        cpu.state.flag_interrupt_disable = interrupt_flag;
        cpu.state.flag_decimal = decimal_flag;
        cpu.state.flag_overflow = overflow_flag;
        cpu.state.flag_negative = negative_flag;
        
        // Save initial state for comparison
        let initial_pc = cpu.state.pc;
        
        // Execute one step
        let result = cpu.step();
        
        // BRK instruction halts execution, which is expected behavior
        if opcode == 0x00 {
            prop_assert!(result.is_err(),
                "BRK instruction should halt execution");
            prop_assert!(cpu.halted,
                "CPU should be halted after BRK instruction");
            return Ok(());
        }
        
        // All other instructions should execute successfully
        prop_assert!(result.is_ok(),
            "Instruction 0x{:02X} at PC 0x{:04X} should execute successfully, but got error: {:?}",
            opcode, start_pc, result.err());
        
        // CPU should not be halted after successful execution
        prop_assert!(!cpu.halted,
            "CPU should not be halted after successful execution of instruction 0x{:02X}",
            opcode);
        
        // PC should have changed (instruction was executed)
        // Exception: Branch instructions with offset -2 can branch to themselves (infinite loop)
        // This happens when: PC_new = PC_old + 2 (instruction length) + (-2) = PC_old
        let is_branch_to_self = {
            let branch_opcodes = [0x90, 0xB0, 0xF0, 0x30, 0xD0, 0x10, 0x50, 0x70];
            branch_opcodes.contains(&opcode) && memory_value == 0xFE
        };
        
        if !is_branch_to_self {
            prop_assert_ne!(cpu.state.pc, initial_pc,
                "PC should have changed after executing instruction 0x{:02X} at 0x{:04X}",
                opcode, initial_pc);
        }
        
        // For most instructions, PC should advance by instruction length
        // Exceptions: branches (when taken), jumps, JSR, RTS, RTI, BRK
        let control_flow_opcodes = [
            0x90, 0xB0, 0xF0, 0x30, 0xD0, 0x10, 0x50, 0x70, // Branches
            0x4C, 0x6C, // JMP
            0x20, // JSR
            0x60, // RTS
            0x40, // RTI
            0x00, // BRK
        ];
        
        if !control_flow_opcodes.contains(&opcode) {
            // For non-control-flow instructions, PC should advance by instruction length
            let expected_pc = initial_pc.wrapping_add(decoded.length as u16);
            prop_assert_eq!(cpu.state.pc, expected_pc,
                "Instruction 0x{:02X} (length {}) at PC 0x{:04X} should advance PC to 0x{:04X}, but PC is 0x{:04X}",
                opcode, decoded.length, initial_pc, expected_pc, cpu.state.pc);
        }
    }
}

// Test that step() leaves CPU in a valid state ready for next instruction
proptest! {
    #[test]
    fn step_leaves_cpu_in_valid_state(
        opcode_index in 0usize..VALID_OPCODES.len(),
        start_pc in 0x0200u16..=0xFFF0,
        a_value in 0u8..=0xFF,
        x_value in 0u8..=0xFF,
        y_value in 0u8..=0xFF,
        sp_value in 0x02u8..=0xFF,
        memory_value in 0u8..=0xFF
    ) {
        let opcode = VALID_OPCODES[opcode_index];
        
        // Decode the instruction
        let decoded = decode_opcode(opcode).unwrap();
        
        // Set up memory
        let mut memory = Memory::new();
        memory.write(start_pc, opcode);
        
        if decoded.length >= 2 {
            memory.write(start_pc.wrapping_add(1), memory_value);
        }
        if decoded.length >= 3 {
            memory.write(start_pc.wrapping_add(2), memory_value);
        }
        
        // Set up valid memory locations
        for addr in 0x00..=0xFF {
            memory.write(addr, memory_value);
        }
        for addr in 0x0100..=0x01FF {
            memory.write(addr, memory_value);
        }
        memory.write(0xFFFE, 0x00);
        memory.write(0xFFFF, 0x02);
        
        // Create CPU
        let mut cpu = Cpu::new(memory, start_pc);
        cpu.state.a = a_value;
        cpu.state.x = x_value;
        cpu.state.y = y_value;
        cpu.state.sp = sp_value;
        
        // Execute step
        let result = cpu.step();
        
        // BRK instruction halts execution, which is expected behavior
        if opcode == 0x00 {
            prop_assert!(result.is_err(),
                "BRK instruction should halt execution");
            prop_assert!(cpu.halted,
                "CPU should be halted after BRK instruction");
            return Ok(());
        }
        
        prop_assert!(result.is_ok());
        
        // Verify CPU is in a valid state:
        
        // 1. CPU should not be halted
        prop_assert!(!cpu.halted,
            "CPU should not be halted after successful instruction execution");
        
        // 2. PC should point to a valid memory location (always true for u16)
        // We just verify the CPU is ready for the next instruction
        
        // 3. CPU should be ready for next instruction (can execute another step)
        // We verify this by checking that the CPU is not in an error state
        let next_opcode = cpu.memory.read(cpu.state.pc);
        let can_decode = decode_opcode(next_opcode);
        
        // If the next opcode is valid, we should be able to execute it
        if can_decode.is_ok() {
            // CPU should be ready to execute the next instruction
            prop_assert!(!cpu.halted,
                "CPU should be ready to execute next instruction");
        }
    }
}

// Test that step() executes complete instruction atomically
// (no partial execution, all side effects occur together)
proptest! {
    #[test]
    fn step_executes_instruction_atomically(
        start_pc in 0x0200u16..=0xFFF0,
        value1 in 0u8..=0xFF,
        value2 in 0u8..=0xFF
    ) {
        // Test with a multi-step instruction like ADC that:
        // 1. Reads from memory
        // 2. Performs calculation
        // 3. Updates accumulator
        // 4. Updates multiple flags
        // 5. Advances PC
        //
        // All of these should happen atomically in one step() call
        
        let mut memory = Memory::new();
        
        // ADC Immediate: 0x69
        memory.write(start_pc, 0x69);
        memory.write(start_pc.wrapping_add(1), value2);
        
        let mut cpu = Cpu::new(memory, start_pc);
        cpu.state.a = value1;
        cpu.state.flag_carry = false;
        
        // Save initial state
        let initial_pc = cpu.state.pc;
        
        // Execute one step
        cpu.step().unwrap();
        
        // Verify all side effects occurred:
        
        // 1. Accumulator was updated
        let expected_result = value1.wrapping_add(value2);
        prop_assert_eq!(cpu.state.a, expected_result,
            "Accumulator should be updated with result of addition");
        
        // 2. Flags were updated
        prop_assert_eq!(cpu.state.flag_zero, expected_result == 0,
            "Zero flag should be updated based on result");
        prop_assert_eq!(cpu.state.flag_negative, (expected_result & 0x80) != 0,
            "Negative flag should be updated based on result");
        
        // 3. PC was advanced
        prop_assert_eq!(cpu.state.pc, initial_pc.wrapping_add(2),
            "PC should be advanced by instruction length");
        
        // 4. No partial state: PC should only advance if instruction completed
        // The instruction completed successfully, so PC must have advanced
        prop_assert_ne!(cpu.state.pc, initial_pc,
            "PC must have advanced after successful instruction execution");
    }
}

// Test that step() handles multiple consecutive instructions correctly
proptest! {
    #[test]
    fn step_executes_multiple_instructions_sequentially(
        start_pc in 0x0200u16..=0xFFE0,
        value1 in 0u8..=0xFF,
        value2 in 0u8..=0xFF
    ) {
        // Set up a sequence of three instructions
        let mut memory = Memory::new();
        
        // Instruction 1: LDA #value1
        memory.write(start_pc, 0xA9);
        memory.write(start_pc.wrapping_add(1), value1);
        
        // Instruction 2: ADC #value2
        memory.write(start_pc.wrapping_add(2), 0x69);
        memory.write(start_pc.wrapping_add(3), value2);
        
        // Instruction 3: STA $10
        memory.write(start_pc.wrapping_add(4), 0x85);
        memory.write(start_pc.wrapping_add(5), 0x10);
        
        let mut cpu = Cpu::new(memory, start_pc);
        cpu.state.flag_carry = false;
        
        // Execute first step: LDA #value1
        cpu.step().unwrap();
        prop_assert_eq!(cpu.state.a, value1,
            "First step should load accumulator");
        prop_assert_eq!(cpu.state.pc, start_pc.wrapping_add(2),
            "PC should advance by 2 after first instruction");
        
        // Execute second step: ADC #value2
        cpu.step().unwrap();
        let expected_sum = value1.wrapping_add(value2);
        prop_assert_eq!(cpu.state.a, expected_sum,
            "Second step should add to accumulator");
        prop_assert_eq!(cpu.state.pc, start_pc.wrapping_add(4),
            "PC should advance by 2 after second instruction");
        
        // Execute third step: STA $10
        cpu.step().unwrap();
        prop_assert_eq!(cpu.memory.read(0x10), expected_sum,
            "Third step should store accumulator to memory");
        prop_assert_eq!(cpu.state.pc, start_pc.wrapping_add(6),
            "PC should advance by 2 after third instruction");
        
        // Verify each step executed exactly one instruction
        // (no skipping, no double execution)
    }
}

// Test that step() with invalid opcode leaves CPU in safe state
proptest! {
    #[test]
    fn step_with_invalid_opcode_halts_safely(
        start_pc in 0x0200u16..=0xFFF0,
        invalid_opcode in prop::sample::select(vec![
            0x02, 0x03, 0x04, 0x07, 0x0B, 0x0C, 0x0F,
            0x12, 0x13, 0x14, 0x17, 0x1A, 0x1B, 0x1C, 0x1F,
            0x22, 0x23, 0x27, 0x2B, 0x2F,
            0x32, 0x33, 0x34, 0x37, 0x3A, 0x3B, 0x3C, 0x3F,
            0x42, 0x43, 0x44, 0x47, 0x4B, 0x4F,
            0x52, 0x53, 0x54, 0x57, 0x5A, 0x5B, 0x5C, 0x5F,
            0x62, 0x63, 0x64, 0x67, 0x6B, 0x6F,
            0x72, 0x73, 0x74, 0x77, 0x7A, 0x7B, 0x7C, 0x7F,
            0x80, 0x82, 0x83, 0x87, 0x89, 0x8B, 0x8F,
            0x92, 0x93, 0x97, 0x9B, 0x9C, 0x9E, 0x9F,
            0xA3, 0xA7, 0xAB, 0xAF,
            0xB2, 0xB3, 0xB7, 0xBB, 0xBF,
            0xC2, 0xC3, 0xC7, 0xCB, 0xCF,
            0xD2, 0xD3, 0xD4, 0xD7, 0xDA, 0xDB, 0xDC, 0xDF,
            0xE2, 0xE3, 0xE7, 0xEB, 0xEF,
            0xF2, 0xF3, 0xF4, 0xF7, 0xFA, 0xFB, 0xFC, 0xFF,
        ]),
        a_value in 0u8..=0xFF,
        x_value in 0u8..=0xFF,
        y_value in 0u8..=0xFF
    ) {
        let mut memory = Memory::new();
        memory.write(start_pc, invalid_opcode);
        
        let mut cpu = Cpu::new(memory, start_pc);
        cpu.state.a = a_value;
        cpu.state.x = x_value;
        cpu.state.y = y_value;
        
        // Save initial state
        let initial_a = cpu.state.a;
        let initial_x = cpu.state.x;
        let initial_y = cpu.state.y;
        let initial_pc = cpu.state.pc;
        
        // Execute step with invalid opcode
        let result = cpu.step();
        
        // Should return error
        prop_assert!(result.is_err(),
            "Step should return error for invalid opcode 0x{:02X}",
            invalid_opcode);
        
        // CPU should be halted
        prop_assert!(cpu.halted,
            "CPU should be halted after invalid opcode");
        
        // Registers should be unchanged (no partial execution)
        prop_assert_eq!(cpu.state.a, initial_a,
            "Accumulator should be unchanged after invalid opcode");
        prop_assert_eq!(cpu.state.x, initial_x,
            "X register should be unchanged after invalid opcode");
        prop_assert_eq!(cpu.state.y, initial_y,
            "Y register should be unchanged after invalid opcode");
        
        // PC should be unchanged (instruction didn't execute)
        prop_assert_eq!(cpu.state.pc, initial_pc,
            "PC should be unchanged after invalid opcode");
        
        // Subsequent step() calls should also fail
        let result2 = cpu.step();
        prop_assert!(result2.is_err(),
            "Subsequent step should fail when CPU is halted");
    }
}
