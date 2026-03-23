// Property-based tests for store instructions (STA, STX, STY)
// Feature: 6502-cpu-emulator, Property 14: Store Instruction Correctness

use cpu_6502_emulator::cpu::Cpu;
use cpu_6502_emulator::memory::Memory;
use cpu_6502_emulator::instruction::decode_opcode;
use proptest::prelude::*;

// Feature: 6502-cpu-emulator, Property 14: Store Instruction Correctness
// **Validates: Requirements 8.1**
//
// For any register value and any memory address, executing a store instruction
// (STA, STX, STY) should:
// - Write the register value to the effective address
// - Leave all registers and flags unchanged

proptest! {
    #[test]
    fn sta_zero_page_stores_value_without_flag_changes(
        zp_addr in 0u8..=0xFF,
        a_value in 0u8..=0xFF,
        x_value in 0u8..=0xFF,
        y_value in 0u8..=0xFF,
        carry in proptest::bool::ANY,
        zero in proptest::bool::ANY,
        overflow in proptest::bool::ANY,
        negative in proptest::bool::ANY
    ) {
        let mut memory = Memory::new();
        // Use PC at 0x1000 to avoid conflicts with zero page
        memory.write(0x1000, 0x85); // STA zero page
        memory.write(0x1001, zp_addr);
        
        let mut cpu = Cpu::new(memory, 0x1000);
        cpu.state.a = a_value;
        cpu.state.x = x_value;
        cpu.state.y = y_value;
        cpu.state.flag_carry = carry;
        cpu.state.flag_zero = zero;
        cpu.state.flag_overflow = overflow;
        cpu.state.flag_negative = negative;
        
        let decoded = decode_opcode(0x85).unwrap();
        cpu.execute_instruction(decoded).unwrap();
        
        // Check value was written to memory
        prop_assert_eq!(cpu.memory.read(zp_addr as u16), a_value);
        
        // Check all registers unchanged
        prop_assert_eq!(cpu.state.a, a_value);
        prop_assert_eq!(cpu.state.x, x_value);
        prop_assert_eq!(cpu.state.y, y_value);
        
        // Check all flags unchanged
        prop_assert_eq!(cpu.state.flag_carry, carry);
        prop_assert_eq!(cpu.state.flag_zero, zero);
        prop_assert_eq!(cpu.state.flag_overflow, overflow);
        prop_assert_eq!(cpu.state.flag_negative, negative);
    }
    
    #[test]
    fn sta_zero_page_x_stores_with_wrapping(
        zp_base in 0u8..=0xFF,
        x_offset in 0u8..=0xFF,
        a_value in 0u8..=0xFF
    ) {
        let mut memory = Memory::new();
        // Use PC at 0x1000 to avoid conflicts with zero page
        memory.write(0x1000, 0x95); // STA zero page,X
        memory.write(0x1001, zp_base);
        
        let mut cpu = Cpu::new(memory, 0x1000);
        cpu.state.a = a_value;
        cpu.state.x = x_offset;
        
        // Calculate effective address with wrapping
        let effective_addr = zp_base.wrapping_add(x_offset) as u16;
        
        let decoded = decode_opcode(0x95).unwrap();
        cpu.execute_instruction(decoded).unwrap();
        
        // Check value was written to correct address
        prop_assert_eq!(cpu.memory.read(effective_addr), a_value);
        
        // Check registers unchanged
        prop_assert_eq!(cpu.state.a, a_value);
        prop_assert_eq!(cpu.state.x, x_offset);
    }
    
    #[test]
    fn sta_absolute_stores_value(
        addr in 0x0200u16..=0xFFFF, // Avoid zero page and stack
        a_value in 0u8..=0xFF
    ) {
        let mut memory = Memory::new();
        // Use PC at 0x1000 to avoid conflicts
        memory.write(0x1000, 0x8D); // STA absolute
        memory.write(0x1001, (addr & 0xFF) as u8); // low byte
        memory.write(0x1002, (addr >> 8) as u8);   // high byte
        
        let mut cpu = Cpu::new(memory, 0x1000);
        cpu.state.a = a_value;
        
        let decoded = decode_opcode(0x8D).unwrap();
        cpu.execute_instruction(decoded).unwrap();
        
        // Check value was written to memory
        prop_assert_eq!(cpu.memory.read(addr), a_value);
        
        // Check register unchanged
        prop_assert_eq!(cpu.state.a, a_value);
    }
    
    #[test]
    fn sta_absolute_x_stores_with_offset(
        base_addr in 0x0200u16..=0xFEFF, // Avoid wrapping at top
        x_offset in 0u8..=0xFF,
        a_value in 0u8..=0xFF
    ) {
        let mut memory = Memory::new();
        // Use PC at 0x1000 to avoid conflicts
        memory.write(0x1000, 0x9D); // STA absolute,X
        memory.write(0x1001, (base_addr & 0xFF) as u8);
        memory.write(0x1002, (base_addr >> 8) as u8);
        
        let mut cpu = Cpu::new(memory, 0x1000);
        cpu.state.a = a_value;
        cpu.state.x = x_offset;
        
        let effective_addr = base_addr.wrapping_add(x_offset as u16);
        
        let decoded = decode_opcode(0x9D).unwrap();
        cpu.execute_instruction(decoded).unwrap();
        
        // Check value was written to correct address
        prop_assert_eq!(cpu.memory.read(effective_addr), a_value);
        
        // Check registers unchanged
        prop_assert_eq!(cpu.state.a, a_value);
        prop_assert_eq!(cpu.state.x, x_offset);
    }
    
    #[test]
    fn sta_absolute_y_stores_with_offset(
        base_addr in 0x0200u16..=0xFEFF,
        y_offset in 0u8..=0xFF,
        a_value in 0u8..=0xFF
    ) {
        let mut memory = Memory::new();
        // Use PC at 0x1000 to avoid conflicts
        memory.write(0x1000, 0x99); // STA absolute,Y
        memory.write(0x1001, (base_addr & 0xFF) as u8);
        memory.write(0x1002, (base_addr >> 8) as u8);
        
        let mut cpu = Cpu::new(memory, 0x1000);
        cpu.state.a = a_value;
        cpu.state.y = y_offset;
        
        let effective_addr = base_addr.wrapping_add(y_offset as u16);
        
        let decoded = decode_opcode(0x99).unwrap();
        cpu.execute_instruction(decoded).unwrap();
        
        // Check value was written to correct address
        prop_assert_eq!(cpu.memory.read(effective_addr), a_value);
        
        // Check registers unchanged
        prop_assert_eq!(cpu.state.a, a_value);
        prop_assert_eq!(cpu.state.y, y_offset);
    }
    
    #[test]
    fn sta_indexed_indirect_stores_correctly(
        zp_base in 0u8..=0xFE, // Leave room for pointer
        x_offset in 0u8..=0xFF,
        target_addr in 0x0200u16..=0xFFFF, // Avoid zero page and stack
        a_value in 0u8..=0xFF
    ) {
        let mut memory = Memory::new();
        // Use PC at 0x1000 to avoid conflicts
        memory.write(0x1000, 0x81); // STA (indirect,X)
        memory.write(0x1001, zp_base);
        
        // Calculate pointer location with wrapping
        let ptr_addr = zp_base.wrapping_add(x_offset) as u16;
        
        // Store target address at pointer location (little-endian)
        memory.write(ptr_addr, (target_addr & 0xFF) as u8);
        memory.write(ptr_addr.wrapping_add(1), (target_addr >> 8) as u8);
        
        let mut cpu = Cpu::new(memory, 0x1000);
        cpu.state.a = a_value;
        cpu.state.x = x_offset;
        
        let decoded = decode_opcode(0x81).unwrap();
        cpu.execute_instruction(decoded).unwrap();
        
        // Check value was written to target address
        prop_assert_eq!(cpu.memory.read(target_addr), a_value);
        
        // Check registers unchanged
        prop_assert_eq!(cpu.state.a, a_value);
        prop_assert_eq!(cpu.state.x, x_offset);
    }
    
    #[test]
    fn sta_indirect_indexed_stores_correctly(
        zp_ptr in 0u8..=0xFE, // Leave room for 16-bit pointer
        base_addr in 0x0200u16..=0xFE00, // Avoid wrapping issues
        y_offset in 0u8..=0xFF,
        a_value in 0u8..=0xFF
    ) {
        let mut memory = Memory::new();
        // Use PC at 0x1000 to avoid conflicts
        memory.write(0x1000, 0x91); // STA (indirect),Y
        memory.write(0x1001, zp_ptr);
        
        // Store base address at zero page pointer (little-endian)
        memory.write(zp_ptr as u16, (base_addr & 0xFF) as u8);
        memory.write((zp_ptr as u16).wrapping_add(1), (base_addr >> 8) as u8);
        
        let mut cpu = Cpu::new(memory, 0x1000);
        cpu.state.a = a_value;
        cpu.state.y = y_offset;
        
        // Calculate effective address
        let effective_addr = base_addr.wrapping_add(y_offset as u16);
        
        let decoded = decode_opcode(0x91).unwrap();
        cpu.execute_instruction(decoded).unwrap();
        
        // Check value was written to effective address
        prop_assert_eq!(cpu.memory.read(effective_addr), a_value);
        
        // Check registers unchanged
        prop_assert_eq!(cpu.state.a, a_value);
        prop_assert_eq!(cpu.state.y, y_offset);
    }
    
    #[test]
    fn stx_zero_page_stores_value(
        zp_addr in 0u8..=0xFF,
        x_value in 0u8..=0xFF,
        a_value in 0u8..=0xFF,
        y_value in 0u8..=0xFF
    ) {
        let mut memory = Memory::new();
        // Use PC at 0x1000 to avoid conflicts with zero page
        memory.write(0x1000, 0x86); // STX zero page
        memory.write(0x1001, zp_addr);
        
        let mut cpu = Cpu::new(memory, 0x1000);
        cpu.state.x = x_value;
        cpu.state.a = a_value;
        cpu.state.y = y_value;
        
        let decoded = decode_opcode(0x86).unwrap();
        cpu.execute_instruction(decoded).unwrap();
        
        // Check value was written to memory
        prop_assert_eq!(cpu.memory.read(zp_addr as u16), x_value);
        
        // Check all registers unchanged
        prop_assert_eq!(cpu.state.x, x_value);
        prop_assert_eq!(cpu.state.a, a_value);
        prop_assert_eq!(cpu.state.y, y_value);
    }
    
    #[test]
    fn stx_zero_page_y_stores_with_wrapping(
        zp_base in 0u8..=0xFF,
        y_offset in 0u8..=0xFF,
        x_value in 0u8..=0xFF
    ) {
        let mut memory = Memory::new();
        // Use PC at 0x1000 to avoid conflicts with zero page
        memory.write(0x1000, 0x96); // STX zero page,Y
        memory.write(0x1001, zp_base);
        
        let mut cpu = Cpu::new(memory, 0x1000);
        cpu.state.x = x_value;
        cpu.state.y = y_offset;
        
        // Calculate effective address with wrapping
        let effective_addr = zp_base.wrapping_add(y_offset) as u16;
        
        let decoded = decode_opcode(0x96).unwrap();
        cpu.execute_instruction(decoded).unwrap();
        
        // Check value was written to correct address
        prop_assert_eq!(cpu.memory.read(effective_addr), x_value);
        
        // Check registers unchanged
        prop_assert_eq!(cpu.state.x, x_value);
        prop_assert_eq!(cpu.state.y, y_offset);
    }
    
    #[test]
    fn stx_absolute_stores_value(
        addr in 0x0200u16..=0xFFFF, // Avoid zero page and stack
        x_value in 0u8..=0xFF
    ) {
        let mut memory = Memory::new();
        // Use PC at 0x1000 to avoid conflicts
        memory.write(0x1000, 0x8E); // STX absolute
        memory.write(0x1001, (addr & 0xFF) as u8); // low byte
        memory.write(0x1002, (addr >> 8) as u8);   // high byte
        
        let mut cpu = Cpu::new(memory, 0x1000);
        cpu.state.x = x_value;
        
        let decoded = decode_opcode(0x8E).unwrap();
        cpu.execute_instruction(decoded).unwrap();
        
        // Check value was written to memory
        prop_assert_eq!(cpu.memory.read(addr), x_value);
        
        // Check register unchanged
        prop_assert_eq!(cpu.state.x, x_value);
    }
    
    #[test]
    fn sty_zero_page_stores_value(
        zp_addr in 0u8..=0xFF,
        y_value in 0u8..=0xFF,
        a_value in 0u8..=0xFF,
        x_value in 0u8..=0xFF
    ) {
        let mut memory = Memory::new();
        // Use PC at 0x1000 to avoid conflicts with zero page
        memory.write(0x1000, 0x84); // STY zero page
        memory.write(0x1001, zp_addr);
        
        let mut cpu = Cpu::new(memory, 0x1000);
        cpu.state.y = y_value;
        cpu.state.a = a_value;
        cpu.state.x = x_value;
        
        let decoded = decode_opcode(0x84).unwrap();
        cpu.execute_instruction(decoded).unwrap();
        
        // Check value was written to memory
        prop_assert_eq!(cpu.memory.read(zp_addr as u16), y_value);
        
        // Check all registers unchanged
        prop_assert_eq!(cpu.state.y, y_value);
        prop_assert_eq!(cpu.state.a, a_value);
        prop_assert_eq!(cpu.state.x, x_value);
    }
    
    #[test]
    fn sty_zero_page_x_stores_with_wrapping(
        zp_base in 0u8..=0xFF,
        x_offset in 0u8..=0xFF,
        y_value in 0u8..=0xFF
    ) {
        let mut memory = Memory::new();
        // Use PC at 0x1000 to avoid conflicts with zero page
        memory.write(0x1000, 0x94); // STY zero page,X
        memory.write(0x1001, zp_base);
        
        let mut cpu = Cpu::new(memory, 0x1000);
        cpu.state.y = y_value;
        cpu.state.x = x_offset;
        
        // Calculate effective address with wrapping
        let effective_addr = zp_base.wrapping_add(x_offset) as u16;
        
        let decoded = decode_opcode(0x94).unwrap();
        cpu.execute_instruction(decoded).unwrap();
        
        // Check value was written to correct address
        prop_assert_eq!(cpu.memory.read(effective_addr), y_value);
        
        // Check registers unchanged
        prop_assert_eq!(cpu.state.y, y_value);
        prop_assert_eq!(cpu.state.x, x_offset);
    }
    
    #[test]
    fn sty_absolute_stores_value(
        addr in 0x0200u16..=0xFFFF, // Avoid zero page and stack
        y_value in 0u8..=0xFF
    ) {
        let mut memory = Memory::new();
        // Use PC at 0x1000 to avoid conflicts
        memory.write(0x1000, 0x8C); // STY absolute
        memory.write(0x1001, (addr & 0xFF) as u8); // low byte
        memory.write(0x1002, (addr >> 8) as u8);   // high byte
        
        let mut cpu = Cpu::new(memory, 0x1000);
        cpu.state.y = y_value;
        
        let decoded = decode_opcode(0x8C).unwrap();
        cpu.execute_instruction(decoded).unwrap();
        
        // Check value was written to memory
        prop_assert_eq!(cpu.memory.read(addr), y_value);
        
        // Check register unchanged
        prop_assert_eq!(cpu.state.y, y_value);
    }
    
    #[test]
    fn store_instructions_preserve_all_flags(
        zp_addr in 0u8..=0xFF,
        value in 0u8..=0xFF,
        carry in proptest::bool::ANY,
        zero in proptest::bool::ANY,
        interrupt_disable in proptest::bool::ANY,
        decimal in proptest::bool::ANY,
        overflow in proptest::bool::ANY,
        negative in proptest::bool::ANY
    ) {
        let mut memory = Memory::new();
        // Use PC at 0x1000 to avoid conflicts with zero page
        memory.write(0x1000, 0x85); // STA zero page
        memory.write(0x1001, zp_addr);
        
        let mut cpu = Cpu::new(memory, 0x1000);
        cpu.state.a = value;
        cpu.state.flag_carry = carry;
        cpu.state.flag_zero = zero;
        cpu.state.flag_interrupt_disable = interrupt_disable;
        cpu.state.flag_decimal = decimal;
        cpu.state.flag_overflow = overflow;
        cpu.state.flag_negative = negative;
        
        let decoded = decode_opcode(0x85).unwrap();
        cpu.execute_instruction(decoded).unwrap();
        
        // Check all flags unchanged
        prop_assert_eq!(cpu.state.flag_carry, carry);
        prop_assert_eq!(cpu.state.flag_zero, zero);
        prop_assert_eq!(cpu.state.flag_interrupt_disable, interrupt_disable);
        prop_assert_eq!(cpu.state.flag_decimal, decimal);
        prop_assert_eq!(cpu.state.flag_overflow, overflow);
        prop_assert_eq!(cpu.state.flag_negative, negative);
    }
}
