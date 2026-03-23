// Unit tests for CPU initialization
// Validates Requirements 3.1, 3.2, 3.3, 3.4, 3.5, 2.2, 4.1

use cpu_6502_emulator::cpu::{Cpu, CpuState};
use cpu_6502_emulator::memory::Memory;

#[test]
fn test_cpu_state_initialization_with_start_address() {
    // Test with various start addresses
    let test_addresses = [0x0000, 0x8000, 0xC000, 0xFFFF];
    
    for &start_addr in &test_addresses {
        let cpu_state = CpuState::new(start_addr);
        
        // Requirement 2.2: PC should be set to the provided start address
        assert_eq!(cpu_state.pc, start_addr, 
            "Program Counter should be set to start address 0x{:04X}", start_addr);
        
        // Requirement 3.1: Accumulator should be initialized to 0x00
        assert_eq!(cpu_state.a, 0x00, 
            "Accumulator should be initialized to 0x00");
        
        // Requirement 3.2: Index Register X should be initialized to 0x00
        assert_eq!(cpu_state.x, 0x00, 
            "Index Register X should be initialized to 0x00");
        
        // Requirement 3.3: Index Register Y should be initialized to 0x00
        assert_eq!(cpu_state.y, 0x00, 
            "Index Register Y should be initialized to 0x00");
        
        // Requirement 3.4: Stack Pointer should be initialized to 0xFF
        assert_eq!(cpu_state.sp, 0xFF, 
            "Stack Pointer should be initialized to 0xFF");
        
        // Requirement 3.5: Status Register should be initialized to 0x00
        // All flags should be false (cleared)
        assert_eq!(cpu_state.flag_carry, false, "Carry flag should be cleared");
        assert_eq!(cpu_state.flag_zero, false, "Zero flag should be cleared");
        assert_eq!(cpu_state.flag_interrupt_disable, false, "Interrupt Disable flag should be cleared");
        assert_eq!(cpu_state.flag_decimal, false, "Decimal flag should be cleared");
        assert_eq!(cpu_state.flag_break, false, "Break flag should be cleared");
        assert_eq!(cpu_state.flag_overflow, false, "Overflow flag should be cleared");
        assert_eq!(cpu_state.flag_negative, false, "Negative flag should be cleared");
        
        // Verify status byte is 0x20 (bit 5 always set, all other flags clear)
        assert_eq!(cpu_state.get_status_byte(), 0x20, 
            "Status byte should be 0x20 (only bit 5 set)");
    }
}

#[test]
fn test_cpu_state_initialization_boundary_addresses() {
    // Test boundary conditions for start address
    
    // Minimum address
    let cpu_min = CpuState::new(0x0000);
    assert_eq!(cpu_min.pc, 0x0000);
    
    // Maximum address
    let cpu_max = CpuState::new(0xFFFF);
    assert_eq!(cpu_max.pc, 0xFFFF);
    
    // Common reset vector address
    let cpu_reset = CpuState::new(0xFFFC);
    assert_eq!(cpu_reset.pc, 0xFFFC);
}

#[test]
fn test_status_byte_packing_unpacking() {
    let mut cpu_state = CpuState::new(0x0000);
    
    // Test that initial status byte has bit 5 set
    assert_eq!(cpu_state.get_status_byte(), 0x20);
    
    // Set all flags
    cpu_state.flag_carry = true;
    cpu_state.flag_zero = true;
    cpu_state.flag_interrupt_disable = true;
    cpu_state.flag_decimal = true;
    cpu_state.flag_break = true;
    cpu_state.flag_overflow = true;
    cpu_state.flag_negative = true;
    
    // Get status byte with all flags set
    let status_all_set = cpu_state.get_status_byte();
    assert_eq!(status_all_set, 0xFF, "All flags set should produce 0xFF");
    
    // Clear all flags and set from byte
    cpu_state.flag_carry = false;
    cpu_state.flag_zero = false;
    cpu_state.flag_interrupt_disable = false;
    cpu_state.flag_decimal = false;
    cpu_state.flag_break = false;
    cpu_state.flag_overflow = false;
    cpu_state.flag_negative = false;
    
    // Unpack the status byte
    cpu_state.set_status_byte(0xFF);
    
    // Verify all flags are set
    assert!(cpu_state.flag_carry, "Carry flag should be set");
    assert!(cpu_state.flag_zero, "Zero flag should be set");
    assert!(cpu_state.flag_interrupt_disable, "Interrupt Disable flag should be set");
    assert!(cpu_state.flag_decimal, "Decimal flag should be set");
    assert!(cpu_state.flag_break, "Break flag should be set");
    assert!(cpu_state.flag_overflow, "Overflow flag should be set");
    assert!(cpu_state.flag_negative, "Negative flag should be set");
}

#[test]
fn test_update_zero_negative_flags() {
    let mut cpu_state = CpuState::new(0x0000);
    
    // Test with zero value
    cpu_state.update_zero_negative(0x00);
    assert!(cpu_state.flag_zero, "Zero flag should be set for value 0x00");
    assert!(!cpu_state.flag_negative, "Negative flag should be clear for value 0x00");
    
    // Test with positive value (bit 7 = 0)
    cpu_state.update_zero_negative(0x42);
    assert!(!cpu_state.flag_zero, "Zero flag should be clear for value 0x42");
    assert!(!cpu_state.flag_negative, "Negative flag should be clear for value 0x42");
    
    // Test with negative value (bit 7 = 1)
    cpu_state.update_zero_negative(0x80);
    assert!(!cpu_state.flag_zero, "Zero flag should be clear for value 0x80");
    assert!(cpu_state.flag_negative, "Negative flag should be set for value 0x80");
    
    // Test with 0xFF (negative in signed interpretation)
    cpu_state.update_zero_negative(0xFF);
    assert!(!cpu_state.flag_zero, "Zero flag should be clear for value 0xFF");
    assert!(cpu_state.flag_negative, "Negative flag should be set for value 0xFF");
    
    // Test with 0x7F (maximum positive in signed interpretation)
    cpu_state.update_zero_negative(0x7F);
    assert!(!cpu_state.flag_zero, "Zero flag should be clear for value 0x7F");
    assert!(!cpu_state.flag_negative, "Negative flag should be clear for value 0x7F");
}

#[test]
fn test_cpu_struct_initialization() {
    // Test that Cpu struct properly combines CpuState and Memory
    // Validates Requirement 4.1
    
    let memory = Memory::new();
    let start_address = 0x8000;
    let cpu = Cpu::new(memory, start_address);
    
    // Verify CPU state is initialized correctly
    assert_eq!(cpu.state.pc, start_address, 
        "CPU Program Counter should be set to start address");
    assert_eq!(cpu.state.a, 0x00, "CPU Accumulator should be 0x00");
    assert_eq!(cpu.state.x, 0x00, "CPU X register should be 0x00");
    assert_eq!(cpu.state.y, 0x00, "CPU Y register should be 0x00");
    assert_eq!(cpu.state.sp, 0xFF, "CPU Stack Pointer should be 0xFF");
    
    // Verify halted flag is initialized to false
    assert_eq!(cpu.halted, false, "CPU should not be halted on initialization");
    
    // Verify memory is accessible
    let value = cpu.memory.read(0x0000);
    assert_eq!(value, 0x00, "Memory should be initialized to zeros");
}

#[test]
fn test_cpu_with_preloaded_memory() {
    // Test that Cpu struct works with memory that has been preloaded
    
    let mut memory = Memory::new();
    // Write some test values to memory
    memory.write(0x8000, 0xA9); // LDA immediate
    memory.write(0x8001, 0x42); // value 0x42
    
    let cpu = Cpu::new(memory, 0x8000);
    
    // Verify we can read the preloaded values through the CPU
    assert_eq!(cpu.memory.read(0x8000), 0xA9);
    assert_eq!(cpu.memory.read(0x8001), 0x42);
    assert_eq!(cpu.state.pc, 0x8000);
}

#[test]
fn test_cpu_halted_flag() {
    // Test that the halted flag can be used for execution control
    
    let memory = Memory::new();
    let mut cpu = Cpu::new(memory, 0x0000);
    
    // Initially not halted
    assert!(!cpu.halted, "CPU should not be halted initially");
    
    // Set halted flag
    cpu.halted = true;
    assert!(cpu.halted, "CPU should be halted after setting flag");
    
    // Clear halted flag
    cpu.halted = false;
    assert!(!cpu.halted, "CPU should not be halted after clearing flag");
}
