// Unit tests for control flow and stack instructions

use cpu_6502_emulator::cpu::Cpu;
use cpu_6502_emulator::memory::Memory;

// ============================================================================
// JMP (Jump) Tests
// ============================================================================

#[test]
fn test_jmp_absolute() {
    let mut memory = Memory::new();
    memory.write(0x1000, 0x4C);  // JMP $1234 opcode
    memory.write(0x1001, 0x34);
    memory.write(0x1002, 0x12);
    
    let mut cpu = Cpu::new(memory, 0x1000);
    
    cpu.step().unwrap();
    
    assert_eq!(cpu.state.pc, 0x1234);
}

#[test]
fn test_jmp_indirect() {
    let mut memory = Memory::new();
    memory.write(0x1000, 0x6C);  // JMP ($2000) opcode
    memory.write(0x1001, 0x00);
    memory.write(0x1002, 0x20);
    memory.write(0x2000, 0x34);  // Target address low byte
    memory.write(0x2001, 0x12);  // Target address high byte
    
    let mut cpu = Cpu::new(memory, 0x1000);
    
    cpu.step().unwrap();
    
    assert_eq!(cpu.state.pc, 0x1234);
}

// ============================================================================
// JSR/RTS (Subroutine) Tests
// ============================================================================

#[test]
fn test_jsr_pushes_return_address() {
    let mut memory = Memory::new();
    memory.write(0x1000, 0x20);  // JSR $1234 opcode
    memory.write(0x1001, 0x34);
    memory.write(0x1002, 0x12);
    
    let mut cpu = Cpu::new(memory, 0x1000);
    let initial_sp = cpu.state.sp;
    
    cpu.step().unwrap();
    
    assert_eq!(cpu.state.pc, 0x1234);
    assert_eq!(cpu.state.sp, initial_sp.wrapping_sub(2));  // SP decremented by 2
    
    // Check that return address (0x1002) was pushed
    // Stack grows downward: high byte pushed first, then low byte
    let return_low = cpu.memory.read(0x0100 | ((cpu.state.sp.wrapping_add(1)) as u16));
    let return_high = cpu.memory.read(0x0100 | ((cpu.state.sp.wrapping_add(2)) as u16));
    let return_addr = ((return_high as u16) << 8) | (return_low as u16);
    assert_eq!(return_addr, 0x1002);
}

#[test]
fn test_rts_returns_to_caller() {
    let mut memory = Memory::new();
    // Set up stack with return address
    memory.write(0x01FF, 0x10);  // Return address high byte
    memory.write(0x01FE, 0x02);  // Return address low byte
    memory.write(0x1000, 0x60);  // RTS opcode
    
    let mut cpu = Cpu::new(memory, 0x1000);
    cpu.state.sp = 0xFD;  // SP points below the return address
    
    cpu.step().unwrap();
    
    assert_eq!(cpu.state.pc, 0x1003);  // Return address + 1
    assert_eq!(cpu.state.sp, 0xFF);    // SP restored
}

#[test]
fn test_jsr_rts_round_trip() {
    let mut memory = Memory::new();
    // JSR at 0x1000
    memory.write(0x1000, 0x20);  // JSR $2000
    memory.write(0x1001, 0x00);
    memory.write(0x1002, 0x20);
    // RTS at 0x2000
    memory.write(0x2000, 0x60);  // RTS
    // Next instruction at 0x1003
    memory.write(0x1003, 0xEA);  // NOP
    
    let mut cpu = Cpu::new(memory, 0x1000);
    
    // Execute JSR
    cpu.step().unwrap();
    assert_eq!(cpu.state.pc, 0x2000);
    
    // Execute RTS
    cpu.step().unwrap();
    assert_eq!(cpu.state.pc, 0x1003);
}

// ============================================================================
// RTI (Return from Interrupt) Tests
// ============================================================================

#[test]
fn test_rti_restores_status_and_pc() {
    let mut memory = Memory::new();
    // Set up stack with status and return address
    memory.write(0x01FF, 0x12);  // PC high byte
    memory.write(0x01FE, 0x34);  // PC low byte
    memory.write(0x01FD, 0b1100_0011);  // Status byte (N=1, V=1, Z=1, C=1)
    memory.write(0x1000, 0x40);  // RTI opcode
    
    let mut cpu = Cpu::new(memory, 0x1000);
    cpu.state.sp = 0xFC;  // SP points below the saved data
    
    cpu.step().unwrap();
    
    assert_eq!(cpu.state.pc, 0x1234);
    assert!(cpu.state.flag_negative);
    assert!(cpu.state.flag_overflow);
    assert!(cpu.state.flag_zero);
    assert!(cpu.state.flag_carry);
}

// ============================================================================
// Stack Operations (PHA, PHP, PLA, PLP) Tests
// ============================================================================

#[test]
fn test_pha_pushes_accumulator() {
    let mut memory = Memory::new();
    memory.write(0x1000, 0x48);  // PHA opcode
    
    let mut cpu = Cpu::new(memory, 0x1000);
    cpu.state.a = 0x42;
    let initial_sp = cpu.state.sp;
    
    cpu.step().unwrap();
    
    assert_eq!(cpu.state.sp, initial_sp.wrapping_sub(1));
    assert_eq!(cpu.memory.read(0x0100 | (initial_sp as u16)), 0x42);
}

#[test]
fn test_pla_pulls_accumulator() {
    let mut memory = Memory::new();
    memory.write(0x01FF, 0x42);  // Value on stack
    memory.write(0x1000, 0x68);  // PLA opcode
    
    let mut cpu = Cpu::new(memory, 0x1000);
    cpu.state.sp = 0xFE;
    cpu.state.a = 0x00;
    
    cpu.step().unwrap();
    
    assert_eq!(cpu.state.a, 0x42);
    assert_eq!(cpu.state.sp, 0xFF);
    assert!(!cpu.state.flag_zero);
    assert!(!cpu.state.flag_negative);
}

#[test]
fn test_pla_updates_flags() {
    let mut memory = Memory::new();
    memory.write(0x01FF, 0x00);  // Zero value on stack
    memory.write(0x1000, 0x68);  // PLA opcode
    
    let mut cpu = Cpu::new(memory, 0x1000);
    cpu.state.sp = 0xFE;
    
    cpu.step().unwrap();
    
    assert_eq!(cpu.state.a, 0x00);
    assert!(cpu.state.flag_zero);
}

#[test]
fn test_php_pushes_status() {
    let mut memory = Memory::new();
    memory.write(0x1000, 0x08);  // PHP opcode
    
    let mut cpu = Cpu::new(memory, 0x1000);
    cpu.state.flag_carry = true;
    cpu.state.flag_zero = true;
    cpu.state.flag_negative = true;
    let initial_sp = cpu.state.sp;
    
    cpu.step().unwrap();
    
    assert_eq!(cpu.state.sp, initial_sp.wrapping_sub(1));
    let pushed_status = cpu.memory.read(0x0100 | (initial_sp as u16));
    // PHP sets the B flag when pushing
    assert_eq!(pushed_status & 0b1000_0011, 0b1000_0011);  // N, Z, C set
}

#[test]
fn test_php_writes_status_byte_to_memory() {
    // This test explicitly validates that PHP writes the status byte to memory
    // at the stack pointer location before decrementing SP
    let mut memory = Memory::new();
    memory.write(0x1000, 0x08);  // PHP opcode
    
    let mut cpu = Cpu::new(memory, 0x1000);
    
    // Set specific flags to create a known status byte
    cpu.state.flag_negative = true;   // Bit 7
    cpu.state.flag_overflow = true;   // Bit 6
    cpu.state.flag_decimal = true;    // Bit 3
    cpu.state.flag_interrupt_disable = true;  // Bit 2
    cpu.state.flag_zero = true;       // Bit 1
    cpu.state.flag_carry = true;      // Bit 0
    
    let initial_sp = cpu.state.sp;  // Should be 0xFF initially
    let stack_address = 0x0100 | (initial_sp as u16);  // Should be 0x01FF
    
    // Verify memory at stack location is initially zero
    assert_eq!(cpu.memory.read(stack_address), 0x00, 
        "Memory at stack location should be 0x00 before PHP");
    
    // Execute PHP
    cpu.step().unwrap();
    
    // Verify SP was decremented
    assert_eq!(cpu.state.sp, initial_sp.wrapping_sub(1), 
        "Stack pointer should be decremented after PHP");
    
    // Verify the status byte was written to memory at the original SP location
    let written_byte = cpu.memory.read(stack_address);
    
    // PHP sets the B flag (bit 4) when pushing, and bit 5 is always 1
    // Expected: NV1B DIZC = 1101 1111 = 0xDF
    assert_ne!(written_byte, 0x00, 
        "PHP should write status byte to memory at stack location");
    
    // Verify specific flags are set in the written byte
    assert_eq!(written_byte & 0b1000_0000, 0b1000_0000, "Negative flag should be set");
    assert_eq!(written_byte & 0b0100_0000, 0b0100_0000, "Overflow flag should be set");
    assert_eq!(written_byte & 0b0010_0000, 0b0010_0000, "Bit 5 should always be set");
    assert_eq!(written_byte & 0b0001_0000, 0b0001_0000, "B flag should be set by PHP");
    assert_eq!(written_byte & 0b0000_1000, 0b0000_1000, "Decimal flag should be set");
    assert_eq!(written_byte & 0b0000_0100, 0b0000_0100, "Interrupt disable flag should be set");
    assert_eq!(written_byte & 0b0000_0010, 0b0000_0010, "Zero flag should be set");
    assert_eq!(written_byte & 0b0000_0001, 0b0000_0001, "Carry flag should be set");
    
    // Verify the complete byte value
    // With all flags set: N(0x80) + V(0x40) + bit5(0x20) + B(0x10) + D(0x08) + I(0x04) + Z(0x02) + C(0x01)
    assert_eq!(written_byte, 0b1111_1111, 
        "Status byte should be 0xFF (all flags set including B flag and bit 5)");
}

#[test]
fn test_plp_pulls_status() {
    let mut memory = Memory::new();
    memory.write(0x01FF, 0b1100_0011);  // Status on stack (N=1, V=1, Z=1, C=1)
    memory.write(0x1000, 0x28);  // PLP opcode
    
    let mut cpu = Cpu::new(memory, 0x1000);
    cpu.state.sp = 0xFE;
    
    cpu.step().unwrap();
    
    assert_eq!(cpu.state.sp, 0xFF);
    assert!(cpu.state.flag_negative);
    assert!(cpu.state.flag_overflow);
    assert!(cpu.state.flag_zero);
    assert!(cpu.state.flag_carry);
}

#[test]
fn test_pha_pla_round_trip() {
    let mut memory = Memory::new();
    memory.write(0x1000, 0x48);  // PHA
    memory.write(0x1001, 0x68);  // PLA
    
    let mut cpu = Cpu::new(memory, 0x1000);
    cpu.state.a = 0x42;
    
    cpu.step().unwrap();  // PHA
    cpu.state.a = 0x00;   // Clear accumulator
    cpu.step().unwrap();  // PLA
    
    assert_eq!(cpu.state.a, 0x42);
}

// ============================================================================
// Stack Pointer Transfer (TSX, TXS) Tests
// ============================================================================

#[test]
fn test_tsx_transfers_sp_to_x() {
    let mut memory = Memory::new();
    memory.write(0x1000, 0xBA);  // TSX opcode
    
    let mut cpu = Cpu::new(memory, 0x1000);
    cpu.state.sp = 0x42;
    cpu.state.x = 0x00;
    
    cpu.step().unwrap();
    
    assert_eq!(cpu.state.x, 0x42);
    assert!(!cpu.state.flag_zero);
    assert!(!cpu.state.flag_negative);
}

#[test]
fn test_tsx_updates_flags() {
    let mut memory = Memory::new();
    memory.write(0x1000, 0xBA);  // TSX opcode
    
    let mut cpu = Cpu::new(memory, 0x1000);
    cpu.state.sp = 0x00;
    
    cpu.step().unwrap();
    
    assert_eq!(cpu.state.x, 0x00);
    assert!(cpu.state.flag_zero);
}

#[test]
fn test_txs_transfers_x_to_sp() {
    let mut memory = Memory::new();
    memory.write(0x1000, 0x9A);  // TXS opcode
    
    let mut cpu = Cpu::new(memory, 0x1000);
    cpu.state.x = 0x42;
    cpu.state.sp = 0xFF;
    
    cpu.step().unwrap();
    
    assert_eq!(cpu.state.sp, 0x42);
}

#[test]
fn test_txs_doesnt_update_flags() {
    let mut memory = Memory::new();
    memory.write(0x1000, 0x9A);  // TXS opcode
    
    let mut cpu = Cpu::new(memory, 0x1000);
    cpu.state.x = 0x00;
    cpu.state.flag_zero = false;
    cpu.state.flag_negative = false;
    
    cpu.step().unwrap();
    
    // TXS should not update flags
    assert!(!cpu.state.flag_zero);
    assert!(!cpu.state.flag_negative);
}

// ============================================================================
// BRK (Break) Tests
// ============================================================================

#[test]
fn test_brk_pushes_pc_and_status() {
    let mut memory = Memory::new();
    memory.write(0x1000, 0x00);  // BRK opcode
    memory.write(0xFFFE, 0x00);  // IRQ vector low
    memory.write(0xFFFF, 0x20);  // IRQ vector high
    
    let mut cpu = Cpu::new(memory, 0x1000);
    let initial_sp = cpu.state.sp;
    
    // BRK halts execution, so step() returns an error
    let result = cpu.step();
    assert!(result.is_err(), "BRK should halt execution");
    assert!(cpu.halted, "CPU should be halted after BRK");
    
    // Check PC jumped to IRQ vector (minus instruction length which will be added)
    // BRK is 1 byte, so PC should be at 0x2000 - 1 = 0x1FFF
    assert_eq!(cpu.state.pc, 0x1FFF);
    
    // Check interrupt disable flag is set
    assert!(cpu.state.flag_interrupt_disable);
    
    // Check stack pointer decremented by 3
    assert_eq!(cpu.state.sp, initial_sp.wrapping_sub(3));
}

#[test]
fn test_brk_stack_contents() {
    let mut memory = Memory::new();
    memory.write(0x1000, 0x00);  // BRK opcode
    memory.write(0xFFFE, 0x00);  // IRQ vector low
    memory.write(0xFFFF, 0x20);  // IRQ vector high
    
    let mut cpu = Cpu::new(memory, 0x1000);
    
    // Set some flags to verify they're pushed correctly
    cpu.state.flag_carry = true;
    cpu.state.flag_zero = false;
    cpu.state.flag_overflow = true;
    cpu.state.flag_negative = false;
    
    let initial_sp = cpu.state.sp;
    
    // BRK halts execution, so step() returns an error
    let result = cpu.step();
    assert!(result.is_err(), "BRK should halt execution");
    assert!(cpu.halted, "CPU should be halted after BRK");
    
    // Verify stack contents
    // BRK pushes: PC high byte, PC low byte, Status (with B flag set)
    
    // PC+2 should be pushed (0x1002)
    let pushed_pc_high = cpu.memory.read(0x0100 | (initial_sp as u16));
    let pushed_pc_low = cpu.memory.read(0x0100 | (initial_sp.wrapping_sub(1) as u16));
    let pushed_pc = ((pushed_pc_high as u16) << 8) | (pushed_pc_low as u16);
    assert_eq!(pushed_pc, 0x1002, "BRK should push PC+2");
    
    // Status register should be pushed with B flag set
    let pushed_status = cpu.memory.read(0x0100 | (initial_sp.wrapping_sub(2) as u16));
    
    // B flag (bit 4) should be set in pushed status
    assert_eq!(pushed_status & 0b0001_0000, 0b0001_0000, "B flag should be set in pushed status");
    
    // Verify other flags are preserved
    assert_eq!(pushed_status & 0b0000_0001, 0b0000_0001, "Carry flag should be preserved");
    assert_eq!(pushed_status & 0b0000_0010, 0b0000_0000, "Zero flag should be preserved");
    assert_eq!(pushed_status & 0b0100_0000, 0b0100_0000, "Overflow flag should be preserved");
    assert_eq!(pushed_status & 0b1000_0000, 0b0000_0000, "Negative flag should be preserved");
}

#[test]
fn test_brk_sets_interrupt_disable() {
    let mut memory = Memory::new();
    memory.write(0x1000, 0x00);  // BRK opcode
    memory.write(0xFFFE, 0x34);  // IRQ vector low
    memory.write(0xFFFF, 0x12);  // IRQ vector high
    
    let mut cpu = Cpu::new(memory, 0x1000);
    
    // Ensure interrupt disable is initially clear
    cpu.state.flag_interrupt_disable = false;
    
    // BRK halts execution, so step() returns an error
    let result = cpu.step();
    assert!(result.is_err(), "BRK should halt execution");
    assert!(cpu.halted, "CPU should be halted after BRK");
    
    // BRK should set the interrupt disable flag
    assert!(cpu.state.flag_interrupt_disable, "BRK should set interrupt disable flag");
    
    // Verify PC jumped to IRQ vector (minus instruction length which will be added)
    // BRK is 1 byte, so PC should be at 0x1234 - 1 = 0x1233
    assert_eq!(cpu.state.pc, 0x1233);
}
