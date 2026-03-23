// Property-based tests for arithmetic instructions
// Feature: 6502-cpu-emulator, Property 15: Arithmetic Instruction Correctness

use cpu_6502_emulator::cpu::Cpu;
use cpu_6502_emulator::memory::Memory;
use cpu_6502_emulator::instruction::decode_opcode;
use proptest::prelude::*;

// Feature: 6502-cpu-emulator, Property 15: Arithmetic Instruction Correctness
// **Validates: Requirements 8.2**
//
// This property test verifies that arithmetic instructions (ADC, SBC, INC, DEC, INX, INY, DEX, DEY)
// correctly perform their operations and update flags appropriately.
//
// The test should:
// 1. For ADC/SBC: Verify correct addition/subtraction with carry/borrow
// 2. For INC/DEC: Verify correct increment/decrement of memory values
// 3. For INX/INY/DEX/DEY: Verify correct increment/decrement of registers
// 4. Verify Z and N flags are updated correctly for all operations
// 5. For ADC/SBC: Verify C and V flags are updated correctly

// ============================================================================
// ADC (Add with Carry) Property Tests
// ============================================================================

proptest! {
    #[test]
    fn adc_immediate_produces_correct_sum(
        a_value in 0u8..=0xFF,
        operand in 0u8..=0xFF,
        carry_in in proptest::bool::ANY
    ) {
        let mut memory = Memory::new();
        memory.write(0x1000, 0x69); // ADC immediate
        memory.write(0x1001, operand);
        
        let mut cpu = Cpu::new(memory, 0x1000);
        cpu.state.a = a_value;
        cpu.state.flag_carry = carry_in;
        
        let decoded = decode_opcode(0x69).unwrap();
        cpu.execute_instruction(decoded).unwrap();
        
        // Calculate expected result
        let carry_val = if carry_in { 1u16 } else { 0u16 };
        let sum = (a_value as u16) + (operand as u16) + carry_val;
        let expected_result = sum as u8;
        
        // Verify accumulator is updated correctly
        prop_assert_eq!(cpu.state.a, expected_result,
            "ADC: 0x{:02X} + 0x{:02X} + {} should equal 0x{:02X}",
            a_value, operand, carry_val, expected_result);
        
        // Verify Zero flag
        prop_assert_eq!(cpu.state.flag_zero, expected_result == 0,
            "Zero flag should be {} for result 0x{:02X}", expected_result == 0, expected_result);
        
        // Verify Negative flag
        prop_assert_eq!(cpu.state.flag_negative, (expected_result & 0x80) != 0,
            "Negative flag should be {} for result 0x{:02X}",
            (expected_result & 0x80) != 0, expected_result);
        
        // Verify Carry flag
        prop_assert_eq!(cpu.state.flag_carry, sum > 0xFF,
            "Carry flag should be {} when sum is 0x{:03X}", sum > 0xFF, sum);
        
        // Verify Overflow flag
        let a_sign = a_value & 0x80;
        let operand_sign = operand & 0x80;
        let result_sign = expected_result & 0x80;
        let expected_overflow = (a_sign == operand_sign) && (a_sign != result_sign);
        prop_assert_eq!(cpu.state.flag_overflow, expected_overflow,
            "Overflow flag should be {} for 0x{:02X} + 0x{:02X} = 0x{:02X}",
            expected_overflow, a_value, operand, expected_result);
    }
    
    #[test]
    fn adc_zero_page_produces_correct_sum(
        zp_addr in 0u8..=0xFF,
        a_value in 0u8..=0xFF,
        operand in 0u8..=0xFF,
        carry_in in proptest::bool::ANY
    ) {
        let mut memory = Memory::new();
        memory.write(0x1000, 0x65); // ADC zero page
        memory.write(0x1001, zp_addr);
        memory.write(zp_addr as u16, operand);
        
        let mut cpu = Cpu::new(memory, 0x1000);
        cpu.state.a = a_value;
        cpu.state.flag_carry = carry_in;
        
        let decoded = decode_opcode(0x65).unwrap();
        cpu.execute_instruction(decoded).unwrap();
        
        // Calculate expected result
        let carry_val = if carry_in { 1u16 } else { 0u16 };
        let sum = (a_value as u16) + (operand as u16) + carry_val;
        let expected_result = sum as u8;
        
        // Verify accumulator is updated correctly
        prop_assert_eq!(cpu.state.a, expected_result);
        
        // Verify flags
        prop_assert_eq!(cpu.state.flag_zero, expected_result == 0);
        prop_assert_eq!(cpu.state.flag_negative, (expected_result & 0x80) != 0);
        prop_assert_eq!(cpu.state.flag_carry, sum > 0xFF);
    }

    #[test]
    fn adc_absolute_produces_correct_sum(
        addr in 0x0200u16..=0x0FFF, // Exclude instruction area at 0x1000-0x1002
        a_value in 0u8..=0xFF,
        operand in 0u8..=0xFF,
        carry_in in proptest::bool::ANY
    ) {
        let mut memory = Memory::new();
        memory.write(addr, operand); // Write operand first to avoid overwriting instruction bytes
        memory.write(0x1000, 0x6D); // ADC absolute
        memory.write(0x1001, (addr & 0xFF) as u8);
        memory.write(0x1002, (addr >> 8) as u8);
        
        let mut cpu = Cpu::new(memory, 0x1000);
        cpu.state.a = a_value;
        cpu.state.flag_carry = carry_in;
        
        let decoded = decode_opcode(0x6D).unwrap();
        cpu.execute_instruction(decoded).unwrap();
        
        // Calculate expected result
        let carry_val = if carry_in { 1u16 } else { 0u16 };
        let sum = (a_value as u16) + (operand as u16) + carry_val;
        let expected_result = sum as u8;
        
        // Verify accumulator and flags
        prop_assert_eq!(cpu.state.a, expected_result);
        prop_assert_eq!(cpu.state.flag_zero, expected_result == 0);
        prop_assert_eq!(cpu.state.flag_negative, (expected_result & 0x80) != 0);
        prop_assert_eq!(cpu.state.flag_carry, sum > 0xFF);
    }
}

// ============================================================================
// SBC (Subtract with Carry) Property Tests
// ============================================================================

proptest! {
    #[test]
    fn sbc_immediate_produces_correct_difference(
        a_value in 0u8..=0xFF,
        operand in 0u8..=0xFF,
        carry_in in proptest::bool::ANY
    ) {
        let mut memory = Memory::new();
        memory.write(0x1000, 0xE9); // SBC immediate
        memory.write(0x1001, operand);
        
        let mut cpu = Cpu::new(memory, 0x1000);
        cpu.state.a = a_value;
        cpu.state.flag_carry = carry_in;
        
        let decoded = decode_opcode(0xE9).unwrap();
        cpu.execute_instruction(decoded).unwrap();
        
        // Calculate expected result: A - M - (1 - C) = A + ~M + C
        let carry_val = if carry_in { 1u16 } else { 0u16 };
        let sum = (a_value as u16) + (!operand as u16) + carry_val;
        let expected_result = sum as u8;
        
        // Verify accumulator is updated correctly
        prop_assert_eq!(cpu.state.a, expected_result,
            "SBC: 0x{:02X} - 0x{:02X} - {} should equal 0x{:02X}",
            a_value, operand, if carry_in { 0 } else { 1 }, expected_result);
        
        // Verify Zero flag
        prop_assert_eq!(cpu.state.flag_zero, expected_result == 0,
            "Zero flag should be {} for result 0x{:02X}", expected_result == 0, expected_result);
        
        // Verify Negative flag
        prop_assert_eq!(cpu.state.flag_negative, (expected_result & 0x80) != 0,
            "Negative flag should be {} for result 0x{:02X}",
            (expected_result & 0x80) != 0, expected_result);
        
        // Verify Carry flag (set if no borrow occurred)
        prop_assert_eq!(cpu.state.flag_carry, sum > 0xFF,
            "Carry flag should be {} when sum is 0x{:03X}", sum > 0xFF, sum);
        
        // Verify Overflow flag
        let a_sign = a_value & 0x80;
        let operand_sign = operand & 0x80;
        let result_sign = expected_result & 0x80;
        let expected_overflow = (a_sign != operand_sign) && (a_sign != result_sign);
        prop_assert_eq!(cpu.state.flag_overflow, expected_overflow,
            "Overflow flag should be {} for 0x{:02X} - 0x{:02X} = 0x{:02X}",
            expected_overflow, a_value, operand, expected_result);
    }
    
    #[test]
    fn sbc_zero_page_produces_correct_difference(
        zp_addr in 0u8..=0xFF,
        a_value in 0u8..=0xFF,
        operand in 0u8..=0xFF,
        carry_in in proptest::bool::ANY
    ) {
        let mut memory = Memory::new();
        memory.write(0x1000, 0xE5); // SBC zero page
        memory.write(0x1001, zp_addr);
        memory.write(zp_addr as u16, operand);
        
        let mut cpu = Cpu::new(memory, 0x1000);
        cpu.state.a = a_value;
        cpu.state.flag_carry = carry_in;
        
        let decoded = decode_opcode(0xE5).unwrap();
        cpu.execute_instruction(decoded).unwrap();
        
        // Calculate expected result
        let carry_val = if carry_in { 1u16 } else { 0u16 };
        let sum = (a_value as u16) + (!operand as u16) + carry_val;
        let expected_result = sum as u8;
        
        // Verify accumulator and flags
        prop_assert_eq!(cpu.state.a, expected_result);
        prop_assert_eq!(cpu.state.flag_zero, expected_result == 0);
        prop_assert_eq!(cpu.state.flag_negative, (expected_result & 0x80) != 0);
        prop_assert_eq!(cpu.state.flag_carry, sum > 0xFF);
    }
}

// ============================================================================
// INC/DEC Memory Property Tests
// ============================================================================

proptest! {
    #[test]
    fn inc_zero_page_increments_memory_correctly(
        zp_addr in 0u8..=0xFF,
        initial_value in 0u8..=0xFF
    ) {
        let mut memory = Memory::new();
        memory.write(0x1000, 0xE6); // INC zero page
        memory.write(0x1001, zp_addr);
        memory.write(zp_addr as u16, initial_value);
        
        let mut cpu = Cpu::new(memory, 0x1000);
        
        let decoded = decode_opcode(0xE6).unwrap();
        cpu.execute_instruction(decoded).unwrap();
        
        let expected_result = initial_value.wrapping_add(1);
        
        // Verify memory is incremented
        prop_assert_eq!(cpu.memory.read(zp_addr as u16), expected_result,
            "INC should increment 0x{:02X} to 0x{:02X}", initial_value, expected_result);
        
        // Verify Zero flag
        prop_assert_eq!(cpu.state.flag_zero, expected_result == 0,
            "Zero flag should be {} for result 0x{:02X}", expected_result == 0, expected_result);
        
        // Verify Negative flag
        prop_assert_eq!(cpu.state.flag_negative, (expected_result & 0x80) != 0,
            "Negative flag should be {} for result 0x{:02X}",
            (expected_result & 0x80) != 0, expected_result);
    }
    
    #[test]
    fn inc_absolute_increments_memory_correctly(
        addr in 0x0200u16..=0x0FFF, // Exclude instruction area at 0x1000-0x1002
        initial_value in 0u8..=0xFF
    ) {
        let mut memory = Memory::new();
        memory.write(addr, initial_value); // Write operand first to avoid overwriting instruction bytes
        memory.write(0x1000, 0xEE); // INC absolute
        memory.write(0x1001, (addr & 0xFF) as u8);
        memory.write(0x1002, (addr >> 8) as u8);
        
        let mut cpu = Cpu::new(memory, 0x1000);
        
        let decoded = decode_opcode(0xEE).unwrap();
        cpu.execute_instruction(decoded).unwrap();
        
        let expected_result = initial_value.wrapping_add(1);
        
        // Verify memory is incremented and flags are correct
        prop_assert_eq!(cpu.memory.read(addr), expected_result);
        prop_assert_eq!(cpu.state.flag_zero, expected_result == 0);
        prop_assert_eq!(cpu.state.flag_negative, (expected_result & 0x80) != 0);
    }
    
    #[test]
    fn dec_zero_page_decrements_memory_correctly(
        zp_addr in 0u8..=0xFF,
        initial_value in 0u8..=0xFF
    ) {
        let mut memory = Memory::new();
        memory.write(0x1000, 0xC6); // DEC zero page
        memory.write(0x1001, zp_addr);
        memory.write(zp_addr as u16, initial_value);
        
        let mut cpu = Cpu::new(memory, 0x1000);
        
        let decoded = decode_opcode(0xC6).unwrap();
        cpu.execute_instruction(decoded).unwrap();
        
        let expected_result = initial_value.wrapping_sub(1);
        
        // Verify memory is decremented
        prop_assert_eq!(cpu.memory.read(zp_addr as u16), expected_result,
            "DEC should decrement 0x{:02X} to 0x{:02X}", initial_value, expected_result);
        
        // Verify Zero flag
        prop_assert_eq!(cpu.state.flag_zero, expected_result == 0,
            "Zero flag should be {} for result 0x{:02X}", expected_result == 0, expected_result);
        
        // Verify Negative flag
        prop_assert_eq!(cpu.state.flag_negative, (expected_result & 0x80) != 0,
            "Negative flag should be {} for result 0x{:02X}",
            (expected_result & 0x80) != 0, expected_result);
    }
    
    #[test]
    fn dec_absolute_decrements_memory_correctly(
        addr in 0x0200u16..=0x0FFF, // Exclude instruction area at 0x1000-0x1002
        initial_value in 0u8..=0xFF
    ) {
        let mut memory = Memory::new();
        memory.write(addr, initial_value); // Write operand first to avoid overwriting instruction bytes
        memory.write(0x1000, 0xCE); // DEC absolute
        memory.write(0x1001, (addr & 0xFF) as u8);
        memory.write(0x1002, (addr >> 8) as u8);
        
        let mut cpu = Cpu::new(memory, 0x1000);
        
        let decoded = decode_opcode(0xCE).unwrap();
        cpu.execute_instruction(decoded).unwrap();
        
        let expected_result = initial_value.wrapping_sub(1);
        
        // Verify memory is decremented and flags are correct
        prop_assert_eq!(cpu.memory.read(addr), expected_result);
        prop_assert_eq!(cpu.state.flag_zero, expected_result == 0);
        prop_assert_eq!(cpu.state.flag_negative, (expected_result & 0x80) != 0);
    }
}

// ============================================================================
// INX/INY/DEX/DEY Register Property Tests
// ============================================================================

proptest! {
    #[test]
    fn inx_increments_x_register_correctly(
        initial_x in 0u8..=0xFF,
        a_value in 0u8..=0xFF,
        y_value in 0u8..=0xFF
    ) {
        let mut memory = Memory::new();
        memory.write(0x1000, 0xE8); // INX
        
        let mut cpu = Cpu::new(memory, 0x1000);
        cpu.state.x = initial_x;
        cpu.state.a = a_value;
        cpu.state.y = y_value;
        
        let decoded = decode_opcode(0xE8).unwrap();
        cpu.execute_instruction(decoded).unwrap();
        
        let expected_x = initial_x.wrapping_add(1);
        
        // Verify X register is incremented
        prop_assert_eq!(cpu.state.x, expected_x,
            "INX should increment X from 0x{:02X} to 0x{:02X}", initial_x, expected_x);
        
        // Verify other registers unchanged
        prop_assert_eq!(cpu.state.a, a_value, "A register should be unchanged");
        prop_assert_eq!(cpu.state.y, y_value, "Y register should be unchanged");
        
        // Verify Zero flag
        prop_assert_eq!(cpu.state.flag_zero, expected_x == 0,
            "Zero flag should be {} for result 0x{:02X}", expected_x == 0, expected_x);
        
        // Verify Negative flag
        prop_assert_eq!(cpu.state.flag_negative, (expected_x & 0x80) != 0,
            "Negative flag should be {} for result 0x{:02X}",
            (expected_x & 0x80) != 0, expected_x);
    }
    
    #[test]
    fn iny_increments_y_register_correctly(
        initial_y in 0u8..=0xFF,
        a_value in 0u8..=0xFF,
        x_value in 0u8..=0xFF
    ) {
        let mut memory = Memory::new();
        memory.write(0x1000, 0xC8); // INY
        
        let mut cpu = Cpu::new(memory, 0x1000);
        cpu.state.y = initial_y;
        cpu.state.a = a_value;
        cpu.state.x = x_value;
        
        let decoded = decode_opcode(0xC8).unwrap();
        cpu.execute_instruction(decoded).unwrap();
        
        let expected_y = initial_y.wrapping_add(1);
        
        // Verify Y register is incremented
        prop_assert_eq!(cpu.state.y, expected_y,
            "INY should increment Y from 0x{:02X} to 0x{:02X}", initial_y, expected_y);
        
        // Verify other registers unchanged
        prop_assert_eq!(cpu.state.a, a_value, "A register should be unchanged");
        prop_assert_eq!(cpu.state.x, x_value, "X register should be unchanged");
        
        // Verify flags
        prop_assert_eq!(cpu.state.flag_zero, expected_y == 0);
        prop_assert_eq!(cpu.state.flag_negative, (expected_y & 0x80) != 0);
    }
    
    #[test]
    fn dex_decrements_x_register_correctly(
        initial_x in 0u8..=0xFF,
        a_value in 0u8..=0xFF,
        y_value in 0u8..=0xFF
    ) {
        let mut memory = Memory::new();
        memory.write(0x1000, 0xCA); // DEX
        
        let mut cpu = Cpu::new(memory, 0x1000);
        cpu.state.x = initial_x;
        cpu.state.a = a_value;
        cpu.state.y = y_value;
        
        let decoded = decode_opcode(0xCA).unwrap();
        cpu.execute_instruction(decoded).unwrap();
        
        let expected_x = initial_x.wrapping_sub(1);
        
        // Verify X register is decremented
        prop_assert_eq!(cpu.state.x, expected_x,
            "DEX should decrement X from 0x{:02X} to 0x{:02X}", initial_x, expected_x);
        
        // Verify other registers unchanged
        prop_assert_eq!(cpu.state.a, a_value, "A register should be unchanged");
        prop_assert_eq!(cpu.state.y, y_value, "Y register should be unchanged");
        
        // Verify flags
        prop_assert_eq!(cpu.state.flag_zero, expected_x == 0);
        prop_assert_eq!(cpu.state.flag_negative, (expected_x & 0x80) != 0);
    }
    
    #[test]
    fn dey_decrements_y_register_correctly(
        initial_y in 0u8..=0xFF,
        a_value in 0u8..=0xFF,
        x_value in 0u8..=0xFF
    ) {
        let mut memory = Memory::new();
        memory.write(0x1000, 0x88); // DEY
        
        let mut cpu = Cpu::new(memory, 0x1000);
        cpu.state.y = initial_y;
        cpu.state.a = a_value;
        cpu.state.x = x_value;
        
        let decoded = decode_opcode(0x88).unwrap();
        cpu.execute_instruction(decoded).unwrap();
        
        let expected_y = initial_y.wrapping_sub(1);
        
        // Verify Y register is decremented
        prop_assert_eq!(cpu.state.y, expected_y,
            "DEY should decrement Y from 0x{:02X} to 0x{:02X}", initial_y, expected_y);
        
        // Verify other registers unchanged
        prop_assert_eq!(cpu.state.a, a_value, "A register should be unchanged");
        prop_assert_eq!(cpu.state.x, x_value, "X register should be unchanged");
        
        // Verify flags
        prop_assert_eq!(cpu.state.flag_zero, expected_y == 0);
        prop_assert_eq!(cpu.state.flag_negative, (expected_y & 0x80) != 0);
    }
}

// ============================================================================
// Property 10: Carry Flag for Addition
// **Validates: Requirements 9.3**
// ============================================================================
//
// This property test specifically verifies that the carry flag is set correctly
// during ADC operations. The carry flag should be set when the result exceeds 255
// (unsigned overflow).
//
// The test should:
// 1. Generate random values for accumulator, operand, and initial carry flag
// 2. Execute ADC instruction
// 3. Verify that the carry flag is set if and only if the sum exceeds 255
// 4. Test across multiple addressing modes

proptest! {
    #[test]
    fn adc_carry_flag_set_on_unsigned_overflow_immediate(
        a_value in 0u8..=0xFF,
        operand in 0u8..=0xFF,
        carry_in in proptest::bool::ANY
    ) {
        let mut memory = Memory::new();
        memory.write(0x1000, 0x69); // ADC immediate
        memory.write(0x1001, operand);
        
        let mut cpu = Cpu::new(memory, 0x1000);
        cpu.state.a = a_value;
        cpu.state.flag_carry = carry_in;
        
        let decoded = decode_opcode(0x69).unwrap();
        cpu.execute_instruction(decoded).unwrap();
        
        // Calculate the full sum including carry
        let carry_val = if carry_in { 1u16 } else { 0u16 };
        let sum = (a_value as u16) + (operand as u16) + carry_val;
        
        // The carry flag should be set if and only if the sum exceeds 255
        let expected_carry = sum > 0xFF;
        
        prop_assert_eq!(cpu.state.flag_carry, expected_carry,
            "Carry flag should be {} when 0x{:02X} + 0x{:02X} + {} = 0x{:03X} (sum {} 255)",
            expected_carry, a_value, operand, carry_val, sum,
            if expected_carry { ">" } else { "<=" });
    }
    
    #[test]
    fn adc_carry_flag_set_on_unsigned_overflow_zero_page(
        zp_addr in 0u8..=0xFF,
        a_value in 0u8..=0xFF,
        operand in 0u8..=0xFF,
        carry_in in proptest::bool::ANY
    ) {
        let mut memory = Memory::new();
        memory.write(0x1000, 0x65); // ADC zero page
        memory.write(0x1001, zp_addr);
        memory.write(zp_addr as u16, operand);
        
        let mut cpu = Cpu::new(memory, 0x1000);
        cpu.state.a = a_value;
        cpu.state.flag_carry = carry_in;
        
        let decoded = decode_opcode(0x65).unwrap();
        cpu.execute_instruction(decoded).unwrap();
        
        let carry_val = if carry_in { 1u16 } else { 0u16 };
        let sum = (a_value as u16) + (operand as u16) + carry_val;
        let expected_carry = sum > 0xFF;
        
        prop_assert_eq!(cpu.state.flag_carry, expected_carry,
            "ADC ZP: Carry flag should be {} when sum is 0x{:03X}",
            expected_carry, sum);
    }
    
    #[test]
    fn adc_carry_flag_set_on_unsigned_overflow_zero_page_x(
        zp_addr in 0u8..=0xF0, // Leave room for X offset
        x_value in 0u8..=0x0F,
        a_value in 0u8..=0xFF,
        operand in 0u8..=0xFF,
        carry_in in proptest::bool::ANY
    ) {
        let mut memory = Memory::new();
        memory.write(0x1000, 0x75); // ADC zero page,X
        memory.write(0x1001, zp_addr);
        let effective_addr = zp_addr.wrapping_add(x_value);
        memory.write(effective_addr as u16, operand);
        
        let mut cpu = Cpu::new(memory, 0x1000);
        cpu.state.a = a_value;
        cpu.state.x = x_value;
        cpu.state.flag_carry = carry_in;
        
        let decoded = decode_opcode(0x75).unwrap();
        cpu.execute_instruction(decoded).unwrap();
        
        let carry_val = if carry_in { 1u16 } else { 0u16 };
        let sum = (a_value as u16) + (operand as u16) + carry_val;
        let expected_carry = sum > 0xFF;
        
        prop_assert_eq!(cpu.state.flag_carry, expected_carry,
            "ADC ZP,X: Carry flag should be {} when sum is 0x{:03X}",
            expected_carry, sum);
    }
    
    #[test]
    fn adc_carry_flag_set_on_unsigned_overflow_absolute(
        addr in 0x0200u16..=0x0FFF, // Exclude instruction area at 0x1000-0x1002
        a_value in 0u8..=0xFF,
        operand in 0u8..=0xFF,
        carry_in in proptest::bool::ANY
    ) {
        let mut memory = Memory::new();
        memory.write(addr, operand); // Write operand first
        memory.write(0x1000, 0x6D); // ADC absolute
        memory.write(0x1001, (addr & 0xFF) as u8);
        memory.write(0x1002, (addr >> 8) as u8);
        
        let mut cpu = Cpu::new(memory, 0x1000);
        cpu.state.a = a_value;
        cpu.state.flag_carry = carry_in;
        
        let decoded = decode_opcode(0x6D).unwrap();
        cpu.execute_instruction(decoded).unwrap();
        
        let carry_val = if carry_in { 1u16 } else { 0u16 };
        let sum = (a_value as u16) + (operand as u16) + carry_val;
        let expected_carry = sum > 0xFF;
        
        prop_assert_eq!(cpu.state.flag_carry, expected_carry,
            "ADC ABS: Carry flag should be {} when sum is 0x{:03X}",
            expected_carry, sum);
    }
    
    #[test]
    fn adc_carry_flag_set_on_unsigned_overflow_absolute_x(
        base_addr in 0x0200u16..=0xFFF0,
        x_value in 0u8..=0x0F,
        a_value in 0u8..=0xFF,
        operand in 0u8..=0xFF,
        carry_in in proptest::bool::ANY
    ) {
        let mut memory = Memory::new();
        let effective_addr = base_addr + (x_value as u16);
        memory.write(effective_addr, operand); // Write operand first
        memory.write(0x1000, 0x7D); // ADC absolute,X
        memory.write(0x1001, (base_addr & 0xFF) as u8);
        memory.write(0x1002, (base_addr >> 8) as u8);
        
        let mut cpu = Cpu::new(memory, 0x1000);
        cpu.state.a = a_value;
        cpu.state.x = x_value;
        cpu.state.flag_carry = carry_in;
        
        let decoded = decode_opcode(0x7D).unwrap();
        cpu.execute_instruction(decoded).unwrap();
        
        let carry_val = if carry_in { 1u16 } else { 0u16 };
        let sum = (a_value as u16) + (operand as u16) + carry_val;
        let expected_carry = sum > 0xFF;
        
        prop_assert_eq!(cpu.state.flag_carry, expected_carry,
            "ADC ABS,X: Carry flag should be {} when sum is 0x{:03X}",
            expected_carry, sum);
    }
    
    #[test]
    fn adc_carry_flag_set_on_unsigned_overflow_absolute_y(
        base_addr in 0x0200u16..=0xFFF0,
        y_value in 0u8..=0x0F,
        a_value in 0u8..=0xFF,
        operand in 0u8..=0xFF,
        carry_in in proptest::bool::ANY
    ) {
        let mut memory = Memory::new();
        let effective_addr = base_addr + (y_value as u16);
        memory.write(effective_addr, operand); // Write operand first
        memory.write(0x1000, 0x79); // ADC absolute,Y
        memory.write(0x1001, (base_addr & 0xFF) as u8);
        memory.write(0x1002, (base_addr >> 8) as u8);
        
        let mut cpu = Cpu::new(memory, 0x1000);
        cpu.state.a = a_value;
        cpu.state.y = y_value;
        cpu.state.flag_carry = carry_in;
        
        let decoded = decode_opcode(0x79).unwrap();
        cpu.execute_instruction(decoded).unwrap();
        
        let carry_val = if carry_in { 1u16 } else { 0u16 };
        let sum = (a_value as u16) + (operand as u16) + carry_val;
        let expected_carry = sum > 0xFF;
        
        prop_assert_eq!(cpu.state.flag_carry, expected_carry,
            "ADC ABS,Y: Carry flag should be {} when sum is 0x{:03X}",
            expected_carry, sum);
    }
    
    #[test]
    fn adc_carry_flag_set_on_unsigned_overflow_indexed_indirect(
        zp_base in 0u8..=0xF0,
        x_value in 0u8..=0x0E, // Limit to 0x0E to avoid pointer wrapping to 0x00 when zp_base is 0xF0
        target_addr in 0x0200u16..=0x0FFF, // Exclude instruction area at 0x1000-0x1002
        a_value in 0u8..=0xFF,
        operand in 0u8..=0xFF,
        carry_in in proptest::bool::ANY
    ) {
        let mut memory = Memory::new();
        
        // Set up the indirect address first to avoid overwriting instruction bytes
        let pointer_addr = zp_base.wrapping_add(x_value);
        memory.write(pointer_addr as u16, (target_addr & 0xFF) as u8);
        memory.write(pointer_addr.wrapping_add(1) as u16, (target_addr >> 8) as u8);
        memory.write(target_addr, operand);
        
        // Now write the instruction
        memory.write(0x1000, 0x61); // ADC (indirect,X)
        memory.write(0x1001, zp_base);
        
        let mut cpu = Cpu::new(memory, 0x1000);
        cpu.state.a = a_value;
        cpu.state.x = x_value;
        cpu.state.flag_carry = carry_in;
        
        let decoded = decode_opcode(0x61).unwrap();
        cpu.execute_instruction(decoded).unwrap();
        
        let carry_val = if carry_in { 1u16 } else { 0u16 };
        let sum = (a_value as u16) + (operand as u16) + carry_val;
        let expected_carry = sum > 0xFF;
        
        prop_assert_eq!(cpu.state.flag_carry, expected_carry,
            "ADC (IND,X): Carry flag should be {} when sum is 0x{:03X}",
            expected_carry, sum);
    }
    
    #[test]
    fn adc_carry_flag_set_on_unsigned_overflow_indirect_indexed(
        zp_addr in 0u8..=0xFE,
        base_addr in 0x0200u16..=0xFFF0,
        y_value in 0u8..=0x0F,
        a_value in 0u8..=0xFF,
        operand in 0u8..=0xFF,
        carry_in in proptest::bool::ANY
    ) {
        let effective_addr = base_addr + (y_value as u16);
        // Avoid overlap with instruction at 0x1000-0x1002
        prop_assume!(effective_addr < 0x1000 || effective_addr > 0x1002);
        
        let mut memory = Memory::new();
        memory.write(0x1000, 0x71); // ADC (indirect),Y
        memory.write(0x1001, zp_addr);
        
        // Set up the indirect address
        memory.write(zp_addr as u16, (base_addr & 0xFF) as u8);
        memory.write((zp_addr + 1) as u16, (base_addr >> 8) as u8);
        memory.write(effective_addr, operand);
        
        let mut cpu = Cpu::new(memory, 0x1000);
        cpu.state.a = a_value;
        cpu.state.y = y_value;
        cpu.state.flag_carry = carry_in;
        
        let decoded = decode_opcode(0x71).unwrap();
        cpu.execute_instruction(decoded).unwrap();
        
        let carry_val = if carry_in { 1u16 } else { 0u16 };
        let sum = (a_value as u16) + (operand as u16) + carry_val;
        let expected_carry = sum > 0xFF;
        
        prop_assert_eq!(cpu.state.flag_carry, expected_carry,
            "ADC (IND),Y: Carry flag should be {} when sum is 0x{:03X}",
            expected_carry, sum);
    }
    
    #[test]
    fn adc_carry_flag_boundary_cases(
        a_value in 0u8..=0xFF,
        operand in 0u8..=0xFF
    ) {
        // Test with carry clear
        {
            let mut memory = Memory::new();
            memory.write(0x1000, 0x69); // ADC immediate
            memory.write(0x1001, operand);
            
            let mut cpu = Cpu::new(memory, 0x1000);
            cpu.state.a = a_value;
            cpu.state.flag_carry = false;
            
            let decoded = decode_opcode(0x69).unwrap();
            cpu.execute_instruction(decoded).unwrap();
            
            let sum = (a_value as u16) + (operand as u16);
            prop_assert_eq!(cpu.state.flag_carry, sum > 0xFF,
                "With carry clear: 0x{:02X} + 0x{:02X} = 0x{:03X}, carry should be {}",
                a_value, operand, sum, sum > 0xFF);
        }
        
        // Test with carry set
        {
            let mut memory = Memory::new();
            memory.write(0x2000, 0x69); // ADC immediate
            memory.write(0x2001, operand);
            
            let mut cpu = Cpu::new(memory, 0x2000);
            cpu.state.a = a_value;
            cpu.state.flag_carry = true;
            
            let decoded = decode_opcode(0x69).unwrap();
            cpu.execute_instruction(decoded).unwrap();
            
            let sum = (a_value as u16) + (operand as u16) + 1;
            prop_assert_eq!(cpu.state.flag_carry, sum > 0xFF,
                "With carry set: 0x{:02X} + 0x{:02X} + 1 = 0x{:03X}, carry should be {}",
                a_value, operand, sum, sum > 0xFF);
        }
    }
}

// ============================================================================
// Combined Property Tests: Verify other flags/registers unchanged
// ============================================================================

proptest! {
    #[test]
    fn arithmetic_instructions_preserve_unaffected_flags(
        a_value in 0u8..=0xFF,
        operand in 0u8..=0xFF,
        interrupt_disable in proptest::bool::ANY,
        decimal in proptest::bool::ANY,
        break_flag in proptest::bool::ANY
    ) {
        let mut memory = Memory::new();
        memory.write(0x1000, 0x69); // ADC immediate
        memory.write(0x1001, operand);
        
        let mut cpu = Cpu::new(memory, 0x1000);
        cpu.state.a = a_value;
        cpu.state.flag_interrupt_disable = interrupt_disable;
        cpu.state.flag_decimal = decimal;
        cpu.state.flag_break = break_flag;
        
        let decoded = decode_opcode(0x69).unwrap();
        cpu.execute_instruction(decoded).unwrap();
        
        // Verify unaffected flags remain unchanged
        prop_assert_eq!(cpu.state.flag_interrupt_disable, interrupt_disable,
            "Interrupt Disable flag should remain unchanged");
        prop_assert_eq!(cpu.state.flag_decimal, decimal,
            "Decimal flag should remain unchanged");
        prop_assert_eq!(cpu.state.flag_break, break_flag,
            "Break flag should remain unchanged");
    }
    
    #[test]
    fn inc_dec_do_not_affect_carry_or_overflow(
        zp_addr in 0u8..=0xFF,
        initial_value in 0u8..=0xFF,
        carry in proptest::bool::ANY,
        overflow in proptest::bool::ANY
    ) {
        let mut memory = Memory::new();
        memory.write(0x1000, 0xE6); // INC zero page
        memory.write(0x1001, zp_addr);
        memory.write(zp_addr as u16, initial_value);
        
        let mut cpu = Cpu::new(memory, 0x1000);
        cpu.state.flag_carry = carry;
        cpu.state.flag_overflow = overflow;
        
        let decoded = decode_opcode(0xE6).unwrap();
        cpu.execute_instruction(decoded).unwrap();
        
        // Verify Carry and Overflow flags remain unchanged
        prop_assert_eq!(cpu.state.flag_carry, carry,
            "INC should not affect Carry flag");
        prop_assert_eq!(cpu.state.flag_overflow, overflow,
            "INC should not affect Overflow flag");
    }
    
    #[test]
    fn register_inc_dec_do_not_affect_carry_or_overflow(
        initial_x in 0u8..=0xFF,
        carry in proptest::bool::ANY,
        overflow in proptest::bool::ANY
    ) {
        let mut memory = Memory::new();
        memory.write(0x1000, 0xE8); // INX
        
        let mut cpu = Cpu::new(memory, 0x1000);
        cpu.state.x = initial_x;
        cpu.state.flag_carry = carry;
        cpu.state.flag_overflow = overflow;
        
        let decoded = decode_opcode(0xE8).unwrap();
        cpu.execute_instruction(decoded).unwrap();
        
        // Verify Carry and Overflow flags remain unchanged
        prop_assert_eq!(cpu.state.flag_carry, carry,
            "INX should not affect Carry flag");
        prop_assert_eq!(cpu.state.flag_overflow, overflow,
            "INX should not affect Overflow flag");
    }
}

// ============================================================================
// Property 11: Overflow Flag for Signed Addition
// **Validates: Requirements 9.4**
// ============================================================================
//
// This property test specifically verifies that the overflow flag is set correctly
// during ADC and SBC operations. The overflow flag indicates signed overflow - when
// the result of a signed operation is outside the range -128 to +127.
//
// The overflow flag logic:
// - For ADC: V = (A^result) & (M^result) & 0x80
//   Overflow occurs when adding two numbers with the same sign produces a result
//   with a different sign (positive + positive = negative, or negative + negative = positive)
// - For SBC: V = (A^result) & ((~M)^result) & 0x80
//   Overflow occurs when subtracting numbers with different signs produces a result
//   with the wrong sign
//
// The test should:
// 1. Generate random values for accumulator, operand, and initial carry flag
// 2. Execute ADC or SBC instruction
// 3. Verify that the overflow flag is set correctly according to the 6502 specification
// 4. Test across multiple addressing modes

proptest! {
    #[test]
    fn adc_overflow_flag_detects_signed_overflow_immediate(
        a_value in 0u8..=0xFF,
        operand in 0u8..=0xFF,
        carry_in in proptest::bool::ANY
    ) {
        let mut memory = Memory::new();
        memory.write(0x1000, 0x69); // ADC immediate
        memory.write(0x1001, operand);
        
        let mut cpu = Cpu::new(memory, 0x1000);
        cpu.state.a = a_value;
        cpu.state.flag_carry = carry_in;
        
        let decoded = decode_opcode(0x69).unwrap();
        cpu.execute_instruction(decoded).unwrap();
        
        // Calculate the result
        let carry_val = if carry_in { 1u16 } else { 0u16 };
        let sum = (a_value as u16) + (operand as u16) + carry_val;
        let result = sum as u8;
        
        // Check if overflow occurred
        // Overflow happens when:
        // - Both operands have the same sign (bit 7)
        // - The result has a different sign than the operands
        let a_sign = a_value & 0x80;
        let operand_sign = operand & 0x80;
        let result_sign = result & 0x80;
        
        // Overflow occurs when adding two positive numbers gives negative,
        // or adding two negative numbers gives positive
        let expected_overflow = (a_sign == operand_sign) && (a_sign != result_sign);
        
        prop_assert_eq!(cpu.state.flag_overflow, expected_overflow,
            "ADC overflow flag incorrect: 0x{:02X} (sign={}) + 0x{:02X} (sign={}) + {} = 0x{:02X} (sign={}), expected overflow={}",
            a_value, if a_sign != 0 { "neg" } else { "pos" },
            operand, if operand_sign != 0 { "neg" } else { "pos" },
            carry_val,
            result, if result_sign != 0 { "neg" } else { "pos" },
            expected_overflow);
    }
    
    #[test]
    fn adc_overflow_flag_detects_signed_overflow_zero_page(
        zp_addr in 0u8..=0xFF,
        a_value in 0u8..=0xFF,
        operand in 0u8..=0xFF,
        carry_in in proptest::bool::ANY
    ) {
        let mut memory = Memory::new();
        memory.write(0x1000, 0x65); // ADC zero page
        memory.write(0x1001, zp_addr);
        memory.write(zp_addr as u16, operand);
        
        let mut cpu = Cpu::new(memory, 0x1000);
        cpu.state.a = a_value;
        cpu.state.flag_carry = carry_in;
        
        let decoded = decode_opcode(0x65).unwrap();
        cpu.execute_instruction(decoded).unwrap();
        
        let carry_val = if carry_in { 1u16 } else { 0u16 };
        let sum = (a_value as u16) + (operand as u16) + carry_val;
        let result = sum as u8;
        
        let a_sign = a_value & 0x80;
        let operand_sign = operand & 0x80;
        let result_sign = result & 0x80;
        let expected_overflow = (a_sign == operand_sign) && (a_sign != result_sign);
        
        prop_assert_eq!(cpu.state.flag_overflow, expected_overflow,
            "ADC ZP overflow flag incorrect for 0x{:02X} + 0x{:02X} + {} = 0x{:02X}",
            a_value, operand, carry_val, result);
    }
    
    #[test]
    fn adc_overflow_flag_detects_signed_overflow_absolute(
        addr in 0x0200u16..=0x0FFF, // Exclude instruction area at 0x1000-0x1002
        a_value in 0u8..=0xFF,
        operand in 0u8..=0xFF,
        carry_in in proptest::bool::ANY
    ) {
        let mut memory = Memory::new();
        memory.write(addr, operand); // Write operand first
        memory.write(0x1000, 0x6D); // ADC absolute
        memory.write(0x1001, (addr & 0xFF) as u8);
        memory.write(0x1002, (addr >> 8) as u8);
        
        let mut cpu = Cpu::new(memory, 0x1000);
        cpu.state.a = a_value;
        cpu.state.flag_carry = carry_in;
        
        let decoded = decode_opcode(0x6D).unwrap();
        cpu.execute_instruction(decoded).unwrap();
        
        let carry_val = if carry_in { 1u16 } else { 0u16 };
        let sum = (a_value as u16) + (operand as u16) + carry_val;
        let result = sum as u8;
        
        let a_sign = a_value & 0x80;
        let operand_sign = operand & 0x80;
        let result_sign = result & 0x80;
        let expected_overflow = (a_sign == operand_sign) && (a_sign != result_sign);
        
        prop_assert_eq!(cpu.state.flag_overflow, expected_overflow,
            "ADC ABS overflow flag incorrect for 0x{:02X} + 0x{:02X} + {} = 0x{:02X}",
            a_value, operand, carry_val, result);
    }
    
    #[test]
    fn sbc_overflow_flag_detects_signed_overflow_immediate(
        a_value in 0u8..=0xFF,
        operand in 0u8..=0xFF,
        carry_in in proptest::bool::ANY
    ) {
        let mut memory = Memory::new();
        memory.write(0x1000, 0xE9); // SBC immediate
        memory.write(0x1001, operand);
        
        let mut cpu = Cpu::new(memory, 0x1000);
        cpu.state.a = a_value;
        cpu.state.flag_carry = carry_in;
        
        let decoded = decode_opcode(0xE9).unwrap();
        cpu.execute_instruction(decoded).unwrap();
        
        // SBC is implemented as A + ~M + C
        let carry_val = if carry_in { 1u16 } else { 0u16 };
        let sum = (a_value as u16) + (!operand as u16) + carry_val;
        let result = sum as u8;
        
        // For SBC, overflow occurs when:
        // - The operands have different signs (A and M have different signs)
        // - The result has a different sign than A
        let a_sign = a_value & 0x80;
        let operand_sign = operand & 0x80;
        let result_sign = result & 0x80;
        
        // Overflow in subtraction: subtracting a negative from positive gives negative,
        // or subtracting a positive from negative gives positive
        let expected_overflow = (a_sign != operand_sign) && (a_sign != result_sign);
        
        prop_assert_eq!(cpu.state.flag_overflow, expected_overflow,
            "SBC overflow flag incorrect: 0x{:02X} (sign={}) - 0x{:02X} (sign={}) - {} = 0x{:02X} (sign={}), expected overflow={}",
            a_value, if a_sign != 0 { "neg" } else { "pos" },
            operand, if operand_sign != 0 { "neg" } else { "pos" },
            if carry_in { 0 } else { 1 },
            result, if result_sign != 0 { "neg" } else { "pos" },
            expected_overflow);
    }
    
    #[test]
    fn sbc_overflow_flag_detects_signed_overflow_zero_page(
        zp_addr in 0u8..=0xFF,
        a_value in 0u8..=0xFF,
        operand in 0u8..=0xFF,
        carry_in in proptest::bool::ANY
    ) {
        let mut memory = Memory::new();
        memory.write(0x1000, 0xE5); // SBC zero page
        memory.write(0x1001, zp_addr);
        memory.write(zp_addr as u16, operand);
        
        let mut cpu = Cpu::new(memory, 0x1000);
        cpu.state.a = a_value;
        cpu.state.flag_carry = carry_in;
        
        let decoded = decode_opcode(0xE5).unwrap();
        cpu.execute_instruction(decoded).unwrap();
        
        let carry_val = if carry_in { 1u16 } else { 0u16 };
        let sum = (a_value as u16) + (!operand as u16) + carry_val;
        let result = sum as u8;
        
        let a_sign = a_value & 0x80;
        let operand_sign = operand & 0x80;
        let result_sign = result & 0x80;
        let expected_overflow = (a_sign != operand_sign) && (a_sign != result_sign);
        
        prop_assert_eq!(cpu.state.flag_overflow, expected_overflow,
            "SBC ZP overflow flag incorrect for 0x{:02X} - 0x{:02X} - {} = 0x{:02X}",
            a_value, operand, if carry_in { 0 } else { 1 }, result);
    }
    
    #[test]
    fn sbc_overflow_flag_detects_signed_overflow_absolute(
        addr in 0x0200u16..=0x0FFF, // Exclude instruction area at 0x1000-0x1002
        a_value in 0u8..=0xFF,
        operand in 0u8..=0xFF,
        carry_in in proptest::bool::ANY
    ) {
        let mut memory = Memory::new();
        memory.write(addr, operand); // Write operand first
        memory.write(0x1000, 0xED); // SBC absolute
        memory.write(0x1001, (addr & 0xFF) as u8);
        memory.write(0x1002, (addr >> 8) as u8);
        
        let mut cpu = Cpu::new(memory, 0x1000);
        cpu.state.a = a_value;
        cpu.state.flag_carry = carry_in;
        
        let decoded = decode_opcode(0xED).unwrap();
        cpu.execute_instruction(decoded).unwrap();
        
        let carry_val = if carry_in { 1u16 } else { 0u16 };
        let sum = (a_value as u16) + (!operand as u16) + carry_val;
        let result = sum as u8;
        
        let a_sign = a_value & 0x80;
        let operand_sign = operand & 0x80;
        let result_sign = result & 0x80;
        let expected_overflow = (a_sign != operand_sign) && (a_sign != result_sign);
        
        prop_assert_eq!(cpu.state.flag_overflow, expected_overflow,
            "SBC ABS overflow flag incorrect for 0x{:02X} - 0x{:02X} - {} = 0x{:02X}",
            a_value, operand, if carry_in { 0 } else { 1 }, result);
    }
    
    #[test]
    fn overflow_flag_specific_edge_cases(
        _carry_in in proptest::bool::ANY
    ) {
        // Test specific known overflow cases
        
        // Case 1: Positive + Positive = Negative (overflow)
        // 127 + 1 = 128 (0x80, which is -128 in signed)
        {
            let mut memory = Memory::new();
            memory.write(0x1000, 0x69); // ADC immediate
            memory.write(0x1001, 0x01);
            
            let mut cpu = Cpu::new(memory, 0x1000);
            cpu.state.a = 0x7F; // 127
            cpu.state.flag_carry = false;
            
            let decoded = decode_opcode(0x69).unwrap();
            cpu.execute_instruction(decoded).unwrap();
            
            prop_assert!(cpu.state.flag_overflow,
                "Overflow should be set when 127 + 1 = -128");
            prop_assert_eq!(cpu.state.a, 0x80);
        }
        
        // Case 2: Negative + Negative = Positive (overflow)
        // -128 + -1 = -129 (wraps to 127)
        {
            let mut memory = Memory::new();
            memory.write(0x2000, 0x69); // ADC immediate
            memory.write(0x2001, 0xFF); // -1
            
            let mut cpu = Cpu::new(memory, 0x2000);
            cpu.state.a = 0x80; // -128
            cpu.state.flag_carry = false;
            
            let decoded = decode_opcode(0x69).unwrap();
            cpu.execute_instruction(decoded).unwrap();
            
            prop_assert!(cpu.state.flag_overflow,
                "Overflow should be set when -128 + -1 wraps to 127");
            prop_assert_eq!(cpu.state.a, 0x7F);
        }
        
        // Case 3: Positive + Negative = result (no overflow)
        // 100 + -50 = 50
        {
            let mut memory = Memory::new();
            memory.write(0x3000, 0x69); // ADC immediate
            memory.write(0x3001, 0xCE); // -50 (206 unsigned)
            
            let mut cpu = Cpu::new(memory, 0x3000);
            cpu.state.a = 0x64; // 100
            cpu.state.flag_carry = false;
            
            let decoded = decode_opcode(0x69).unwrap();
            cpu.execute_instruction(decoded).unwrap();
            
            prop_assert!(!cpu.state.flag_overflow,
                "Overflow should NOT be set when adding positive and negative");
            prop_assert_eq!(cpu.state.a, 0x32); // 50
        }
        
        // Case 4: SBC overflow - Positive - Negative = Negative (overflow)
        // 127 - (-1) = 128 (overflow to -128)
        {
            let mut memory = Memory::new();
            memory.write(0x4000, 0xE9); // SBC immediate
            memory.write(0x4001, 0xFF); // -1
            
            let mut cpu = Cpu::new(memory, 0x4000);
            cpu.state.a = 0x7F; // 127
            cpu.state.flag_carry = true; // No borrow
            
            let decoded = decode_opcode(0xE9).unwrap();
            cpu.execute_instruction(decoded).unwrap();
            
            prop_assert!(cpu.state.flag_overflow,
                "Overflow should be set when 127 - (-1) = 128 (overflow)");
            prop_assert_eq!(cpu.state.a, 0x80); // -128
        }
        
        // Case 5: SBC no overflow - Negative - Positive = Negative
        // -100 - 20 = -120 (no overflow)
        {
            let mut memory = Memory::new();
            memory.write(0x5000, 0xE9); // SBC immediate
            memory.write(0x5001, 0x14); // 20
            
            let mut cpu = Cpu::new(memory, 0x5000);
            cpu.state.a = 0x9C; // -100 (156 unsigned)
            cpu.state.flag_carry = true; // No borrow
            
            let decoded = decode_opcode(0xE9).unwrap();
            cpu.execute_instruction(decoded).unwrap();
            
            prop_assert!(!cpu.state.flag_overflow,
                "Overflow should NOT be set when -100 - 20 = -120");
            prop_assert_eq!(cpu.state.a, 0x88); // -120 (136 unsigned)
        }
    }
}
