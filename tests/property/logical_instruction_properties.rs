// Property-based tests for logical instructions (AND, ORA, EOR)
// Feature: 6502-cpu-emulator, Property 16: Logical Instruction Correctness

use cpu_6502_emulator::cpu::Cpu;
use cpu_6502_emulator::memory::Memory;
use cpu_6502_emulator::instruction::decode_opcode;
use proptest::prelude::*;

// Feature: 6502-cpu-emulator, Property 16: Logical Instruction Correctness
// **Validates: Requirements 8.3**
//
// For any two values, logical instructions (AND, ORA, EOR) should:
// - Produce results matching bitwise operations
// - Update Zero and Negative flags based on the result
// - Leave Carry and Overflow flags unchanged

proptest! {
    #[test]
    fn and_immediate_performs_bitwise_and(
        a_value in 0u8..=0xFF,
        operand in 0u8..=0xFF
    ) {
        let mut memory = Memory::new();
        memory.write(0x1000, 0x29); // AND immediate
        memory.write(0x1001, operand);
        
        let mut cpu = Cpu::new(memory, 0x1000);
        cpu.state.a = a_value;
        
        // Set other flags to verify they're unchanged
        cpu.state.flag_carry = true;
        cpu.state.flag_overflow = true;
        cpu.state.flag_interrupt_disable = true;
        
        let decoded = decode_opcode(0x29).unwrap();
        cpu.execute_instruction(decoded).unwrap();
        
        let expected_result = a_value & operand;
        
        // Check result is correct
        prop_assert_eq!(cpu.state.a, expected_result);
        
        // Check Zero flag
        prop_assert_eq!(cpu.state.flag_zero, expected_result == 0);
        
        // Check Negative flag
        prop_assert_eq!(cpu.state.flag_negative, (expected_result & 0x80) != 0);
        
        // Check Carry and Overflow flags are unchanged
        prop_assert!(cpu.state.flag_carry);
        prop_assert!(cpu.state.flag_overflow);
        prop_assert!(cpu.state.flag_interrupt_disable);
    }
    
    #[test]
    fn ora_immediate_performs_bitwise_or(
        a_value in 0u8..=0xFF,
        operand in 0u8..=0xFF
    ) {
        let mut memory = Memory::new();
        memory.write(0x1000, 0x09); // ORA immediate
        memory.write(0x1001, operand);
        
        let mut cpu = Cpu::new(memory, 0x1000);
        cpu.state.a = a_value;
        
        // Set other flags to verify they're unchanged
        cpu.state.flag_carry = true;
        cpu.state.flag_overflow = true;
        cpu.state.flag_interrupt_disable = true;
        
        let decoded = decode_opcode(0x09).unwrap();
        cpu.execute_instruction(decoded).unwrap();
        
        let expected_result = a_value | operand;
        
        // Check result is correct
        prop_assert_eq!(cpu.state.a, expected_result);
        
        // Check Zero flag
        prop_assert_eq!(cpu.state.flag_zero, expected_result == 0);
        
        // Check Negative flag
        prop_assert_eq!(cpu.state.flag_negative, (expected_result & 0x80) != 0);
        
        // Check Carry and Overflow flags are unchanged
        prop_assert!(cpu.state.flag_carry);
        prop_assert!(cpu.state.flag_overflow);
        prop_assert!(cpu.state.flag_interrupt_disable);
    }
    
    #[test]
    fn eor_immediate_performs_bitwise_xor(
        a_value in 0u8..=0xFF,
        operand in 0u8..=0xFF
    ) {
        let mut memory = Memory::new();
        memory.write(0x1000, 0x49); // EOR immediate
        memory.write(0x1001, operand);
        
        let mut cpu = Cpu::new(memory, 0x1000);
        cpu.state.a = a_value;
        
        // Set other flags to verify they're unchanged
        cpu.state.flag_carry = true;
        cpu.state.flag_overflow = true;
        cpu.state.flag_interrupt_disable = true;
        
        let decoded = decode_opcode(0x49).unwrap();
        cpu.execute_instruction(decoded).unwrap();
        
        let expected_result = a_value ^ operand;
        
        // Check result is correct
        prop_assert_eq!(cpu.state.a, expected_result);
        
        // Check Zero flag
        prop_assert_eq!(cpu.state.flag_zero, expected_result == 0);
        
        // Check Negative flag
        prop_assert_eq!(cpu.state.flag_negative, (expected_result & 0x80) != 0);
        
        // Check Carry and Overflow flags are unchanged
        prop_assert!(cpu.state.flag_carry);
        prop_assert!(cpu.state.flag_overflow);
        prop_assert!(cpu.state.flag_interrupt_disable);
    }
    
    #[test]
    fn and_zero_page_performs_bitwise_and(
        a_value in 0u8..=0xFF,
        zp_addr in 0u8..=0xFF,
        operand in 0u8..=0xFF
    ) {
        let mut memory = Memory::new();
        memory.write(0x1000, 0x25); // AND zero page
        memory.write(0x1001, zp_addr);
        memory.write(zp_addr as u16, operand);
        
        let mut cpu = Cpu::new(memory, 0x1000);
        cpu.state.a = a_value;
        cpu.state.flag_carry = true;
        cpu.state.flag_overflow = true;
        
        let decoded = decode_opcode(0x25).unwrap();
        cpu.execute_instruction(decoded).unwrap();
        
        let expected_result = a_value & operand;
        prop_assert_eq!(cpu.state.a, expected_result);
        prop_assert_eq!(cpu.state.flag_zero, expected_result == 0);
        prop_assert_eq!(cpu.state.flag_negative, (expected_result & 0x80) != 0);
        prop_assert!(cpu.state.flag_carry);
        prop_assert!(cpu.state.flag_overflow);
    }
    
    #[test]
    fn ora_zero_page_performs_bitwise_or(
        a_value in 0u8..=0xFF,
        zp_addr in 0u8..=0xFF,
        operand in 0u8..=0xFF
    ) {
        let mut memory = Memory::new();
        memory.write(0x1000, 0x05); // ORA zero page
        memory.write(0x1001, zp_addr);
        memory.write(zp_addr as u16, operand);
        
        let mut cpu = Cpu::new(memory, 0x1000);
        cpu.state.a = a_value;
        cpu.state.flag_carry = true;
        cpu.state.flag_overflow = true;
        
        let decoded = decode_opcode(0x05).unwrap();
        cpu.execute_instruction(decoded).unwrap();
        
        let expected_result = a_value | operand;
        prop_assert_eq!(cpu.state.a, expected_result);
        prop_assert_eq!(cpu.state.flag_zero, expected_result == 0);
        prop_assert_eq!(cpu.state.flag_negative, (expected_result & 0x80) != 0);
        prop_assert!(cpu.state.flag_carry);
        prop_assert!(cpu.state.flag_overflow);
    }
    
    #[test]
    fn eor_zero_page_performs_bitwise_xor(
        a_value in 0u8..=0xFF,
        zp_addr in 0u8..=0xFF,
        operand in 0u8..=0xFF
    ) {
        let mut memory = Memory::new();
        memory.write(0x1000, 0x45); // EOR zero page
        memory.write(0x1001, zp_addr);
        memory.write(zp_addr as u16, operand);
        
        let mut cpu = Cpu::new(memory, 0x1000);
        cpu.state.a = a_value;
        cpu.state.flag_carry = true;
        cpu.state.flag_overflow = true;
        
        let decoded = decode_opcode(0x45).unwrap();
        cpu.execute_instruction(decoded).unwrap();
        
        let expected_result = a_value ^ operand;
        prop_assert_eq!(cpu.state.a, expected_result);
        prop_assert_eq!(cpu.state.flag_zero, expected_result == 0);
        prop_assert_eq!(cpu.state.flag_negative, (expected_result & 0x80) != 0);
        prop_assert!(cpu.state.flag_carry);
        prop_assert!(cpu.state.flag_overflow);
    }
    
    #[test]
    fn and_absolute_performs_bitwise_and(
        a_value in 0u8..=0xFF,
        addr in 0x0200u16..=0xFFFF,
        operand in 0u8..=0xFF
    ) {
        // Avoid overlap with instruction at 0x1000-0x1002
        prop_assume!(addr < 0x1000 || addr > 0x1002);
        
        let mut memory = Memory::new();
        memory.write(0x1000, 0x2D); // AND absolute
        memory.write(0x1001, (addr & 0xFF) as u8);
        memory.write(0x1002, (addr >> 8) as u8);
        memory.write(addr, operand);
        
        let mut cpu = Cpu::new(memory, 0x1000);
        cpu.state.a = a_value;
        cpu.state.flag_carry = true;
        cpu.state.flag_overflow = true;
        
        let decoded = decode_opcode(0x2D).unwrap();
        cpu.execute_instruction(decoded).unwrap();
        
        let expected_result = a_value & operand;
        prop_assert_eq!(cpu.state.a, expected_result);
        prop_assert_eq!(cpu.state.flag_zero, expected_result == 0);
        prop_assert_eq!(cpu.state.flag_negative, (expected_result & 0x80) != 0);
        prop_assert!(cpu.state.flag_carry);
        prop_assert!(cpu.state.flag_overflow);
    }
    
    #[test]
    fn ora_absolute_x_performs_bitwise_or(
        a_value in 0u8..=0xFF,
        base_addr in 0x0200u16..=0xFE00,
        x_offset in 0u8..=0xFF,
        operand in 0u8..=0xFF
    ) {
        let effective_addr = base_addr.wrapping_add(x_offset as u16);
        // Avoid overlap with instruction at 0x1000-0x1002
        prop_assume!(effective_addr < 0x1000 || effective_addr > 0x1002);
        
        let mut memory = Memory::new();
        memory.write(0x1000, 0x1D); // ORA absolute,X
        memory.write(0x1001, (base_addr & 0xFF) as u8);
        memory.write(0x1002, (base_addr >> 8) as u8);
        
        memory.write(effective_addr, operand);
        
        let mut cpu = Cpu::new(memory, 0x1000);
        cpu.state.a = a_value;
        cpu.state.x = x_offset;
        cpu.state.flag_carry = true;
        cpu.state.flag_overflow = true;
        
        let decoded = decode_opcode(0x1D).unwrap();
        cpu.execute_instruction(decoded).unwrap();
        
        let expected_result = a_value | operand;
        prop_assert_eq!(cpu.state.a, expected_result);
        prop_assert_eq!(cpu.state.flag_zero, expected_result == 0);
        prop_assert_eq!(cpu.state.flag_negative, (expected_result & 0x80) != 0);
        prop_assert!(cpu.state.flag_carry);
        prop_assert!(cpu.state.flag_overflow);
    }
    
    #[test]
    fn eor_absolute_y_performs_bitwise_xor(
        a_value in 0u8..=0xFF,
        base_addr in 0x0200u16..=0xFE00,
        y_offset in 0u8..=0xFF,
        operand in 0u8..=0xFF
    ) {
        let effective_addr = base_addr.wrapping_add(y_offset as u16);
        // Avoid overlap with instruction at 0x1000-0x1002
        prop_assume!(effective_addr < 0x1000 || effective_addr > 0x1002);
        
        let mut memory = Memory::new();
        memory.write(0x1000, 0x59); // EOR absolute,Y
        memory.write(0x1001, (base_addr & 0xFF) as u8);
        memory.write(0x1002, (base_addr >> 8) as u8);
        
        memory.write(effective_addr, operand);
        
        let mut cpu = Cpu::new(memory, 0x1000);
        cpu.state.a = a_value;
        cpu.state.y = y_offset;
        cpu.state.flag_carry = true;
        cpu.state.flag_overflow = true;
        
        let decoded = decode_opcode(0x59).unwrap();
        cpu.execute_instruction(decoded).unwrap();
        
        let expected_result = a_value ^ operand;
        prop_assert_eq!(cpu.state.a, expected_result);
        prop_assert_eq!(cpu.state.flag_zero, expected_result == 0);
        prop_assert_eq!(cpu.state.flag_negative, (expected_result & 0x80) != 0);
        prop_assert!(cpu.state.flag_carry);
        prop_assert!(cpu.state.flag_overflow);
    }
    
    #[test]
    fn and_indexed_indirect_performs_bitwise_and(
        a_value in 0u8..=0xFF,
        zp_base in 0u8..=0xFE,
        x_offset in 0u8..=0xFF,
        target_addr in 0x0200u16..=0xFFFF,
        operand in 0u8..=0xFF
    ) {
        let mut memory = Memory::new();
        memory.write(0x1000, 0x21); // AND (indirect,X)
        memory.write(0x1001, zp_base);
        
        let ptr_addr = zp_base.wrapping_add(x_offset) as u16;
        memory.write(ptr_addr, (target_addr & 0xFF) as u8);
        memory.write(ptr_addr.wrapping_add(1), (target_addr >> 8) as u8);
        memory.write(target_addr, operand);
        
        let mut cpu = Cpu::new(memory, 0x1000);
        cpu.state.a = a_value;
        cpu.state.x = x_offset;
        cpu.state.flag_carry = true;
        cpu.state.flag_overflow = true;
        
        let decoded = decode_opcode(0x21).unwrap();
        cpu.execute_instruction(decoded).unwrap();
        
        let expected_result = a_value & operand;
        prop_assert_eq!(cpu.state.a, expected_result);
        prop_assert_eq!(cpu.state.flag_zero, expected_result == 0);
        prop_assert_eq!(cpu.state.flag_negative, (expected_result & 0x80) != 0);
        prop_assert!(cpu.state.flag_carry);
        prop_assert!(cpu.state.flag_overflow);
    }
    
    #[test]
    fn ora_indirect_indexed_performs_bitwise_or(
        a_value in 0u8..=0xFF,
        zp_ptr in 0u8..=0xFE,
        base_addr in 0x0200u16..=0xFE00,
        y_offset in 0u8..=0xFF,
        operand in 0u8..=0xFF
    ) {
        let mut memory = Memory::new();
        memory.write(0x1000, 0x11); // ORA (indirect),Y
        memory.write(0x1001, zp_ptr);
        
        memory.write(zp_ptr as u16, (base_addr & 0xFF) as u8);
        memory.write((zp_ptr as u16).wrapping_add(1), (base_addr >> 8) as u8);
        
        let effective_addr = base_addr.wrapping_add(y_offset as u16);
        memory.write(effective_addr, operand);
        
        let mut cpu = Cpu::new(memory, 0x1000);
        cpu.state.a = a_value;
        cpu.state.y = y_offset;
        cpu.state.flag_carry = true;
        cpu.state.flag_overflow = true;
        
        let decoded = decode_opcode(0x11).unwrap();
        cpu.execute_instruction(decoded).unwrap();
        
        let expected_result = a_value | operand;
        prop_assert_eq!(cpu.state.a, expected_result);
        prop_assert_eq!(cpu.state.flag_zero, expected_result == 0);
        prop_assert_eq!(cpu.state.flag_negative, (expected_result & 0x80) != 0);
        prop_assert!(cpu.state.flag_carry);
        prop_assert!(cpu.state.flag_overflow);
    }
    
    #[test]
    fn eor_indirect_indexed_performs_bitwise_xor(
        a_value in 0u8..=0xFF,
        zp_ptr in 0u8..=0xFE,
        base_addr in 0x0200u16..=0xFE00,
        y_offset in 0u8..=0xFF,
        operand in 0u8..=0xFF
    ) {
        let mut memory = Memory::new();
        memory.write(0x1000, 0x51); // EOR (indirect),Y
        memory.write(0x1001, zp_ptr);
        
        memory.write(zp_ptr as u16, (base_addr & 0xFF) as u8);
        memory.write((zp_ptr as u16).wrapping_add(1), (base_addr >> 8) as u8);
        
        let effective_addr = base_addr.wrapping_add(y_offset as u16);
        memory.write(effective_addr, operand);
        
        let mut cpu = Cpu::new(memory, 0x1000);
        cpu.state.a = a_value;
        cpu.state.y = y_offset;
        cpu.state.flag_carry = true;
        cpu.state.flag_overflow = true;
        
        let decoded = decode_opcode(0x51).unwrap();
        cpu.execute_instruction(decoded).unwrap();
        
        let expected_result = a_value ^ operand;
        prop_assert_eq!(cpu.state.a, expected_result);
        prop_assert_eq!(cpu.state.flag_zero, expected_result == 0);
        prop_assert_eq!(cpu.state.flag_negative, (expected_result & 0x80) != 0);
        prop_assert!(cpu.state.flag_carry);
        prop_assert!(cpu.state.flag_overflow);
    }
}
