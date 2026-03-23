// Property-based tests for CPU flag updates
// Tests universal properties for Zero and Negative flag behavior

use cpu_6502_emulator::cpu::{Cpu, CpuState};
use cpu_6502_emulator::memory::Memory;
use proptest::prelude::*;

// Property 8: Zero Flag Correctness
// For any instruction that affects the Zero flag, after execution:
// - If the result is 0x00, the Zero flag should be set
// - If the result is non-zero, the Zero flag should be clear
// **Validates: Requirements 9.1**
proptest! {
    #[test]
    fn prop_zero_flag_set_when_result_is_zero(
        start_address in 0u16..=0xFFFF,
    ) {
        let mut cpu_state = CpuState::new(start_address);
        
        // Test with zero value
        cpu_state.update_zero_negative(0x00);
        
        prop_assert!(cpu_state.flag_zero,
            "Zero flag should be set when result is 0x00");
    }
    
    #[test]
    fn prop_zero_flag_clear_when_result_is_nonzero(
        start_address in 0u16..=0xFFFF,
        value in 1u8..=0xFF,  // Any non-zero value
    ) {
        let mut cpu_state = CpuState::new(start_address);
        
        // Test with non-zero value
        cpu_state.update_zero_negative(value);
        
        prop_assert!(!cpu_state.flag_zero,
            "Zero flag should be clear when result is 0x{:02X} (non-zero)", value);
    }
}

// Property 9: Negative Flag Correctness
// For any instruction that affects the Negative flag, after execution:
// - If bit 7 of the result is 1, the Negative flag should be set
// - If bit 7 of the result is 0, the Negative flag should be clear
// **Validates: Requirements 9.2**
proptest! {
    #[test]
    fn prop_negative_flag_set_when_bit7_is_one(
        start_address in 0u16..=0xFFFF,
        value in 0x80u8..=0xFF,  // Values with bit 7 set (0x80-0xFF)
    ) {
        let mut cpu_state = CpuState::new(start_address);
        
        // Test with value that has bit 7 set
        cpu_state.update_zero_negative(value);
        
        prop_assert!(cpu_state.flag_negative,
            "Negative flag should be set when bit 7 is 1 (value 0x{:02X})", value);
    }
    
    #[test]
    fn prop_negative_flag_clear_when_bit7_is_zero(
        start_address in 0u16..=0xFFFF,
        value in 0x00u8..=0x7F,  // Values with bit 7 clear (0x00-0x7F)
    ) {
        let mut cpu_state = CpuState::new(start_address);
        
        // Test with value that has bit 7 clear
        cpu_state.update_zero_negative(value);
        
        prop_assert!(!cpu_state.flag_negative,
            "Negative flag should be clear when bit 7 is 0 (value 0x{:02X})", value);
    }
}

// Combined property test: Both flags updated correctly for any value
proptest! {
    #[test]
    fn prop_zero_and_negative_flags_updated_correctly(
        start_address in 0u16..=0xFFFF,
        value in 0u8..=0xFF,
    ) {
        let mut cpu_state = CpuState::new(start_address);
        
        // Update flags based on value
        cpu_state.update_zero_negative(value);
        
        // Check Zero flag
        let expected_zero = value == 0x00;
        prop_assert_eq!(cpu_state.flag_zero, expected_zero,
            "Zero flag should be {} for value 0x{:02X}",
            expected_zero, value);
        
        // Check Negative flag
        let expected_negative = (value & 0x80) != 0;
        prop_assert_eq!(cpu_state.flag_negative, expected_negative,
            "Negative flag should be {} for value 0x{:02X}",
            expected_negative, value);
    }
}

// Property test: Other flags should remain unchanged when updating Z and N
proptest! {
    #[test]
    fn prop_other_flags_unchanged_when_updating_zero_negative(
        start_address in 0u16..=0xFFFF,
        value in 0u8..=0xFF,
        initial_carry: bool,
        initial_interrupt: bool,
        initial_decimal: bool,
        initial_break: bool,
        initial_overflow: bool,
    ) {
        let mut cpu_state = CpuState::new(start_address);
        
        // Set initial flag states
        cpu_state.flag_carry = initial_carry;
        cpu_state.flag_interrupt_disable = initial_interrupt;
        cpu_state.flag_decimal = initial_decimal;
        cpu_state.flag_break = initial_break;
        cpu_state.flag_overflow = initial_overflow;
        
        // Update zero and negative flags
        cpu_state.update_zero_negative(value);
        
        // Verify other flags remain unchanged
        prop_assert_eq!(cpu_state.flag_carry, initial_carry,
            "Carry flag should remain unchanged");
        prop_assert_eq!(cpu_state.flag_interrupt_disable, initial_interrupt,
            "Interrupt Disable flag should remain unchanged");
        prop_assert_eq!(cpu_state.flag_decimal, initial_decimal,
            "Decimal flag should remain unchanged");
        prop_assert_eq!(cpu_state.flag_break, initial_break,
            "Break flag should remain unchanged");
        prop_assert_eq!(cpu_state.flag_overflow, initial_overflow,
            "Overflow flag should remain unchanged");
    }
}

// Property 12: Flag Manipulation Instructions
// For any flag manipulation instruction (CLC, SEC, CLI, SEI, CLV, CLD, SED):
// - The instruction should set or clear exactly the specified flag
// - All other flags should remain unchanged
// **Validates: Requirements 9.5**
proptest! {
    #[test]
    fn prop_clc_clears_only_carry_flag(
        start_address in 0u16..=0xFFFF,
        initial_carry: bool,
        initial_zero: bool,
        initial_interrupt: bool,
        initial_decimal: bool,
        initial_break: bool,
        initial_overflow: bool,
        initial_negative: bool,
    ) {
        let mut memory = Memory::new();
        memory.write(start_address, 0x18);  // CLC opcode
        
        let mut cpu = Cpu::new(memory, start_address);
        
        // Set initial flag states
        cpu.state.flag_carry = initial_carry;
        cpu.state.flag_zero = initial_zero;
        cpu.state.flag_interrupt_disable = initial_interrupt;
        cpu.state.flag_decimal = initial_decimal;
        cpu.state.flag_break = initial_break;
        cpu.state.flag_overflow = initial_overflow;
        cpu.state.flag_negative = initial_negative;
        
        // Execute CLC
        cpu.step().unwrap();
        
        // Verify carry flag is cleared
        prop_assert!(!cpu.state.flag_carry, "CLC should clear carry flag");
        
        // Verify all other flags remain unchanged
        prop_assert_eq!(cpu.state.flag_zero, initial_zero, "Zero flag should remain unchanged");
        prop_assert_eq!(cpu.state.flag_interrupt_disable, initial_interrupt, "Interrupt flag should remain unchanged");
        prop_assert_eq!(cpu.state.flag_decimal, initial_decimal, "Decimal flag should remain unchanged");
        prop_assert_eq!(cpu.state.flag_break, initial_break, "Break flag should remain unchanged");
        prop_assert_eq!(cpu.state.flag_overflow, initial_overflow, "Overflow flag should remain unchanged");
        prop_assert_eq!(cpu.state.flag_negative, initial_negative, "Negative flag should remain unchanged");
    }
    
    #[test]
    fn prop_sec_sets_only_carry_flag(
        start_address in 0u16..=0xFFFF,
        initial_carry: bool,
        initial_zero: bool,
        initial_interrupt: bool,
        initial_decimal: bool,
        initial_break: bool,
        initial_overflow: bool,
        initial_negative: bool,
    ) {
        let mut memory = Memory::new();
        memory.write(start_address, 0x38);  // SEC opcode
        
        let mut cpu = Cpu::new(memory, start_address);
        
        // Set initial flag states
        cpu.state.flag_carry = initial_carry;
        cpu.state.flag_zero = initial_zero;
        cpu.state.flag_interrupt_disable = initial_interrupt;
        cpu.state.flag_decimal = initial_decimal;
        cpu.state.flag_break = initial_break;
        cpu.state.flag_overflow = initial_overflow;
        cpu.state.flag_negative = initial_negative;
        
        // Execute SEC
        cpu.step().unwrap();
        
        // Verify carry flag is set
        prop_assert!(cpu.state.flag_carry, "SEC should set carry flag");
        
        // Verify all other flags remain unchanged
        prop_assert_eq!(cpu.state.flag_zero, initial_zero, "Zero flag should remain unchanged");
        prop_assert_eq!(cpu.state.flag_interrupt_disable, initial_interrupt, "Interrupt flag should remain unchanged");
        prop_assert_eq!(cpu.state.flag_decimal, initial_decimal, "Decimal flag should remain unchanged");
        prop_assert_eq!(cpu.state.flag_break, initial_break, "Break flag should remain unchanged");
        prop_assert_eq!(cpu.state.flag_overflow, initial_overflow, "Overflow flag should remain unchanged");
        prop_assert_eq!(cpu.state.flag_negative, initial_negative, "Negative flag should remain unchanged");
    }
    
    #[test]
    fn prop_cli_clears_only_interrupt_flag(
        start_address in 0u16..=0xFFFF,
        initial_carry: bool,
        initial_zero: bool,
        initial_interrupt: bool,
        initial_decimal: bool,
        initial_break: bool,
        initial_overflow: bool,
        initial_negative: bool,
    ) {
        let mut memory = Memory::new();
        memory.write(start_address, 0x58);  // CLI opcode
        
        let mut cpu = Cpu::new(memory, start_address);
        
        // Set initial flag states
        cpu.state.flag_carry = initial_carry;
        cpu.state.flag_zero = initial_zero;
        cpu.state.flag_interrupt_disable = initial_interrupt;
        cpu.state.flag_decimal = initial_decimal;
        cpu.state.flag_break = initial_break;
        cpu.state.flag_overflow = initial_overflow;
        cpu.state.flag_negative = initial_negative;
        
        // Execute CLI
        cpu.step().unwrap();
        
        // Verify interrupt disable flag is cleared
        prop_assert!(!cpu.state.flag_interrupt_disable, "CLI should clear interrupt disable flag");
        
        // Verify all other flags remain unchanged
        prop_assert_eq!(cpu.state.flag_carry, initial_carry, "Carry flag should remain unchanged");
        prop_assert_eq!(cpu.state.flag_zero, initial_zero, "Zero flag should remain unchanged");
        prop_assert_eq!(cpu.state.flag_decimal, initial_decimal, "Decimal flag should remain unchanged");
        prop_assert_eq!(cpu.state.flag_break, initial_break, "Break flag should remain unchanged");
        prop_assert_eq!(cpu.state.flag_overflow, initial_overflow, "Overflow flag should remain unchanged");
        prop_assert_eq!(cpu.state.flag_negative, initial_negative, "Negative flag should remain unchanged");
    }
    
    #[test]
    fn prop_sei_sets_only_interrupt_flag(
        start_address in 0u16..=0xFFFF,
        initial_carry: bool,
        initial_zero: bool,
        initial_interrupt: bool,
        initial_decimal: bool,
        initial_break: bool,
        initial_overflow: bool,
        initial_negative: bool,
    ) {
        let mut memory = Memory::new();
        memory.write(start_address, 0x78);  // SEI opcode
        
        let mut cpu = Cpu::new(memory, start_address);
        
        // Set initial flag states
        cpu.state.flag_carry = initial_carry;
        cpu.state.flag_zero = initial_zero;
        cpu.state.flag_interrupt_disable = initial_interrupt;
        cpu.state.flag_decimal = initial_decimal;
        cpu.state.flag_break = initial_break;
        cpu.state.flag_overflow = initial_overflow;
        cpu.state.flag_negative = initial_negative;
        
        // Execute SEI
        cpu.step().unwrap();
        
        // Verify interrupt disable flag is set
        prop_assert!(cpu.state.flag_interrupt_disable, "SEI should set interrupt disable flag");
        
        // Verify all other flags remain unchanged
        prop_assert_eq!(cpu.state.flag_carry, initial_carry, "Carry flag should remain unchanged");
        prop_assert_eq!(cpu.state.flag_zero, initial_zero, "Zero flag should remain unchanged");
        prop_assert_eq!(cpu.state.flag_decimal, initial_decimal, "Decimal flag should remain unchanged");
        prop_assert_eq!(cpu.state.flag_break, initial_break, "Break flag should remain unchanged");
        prop_assert_eq!(cpu.state.flag_overflow, initial_overflow, "Overflow flag should remain unchanged");
        prop_assert_eq!(cpu.state.flag_negative, initial_negative, "Negative flag should remain unchanged");
    }
    
    #[test]
    fn prop_clv_clears_only_overflow_flag(
        start_address in 0u16..=0xFFFF,
        initial_carry: bool,
        initial_zero: bool,
        initial_interrupt: bool,
        initial_decimal: bool,
        initial_break: bool,
        initial_overflow: bool,
        initial_negative: bool,
    ) {
        let mut memory = Memory::new();
        memory.write(start_address, 0xB8);  // CLV opcode
        
        let mut cpu = Cpu::new(memory, start_address);
        
        // Set initial flag states
        cpu.state.flag_carry = initial_carry;
        cpu.state.flag_zero = initial_zero;
        cpu.state.flag_interrupt_disable = initial_interrupt;
        cpu.state.flag_decimal = initial_decimal;
        cpu.state.flag_break = initial_break;
        cpu.state.flag_overflow = initial_overflow;
        cpu.state.flag_negative = initial_negative;
        
        // Execute CLV
        cpu.step().unwrap();
        
        // Verify overflow flag is cleared
        prop_assert!(!cpu.state.flag_overflow, "CLV should clear overflow flag");
        
        // Verify all other flags remain unchanged
        prop_assert_eq!(cpu.state.flag_carry, initial_carry, "Carry flag should remain unchanged");
        prop_assert_eq!(cpu.state.flag_zero, initial_zero, "Zero flag should remain unchanged");
        prop_assert_eq!(cpu.state.flag_interrupt_disable, initial_interrupt, "Interrupt flag should remain unchanged");
        prop_assert_eq!(cpu.state.flag_decimal, initial_decimal, "Decimal flag should remain unchanged");
        prop_assert_eq!(cpu.state.flag_break, initial_break, "Break flag should remain unchanged");
        prop_assert_eq!(cpu.state.flag_negative, initial_negative, "Negative flag should remain unchanged");
    }
    
    #[test]
    fn prop_cld_clears_only_decimal_flag(
        start_address in 0u16..=0xFFFF,
        initial_carry: bool,
        initial_zero: bool,
        initial_interrupt: bool,
        initial_decimal: bool,
        initial_break: bool,
        initial_overflow: bool,
        initial_negative: bool,
    ) {
        let mut memory = Memory::new();
        memory.write(start_address, 0xD8);  // CLD opcode
        
        let mut cpu = Cpu::new(memory, start_address);
        
        // Set initial flag states
        cpu.state.flag_carry = initial_carry;
        cpu.state.flag_zero = initial_zero;
        cpu.state.flag_interrupt_disable = initial_interrupt;
        cpu.state.flag_decimal = initial_decimal;
        cpu.state.flag_break = initial_break;
        cpu.state.flag_overflow = initial_overflow;
        cpu.state.flag_negative = initial_negative;
        
        // Execute CLD
        cpu.step().unwrap();
        
        // Verify decimal flag is cleared
        prop_assert!(!cpu.state.flag_decimal, "CLD should clear decimal flag");
        
        // Verify all other flags remain unchanged
        prop_assert_eq!(cpu.state.flag_carry, initial_carry, "Carry flag should remain unchanged");
        prop_assert_eq!(cpu.state.flag_zero, initial_zero, "Zero flag should remain unchanged");
        prop_assert_eq!(cpu.state.flag_interrupt_disable, initial_interrupt, "Interrupt flag should remain unchanged");
        prop_assert_eq!(cpu.state.flag_break, initial_break, "Break flag should remain unchanged");
        prop_assert_eq!(cpu.state.flag_overflow, initial_overflow, "Overflow flag should remain unchanged");
        prop_assert_eq!(cpu.state.flag_negative, initial_negative, "Negative flag should remain unchanged");
    }
    
    #[test]
    fn prop_sed_sets_only_decimal_flag(
        start_address in 0u16..=0xFFFF,
        initial_carry: bool,
        initial_zero: bool,
        initial_interrupt: bool,
        initial_decimal: bool,
        initial_break: bool,
        initial_overflow: bool,
        initial_negative: bool,
    ) {
        let mut memory = Memory::new();
        memory.write(start_address, 0xF8);  // SED opcode
        
        let mut cpu = Cpu::new(memory, start_address);
        
        // Set initial flag states
        cpu.state.flag_carry = initial_carry;
        cpu.state.flag_zero = initial_zero;
        cpu.state.flag_interrupt_disable = initial_interrupt;
        cpu.state.flag_decimal = initial_decimal;
        cpu.state.flag_break = initial_break;
        cpu.state.flag_overflow = initial_overflow;
        cpu.state.flag_negative = initial_negative;
        
        // Execute SED
        cpu.step().unwrap();
        
        // Verify decimal flag is set
        prop_assert!(cpu.state.flag_decimal, "SED should set decimal flag");
        
        // Verify all other flags remain unchanged
        prop_assert_eq!(cpu.state.flag_carry, initial_carry, "Carry flag should remain unchanged");
        prop_assert_eq!(cpu.state.flag_zero, initial_zero, "Zero flag should remain unchanged");
        prop_assert_eq!(cpu.state.flag_interrupt_disable, initial_interrupt, "Interrupt flag should remain unchanged");
        prop_assert_eq!(cpu.state.flag_break, initial_break, "Break flag should remain unchanged");
        prop_assert_eq!(cpu.state.flag_overflow, initial_overflow, "Overflow flag should remain unchanged");
        prop_assert_eq!(cpu.state.flag_negative, initial_negative, "Negative flag should remain unchanged");
    }
}
