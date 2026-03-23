// Unit tests for store instructions (STA, STX, STY)

use cpu_6502_emulator::cpu::Cpu;
use cpu_6502_emulator::memory::Memory;
use cpu_6502_emulator::instruction::decode_opcode;

#[test]
fn test_sta_zero_page() {
    let mut memory = Memory::new();
    // STA $50
    memory.write(0x0000, 0x85); // STA zero page opcode
    memory.write(0x0001, 0x50); // zero page address
    
    let mut cpu = Cpu::new(memory, 0x0000);
    cpu.state.a = 0x42; // Set accumulator value
    
    let decoded = decode_opcode(0x85).unwrap();
    cpu.execute_instruction(decoded).unwrap();
    
    // Check that value was written to memory
    assert_eq!(cpu.memory.read(0x0050), 0x42);
    // Check that accumulator is unchanged
    assert_eq!(cpu.state.a, 0x42);
}

#[test]
fn test_sta_zero_page_x() {
    let mut memory = Memory::new();
    // STA $50,X with X = $05
    memory.write(0x0000, 0x95); // STA zero page,X opcode
    memory.write(0x0001, 0x50); // zero page base address
    
    let mut cpu = Cpu::new(memory, 0x0000);
    cpu.state.a = 0x33;
    cpu.state.x = 0x05;
    
    let decoded = decode_opcode(0x95).unwrap();
    cpu.execute_instruction(decoded).unwrap();
    
    // Check that value was written to $55 ($50 + $05)
    assert_eq!(cpu.memory.read(0x0055), 0x33);
    assert_eq!(cpu.state.a, 0x33);
}

#[test]
fn test_sta_absolute() {
    let mut memory = Memory::new();
    // STA $1234
    memory.write(0x0000, 0x8D); // STA absolute opcode
    memory.write(0x0001, 0x34); // low byte of address
    memory.write(0x0002, 0x12); // high byte of address
    
    let mut cpu = Cpu::new(memory, 0x0000);
    cpu.state.a = 0x99;
    
    let decoded = decode_opcode(0x8D).unwrap();
    cpu.execute_instruction(decoded).unwrap();
    
    // Check that value was written to $1234
    assert_eq!(cpu.memory.read(0x1234), 0x99);
    assert_eq!(cpu.state.a, 0x99);
}

#[test]
fn test_sta_absolute_x() {
    let mut memory = Memory::new();
    // STA $2000,X with X = $10
    memory.write(0x0000, 0x9D); // STA absolute,X opcode
    memory.write(0x0001, 0x00); // low byte of base address
    memory.write(0x0002, 0x20); // high byte of base address
    
    let mut cpu = Cpu::new(memory, 0x0000);
    cpu.state.a = 0x77;
    cpu.state.x = 0x10;
    
    let decoded = decode_opcode(0x9D).unwrap();
    cpu.execute_instruction(decoded).unwrap();
    
    // Check that value was written to $2010 ($2000 + $10)
    assert_eq!(cpu.memory.read(0x2010), 0x77);
    assert_eq!(cpu.state.a, 0x77);
}

#[test]
fn test_sta_absolute_y() {
    let mut memory = Memory::new();
    // STA $3000,Y with Y = $20
    memory.write(0x0000, 0x99); // STA absolute,Y opcode
    memory.write(0x0001, 0x00); // low byte of base address
    memory.write(0x0002, 0x30); // high byte of base address
    
    let mut cpu = Cpu::new(memory, 0x0000);
    cpu.state.a = 0xAA;
    cpu.state.y = 0x20;
    
    let decoded = decode_opcode(0x99).unwrap();
    cpu.execute_instruction(decoded).unwrap();
    
    // Check that value was written to $3020 ($3000 + $20)
    assert_eq!(cpu.memory.read(0x3020), 0xAA);
    assert_eq!(cpu.state.a, 0xAA);
}

#[test]
fn test_sta_indexed_indirect() {
    let mut memory = Memory::new();
    // STA ($40,X) with X = $05
    memory.write(0x0000, 0x81); // STA indexed indirect opcode
    memory.write(0x0001, 0x40); // zero page base
    
    // Pointer at $45 (0x40 + 0x05) points to $1234
    memory.write(0x0045, 0x34); // low byte of target address
    memory.write(0x0046, 0x12); // high byte of target address
    
    let mut cpu = Cpu::new(memory, 0x0000);
    cpu.state.a = 0x88;
    cpu.state.x = 0x05;
    
    let decoded = decode_opcode(0x81).unwrap();
    cpu.execute_instruction(decoded).unwrap();
    
    // Check that value was written to $1234
    assert_eq!(cpu.memory.read(0x1234), 0x88);
    assert_eq!(cpu.state.a, 0x88);
}

#[test]
fn test_sta_indirect_indexed() {
    let mut memory = Memory::new();
    // STA ($40),Y with Y = $10
    memory.write(0x0000, 0x91); // STA indirect indexed opcode
    memory.write(0x0001, 0x40); // zero page pointer
    
    // Pointer at $40 points to $2000
    memory.write(0x0040, 0x00); // low byte of base address
    memory.write(0x0041, 0x20); // high byte of base address
    
    let mut cpu = Cpu::new(memory, 0x0000);
    cpu.state.a = 0x66;
    cpu.state.y = 0x10;
    
    let decoded = decode_opcode(0x91).unwrap();
    cpu.execute_instruction(decoded).unwrap();
    
    // Check that value was written to $2010 ($2000 + $10)
    assert_eq!(cpu.memory.read(0x2010), 0x66);
    assert_eq!(cpu.state.a, 0x66);
}

#[test]
fn test_stx_zero_page() {
    let mut memory = Memory::new();
    // STX $60
    memory.write(0x0000, 0x86); // STX zero page opcode
    memory.write(0x0001, 0x60); // zero page address
    
    let mut cpu = Cpu::new(memory, 0x0000);
    cpu.state.x = 0x55;
    
    let decoded = decode_opcode(0x86).unwrap();
    cpu.execute_instruction(decoded).unwrap();
    
    // Check that value was written to memory
    assert_eq!(cpu.memory.read(0x0060), 0x55);
    assert_eq!(cpu.state.x, 0x55);
}

#[test]
fn test_stx_zero_page_y() {
    let mut memory = Memory::new();
    // STX $50,Y with Y = $08
    memory.write(0x0000, 0x96); // STX zero page,Y opcode
    memory.write(0x0001, 0x50); // zero page base address
    
    let mut cpu = Cpu::new(memory, 0x0000);
    cpu.state.x = 0x77;
    cpu.state.y = 0x08;
    
    let decoded = decode_opcode(0x96).unwrap();
    cpu.execute_instruction(decoded).unwrap();
    
    // Check that value was written to $58 ($50 + $08)
    assert_eq!(cpu.memory.read(0x0058), 0x77);
    assert_eq!(cpu.state.x, 0x77);
}

#[test]
fn test_stx_absolute() {
    let mut memory = Memory::new();
    // STX $4000
    memory.write(0x0000, 0x8E); // STX absolute opcode
    memory.write(0x0001, 0x00); // low byte of address
    memory.write(0x0002, 0x40); // high byte of address
    
    let mut cpu = Cpu::new(memory, 0x0000);
    cpu.state.x = 0xBB;
    
    let decoded = decode_opcode(0x8E).unwrap();
    cpu.execute_instruction(decoded).unwrap();
    
    // Check that value was written to $4000
    assert_eq!(cpu.memory.read(0x4000), 0xBB);
    assert_eq!(cpu.state.x, 0xBB);
}

#[test]
fn test_sty_zero_page() {
    let mut memory = Memory::new();
    // STY $70
    memory.write(0x0000, 0x84); // STY zero page opcode
    memory.write(0x0001, 0x70); // zero page address
    
    let mut cpu = Cpu::new(memory, 0x0000);
    cpu.state.y = 0xCC;
    
    let decoded = decode_opcode(0x84).unwrap();
    cpu.execute_instruction(decoded).unwrap();
    
    // Check that value was written to memory
    assert_eq!(cpu.memory.read(0x0070), 0xCC);
    assert_eq!(cpu.state.y, 0xCC);
}

#[test]
fn test_sty_zero_page_x() {
    let mut memory = Memory::new();
    // STY $50,X with X = $0A
    memory.write(0x0000, 0x94); // STY zero page,X opcode
    memory.write(0x0001, 0x50); // zero page base address
    
    let mut cpu = Cpu::new(memory, 0x0000);
    cpu.state.y = 0xDD;
    cpu.state.x = 0x0A;
    
    let decoded = decode_opcode(0x94).unwrap();
    cpu.execute_instruction(decoded).unwrap();
    
    // Check that value was written to $5A ($50 + $0A)
    assert_eq!(cpu.memory.read(0x005A), 0xDD);
    assert_eq!(cpu.state.y, 0xDD);
}

#[test]
fn test_sty_absolute() {
    let mut memory = Memory::new();
    // STY $5000
    memory.write(0x0000, 0x8C); // STY absolute opcode
    memory.write(0x0001, 0x00); // low byte of address
    memory.write(0x0002, 0x50); // high byte of address
    
    let mut cpu = Cpu::new(memory, 0x0000);
    cpu.state.y = 0xEE;
    
    let decoded = decode_opcode(0x8C).unwrap();
    cpu.execute_instruction(decoded).unwrap();
    
    // Check that value was written to $5000
    assert_eq!(cpu.memory.read(0x5000), 0xEE);
    assert_eq!(cpu.state.y, 0xEE);
}

#[test]
fn test_store_instructions_dont_update_flags() {
    let mut memory = Memory::new();
    
    // Set up initial state with flags set
    let mut cpu = Cpu::new(memory, 0x0000);
    cpu.state.a = 0x00; // Zero value
    cpu.state.flag_zero = true;
    cpu.state.flag_negative = true;
    cpu.state.flag_carry = true;
    cpu.state.flag_overflow = true;
    
    // STA $50 - store zero value
    cpu.memory.write(0x0000, 0x85);
    cpu.memory.write(0x0001, 0x50);
    let decoded = decode_opcode(0x85).unwrap();
    cpu.execute_instruction(decoded).unwrap();
    
    // Check that flags are unchanged (store doesn't update flags)
    assert!(cpu.state.flag_zero);
    assert!(cpu.state.flag_negative);
    assert!(cpu.state.flag_carry);
    assert!(cpu.state.flag_overflow);
    
    // Verify the value was stored
    assert_eq!(cpu.memory.read(0x0050), 0x00);
}

#[test]
fn test_store_instructions_dont_affect_other_registers() {
    let mut memory = Memory::new();
    
    // Set up initial state
    let mut cpu = Cpu::new(memory, 0x0000);
    cpu.state.a = 0x11;
    cpu.state.x = 0x22;
    cpu.state.y = 0x33;
    
    // STA $50
    cpu.memory.write(0x0000, 0x85);
    cpu.memory.write(0x0001, 0x50);
    let decoded = decode_opcode(0x85).unwrap();
    cpu.execute_instruction(decoded).unwrap();
    
    // Check that all registers are unchanged
    assert_eq!(cpu.state.a, 0x11);
    assert_eq!(cpu.state.x, 0x22);
    assert_eq!(cpu.state.y, 0x33);
    
    // Verify the value was stored
    assert_eq!(cpu.memory.read(0x0050), 0x11);
}

#[test]
fn test_sta_overwrites_existing_value() {
    let mut memory = Memory::new();
    // Pre-populate memory with a value
    memory.write(0x0050, 0xFF);
    
    // STA $50
    memory.write(0x0000, 0x85);
    memory.write(0x0001, 0x50);
    
    let mut cpu = Cpu::new(memory, 0x0000);
    cpu.state.a = 0x42;
    
    let decoded = decode_opcode(0x85).unwrap();
    cpu.execute_instruction(decoded).unwrap();
    
    // Check that old value was overwritten
    assert_eq!(cpu.memory.read(0x0050), 0x42);
}

#[test]
fn test_zero_page_wrapping_stx() {
    let mut memory = Memory::new();
    // STX $FF,Y with Y = $02 should wrap to $01
    memory.write(0x0000, 0x96); // STX zero page,Y opcode
    memory.write(0x0001, 0xFF); // zero page base address
    
    let mut cpu = Cpu::new(memory, 0x0000);
    cpu.state.x = 0xAB;
    cpu.state.y = 0x02;
    
    let decoded = decode_opcode(0x96).unwrap();
    cpu.execute_instruction(decoded).unwrap();
    
    // Check that value was written to $01 (wraps in zero page)
    assert_eq!(cpu.memory.read(0x0001), 0xAB);
}
