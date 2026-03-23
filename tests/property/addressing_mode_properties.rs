// Property-based tests for addressing mode calculations
// Tests universal properties across randomized inputs

use cpu_6502_emulator::cpu::Cpu;
use cpu_6502_emulator::memory::Memory;
use cpu_6502_emulator::instruction::AddressingMode;
use proptest::prelude::*;

// Property 20: Addressing Mode Calculation
// For any addressing mode and any register/memory state, the effective address
// calculation should match 6502 specifications:
// - Zero Page,X/Y should wrap at 0xFF
// - Absolute,X/Y should not wrap (full 16-bit addition)
// - Indexed Indirect should wrap the zero page pointer
// - Indirect Indexed should not wrap the final address
// **Validates: Requirements 7.3**

// Test Zero Page,X wrapping behavior
proptest! {
    #[test]
    fn prop_zero_page_x_wraps_at_0xff(
        base in 0u8..=0xFF,
        x_reg in 0u8..=0xFF,
        pc in 0x1000u16..=0xF000u16
    ) {
        let mut memory = Memory::new();
        memory.write(pc.wrapping_add(1), base);
        
        let mut cpu = Cpu::new(memory, pc);
        cpu.state.x = x_reg;
        
        let addr = cpu.get_effective_address(AddressingMode::ZeroPageX);
        
        // Expected: base + x_reg wrapped to stay in zero page
        let expected = base.wrapping_add(x_reg) as u16;
        
        prop_assert_eq!(addr, expected,
            "Zero Page,X wrapping failed: base=0x{:02X} + X=0x{:02X} should wrap to 0x{:04X} but got 0x{:04X}",
            base, x_reg, expected, addr);
        
        // Verify result is always in zero page
        prop_assert!(addr <= 0xFF,
            "Zero Page,X result 0x{:04X} is not in zero page (0x00-0xFF)", addr);
    }
}

// Test Zero Page,Y wrapping behavior
proptest! {
    #[test]
    fn prop_zero_page_y_wraps_at_0xff(
        base in 0u8..=0xFF,
        y_reg in 0u8..=0xFF,
        pc in 0x1000u16..=0xF000u16
    ) {
        let mut memory = Memory::new();
        memory.write(pc.wrapping_add(1), base);
        
        let mut cpu = Cpu::new(memory, pc);
        cpu.state.y = y_reg;
        
        let addr = cpu.get_effective_address(AddressingMode::ZeroPageY);
        
        // Expected: base + y_reg wrapped to stay in zero page
        let expected = base.wrapping_add(y_reg) as u16;
        
        prop_assert_eq!(addr, expected,
            "Zero Page,Y wrapping failed: base=0x{:02X} + Y=0x{:02X} should wrap to 0x{:04X} but got 0x{:04X}",
            base, y_reg, expected, addr);
        
        // Verify result is always in zero page
        prop_assert!(addr <= 0xFF,
            "Zero Page,Y result 0x{:04X} is not in zero page (0x00-0xFF)", addr);
    }
}

// Test Absolute,X does NOT wrap at page boundaries (full 16-bit addition)
proptest! {
    #[test]
    fn prop_absolute_x_full_16bit_addition(
        base_low in 0u8..=0xFF,
        base_high in 0u8..=0xFF,
        x_reg in 0u8..=0xFF,
        pc in 0x1000u16..=0xF000u16
    ) {
        let mut memory = Memory::new();
        memory.write(pc.wrapping_add(1), base_low);
        memory.write(pc.wrapping_add(2), base_high);
        
        let mut cpu = Cpu::new(memory, pc);
        cpu.state.x = x_reg;
        
        let addr = cpu.get_effective_address(AddressingMode::AbsoluteX);
        
        // Expected: full 16-bit addition with wrapping at 64KB boundary
        let base = ((base_high as u16) << 8) | (base_low as u16);
        let expected = base.wrapping_add(x_reg as u16);
        
        prop_assert_eq!(addr, expected,
            "Absolute,X calculation failed: base=0x{:04X} + X=0x{:02X} should be 0x{:04X} but got 0x{:04X}",
            base, x_reg, expected, addr);
    }
}

// Test Absolute,Y does NOT wrap at page boundaries (full 16-bit addition)
proptest! {
    #[test]
    fn prop_absolute_y_full_16bit_addition(
        base_low in 0u8..=0xFF,
        base_high in 0u8..=0xFF,
        y_reg in 0u8..=0xFF,
        pc in 0x1000u16..=0xF000u16
    ) {
        let mut memory = Memory::new();
        memory.write(pc.wrapping_add(1), base_low);
        memory.write(pc.wrapping_add(2), base_high);
        
        let mut cpu = Cpu::new(memory, pc);
        cpu.state.y = y_reg;
        
        let addr = cpu.get_effective_address(AddressingMode::AbsoluteY);
        
        // Expected: full 16-bit addition with wrapping at 64KB boundary
        let base = ((base_high as u16) << 8) | (base_low as u16);
        let expected = base.wrapping_add(y_reg as u16);
        
        prop_assert_eq!(addr, expected,
            "Absolute,Y calculation failed: base=0x{:04X} + Y=0x{:02X} should be 0x{:04X} but got 0x{:04X}",
            base, y_reg, expected, addr);
    }
}

// Test Indexed Indirect (Indirect,X) wraps the zero page pointer
proptest! {
    #[test]
    fn prop_indexed_indirect_wraps_pointer(
        zp_base in 0u8..=0xFF,
        x_reg in 0u8..=0xFF,
        target_low in 0u8..=0xFF,
        target_high in 0u8..=0xFF,
        pc in 0x1000u16..=0xF000u16
    ) {
        let mut memory = Memory::new();
        memory.write(pc.wrapping_add(1), zp_base);
        
        // Calculate wrapped pointer address
        let ptr = zp_base.wrapping_add(x_reg) as u16;
        
        // Write target address at the pointer location
        memory.write(ptr, target_low);
        memory.write(ptr.wrapping_add(1), target_high);
        
        let mut cpu = Cpu::new(memory, pc);
        cpu.state.x = x_reg;
        
        let addr = cpu.get_effective_address(AddressingMode::IndexedIndirect);
        
        // Expected: address read from (zp_base + x_reg) wrapped in zero page
        let expected = ((target_high as u16) << 8) | (target_low as u16);
        
        prop_assert_eq!(addr, expected,
            "Indexed Indirect failed: (0x{:02X} + X=0x{:02X}) = ptr 0x{:04X} should point to 0x{:04X} but got 0x{:04X}",
            zp_base, x_reg, ptr, expected, addr);
    }
}

// Test Indirect Indexed (Indirect),Y does NOT wrap the final address
proptest! {
    #[test]
    fn prop_indirect_indexed_no_wrap_final_address(
        zp_ptr in 0u8..=0xFF,
        y_reg in 0u8..=0xFF,
        base_low in 0u8..=0xFF,
        base_high in 0u8..=0xFF,
        pc in 0x1000u16..=0xF000u16
    ) {
        let mut memory = Memory::new();
        memory.write(pc.wrapping_add(1), zp_ptr);
        
        // Write base address at zero page pointer
        memory.write(zp_ptr as u16, base_low);
        memory.write((zp_ptr as u16).wrapping_add(1), base_high);
        
        let mut cpu = Cpu::new(memory, pc);
        cpu.state.y = y_reg;
        
        let addr = cpu.get_effective_address(AddressingMode::IndirectIndexed);
        
        // Expected: (base address from zero page) + Y with full 16-bit addition
        let base = ((base_high as u16) << 8) | (base_low as u16);
        let expected = base.wrapping_add(y_reg as u16);
        
        prop_assert_eq!(addr, expected,
            "Indirect Indexed failed: (0x{:02X}) = 0x{:04X} + Y=0x{:02X} should be 0x{:04X} but got 0x{:04X}",
            zp_ptr, base, y_reg, expected, addr);
    }
}

// Test Relative addressing with positive offsets
proptest! {
    #[test]
    fn prop_relative_addressing_positive_offset(
        offset in 0i8..=127,
        pc in 0x1000u16..=0xF000u16
    ) {
        let mut memory = Memory::new();
        memory.write(pc.wrapping_add(1), offset as u8);
        
        let cpu = Cpu::new(memory, pc);
        let addr = cpu.get_effective_address(AddressingMode::Relative);
        
        // Expected: PC + 2 (after instruction) + offset
        let expected = pc.wrapping_add(2).wrapping_add(offset as u16);
        
        prop_assert_eq!(addr, expected,
            "Relative addressing (positive) failed: PC=0x{:04X} + 2 + offset={} should be 0x{:04X} but got 0x{:04X}",
            pc, offset, expected, addr);
    }
}

// Test Relative addressing with negative offsets
proptest! {
    #[test]
    fn prop_relative_addressing_negative_offset(
        offset in -128i8..=-1,
        pc in 0x1000u16..=0xF000u16
    ) {
        let mut memory = Memory::new();
        memory.write(pc.wrapping_add(1), offset as u8);
        
        let cpu = Cpu::new(memory, pc);
        let addr = cpu.get_effective_address(AddressingMode::Relative);
        
        // Expected: PC + 2 (after instruction) + signed offset
        let base = pc.wrapping_add(2);
        let expected = base.wrapping_add(offset as i16 as u16);
        
        prop_assert_eq!(addr, expected,
            "Relative addressing (negative) failed: PC=0x{:04X} + 2 + offset={} should be 0x{:04X} but got 0x{:04X}",
            pc, offset, expected, addr);
    }
}

// Test Immediate addressing always returns PC + 1
proptest! {
    #[test]
    fn prop_immediate_addressing_returns_pc_plus_1(
        pc in 0u16..=0xFFFF
    ) {
        let memory = Memory::new();
        let cpu = Cpu::new(memory, pc);
        
        let addr = cpu.get_effective_address(AddressingMode::Immediate);
        let expected = pc.wrapping_add(1);
        
        prop_assert_eq!(addr, expected,
            "Immediate addressing failed: should return PC+1 (0x{:04X}) but got 0x{:04X}",
            expected, addr);
    }
}

// Test Zero Page addressing stays in zero page
proptest! {
    #[test]
    fn prop_zero_page_stays_in_zero_page(
        zp_addr in 0u8..=0xFF,
        pc in 0x1000u16..=0xF000u16
    ) {
        let mut memory = Memory::new();
        memory.write(pc.wrapping_add(1), zp_addr);
        
        let cpu = Cpu::new(memory, pc);
        let addr = cpu.get_effective_address(AddressingMode::ZeroPage);
        
        prop_assert_eq!(addr, zp_addr as u16,
            "Zero Page addressing failed: should return 0x{:04X} but got 0x{:04X}",
            zp_addr as u16, addr);
        
        prop_assert!(addr <= 0xFF,
            "Zero Page result 0x{:04X} is not in zero page (0x00-0xFF)", addr);
    }
}

// Test Absolute addressing returns correct 16-bit address
proptest! {
    #[test]
    fn prop_absolute_addressing_correct_address(
        addr_low in 0u8..=0xFF,
        addr_high in 0u8..=0xFF,
        pc in 0x1000u16..=0xF000u16
    ) {
        let mut memory = Memory::new();
        memory.write(pc.wrapping_add(1), addr_low);
        memory.write(pc.wrapping_add(2), addr_high);
        
        let cpu = Cpu::new(memory, pc);
        let addr = cpu.get_effective_address(AddressingMode::Absolute);
        
        let expected = ((addr_high as u16) << 8) | (addr_low as u16);
        
        prop_assert_eq!(addr, expected,
            "Absolute addressing failed: should return 0x{:04X} but got 0x{:04X}",
            expected, addr);
    }
}

// Test Indirect addressing reads address from pointer
proptest! {
    #[test]
    fn prop_indirect_addressing_reads_from_pointer(
        ptr_low in 0u8..=0xFF,
        ptr_high in 0u8..=0xFF,
        target_low in 0u8..=0xFF,
        target_high in 0u8..=0xFF,
        pc in 0x1000u16..=0xF000u16
    ) {
        let mut memory = Memory::new();
        memory.write(pc.wrapping_add(1), ptr_low);
        memory.write(pc.wrapping_add(2), ptr_high);
        
        let ptr = ((ptr_high as u16) << 8) | (ptr_low as u16);
        memory.write(ptr, target_low);
        memory.write(ptr.wrapping_add(1), target_high);
        
        let cpu = Cpu::new(memory, pc);
        let addr = cpu.get_effective_address(AddressingMode::Indirect);
        
        let expected = ((target_high as u16) << 8) | (target_low as u16);
        
        prop_assert_eq!(addr, expected,
            "Indirect addressing failed: ptr 0x{:04X} should point to 0x{:04X} but got 0x{:04X}",
            ptr, expected, addr);
    }
}
