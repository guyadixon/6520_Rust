// Property-based tests for load instructions (LDA, LDX, LDY)
// Feature: 6502-cpu-emulator, Property 13: Load Instruction Correctness

use cpu_6502_emulator::cpu::Cpu;
use cpu_6502_emulator::memory::Memory;
use cpu_6502_emulator::instruction::decode_opcode;
use proptest::prelude::*;

// Feature: 6502-cpu-emulator, Property 13: Load Instruction Correctness
// **Validates: Requirements 8.1**
//
// For any memory address and any byte value at that address, executing a load instruction
// (LDA, LDX, LDY) should:
// - Set the target register to the value at the effective address
// - Update Zero and Negative flags based on the loaded value
// - Leave all other registers and flags unchanged

proptest! {
    #[test]
    fn lda_immediate_loads_value_and_updates_flags(value in 0u8..=0xFF) {
        let mut memory = Memory::new();
        memory.write(0x0000, 0xA9); // LDA immediate
        memory.write(0x0001, value);
        
        let mut cpu = Cpu::new(memory, 0x0000);
        
        // Set initial state for other registers/flags
        cpu.state.x = 0x11;
        cpu.state.y = 0x22;
        cpu.state.flag_carry = true;
        cpu.state.flag_overflow = true;
        
        let decoded = decode_opcode(0xA9).unwrap();
        cpu.execute_instruction(decoded).unwrap();
        
        // Check register is updated
        prop_assert_eq!(cpu.state.a, value);
        
        // Check flags are updated correctly
        prop_assert_eq!(cpu.state.flag_zero, value == 0);
        prop_assert_eq!(cpu.state.flag_negative, (value & 0x80) != 0);
        
        // Check other registers unchanged
        prop_assert_eq!(cpu.state.x, 0x11);
        prop_assert_eq!(cpu.state.y, 0x22);
        
        // Check other flags unchanged
        prop_assert!(cpu.state.flag_carry);
        prop_assert!(cpu.state.flag_overflow);
    }
    
    #[test]
    fn ldx_immediate_loads_value_and_updates_flags(value in 0u8..=0xFF) {
        let mut memory = Memory::new();
        memory.write(0x0000, 0xA2); // LDX immediate
        memory.write(0x0001, value);
        
        let mut cpu = Cpu::new(memory, 0x0000);
        
        // Set initial state for other registers/flags
        cpu.state.a = 0x33;
        cpu.state.y = 0x44;
        cpu.state.flag_carry = true;
        cpu.state.flag_overflow = true;
        
        let decoded = decode_opcode(0xA2).unwrap();
        cpu.execute_instruction(decoded).unwrap();
        
        // Check register is updated
        prop_assert_eq!(cpu.state.x, value);
        
        // Check flags are updated correctly
        prop_assert_eq!(cpu.state.flag_zero, value == 0);
        prop_assert_eq!(cpu.state.flag_negative, (value & 0x80) != 0);
        
        // Check other registers unchanged
        prop_assert_eq!(cpu.state.a, 0x33);
        prop_assert_eq!(cpu.state.y, 0x44);
        
        // Check other flags unchanged
        prop_assert!(cpu.state.flag_carry);
        prop_assert!(cpu.state.flag_overflow);
    }
    
    #[test]
    fn ldy_immediate_loads_value_and_updates_flags(value in 0u8..=0xFF) {
        let mut memory = Memory::new();
        memory.write(0x0000, 0xA0); // LDY immediate
        memory.write(0x0001, value);
        
        let mut cpu = Cpu::new(memory, 0x0000);
        
        // Set initial state for other registers/flags
        cpu.state.a = 0x55;
        cpu.state.x = 0x66;
        cpu.state.flag_carry = true;
        cpu.state.flag_overflow = true;
        
        let decoded = decode_opcode(0xA0).unwrap();
        cpu.execute_instruction(decoded).unwrap();
        
        // Check register is updated
        prop_assert_eq!(cpu.state.y, value);
        
        // Check flags are updated correctly
        prop_assert_eq!(cpu.state.flag_zero, value == 0);
        prop_assert_eq!(cpu.state.flag_negative, (value & 0x80) != 0);
        
        // Check other registers unchanged
        prop_assert_eq!(cpu.state.a, 0x55);
        prop_assert_eq!(cpu.state.x, 0x66);
        
        // Check other flags unchanged
        prop_assert!(cpu.state.flag_carry);
        prop_assert!(cpu.state.flag_overflow);
    }
    
    #[test]
    fn lda_zero_page_loads_from_memory(
        zp_addr in 0u8..=0xFF,
        value in 0u8..=0xFF
    ) {
        let mut memory = Memory::new();
        // Use PC at 0x1000 to avoid conflicts with zero page
        memory.write(0x1000, 0xA5); // LDA zero page
        memory.write(0x1001, zp_addr);
        memory.write(zp_addr as u16, value);
        
        let mut cpu = Cpu::new(memory, 0x1000);
        let decoded = decode_opcode(0xA5).unwrap();
        cpu.execute_instruction(decoded).unwrap();
        
        prop_assert_eq!(cpu.state.a, value);
        prop_assert_eq!(cpu.state.flag_zero, value == 0);
        prop_assert_eq!(cpu.state.flag_negative, (value & 0x80) != 0);
    }
    
    #[test]
    fn lda_absolute_loads_from_memory(
        addr in 0u16..=0xFFFF,
        value in 0u8..=0xFF
    ) {
        // Skip addresses that overlap with instruction bytes at 0x1000-0x1002
        prop_assume!(addr < 0x1000 || addr > 0x1002);
        
        let mut memory = Memory::new();
        // Write the value first to avoid overwriting instruction bytes
        memory.write(addr, value);
        // Use PC at 0x1000 to avoid conflicts with target address
        memory.write(0x1000, 0xAD); // LDA absolute
        memory.write(0x1001, (addr & 0xFF) as u8); // low byte
        memory.write(0x1002, (addr >> 8) as u8);   // high byte
        
        let mut cpu = Cpu::new(memory, 0x1000);
        let decoded = decode_opcode(0xAD).unwrap();
        cpu.execute_instruction(decoded).unwrap();
        
        prop_assert_eq!(cpu.state.a, value);
        prop_assert_eq!(cpu.state.flag_zero, value == 0);
        prop_assert_eq!(cpu.state.flag_negative, (value & 0x80) != 0);
    }
    
    #[test]
    fn lda_zero_page_x_with_wrapping(
        zp_base in 0u8..=0xFF,
        x_offset in 0u8..=0xFF,
        value in 0u8..=0xFF
    ) {
        let mut memory = Memory::new();
        // Use PC at 0x1000 to avoid conflicts with zero page
        memory.write(0x1000, 0xB5); // LDA zero page,X
        memory.write(0x1001, zp_base);
        
        // Calculate effective address with wrapping
        let effective_addr = zp_base.wrapping_add(x_offset) as u16;
        memory.write(effective_addr, value);
        
        let mut cpu = Cpu::new(memory, 0x1000);
        cpu.state.x = x_offset;
        
        let decoded = decode_opcode(0xB5).unwrap();
        cpu.execute_instruction(decoded).unwrap();
        
        prop_assert_eq!(cpu.state.a, value);
        prop_assert_eq!(cpu.state.flag_zero, value == 0);
        prop_assert_eq!(cpu.state.flag_negative, (value & 0x80) != 0);
    }
    
    #[test]
    fn ldx_zero_page_y_with_wrapping(
        zp_base in 0u8..=0xFF,
        y_offset in 0u8..=0xFF,
        value in 0u8..=0xFF
    ) {
        let mut memory = Memory::new();
        // Use PC at 0x1000 to avoid conflicts with zero page
        memory.write(0x1000, 0xB6); // LDX zero page,Y
        memory.write(0x1001, zp_base);
        
        // Calculate effective address with wrapping
        let effective_addr = zp_base.wrapping_add(y_offset) as u16;
        memory.write(effective_addr, value);
        
        let mut cpu = Cpu::new(memory, 0x1000);
        cpu.state.y = y_offset;
        
        let decoded = decode_opcode(0xB6).unwrap();
        cpu.execute_instruction(decoded).unwrap();
        
        prop_assert_eq!(cpu.state.x, value);
        prop_assert_eq!(cpu.state.flag_zero, value == 0);
        prop_assert_eq!(cpu.state.flag_negative, (value & 0x80) != 0);
    }
    
    #[test]
    fn ldy_zero_page_x_with_wrapping(
        zp_base in 0u8..=0xFF,
        x_offset in 0u8..=0xFF,
        value in 0u8..=0xFF
    ) {
        let mut memory = Memory::new();
        // Use PC at 0x1000 to avoid conflicts with zero page
        memory.write(0x1000, 0xB4); // LDY zero page,X
        memory.write(0x1001, zp_base);
        
        // Calculate effective address with wrapping
        let effective_addr = zp_base.wrapping_add(x_offset) as u16;
        memory.write(effective_addr, value);
        
        let mut cpu = Cpu::new(memory, 0x1000);
        cpu.state.x = x_offset;
        
        let decoded = decode_opcode(0xB4).unwrap();
        cpu.execute_instruction(decoded).unwrap();
        
        prop_assert_eq!(cpu.state.y, value);
        prop_assert_eq!(cpu.state.flag_zero, value == 0);
        prop_assert_eq!(cpu.state.flag_negative, (value & 0x80) != 0);
    }
    
    #[test]
    fn lda_absolute_x_with_page_crossing(
        base_addr in 0x0100u16..=0xFEFF, // Avoid wrapping at top of memory
        x_offset in 0u8..=0xFF,
        value in 0u8..=0xFF
    ) {
        let effective_addr = base_addr.wrapping_add(x_offset as u16);
        // Avoid overlap with instruction at 0x1000-0x1002
        prop_assume!(effective_addr < 0x1000 || effective_addr > 0x1002);
        
        let mut memory = Memory::new();
        // Use PC at 0x1000 to avoid conflicts
        memory.write(0x1000, 0xBD); // LDA absolute,X
        memory.write(0x1001, (base_addr & 0xFF) as u8);
        memory.write(0x1002, (base_addr >> 8) as u8);
        
        memory.write(effective_addr, value);
        
        let mut cpu = Cpu::new(memory, 0x1000);
        cpu.state.x = x_offset;
        
        let decoded = decode_opcode(0xBD).unwrap();
        cpu.execute_instruction(decoded).unwrap();
        
        prop_assert_eq!(cpu.state.a, value);
        prop_assert_eq!(cpu.state.flag_zero, value == 0);
        prop_assert_eq!(cpu.state.flag_negative, (value & 0x80) != 0);
    }
    
    #[test]
    fn lda_absolute_y_with_page_crossing(
        base_addr in 0x0100u16..=0xFEFF,
        y_offset in 0u8..=0xFF,
        value in 0u8..=0xFF
    ) {
        let effective_addr = base_addr.wrapping_add(y_offset as u16);
        // Avoid overlap with instruction at 0x1000-0x1002
        prop_assume!(effective_addr < 0x1000 || effective_addr > 0x1002);
        
        let mut memory = Memory::new();
        // Use PC at 0x1000 to avoid conflicts
        memory.write(0x1000, 0xB9); // LDA absolute,Y
        memory.write(0x1001, (base_addr & 0xFF) as u8);
        memory.write(0x1002, (base_addr >> 8) as u8);
        
        memory.write(effective_addr, value);
        
        let mut cpu = Cpu::new(memory, 0x1000);
        cpu.state.y = y_offset;
        
        let decoded = decode_opcode(0xB9).unwrap();
        cpu.execute_instruction(decoded).unwrap();
        
        prop_assert_eq!(cpu.state.a, value);
        prop_assert_eq!(cpu.state.flag_zero, value == 0);
        prop_assert_eq!(cpu.state.flag_negative, (value & 0x80) != 0);
    }
    
    #[test]
    fn lda_indexed_indirect_correct_addressing(
        zp_base in 0u8..=0xFE, // Leave room for pointer
        x_offset in 0u8..=0xFF,
        target_addr in 0x0200u16..=0xFFFF, // Avoid zero page and stack
        value in 0u8..=0xFF
    ) {
        // Avoid overlap with instruction at 0x1000-0x1002
        prop_assume!(target_addr < 0x1000 || target_addr > 0x1002);
        
        let mut memory = Memory::new();
        // Use PC at 0x1000 to avoid conflicts
        memory.write(0x1000, 0xA1); // LDA (indirect,X)
        memory.write(0x1001, zp_base);
        
        // Calculate pointer location with wrapping
        let ptr_addr = zp_base.wrapping_add(x_offset) as u16;
        
        // Store target address at pointer location (little-endian)
        memory.write(ptr_addr, (target_addr & 0xFF) as u8);
        memory.write(ptr_addr.wrapping_add(1), (target_addr >> 8) as u8);
        
        // Store value at target address
        memory.write(target_addr, value);
        
        let mut cpu = Cpu::new(memory, 0x1000);
        cpu.state.x = x_offset;
        
        let decoded = decode_opcode(0xA1).unwrap();
        cpu.execute_instruction(decoded).unwrap();
        
        prop_assert_eq!(cpu.state.a, value);
        prop_assert_eq!(cpu.state.flag_zero, value == 0);
        prop_assert_eq!(cpu.state.flag_negative, (value & 0x80) != 0);
    }
    
    #[test]
    fn lda_indirect_indexed_correct_addressing(
        zp_ptr in 0u8..=0xFE, // Leave room for 16-bit pointer
        base_addr in 0x0200u16..=0xFE00, // Avoid wrapping issues
        y_offset in 0u8..=0xFF,
        value in 0u8..=0xFF
    ) {
        let mut memory = Memory::new();
        // Use PC at 0x1000 to avoid conflicts
        memory.write(0x1000, 0xB1); // LDA (indirect),Y
        memory.write(0x1001, zp_ptr);
        
        // Store base address at zero page pointer (little-endian)
        memory.write(zp_ptr as u16, (base_addr & 0xFF) as u8);
        memory.write((zp_ptr as u16).wrapping_add(1), (base_addr >> 8) as u8);
        
        // Calculate effective address
        let effective_addr = base_addr.wrapping_add(y_offset as u16);
        memory.write(effective_addr, value);
        
        let mut cpu = Cpu::new(memory, 0x1000);
        cpu.state.y = y_offset;
        
        let decoded = decode_opcode(0xB1).unwrap();
        cpu.execute_instruction(decoded).unwrap();
        
        prop_assert_eq!(cpu.state.a, value);
        prop_assert_eq!(cpu.state.flag_zero, value == 0);
        prop_assert_eq!(cpu.state.flag_negative, (value & 0x80) != 0);
    }
    
    #[test]
    fn ldx_zero_page_loads_from_memory(
        zp_addr in 0u8..=0xFF,
        value in 0u8..=0xFF
    ) {
        let mut memory = Memory::new();
        // Use PC at 0x1000 to avoid conflicts with zero page
        memory.write(0x1000, 0xA6); // LDX zero page
        memory.write(0x1001, zp_addr);
        memory.write(zp_addr as u16, value);
        
        let mut cpu = Cpu::new(memory, 0x1000);
        let decoded = decode_opcode(0xA6).unwrap();
        cpu.execute_instruction(decoded).unwrap();
        
        prop_assert_eq!(cpu.state.x, value);
        prop_assert_eq!(cpu.state.flag_zero, value == 0);
        prop_assert_eq!(cpu.state.flag_negative, (value & 0x80) != 0);
    }
    
    #[test]
    fn ldx_absolute_loads_from_memory(
        addr in 0u16..=0xFFFF,
        value in 0u8..=0xFF
    ) {
        // Skip addresses that overlap with instruction bytes at 0x1000-0x1002
        prop_assume!(addr < 0x1000 || addr > 0x1002);
        
        let mut memory = Memory::new();
        // Write the value first to avoid overwriting instruction bytes
        memory.write(addr, value);
        // Use PC at 0x1000 to avoid conflicts with target address
        memory.write(0x1000, 0xAE); // LDX absolute
        memory.write(0x1001, (addr & 0xFF) as u8); // low byte
        memory.write(0x1002, (addr >> 8) as u8);   // high byte
        
        let mut cpu = Cpu::new(memory, 0x1000);
        let decoded = decode_opcode(0xAE).unwrap();
        cpu.execute_instruction(decoded).unwrap();
        
        prop_assert_eq!(cpu.state.x, value);
        prop_assert_eq!(cpu.state.flag_zero, value == 0);
        prop_assert_eq!(cpu.state.flag_negative, (value & 0x80) != 0);
    }
    
    #[test]
    fn ldx_absolute_y_with_page_crossing(
        base_addr in 0x0100u16..=0xFEFF,
        y_offset in 0u8..=0xFF,
        value in 0u8..=0xFF
    ) {
        let effective_addr = base_addr.wrapping_add(y_offset as u16);
        // Avoid overlap with instruction at 0x1000-0x1002
        prop_assume!(effective_addr < 0x1000 || effective_addr > 0x1002);
        
        let mut memory = Memory::new();
        // Use PC at 0x1000 to avoid conflicts
        memory.write(0x1000, 0xBE); // LDX absolute,Y
        memory.write(0x1001, (base_addr & 0xFF) as u8);
        memory.write(0x1002, (base_addr >> 8) as u8);
        
        memory.write(effective_addr, value);
        
        let mut cpu = Cpu::new(memory, 0x1000);
        cpu.state.y = y_offset;
        
        let decoded = decode_opcode(0xBE).unwrap();
        cpu.execute_instruction(decoded).unwrap();
        
        prop_assert_eq!(cpu.state.x, value);
        prop_assert_eq!(cpu.state.flag_zero, value == 0);
        prop_assert_eq!(cpu.state.flag_negative, (value & 0x80) != 0);
    }
    
    #[test]
    fn ldy_zero_page_loads_from_memory(
        zp_addr in 0u8..=0xFF,
        value in 0u8..=0xFF
    ) {
        let mut memory = Memory::new();
        // Use PC at 0x1000 to avoid conflicts with zero page
        memory.write(0x1000, 0xA4); // LDY zero page
        memory.write(0x1001, zp_addr);
        memory.write(zp_addr as u16, value);
        
        let mut cpu = Cpu::new(memory, 0x1000);
        let decoded = decode_opcode(0xA4).unwrap();
        cpu.execute_instruction(decoded).unwrap();
        
        prop_assert_eq!(cpu.state.y, value);
        prop_assert_eq!(cpu.state.flag_zero, value == 0);
        prop_assert_eq!(cpu.state.flag_negative, (value & 0x80) != 0);
    }
    
    #[test]
    fn ldy_absolute_loads_from_memory(
        addr in 0u16..=0xFFFF,
        value in 0u8..=0xFF
    ) {
        // Skip addresses that overlap with instruction bytes at 0x1000-0x1002
        prop_assume!(addr < 0x1000 || addr > 0x1002);
        
        let mut memory = Memory::new();
        // Write the value first to avoid overwriting instruction bytes
        memory.write(addr, value);
        // Use PC at 0x1000 to avoid conflicts with target address
        memory.write(0x1000, 0xAC); // LDY absolute
        memory.write(0x1001, (addr & 0xFF) as u8); // low byte
        memory.write(0x1002, (addr >> 8) as u8);   // high byte
        
        let mut cpu = Cpu::new(memory, 0x1000);
        let decoded = decode_opcode(0xAC).unwrap();
        cpu.execute_instruction(decoded).unwrap();
        
        prop_assert_eq!(cpu.state.y, value);
        prop_assert_eq!(cpu.state.flag_zero, value == 0);
        prop_assert_eq!(cpu.state.flag_negative, (value & 0x80) != 0);
    }
    
    #[test]
    fn ldy_absolute_x_with_page_crossing(
        base_addr in 0x0100u16..=0xFEFF,
        x_offset in 0u8..=0xFF,
        value in 0u8..=0xFF
    ) {
        let effective_addr = base_addr.wrapping_add(x_offset as u16);
        // Avoid overlap with instruction at 0x1000-0x1002
        prop_assume!(effective_addr < 0x1000 || effective_addr > 0x1002);
        
        let mut memory = Memory::new();
        // Use PC at 0x1000 to avoid conflicts
        memory.write(0x1000, 0xBC); // LDY absolute,X
        memory.write(0x1001, (base_addr & 0xFF) as u8);
        memory.write(0x1002, (base_addr >> 8) as u8);
        
        memory.write(effective_addr, value);
        
        let mut cpu = Cpu::new(memory, 0x1000);
        cpu.state.x = x_offset;
        
        let decoded = decode_opcode(0xBC).unwrap();
        cpu.execute_instruction(decoded).unwrap();
        
        prop_assert_eq!(cpu.state.y, value);
        prop_assert_eq!(cpu.state.flag_zero, value == 0);
        prop_assert_eq!(cpu.state.flag_negative, (value & 0x80) != 0);
    }
}
