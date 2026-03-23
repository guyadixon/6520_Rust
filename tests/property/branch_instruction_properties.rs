// Property-based tests for branch instructions
// Feature: 6502-cpu-emulator, Property 17: Branch Instruction Correctness

use cpu_6502_emulator::cpu::Cpu;
use cpu_6502_emulator::memory::Memory;
use proptest::prelude::*;

// Feature: 6502-cpu-emulator, Property 17: Branch Instruction Correctness
// **Validates: Requirements 8.6**
//
// For any branch instruction and any flag state:
// - If the branch condition is true, PC should be updated by the signed offset
// - If the branch condition is false, PC should advance by 2 (instruction length)
// - No registers or flags should be modified

proptest! {
    #[test]
    fn bcc_branches_when_carry_clear(
        offset in -128i8..=127i8,
        pc_start in 0x0200u16..=0xFE00
    ) {
        let mut memory = Memory::new();
        memory.write(pc_start, 0x90); // BCC opcode
        memory.write(pc_start.wrapping_add(1), offset as u8);
        
        let mut cpu = Cpu::new(memory, pc_start);
        cpu.state.flag_carry = false; // Condition is true
        
        // Save initial state
        let initial_a = cpu.state.a;
        let initial_x = cpu.state.x;
        let initial_y = cpu.state.y;
        let initial_sp = cpu.state.sp;
        let initial_flags = (
            cpu.state.flag_zero,
            cpu.state.flag_negative,
            cpu.state.flag_overflow,
            cpu.state.flag_interrupt_disable,
            cpu.state.flag_decimal,
            cpu.state.flag_break
        );
        
        cpu.step().unwrap();
        
        // PC should be at start + 2 + offset
        let expected_pc = pc_start.wrapping_add(2).wrapping_add(offset as i16 as u16);
        prop_assert_eq!(cpu.state.pc, expected_pc);
        
        // No registers should be modified
        prop_assert_eq!(cpu.state.a, initial_a);
        prop_assert_eq!(cpu.state.x, initial_x);
        prop_assert_eq!(cpu.state.y, initial_y);
        prop_assert_eq!(cpu.state.sp, initial_sp);
        
        // No flags should be modified
        prop_assert_eq!(cpu.state.flag_zero, initial_flags.0);
        prop_assert_eq!(cpu.state.flag_negative, initial_flags.1);
        prop_assert_eq!(cpu.state.flag_overflow, initial_flags.2);
        prop_assert_eq!(cpu.state.flag_interrupt_disable, initial_flags.3);
        prop_assert_eq!(cpu.state.flag_decimal, initial_flags.4);
        prop_assert_eq!(cpu.state.flag_break, initial_flags.5);
        prop_assert_eq!(cpu.state.flag_carry, false);
    }
    
    #[test]
    fn bcc_does_not_branch_when_carry_set(
        offset in -128i8..=127i8,
        pc_start in 0x0200u16..=0xFE00
    ) {
        let mut memory = Memory::new();
        memory.write(pc_start, 0x90); // BCC opcode
        memory.write(pc_start.wrapping_add(1), offset as u8);
        
        let mut cpu = Cpu::new(memory, pc_start);
        cpu.state.flag_carry = true; // Condition is false
        
        cpu.step().unwrap();
        
        // PC should advance by 2 (instruction length)
        prop_assert_eq!(cpu.state.pc, pc_start.wrapping_add(2));
    }
    
    #[test]
    fn bcs_branches_when_carry_set(
        offset in -128i8..=127i8,
        pc_start in 0x0200u16..=0xFE00
    ) {
        let mut memory = Memory::new();
        memory.write(pc_start, 0xB0); // BCS opcode
        memory.write(pc_start.wrapping_add(1), offset as u8);
        
        let mut cpu = Cpu::new(memory, pc_start);
        cpu.state.flag_carry = true; // Condition is true
        
        cpu.step().unwrap();
        
        let expected_pc = pc_start.wrapping_add(2).wrapping_add(offset as i16 as u16);
        prop_assert_eq!(cpu.state.pc, expected_pc);
    }
    
    #[test]
    fn bcs_does_not_branch_when_carry_clear(
        offset in -128i8..=127i8,
        pc_start in 0x0200u16..=0xFE00
    ) {
        let mut memory = Memory::new();
        memory.write(pc_start, 0xB0); // BCS opcode
        memory.write(pc_start.wrapping_add(1), offset as u8);
        
        let mut cpu = Cpu::new(memory, pc_start);
        cpu.state.flag_carry = false; // Condition is false
        
        cpu.step().unwrap();
        
        prop_assert_eq!(cpu.state.pc, pc_start.wrapping_add(2));
    }
    
    #[test]
    fn beq_branches_when_zero_set(
        offset in -128i8..=127i8,
        pc_start in 0x0200u16..=0xFE00
    ) {
        let mut memory = Memory::new();
        memory.write(pc_start, 0xF0); // BEQ opcode
        memory.write(pc_start.wrapping_add(1), offset as u8);
        
        let mut cpu = Cpu::new(memory, pc_start);
        cpu.state.flag_zero = true; // Condition is true
        
        cpu.step().unwrap();
        
        let expected_pc = pc_start.wrapping_add(2).wrapping_add(offset as i16 as u16);
        prop_assert_eq!(cpu.state.pc, expected_pc);
    }
    
    #[test]
    fn beq_does_not_branch_when_zero_clear(
        offset in -128i8..=127i8,
        pc_start in 0x0200u16..=0xFE00
    ) {
        let mut memory = Memory::new();
        memory.write(pc_start, 0xF0); // BEQ opcode
        memory.write(pc_start.wrapping_add(1), offset as u8);
        
        let mut cpu = Cpu::new(memory, pc_start);
        cpu.state.flag_zero = false; // Condition is false
        
        cpu.step().unwrap();
        
        prop_assert_eq!(cpu.state.pc, pc_start.wrapping_add(2));
    }
    
    #[test]
    fn bne_branches_when_zero_clear(
        offset in -128i8..=127i8,
        pc_start in 0x0200u16..=0xFE00
    ) {
        let mut memory = Memory::new();
        memory.write(pc_start, 0xD0); // BNE opcode
        memory.write(pc_start.wrapping_add(1), offset as u8);
        
        let mut cpu = Cpu::new(memory, pc_start);
        cpu.state.flag_zero = false; // Condition is true
        
        cpu.step().unwrap();
        
        let expected_pc = pc_start.wrapping_add(2).wrapping_add(offset as i16 as u16);
        prop_assert_eq!(cpu.state.pc, expected_pc);
    }
    
    #[test]
    fn bne_does_not_branch_when_zero_set(
        offset in -128i8..=127i8,
        pc_start in 0x0200u16..=0xFE00
    ) {
        let mut memory = Memory::new();
        memory.write(pc_start, 0xD0); // BNE opcode
        memory.write(pc_start.wrapping_add(1), offset as u8);
        
        let mut cpu = Cpu::new(memory, pc_start);
        cpu.state.flag_zero = true; // Condition is false
        
        cpu.step().unwrap();
        
        prop_assert_eq!(cpu.state.pc, pc_start.wrapping_add(2));
    }
    
    #[test]
    fn bmi_branches_when_negative_set(
        offset in -128i8..=127i8,
        pc_start in 0x0200u16..=0xFE00
    ) {
        let mut memory = Memory::new();
        memory.write(pc_start, 0x30); // BMI opcode
        memory.write(pc_start.wrapping_add(1), offset as u8);
        
        let mut cpu = Cpu::new(memory, pc_start);
        cpu.state.flag_negative = true; // Condition is true
        
        cpu.step().unwrap();
        
        let expected_pc = pc_start.wrapping_add(2).wrapping_add(offset as i16 as u16);
        prop_assert_eq!(cpu.state.pc, expected_pc);
    }
    
    #[test]
    fn bmi_does_not_branch_when_negative_clear(
        offset in -128i8..=127i8,
        pc_start in 0x0200u16..=0xFE00
    ) {
        let mut memory = Memory::new();
        memory.write(pc_start, 0x30); // BMI opcode
        memory.write(pc_start.wrapping_add(1), offset as u8);
        
        let mut cpu = Cpu::new(memory, pc_start);
        cpu.state.flag_negative = false; // Condition is false
        
        cpu.step().unwrap();
        
        prop_assert_eq!(cpu.state.pc, pc_start.wrapping_add(2));
    }
    
    #[test]
    fn bpl_branches_when_negative_clear(
        offset in -128i8..=127i8,
        pc_start in 0x0200u16..=0xFE00
    ) {
        let mut memory = Memory::new();
        memory.write(pc_start, 0x10); // BPL opcode
        memory.write(pc_start.wrapping_add(1), offset as u8);
        
        let mut cpu = Cpu::new(memory, pc_start);
        cpu.state.flag_negative = false; // Condition is true
        
        cpu.step().unwrap();
        
        let expected_pc = pc_start.wrapping_add(2).wrapping_add(offset as i16 as u16);
        prop_assert_eq!(cpu.state.pc, expected_pc);
    }
    
    #[test]
    fn bpl_does_not_branch_when_negative_set(
        offset in -128i8..=127i8,
        pc_start in 0x0200u16..=0xFE00
    ) {
        let mut memory = Memory::new();
        memory.write(pc_start, 0x10); // BPL opcode
        memory.write(pc_start.wrapping_add(1), offset as u8);
        
        let mut cpu = Cpu::new(memory, pc_start);
        cpu.state.flag_negative = true; // Condition is false
        
        cpu.step().unwrap();
        
        prop_assert_eq!(cpu.state.pc, pc_start.wrapping_add(2));
    }
    
    #[test]
    fn bvc_branches_when_overflow_clear(
        offset in -128i8..=127i8,
        pc_start in 0x0200u16..=0xFE00
    ) {
        let mut memory = Memory::new();
        memory.write(pc_start, 0x50); // BVC opcode
        memory.write(pc_start.wrapping_add(1), offset as u8);
        
        let mut cpu = Cpu::new(memory, pc_start);
        cpu.state.flag_overflow = false; // Condition is true
        
        cpu.step().unwrap();
        
        let expected_pc = pc_start.wrapping_add(2).wrapping_add(offset as i16 as u16);
        prop_assert_eq!(cpu.state.pc, expected_pc);
    }
    
    #[test]
    fn bvc_does_not_branch_when_overflow_set(
        offset in -128i8..=127i8,
        pc_start in 0x0200u16..=0xFE00
    ) {
        let mut memory = Memory::new();
        memory.write(pc_start, 0x50); // BVC opcode
        memory.write(pc_start.wrapping_add(1), offset as u8);
        
        let mut cpu = Cpu::new(memory, pc_start);
        cpu.state.flag_overflow = true; // Condition is false
        
        cpu.step().unwrap();
        
        prop_assert_eq!(cpu.state.pc, pc_start.wrapping_add(2));
    }
    
    #[test]
    fn bvs_branches_when_overflow_set(
        offset in -128i8..=127i8,
        pc_start in 0x0200u16..=0xFE00
    ) {
        let mut memory = Memory::new();
        memory.write(pc_start, 0x70); // BVS opcode
        memory.write(pc_start.wrapping_add(1), offset as u8);
        
        let mut cpu = Cpu::new(memory, pc_start);
        cpu.state.flag_overflow = true; // Condition is true
        
        cpu.step().unwrap();
        
        let expected_pc = pc_start.wrapping_add(2).wrapping_add(offset as i16 as u16);
        prop_assert_eq!(cpu.state.pc, expected_pc);
    }
    
    #[test]
    fn bvs_does_not_branch_when_overflow_clear(
        offset in -128i8..=127i8,
        pc_start in 0x0200u16..=0xFE00
    ) {
        let mut memory = Memory::new();
        memory.write(pc_start, 0x70); // BVS opcode
        memory.write(pc_start.wrapping_add(1), offset as u8);
        
        let mut cpu = Cpu::new(memory, pc_start);
        cpu.state.flag_overflow = false; // Condition is false
        
        cpu.step().unwrap();
        
        prop_assert_eq!(cpu.state.pc, pc_start.wrapping_add(2));
    }
    
    #[test]
    fn branch_forward_wraps_correctly(
        pc_start in 0xFF00u16..=0xFFFF
    ) {
        let offset = 0x10i8; // Forward branch
        
        let mut memory = Memory::new();
        memory.write(pc_start, 0xD0); // BNE opcode
        memory.write(pc_start.wrapping_add(1), offset as u8);
        
        let mut cpu = Cpu::new(memory, pc_start);
        cpu.state.flag_zero = false; // Branch will be taken
        
        cpu.step().unwrap();
        
        // Should wrap around correctly
        let expected_pc = pc_start.wrapping_add(2).wrapping_add(offset as i16 as u16);
        prop_assert_eq!(cpu.state.pc, expected_pc);
    }
    
    #[test]
    fn branch_backward_wraps_correctly(
        pc_start in 0x0200u16..=0x0300
    ) {
        let offset = -32i8; // Backward branch
        
        let mut memory = Memory::new();
        memory.write(pc_start, 0xD0); // BNE opcode
        memory.write(pc_start.wrapping_add(1), offset as u8);
        
        let mut cpu = Cpu::new(memory, pc_start);
        cpu.state.flag_zero = false; // Branch will be taken
        
        cpu.step().unwrap();
        
        // Should wrap around correctly
        let expected_pc = pc_start.wrapping_add(2).wrapping_add(offset as i16 as u16);
        prop_assert_eq!(cpu.state.pc, expected_pc);
    }
}
