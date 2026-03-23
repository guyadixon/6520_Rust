// Property-based tests for control flow instructions
// Feature: 6502-cpu-emulator, Property 19: Subroutine Call Round-Trip

use cpu_6502_emulator::cpu::Cpu;
use cpu_6502_emulator::memory::Memory;
use proptest::prelude::*;

// Feature: 6502-cpu-emulator, Property 19: Subroutine Call Round-Trip
// **Validates: Requirements 8.7**
//
// For any valid subroutine address, executing JSR followed by RTS should return
// the Program Counter to the instruction immediately after the JSR.

proptest! {
    #[test]
    fn jsr_rts_round_trip_returns_to_correct_address(
        jsr_location in 0x0200u16..=0xFD00,
        subroutine_addr in 0x0200u16..=0xFE00
    ) {
        let mut memory = Memory::new();
        
        // Set up JSR instruction at jsr_location
        memory.write(jsr_location, 0x20); // JSR opcode
        memory.write(jsr_location.wrapping_add(1), (subroutine_addr & 0xFF) as u8);
        memory.write(jsr_location.wrapping_add(2), (subroutine_addr >> 8) as u8);
        
        // Set up RTS instruction at subroutine_addr
        memory.write(subroutine_addr, 0x60); // RTS opcode
        
        // Set up a NOP instruction after JSR (at jsr_location + 3)
        memory.write(jsr_location.wrapping_add(3), 0xEA); // NOP opcode
        
        let mut cpu = Cpu::new(memory, jsr_location);
        
        // Save initial state (except PC and SP which will change)
        let initial_a = cpu.state.a;
        let initial_x = cpu.state.x;
        let initial_y = cpu.state.y;
        let initial_flags = (
            cpu.state.flag_carry,
            cpu.state.flag_zero,
            cpu.state.flag_interrupt_disable,
            cpu.state.flag_decimal,
            cpu.state.flag_break,
            cpu.state.flag_overflow,
            cpu.state.flag_negative
        );
        let initial_sp = cpu.state.sp;
        
        // Execute JSR
        cpu.step().unwrap();
        
        // PC should now be at subroutine_addr
        prop_assert_eq!(cpu.state.pc, subroutine_addr);
        
        // Stack pointer should have decremented by 2
        prop_assert_eq!(cpu.state.sp, initial_sp.wrapping_sub(2));
        
        // Execute RTS
        cpu.step().unwrap();
        
        // PC should be at the instruction after JSR (jsr_location + 3)
        let expected_return = jsr_location.wrapping_add(3);
        prop_assert_eq!(cpu.state.pc, expected_return);
        
        // Stack pointer should be restored
        prop_assert_eq!(cpu.state.sp, initial_sp);
        
        // All registers should be unchanged
        prop_assert_eq!(cpu.state.a, initial_a);
        prop_assert_eq!(cpu.state.x, initial_x);
        prop_assert_eq!(cpu.state.y, initial_y);
        
        // All flags should be unchanged
        prop_assert_eq!(cpu.state.flag_carry, initial_flags.0);
        prop_assert_eq!(cpu.state.flag_zero, initial_flags.1);
        prop_assert_eq!(cpu.state.flag_interrupt_disable, initial_flags.2);
        prop_assert_eq!(cpu.state.flag_decimal, initial_flags.3);
        prop_assert_eq!(cpu.state.flag_break, initial_flags.4);
        prop_assert_eq!(cpu.state.flag_overflow, initial_flags.5);
        prop_assert_eq!(cpu.state.flag_negative, initial_flags.6);
    }
    
    #[test]
    fn jsr_rts_preserves_memory_except_stack(
        jsr_location in 0x0200u16..=0xFD00,
        subroutine_addr in 0x0200u16..=0xFE00,
        test_addr in 0x0300u16..=0x0FFF,
        test_value in 0u8..=255
    ) {
        // Ensure test_addr is not in stack area or instruction area
        prop_assume!(test_addr < 0x0100 || test_addr >= 0x0200);
        prop_assume!(test_addr != jsr_location);
        prop_assume!(test_addr != jsr_location.wrapping_add(1));
        prop_assume!(test_addr != jsr_location.wrapping_add(2));
        prop_assume!(test_addr != jsr_location.wrapping_add(3));
        prop_assume!(test_addr != subroutine_addr);
        
        let mut memory = Memory::new();
        
        // Set up JSR instruction
        memory.write(jsr_location, 0x20); // JSR opcode
        memory.write(jsr_location.wrapping_add(1), (subroutine_addr & 0xFF) as u8);
        memory.write(jsr_location.wrapping_add(2), (subroutine_addr >> 8) as u8);
        
        // Set up RTS instruction
        memory.write(subroutine_addr, 0x60); // RTS opcode
        
        // Write test value to memory
        memory.write(test_addr, test_value);
        
        let mut cpu = Cpu::new(memory, jsr_location);
        
        // Execute JSR
        cpu.step().unwrap();
        
        // Execute RTS
        cpu.step().unwrap();
        
        // Memory at test_addr should be unchanged
        prop_assert_eq!(cpu.memory.read(test_addr), test_value);
    }
    
    #[test]
    fn nested_jsr_rts_works_correctly(
        first_jsr_location in 0x0200u16..=0xFC00,
        first_subroutine in 0x0300u16..=0xFD00,
        second_subroutine in 0x0400u16..=0xFE00
    ) {
        // Ensure addresses don't overlap
        prop_assume!(first_subroutine != first_jsr_location);
        prop_assume!(second_subroutine != first_jsr_location);
        prop_assume!(second_subroutine != first_subroutine);
        // Ensure first_subroutine is at least 3 bytes away from first_jsr_location
        // to prevent instruction overlap (JSR is 3 bytes)
        prop_assume!(first_subroutine < first_jsr_location.saturating_sub(3) || 
                     first_subroutine > first_jsr_location + 3);
        prop_assume!(first_subroutine.wrapping_add(4) < second_subroutine || 
                     second_subroutine.wrapping_add(1) < first_subroutine);
        
        let mut memory = Memory::new();
        
        // First JSR at first_jsr_location
        memory.write(first_jsr_location, 0x20); // JSR
        memory.write(first_jsr_location.wrapping_add(1), (first_subroutine & 0xFF) as u8);
        memory.write(first_jsr_location.wrapping_add(2), (first_subroutine >> 8) as u8);
        
        // Second JSR at first_subroutine
        memory.write(first_subroutine, 0x20); // JSR
        memory.write(first_subroutine.wrapping_add(1), (second_subroutine & 0xFF) as u8);
        memory.write(first_subroutine.wrapping_add(2), (second_subroutine >> 8) as u8);
        
        // First RTS at first_subroutine + 3
        memory.write(first_subroutine.wrapping_add(3), 0x60); // RTS
        
        // Second RTS at second_subroutine
        memory.write(second_subroutine, 0x60); // RTS
        
        let mut cpu = Cpu::new(memory, first_jsr_location);
        let initial_sp = cpu.state.sp;
        
        // Execute first JSR
        cpu.step().unwrap();
        prop_assert_eq!(cpu.state.pc, first_subroutine);
        prop_assert_eq!(cpu.state.sp, initial_sp.wrapping_sub(2));
        
        // Execute second JSR
        cpu.step().unwrap();
        prop_assert_eq!(cpu.state.pc, second_subroutine);
        prop_assert_eq!(cpu.state.sp, initial_sp.wrapping_sub(4));
        
        // Execute second RTS (return to first subroutine)
        cpu.step().unwrap();
        prop_assert_eq!(cpu.state.pc, first_subroutine.wrapping_add(3));
        prop_assert_eq!(cpu.state.sp, initial_sp.wrapping_sub(2));
        
        // Execute first RTS (return to main)
        cpu.step().unwrap();
        prop_assert_eq!(cpu.state.pc, first_jsr_location.wrapping_add(3));
        prop_assert_eq!(cpu.state.sp, initial_sp);
    }
    
    #[test]
    fn jsr_with_different_stack_pointer_values(
        jsr_location in 0x0200u16..=0xFD00,
        subroutine_addr in 0x0200u16..=0xFE00,
        initial_sp in 0x02u8..=0xFF
    ) {
        let mut memory = Memory::new();
        
        // Set up JSR instruction
        memory.write(jsr_location, 0x20); // JSR opcode
        memory.write(jsr_location.wrapping_add(1), (subroutine_addr & 0xFF) as u8);
        memory.write(jsr_location.wrapping_add(2), (subroutine_addr >> 8) as u8);
        
        // Set up RTS instruction
        memory.write(subroutine_addr, 0x60); // RTS opcode
        
        let mut cpu = Cpu::new(memory, jsr_location);
        cpu.state.sp = initial_sp;
        
        // Execute JSR
        cpu.step().unwrap();
        
        // Stack pointer should have decremented by 2
        prop_assert_eq!(cpu.state.sp, initial_sp.wrapping_sub(2));
        
        // Execute RTS
        cpu.step().unwrap();
        
        // Stack pointer should be restored
        prop_assert_eq!(cpu.state.sp, initial_sp);
        
        // PC should be at the instruction after JSR
        prop_assert_eq!(cpu.state.pc, jsr_location.wrapping_add(3));
    }
}
