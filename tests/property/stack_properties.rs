// Property-based tests for stack operations
// Feature: 6502-cpu-emulator, Property 18: Stack Operation Correctness

use cpu_6502_emulator::cpu::Cpu;
use cpu_6502_emulator::memory::Memory;
use proptest::prelude::*;

// Feature: 6502-cpu-emulator, Property 18: Stack Operation Correctness
// **Validates: Requirements 8.8**
//
// For any value, pushing to the stack then pulling from the stack should return
// the same value (round-trip property).

proptest! {
    #[test]
    fn pha_pla_round_trip_preserves_accumulator_value(
        initial_a in 0u8..=255,
        initial_sp in 0x02u8..=0xFF
    ) {
        let mut memory = Memory::new();
        
        // Set up PHA instruction at 0x0200
        memory.write(0x0200, 0x48); // PHA opcode
        
        // Set up PLA instruction at 0x0201
        memory.write(0x0201, 0x68); // PLA opcode
        
        let mut cpu = Cpu::new(memory, 0x0200);
        cpu.state.a = initial_a;
        cpu.state.sp = initial_sp;
        
        // Execute PHA
        cpu.step().unwrap();
        
        // Stack pointer should have decremented by 1
        prop_assert_eq!(cpu.state.sp, initial_sp.wrapping_sub(1));
        
        // Accumulator should still have the same value
        prop_assert_eq!(cpu.state.a, initial_a);
        
        // Value should be on the stack
        let stack_addr = 0x0100 | (initial_sp as u16);
        prop_assert_eq!(cpu.memory.read(stack_addr), initial_a);
        
        // Change accumulator to verify PLA restores it
        cpu.state.a = !initial_a;
        
        // Execute PLA
        cpu.step().unwrap();
        
        // Stack pointer should be restored
        prop_assert_eq!(cpu.state.sp, initial_sp);
        
        // Accumulator should be restored to original value
        prop_assert_eq!(cpu.state.a, initial_a);
    }
    
    #[test]
    fn php_plp_round_trip_preserves_status_flags(
        initial_sp in 0x02u8..=0xFF,
        flag_carry in proptest::bool::ANY,
        flag_zero in proptest::bool::ANY,
        flag_interrupt_disable in proptest::bool::ANY,
        flag_decimal in proptest::bool::ANY,
        flag_overflow in proptest::bool::ANY,
        flag_negative in proptest::bool::ANY
    ) {
        let mut memory = Memory::new();
        
        // Set up PHP instruction at 0x0200
        memory.write(0x0200, 0x08); // PHP opcode
        
        // Set up PLP instruction at 0x0201
        memory.write(0x0201, 0x28); // PLP opcode
        
        let mut cpu = Cpu::new(memory, 0x0200);
        cpu.state.sp = initial_sp;
        
        // Set initial flags
        cpu.state.flag_carry = flag_carry;
        cpu.state.flag_zero = flag_zero;
        cpu.state.flag_interrupt_disable = flag_interrupt_disable;
        cpu.state.flag_decimal = flag_decimal;
        cpu.state.flag_overflow = flag_overflow;
        cpu.state.flag_negative = flag_negative;
        
        let initial_status = cpu.state.get_status_byte();
        
        // Execute PHP
        cpu.step().unwrap();
        
        // Stack pointer should have decremented by 1
        prop_assert_eq!(cpu.state.sp, initial_sp.wrapping_sub(1));
        
        // Status should be on the stack (with B flag set)
        let stack_addr = 0x0100 | (initial_sp as u16);
        let pushed_status = cpu.memory.read(stack_addr);
        // PHP sets the B flag when pushing
        prop_assert_eq!(pushed_status, initial_status | 0b0001_0000);
        
        // Change all flags to verify PLP restores them
        cpu.state.flag_carry = !flag_carry;
        cpu.state.flag_zero = !flag_zero;
        cpu.state.flag_interrupt_disable = !flag_interrupt_disable;
        cpu.state.flag_decimal = !flag_decimal;
        cpu.state.flag_overflow = !flag_overflow;
        cpu.state.flag_negative = !flag_negative;
        
        // Execute PLP
        cpu.step().unwrap();
        
        // Stack pointer should be restored
        prop_assert_eq!(cpu.state.sp, initial_sp);
        
        // All flags should be restored to original values
        prop_assert_eq!(cpu.state.flag_carry, flag_carry);
        prop_assert_eq!(cpu.state.flag_zero, flag_zero);
        prop_assert_eq!(cpu.state.flag_interrupt_disable, flag_interrupt_disable);
        prop_assert_eq!(cpu.state.flag_decimal, flag_decimal);
        prop_assert_eq!(cpu.state.flag_overflow, flag_overflow);
        prop_assert_eq!(cpu.state.flag_negative, flag_negative);
    }
    
    #[test]
    fn multiple_pushes_and_pulls_maintain_lifo_order(
        value1 in 0u8..=255,
        value2 in 0u8..=255,
        value3 in 0u8..=255,
        initial_sp in 0x04u8..=0xFF
    ) {
        let mut memory = Memory::new();
        
        // Set up three PHA instructions
        memory.write(0x0200, 0x48); // PHA
        memory.write(0x0201, 0x48); // PHA
        memory.write(0x0202, 0x48); // PHA
        
        // Set up three PLA instructions
        memory.write(0x0203, 0x68); // PLA
        memory.write(0x0204, 0x68); // PLA
        memory.write(0x0205, 0x68); // PLA
        
        let mut cpu = Cpu::new(memory, 0x0200);
        cpu.state.sp = initial_sp;
        
        // Push value1
        cpu.state.a = value1;
        cpu.step().unwrap();
        prop_assert_eq!(cpu.state.sp, initial_sp.wrapping_sub(1));
        
        // Push value2
        cpu.state.a = value2;
        cpu.step().unwrap();
        prop_assert_eq!(cpu.state.sp, initial_sp.wrapping_sub(2));
        
        // Push value3
        cpu.state.a = value3;
        cpu.step().unwrap();
        prop_assert_eq!(cpu.state.sp, initial_sp.wrapping_sub(3));
        
        // Pull should return value3 (LIFO - last in, first out)
        cpu.step().unwrap();
        prop_assert_eq!(cpu.state.a, value3);
        prop_assert_eq!(cpu.state.sp, initial_sp.wrapping_sub(2));
        
        // Pull should return value2
        cpu.step().unwrap();
        prop_assert_eq!(cpu.state.a, value2);
        prop_assert_eq!(cpu.state.sp, initial_sp.wrapping_sub(1));
        
        // Pull should return value1
        cpu.step().unwrap();
        prop_assert_eq!(cpu.state.a, value1);
        prop_assert_eq!(cpu.state.sp, initial_sp);
    }
    
    #[test]
    fn tsx_transfers_stack_pointer_to_x_and_updates_flags(
        initial_sp in 0u8..=255
    ) {
        let mut memory = Memory::new();
        
        // Set up TSX instruction at 0x0200
        memory.write(0x0200, 0xBA); // TSX opcode
        
        let mut cpu = Cpu::new(memory, 0x0200);
        cpu.state.sp = initial_sp;
        cpu.state.x = !initial_sp; // Set X to different value
        
        // Execute TSX
        cpu.step().unwrap();
        
        // X should now equal SP
        prop_assert_eq!(cpu.state.x, initial_sp);
        
        // SP should be unchanged
        prop_assert_eq!(cpu.state.sp, initial_sp);
        
        // Zero flag should be set if SP is 0
        prop_assert_eq!(cpu.state.flag_zero, initial_sp == 0);
        
        // Negative flag should be set if bit 7 of SP is 1
        prop_assert_eq!(cpu.state.flag_negative, (initial_sp & 0x80) != 0);
    }
    
    #[test]
    fn txs_transfers_x_to_stack_pointer_without_affecting_flags(
        initial_x in 0u8..=255,
        initial_sp in 0u8..=255,
        flag_zero in proptest::bool::ANY,
        flag_negative in proptest::bool::ANY
    ) {
        let mut memory = Memory::new();
        
        // Set up TXS instruction at 0x0200
        memory.write(0x0200, 0x9A); // TXS opcode
        
        let mut cpu = Cpu::new(memory, 0x0200);
        cpu.state.x = initial_x;
        cpu.state.sp = initial_sp;
        cpu.state.flag_zero = flag_zero;
        cpu.state.flag_negative = flag_negative;
        
        // Execute TXS
        cpu.step().unwrap();
        
        // SP should now equal X
        prop_assert_eq!(cpu.state.sp, initial_x);
        
        // X should be unchanged
        prop_assert_eq!(cpu.state.x, initial_x);
        
        // Flags should be unchanged (TXS does not affect flags)
        prop_assert_eq!(cpu.state.flag_zero, flag_zero);
        prop_assert_eq!(cpu.state.flag_negative, flag_negative);
    }
    
    #[test]
    fn pla_updates_zero_and_negative_flags(
        value in 0u8..=255,
        initial_sp in 0x02u8..=0xFF
    ) {
        let mut memory = Memory::new();
        
        // Set up PHA instruction at 0x0200
        memory.write(0x0200, 0x48); // PHA opcode
        
        // Set up PLA instruction at 0x0201
        memory.write(0x0201, 0x68); // PLA opcode
        
        let mut cpu = Cpu::new(memory, 0x0200);
        cpu.state.a = value;
        cpu.state.sp = initial_sp;
        
        // Execute PHA
        cpu.step().unwrap();
        
        // Change accumulator and flags
        cpu.state.a = 0;
        cpu.state.flag_zero = false;
        cpu.state.flag_negative = false;
        
        // Execute PLA
        cpu.step().unwrap();
        
        // Accumulator should be restored
        prop_assert_eq!(cpu.state.a, value);
        
        // Zero flag should be set if value is 0
        prop_assert_eq!(cpu.state.flag_zero, value == 0);
        
        // Negative flag should be set if bit 7 is 1
        prop_assert_eq!(cpu.state.flag_negative, (value & 0x80) != 0);
    }
    
    #[test]
    fn stack_operations_wrap_correctly_at_boundaries(
        initial_sp in 0x00u8..=0x02
    ) {
        let mut memory = Memory::new();
        
        // Set up PHA at 0x0200
        memory.write(0x0200, 0x48); // PHA
        
        // Set up PLA at 0x0201
        memory.write(0x0201, 0x68); // PLA
        
        let mut cpu = Cpu::new(memory, 0x0200);
        cpu.state.sp = initial_sp;
        cpu.state.a = 0x42;
        
        // Execute PHA - should wrap if SP is 0
        cpu.step().unwrap();
        
        let expected_sp = initial_sp.wrapping_sub(1);
        prop_assert_eq!(cpu.state.sp, expected_sp);
        
        // Value should be on stack
        let stack_addr = 0x0100 | (initial_sp as u16);
        prop_assert_eq!(cpu.memory.read(stack_addr), 0x42);
        
        // Execute PLA - should restore
        cpu.step().unwrap();
        prop_assert_eq!(cpu.state.sp, initial_sp);
        prop_assert_eq!(cpu.state.a, 0x42);
    }
}
