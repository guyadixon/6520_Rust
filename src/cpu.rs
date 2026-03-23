// CPU module for the 6502 CPU emulator
// Manages CPU state, registers, and instruction execution

use crate::memory::Memory;

/// Represents the reason why the CPU halted execution
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HaltReason {
    /// An invalid/unrecognized opcode was encountered
    InvalidOpcode,
    /// A BRK instruction was executed
    BrkInstruction,
    /// Execution reached the end of code
    EndOfCode,
}

/// Represents the complete state of the 6502 CPU
#[derive(Debug)]
pub struct CpuState {
    // Registers
    pub pc: u16,  // Program Counter (16-bit)
    pub a: u8,    // Accumulator (8-bit)
    pub x: u8,    // X Index Register (8-bit)
    pub y: u8,    // Y Index Register (8-bit)
    pub sp: u8,   // Stack Pointer (8-bit, points to 0x0100-0x01FF)

    // Status Register flags (NV-BDIZC format)
    pub flag_carry: bool,
    pub flag_zero: bool,
    pub flag_interrupt_disable: bool,
    pub flag_decimal: bool,
    pub flag_break: bool,
    pub flag_overflow: bool,
    pub flag_negative: bool,
}

impl CpuState {
    /// Creates a new CPU state with the program counter set to the start address
    /// All registers are initialized to 0, stack pointer to 0xFF, and all flags to false
    pub fn new(start_address: u16) -> Self {
        CpuState {
            pc: start_address,
            a: 0x00,
            x: 0x00,
            y: 0x00,
            sp: 0xFF,
            flag_carry: false,
            flag_zero: false,
            flag_interrupt_disable: false,
            flag_decimal: false,
            flag_break: false,
            flag_overflow: false,
            flag_negative: false,
        }
    }

    /// Packs the status flags into a single byte in NV-BDIZC format
    /// Bit 7: Negative, Bit 6: Overflow, Bit 5: unused (always 1)
    /// Bit 4: Break, Bit 3: Decimal, Bit 2: Interrupt Disable
    /// Bit 1: Zero, Bit 0: Carry
    pub fn get_status_byte(&self) -> u8 {
        let mut status = 0b0010_0000; // Bit 5 is always set
        if self.flag_negative { status |= 0b1000_0000; }
        if self.flag_overflow { status |= 0b0100_0000; }
        if self.flag_break { status |= 0b0001_0000; }
        if self.flag_decimal { status |= 0b0000_1000; }
        if self.flag_interrupt_disable { status |= 0b0000_0100; }
        if self.flag_zero { status |= 0b0000_0010; }
        if self.flag_carry { status |= 0b0000_0001; }
        status
    }

    /// Unpacks a status byte into individual flag fields
    pub fn set_status_byte(&mut self, value: u8) {
        self.flag_negative = (value & 0b1000_0000) != 0;
        self.flag_overflow = (value & 0b0100_0000) != 0;
        self.flag_break = (value & 0b0001_0000) != 0;
        self.flag_decimal = (value & 0b0000_1000) != 0;
        self.flag_interrupt_disable = (value & 0b0000_0100) != 0;
        self.flag_zero = (value & 0b0000_0010) != 0;
        self.flag_carry = (value & 0b0000_0001) != 0;
    }

    /// Updates the Zero and Negative flags based on the given value
    pub fn update_zero_negative(&mut self, value: u8) {
        self.flag_zero = value == 0;
        self.flag_negative = (value & 0b1000_0000) != 0;
    }
}

/// Main CPU emulator structure
#[derive(Debug)]
pub struct Cpu {
    pub state: CpuState,
    pub memory: Memory,
    pub halted: bool,
    pub halt_reason: Option<HaltReason>,
}

impl Cpu {
    /// Creates a new CPU with the given memory and start address
    pub fn new(memory: Memory, start_address: u16) -> Self {
        Cpu {
            state: CpuState::new(start_address),
            memory,
            halted: false,
            halt_reason: None,
        }
    }

    /// Executes one instruction (fetch-decode-execute cycle)
    /// 
    /// This method implements the main CPU execution loop:
    /// 1. Fetch the opcode from the current PC
    /// 2. Decode the opcode to determine instruction and addressing mode
    /// 3. Execute the instruction
    /// 4. Advance PC by the instruction length
    /// 
    /// # Returns
    /// Ok(()) on successful execution, Err with message on failure
    /// 
    /// # Requirements
    /// Validates: Requirements 4.1, 4.2, 4.3, 4.4, 4.5, 4.6, 10.2
    pub fn step(&mut self) -> Result<(), String> {
        // Check if CPU is halted
        if self.halted {
            return Err("CPU is halted".to_string());
        }

        // 1. Fetch: Read the opcode at the current PC
        let opcode = self.memory.read(self.state.pc);
        let pc_at_fetch = self.state.pc; // Save PC for error reporting

        // 2. Decode: Decode the opcode into instruction, mode, and length
        let decoded = match crate::instruction::decode_opcode(opcode) {
            Ok(decoded) => decoded,
            Err(err) => {
                // Invalid opcode encountered - set halted flag and halt reason
                self.halted = true;
                self.halt_reason = Some(HaltReason::InvalidOpcode);
                return Err(format!("{} at PC: 0x{:04X}", err, pc_at_fetch));
            }
        };

        // 3. Execute: Execute the instruction
        self.execute_instruction(decoded)?;

        // Check if execution was halted by the instruction (e.g., BRK)
        if self.halted {
            return Err(format!("Execution halted by instruction at PC: 0x{:04X}", pc_at_fetch));
        }

        // 4. Advance PC: Move PC forward by the instruction length
        self.state.pc = self.state.pc.wrapping_add(decoded.length as u16);

        Ok(())
    }
    
    /// Checks if the CPU is at the end of code (large region of zeros)
    /// 
    /// This method checks if the current PC is pointing to a large region
    /// of consecutive zeros, which typically indicates uninitialized memory
    /// or the end of the loaded program.
    /// 
    /// # Returns
    /// true if at end of code, false otherwise
    pub fn is_at_end_of_code(&self) -> bool {
        let opcode = self.memory.read(self.state.pc);
        
        // If not starting with zero, definitely not end of code
        if opcode != 0x00 {
            return false;
        }
        
        // Check for 256 consecutive zeros
        let mut zero_count = 1;
        for i in 1..256 {
            if self.memory.read(self.state.pc.wrapping_add(i)) == 0x00 {
                zero_count += 1;
            } else {
                break;
            }
        }
        
        zero_count >= 256
    }

    /// Executes a decoded instruction
    /// 
    /// This method implements the execution logic for all 6502 instructions.
    /// It updates CPU state, memory, and flags according to the instruction semantics.
    /// 
    /// # Arguments
    /// * `decoded` - The decoded instruction with addressing mode and length
    /// 
    /// # Returns
    /// Ok(()) on successful execution, Err with message on failure
    pub fn execute_instruction(&mut self, decoded: crate::instruction::DecodedInstruction) -> Result<(), String> {
        use crate::instruction::Instruction::*;
        
        match decoded.instruction {
            // Load instructions
            LDA => {
                let value = self.fetch_operand(decoded.mode);
                self.state.a = value;
                self.state.update_zero_negative(value);
            }
            
            LDX => {
                let value = self.fetch_operand(decoded.mode);
                self.state.x = value;
                self.state.update_zero_negative(value);
            }
            
            LDY => {
                let value = self.fetch_operand(decoded.mode);
                self.state.y = value;
                self.state.update_zero_negative(value);
            }
            
            // Store instructions
            STA => {
                let address = self.get_effective_address(decoded.mode);
                self.memory.write(address, self.state.a);
            }
            
            STX => {
                let address = self.get_effective_address(decoded.mode);
                self.memory.write(address, self.state.x);
            }
            
            STY => {
                let address = self.get_effective_address(decoded.mode);
                self.memory.write(address, self.state.y);
            }
            
            // Increment/Decrement instructions
            INX => {
                self.state.x = self.state.x.wrapping_add(1);
                self.state.update_zero_negative(self.state.x);
            }
            
            INY => {
                self.state.y = self.state.y.wrapping_add(1);
                self.state.update_zero_negative(self.state.y);
            }
            
            DEX => {
                self.state.x = self.state.x.wrapping_sub(1);
                self.state.update_zero_negative(self.state.x);
            }
            
            DEY => {
                self.state.y = self.state.y.wrapping_sub(1);
                self.state.update_zero_negative(self.state.y);
            }
            
            // Transfer instructions
            TAX => {
                self.state.x = self.state.a;
                self.state.update_zero_negative(self.state.x);
            }
            
            TAY => {
                self.state.y = self.state.a;
                self.state.update_zero_negative(self.state.y);
            }
            
            TXA => {
                self.state.a = self.state.x;
                self.state.update_zero_negative(self.state.a);
            }
            
            TYA => {
                self.state.a = self.state.y;
                self.state.update_zero_negative(self.state.a);
            }
            
            // Arithmetic instructions
            ADC => {
                let value = self.fetch_operand(decoded.mode);
                let carry_in = if self.state.flag_carry { 1 } else { 0 };
                
                // Perform 16-bit addition to detect carry
                let sum = (self.state.a as u16) + (value as u16) + carry_in;
                let result = sum as u8;
                
                // Update Carry flag: set if sum exceeds 255
                self.state.flag_carry = sum > 0xFF;
                
                // Update Overflow flag: set if signed overflow occurred
                // Overflow occurs when:
                // - Adding two positive numbers yields a negative result
                // - Adding two negative numbers yields a positive result
                // This can be detected by checking if the sign bits of the operands
                // are the same, but different from the sign bit of the result
                let a_sign = self.state.a & 0x80;
                let value_sign = value & 0x80;
                let result_sign = result & 0x80;
                self.state.flag_overflow = (a_sign == value_sign) && (a_sign != result_sign);
                
                // Update accumulator
                self.state.a = result;
                
                // Update Zero and Negative flags
                self.state.update_zero_negative(result);
            }
            
            SBC => {
                let value = self.fetch_operand(decoded.mode);
                // SBC is A - M - (1 - C), which is equivalent to A + ~M + C
                let carry_in = if self.state.flag_carry { 1 } else { 0 };
                
                // Perform subtraction using two's complement
                let sum = (self.state.a as u16) + (!value as u16) + carry_in;
                let result = sum as u8;
                
                // Update Carry flag: set if no borrow occurred (sum >= 256)
                self.state.flag_carry = sum > 0xFF;
                
                // Update Overflow flag: set if signed overflow occurred
                let a_sign = self.state.a & 0x80;
                let value_sign = value & 0x80;
                let result_sign = result & 0x80;
                // Overflow in subtraction: subtracting a negative from positive gives negative,
                // or subtracting a positive from negative gives positive
                self.state.flag_overflow = (a_sign != value_sign) && (a_sign != result_sign);
                
                // Update accumulator
                self.state.a = result;
                
                // Update Zero and Negative flags
                self.state.update_zero_negative(result);
            }
            
            INC => {
                let address = self.get_effective_address(decoded.mode);
                let value = self.memory.read(address);
                let result = value.wrapping_add(1);
                self.memory.write(address, result);
                self.state.update_zero_negative(result);
            }
            
            DEC => {
                let address = self.get_effective_address(decoded.mode);
                let value = self.memory.read(address);
                let result = value.wrapping_sub(1);
                self.memory.write(address, result);
                self.state.update_zero_negative(result);
            }
            
            // Logical operations
            AND => {
                let value = self.fetch_operand(decoded.mode);
                self.state.a &= value;
                self.state.update_zero_negative(self.state.a);
            }
            
            ORA => {
                let value = self.fetch_operand(decoded.mode);
                self.state.a |= value;
                self.state.update_zero_negative(self.state.a);
            }
            
            EOR => {
                let value = self.fetch_operand(decoded.mode);
                self.state.a ^= value;
                self.state.update_zero_negative(self.state.a);
            }
            
            // Shift and rotate operations
            ASL => {
                let (value, result) = if decoded.mode == crate::instruction::AddressingMode::Accumulator {
                    let value = self.state.a;
                    let result = value << 1;
                    self.state.a = result;
                    (value, result)
                } else {
                    let address = self.get_effective_address(decoded.mode);
                    let value = self.memory.read(address);
                    let result = value << 1;
                    self.memory.write(address, result);
                    (value, result)
                };
                // Carry flag gets bit 7 of original value
                self.state.flag_carry = (value & 0x80) != 0;
                self.state.update_zero_negative(result);
            }
            
            LSR => {
                let (value, result) = if decoded.mode == crate::instruction::AddressingMode::Accumulator {
                    let value = self.state.a;
                    let result = value >> 1;
                    self.state.a = result;
                    (value, result)
                } else {
                    let address = self.get_effective_address(decoded.mode);
                    let value = self.memory.read(address);
                    let result = value >> 1;
                    self.memory.write(address, result);
                    (value, result)
                };
                // Carry flag gets bit 0 of original value
                self.state.flag_carry = (value & 0x01) != 0;
                self.state.update_zero_negative(result);
            }
            
            ROL => {
                let carry_in = if self.state.flag_carry { 1 } else { 0 };
                let (value, result) = if decoded.mode == crate::instruction::AddressingMode::Accumulator {
                    let value = self.state.a;
                    let result = (value << 1) | carry_in;
                    self.state.a = result;
                    (value, result)
                } else {
                    let address = self.get_effective_address(decoded.mode);
                    let value = self.memory.read(address);
                    let result = (value << 1) | carry_in;
                    self.memory.write(address, result);
                    (value, result)
                };
                // Carry flag gets bit 7 of original value
                self.state.flag_carry = (value & 0x80) != 0;
                self.state.update_zero_negative(result);
            }
            
            ROR => {
                let carry_in = if self.state.flag_carry { 0x80 } else { 0 };
                let (value, result) = if decoded.mode == crate::instruction::AddressingMode::Accumulator {
                    let value = self.state.a;
                    let result = (value >> 1) | carry_in;
                    self.state.a = result;
                    (value, result)
                } else {
                    let address = self.get_effective_address(decoded.mode);
                    let value = self.memory.read(address);
                    let result = (value >> 1) | carry_in;
                    self.memory.write(address, result);
                    (value, result)
                };
                // Carry flag gets bit 0 of original value
                self.state.flag_carry = (value & 0x01) != 0;
                self.state.update_zero_negative(result);
            }
            
            // Comparison operations
            CMP => {
                let value = self.fetch_operand(decoded.mode);
                let result = self.state.a.wrapping_sub(value);
                // Carry is set if A >= M (no borrow)
                self.state.flag_carry = self.state.a >= value;
                self.state.update_zero_negative(result);
            }
            
            CPX => {
                let value = self.fetch_operand(decoded.mode);
                let result = self.state.x.wrapping_sub(value);
                // Carry is set if X >= M (no borrow)
                self.state.flag_carry = self.state.x >= value;
                self.state.update_zero_negative(result);
            }
            
            CPY => {
                let value = self.fetch_operand(decoded.mode);
                let result = self.state.y.wrapping_sub(value);
                // Carry is set if Y >= M (no borrow)
                self.state.flag_carry = self.state.y >= value;
                self.state.update_zero_negative(result);
            }
            
            // Branch instructions
            BCC => {
                if !self.state.flag_carry {
                    self.branch(decoded.mode);
                }
            }
            
            BCS => {
                if self.state.flag_carry {
                    self.branch(decoded.mode);
                }
            }
            
            BEQ => {
                if self.state.flag_zero {
                    self.branch(decoded.mode);
                }
            }
            
            BMI => {
                if self.state.flag_negative {
                    self.branch(decoded.mode);
                }
            }
            
            BNE => {
                if !self.state.flag_zero {
                    self.branch(decoded.mode);
                }
            }
            
            BPL => {
                if !self.state.flag_negative {
                    self.branch(decoded.mode);
                }
            }
            
            BVC => {
                if !self.state.flag_overflow {
                    self.branch(decoded.mode);
                }
            }
            
            BVS => {
                if self.state.flag_overflow {
                    self.branch(decoded.mode);
                }
            }
            
            // Jump and subroutine instructions
            JMP => {
                let address = self.get_effective_address(decoded.mode);
                // JMP doesn't advance PC normally - it sets PC directly
                // We need to subtract the instruction length that will be added later
                self.state.pc = address.wrapping_sub(decoded.length as u16);
            }
            
            JSR => {
                // Push return address (PC + 2) onto stack
                // JSR is 3 bytes, so return address is PC + 2 (points to last byte of JSR)
                let return_addr = self.state.pc.wrapping_add(2);
                self.push_word(return_addr);
                
                // Jump to subroutine address
                let address = self.get_effective_address(decoded.mode);
                self.state.pc = address.wrapping_sub(decoded.length as u16);
            }
            
            RTS => {
                // Pull return address from stack and add 1
                let return_addr = self.pull_word();
                // RTS returns to the address + 1 (after the JSR instruction)
                self.state.pc = return_addr.wrapping_add(1).wrapping_sub(decoded.length as u16);
            }
            
            RTI => {
                // Pull status register from stack
                let status = self.pull_byte();
                self.state.set_status_byte(status);
                
                // Pull program counter from stack
                let pc = self.pull_word();
                self.state.pc = pc.wrapping_sub(decoded.length as u16);
            }
            
            // Stack operations
            PHA => {
                self.push_byte(self.state.a);
            }
            
            PHP => {
                // PHP pushes the status register with B flag set
                let status = self.state.get_status_byte() | 0b0001_0000;
                self.push_byte(status);
            }
            
            PLA => {
                let value = self.pull_byte();
                self.state.a = value;
                self.state.update_zero_negative(value);
            }
            
            PLP => {
                let status = self.pull_byte();
                self.state.set_status_byte(status);
            }
            
            TSX => {
                self.state.x = self.state.sp;
                self.state.update_zero_negative(self.state.x);
            }
            
            TXS => {
                self.state.sp = self.state.x;
                // TXS does not affect flags
            }
            
            // Flag manipulation instructions
            CLC => {
                self.state.flag_carry = false;
            }
            
            CLD => {
                self.state.flag_decimal = false;
            }
            
            CLI => {
                self.state.flag_interrupt_disable = false;
            }
            
            CLV => {
                self.state.flag_overflow = false;
            }
            
            SEC => {
                self.state.flag_carry = true;
            }
            
            SED => {
                self.state.flag_decimal = true;
            }
            
            SEI => {
                self.state.flag_interrupt_disable = true;
            }
            
            // Miscellaneous instructions
            NOP => {
                // No operation - do nothing
            }
            
            BRK => {
                // BRK: Push PC+2, push status with B flag set, set I flag, jump to IRQ vector
                // Push PC + 2 (return address)
                let return_addr = self.state.pc.wrapping_add(2);
                self.push_word(return_addr);
                
                // Push status register with B flag set
                let status = self.state.get_status_byte() | 0b0001_0000;
                self.push_byte(status);
                
                // Set interrupt disable flag
                self.state.flag_interrupt_disable = true;
                
                // Jump to IRQ vector at 0xFFFE/0xFFFF
                let irq_vector = self.memory.read_word(0xFFFE);
                self.state.pc = irq_vector.wrapping_sub(decoded.length as u16);
                
                // Halt execution after BRK instruction
                self.halted = true;
                self.halt_reason = Some(HaltReason::BrkInstruction);
            }
        }
        
        Ok(())
    }

    /// Fetches the operand value for an instruction based on the addressing mode
    /// 
    /// For Immediate mode, returns the byte following the opcode.
    /// For all other modes, returns the byte at the effective address.
    /// 
    /// # Arguments
    /// * `mode` - The addressing mode
    /// 
    /// # Returns
    /// The operand byte value
    fn fetch_operand(&self, mode: crate::instruction::AddressingMode) -> u8 {
        use crate::instruction::AddressingMode;
        
        match mode {
            AddressingMode::Immediate => {
                // For immediate mode, the operand is the byte right after the opcode
                self.memory.read(self.state.pc.wrapping_add(1))
            }
            _ => {
                // For all other modes, get the effective address and read from it
                let address = self.get_effective_address(mode);
                self.memory.read(address)
            }
        }
    }

    /// Pushes a byte onto the stack
    /// 
    /// The stack grows downward from 0x01FF to 0x0100.
    /// Stack pointer is decremented after the push.
    fn push_byte(&mut self, value: u8) {
        let address = 0x0100 | (self.state.sp as u16);
        self.memory.write(address, value);
        self.state.sp = self.state.sp.wrapping_sub(1);
    }
    
    /// Pulls a byte from the stack
    /// 
    /// Stack pointer is incremented before the pull.
    fn pull_byte(&mut self) -> u8 {
        self.state.sp = self.state.sp.wrapping_add(1);
        let address = 0x0100 | (self.state.sp as u16);
        self.memory.read(address)
    }
    
    /// Pushes a 16-bit word onto the stack (high byte first, then low byte)
    fn push_word(&mut self, value: u16) {
        let high = (value >> 8) as u8;
        let low = (value & 0xFF) as u8;
        self.push_byte(high);
        self.push_byte(low);
    }
    
    /// Pulls a 16-bit word from the stack (low byte first, then high byte)
    fn pull_word(&mut self) -> u16 {
        let low = self.pull_byte() as u16;
        let high = self.pull_byte() as u16;
        (high << 8) | low
    }
    
    /// Performs a branch by updating the PC with a relative offset
    /// 
    /// This is used by all branch instructions (BCC, BCS, BEQ, etc.)
    /// The offset is a signed 8-bit value relative to PC + 2
    fn branch(&mut self, _mode: crate::instruction::AddressingMode) {
        let offset = self.memory.read(self.state.pc.wrapping_add(1)) as i8;
        // Branch offset is relative to PC + 2 (after the branch instruction)
        let target = self.state.pc.wrapping_add(2).wrapping_add(offset as i16 as u16);
        // Set PC to target - 2 because step() will add 2 (instruction length)
        self.state.pc = target.wrapping_sub(2);
    }

    /// Calculates the effective memory address for a given addressing mode
    /// 
    /// This method handles all 6502 addressing modes with correct wrapping behavior:
    /// - Zero Page,X/Y wraps at 0xFF (stays in zero page)
    /// - Absolute,X/Y does not wrap (full 16-bit addition)
    /// - Indexed Indirect wraps the zero page pointer
    /// - Indirect Indexed does not wrap the final address
    /// 
    /// # Arguments
    /// * `mode` - The addressing mode to resolve
    /// 
    /// # Returns
    /// The effective 16-bit memory address
    pub fn get_effective_address(&self, mode: crate::instruction::AddressingMode) -> u16 {
        use crate::instruction::AddressingMode::*;
        
        match mode {
            // Immediate: operand is the byte following the opcode
            // Return PC + 1 (address of the operand byte)
            Immediate => self.state.pc.wrapping_add(1),
            
            // Zero Page: address is the byte following the opcode (0x00-0xFF)
            ZeroPage => {
                let addr = self.memory.read(self.state.pc.wrapping_add(1));
                addr as u16
            }
            
            // Zero Page,X: zero page address + X register (wraps at 0xFF)
            ZeroPageX => {
                let base = self.memory.read(self.state.pc.wrapping_add(1));
                // Wrapping add keeps result in zero page (0x00-0xFF)
                base.wrapping_add(self.state.x) as u16
            }
            
            // Zero Page,Y: zero page address + Y register (wraps at 0xFF)
            ZeroPageY => {
                let base = self.memory.read(self.state.pc.wrapping_add(1));
                // Wrapping add keeps result in zero page (0x00-0xFF)
                base.wrapping_add(self.state.y) as u16
            }
            
            // Absolute: 16-bit address in next two bytes (little-endian)
            Absolute => {
                self.memory.read_word(self.state.pc.wrapping_add(1))
            }
            
            // Absolute,X: absolute address + X register (no wrapping)
            AbsoluteX => {
                let base = self.memory.read_word(self.state.pc.wrapping_add(1));
                // Full 16-bit addition with wrapping
                base.wrapping_add(self.state.x as u16)
            }
            
            // Absolute,Y: absolute address + Y register (no wrapping)
            AbsoluteY => {
                let base = self.memory.read_word(self.state.pc.wrapping_add(1));
                // Full 16-bit addition with wrapping
                base.wrapping_add(self.state.y as u16)
            }
            
            // Indirect: address stored at the 16-bit address in operand
            // Used only by JMP instruction
            Indirect => {
                let ptr = self.memory.read_word(self.state.pc.wrapping_add(1));
                // Read the actual address from the pointer location
                // Note: 6502 has a bug where if ptr is at page boundary (e.g., 0x01FF),
                // it wraps within the page instead of crossing to next page
                // For now, we implement the bug-free version
                self.memory.read_word(ptr)
            }
            
            // Indexed Indirect (Indirect,X): address at (zero page + X)
            // The zero page pointer wraps at 0xFF
            IndexedIndirect => {
                let base = self.memory.read(self.state.pc.wrapping_add(1));
                // Add X to zero page address with wrapping
                let ptr = base.wrapping_add(self.state.x) as u16;
                // Read the actual address from the zero page pointer
                self.memory.read_word(ptr)
            }
            
            // Indirect Indexed (Indirect),Y: address at (zero page) + Y
            // The zero page pointer does not wrap, but the final address does
            IndirectIndexed => {
                let ptr = self.memory.read(self.state.pc.wrapping_add(1)) as u16;
                // Read base address from zero page
                let base = self.memory.read_word(ptr);
                // Add Y register to base address (full 16-bit addition)
                base.wrapping_add(self.state.y as u16)
            }
            
            // Relative: signed 8-bit offset from PC (for branches)
            // Used by branch instructions
            Relative => {
                let offset = self.memory.read(self.state.pc.wrapping_add(1)) as i8;
                // PC + 2 (after the branch instruction) + signed offset
                let base = self.state.pc.wrapping_add(2);
                // Add the signed offset (handles both positive and negative)
                base.wrapping_add(offset as i16 as u16)
            }
            
            // Implied and Accumulator modes don't have effective addresses
            // These should not be called with get_effective_address
            Implied | Accumulator => {
                // Return 0 as a placeholder - these modes don't access memory
                0
            }
        }
    }

    /// Displays the current CPU state including registers, flags, and current instruction
    /// 
    /// This method formats and prints:
    /// - Program Counter (PC), Accumulator (A), X, Y, and Stack Pointer (SP) in hexadecimal
    /// - All status flags with labels (Carry, Zero, Interrupt Disable, Decimal, Break, Overflow, Negative)
    /// - The current instruction being executed at PC
    /// 
    /// # Requirements
    /// Validates: Requirements 5.1, 5.2, 5.3, 5.4, 5.5, 5.6, 5.7
    pub fn display_state(&self) {
        // Display registers in hexadecimal format
        // SP is displayed as full stack address (0x0100 | sp) for clarity
        println!("PC: 0x{:04X}  A: 0x{:02X}  X: 0x{:02X}  Y: 0x{:02X}  SP: 0x{:04X}",
            self.state.pc, self.state.a, self.state.x, self.state.y, 0x0100 | (self.state.sp as u16));
        
        // Display flags with labels
        println!("Flags: N V - B D I Z C");
        println!("       {} {} {} {} {} {} {} {}",
            if self.state.flag_negative { '1' } else { '0' },
            if self.state.flag_overflow { '1' } else { '0' },
            '1', // Bit 5 is always 1
            if self.state.flag_break { '1' } else { '0' },
            if self.state.flag_decimal { '1' } else { '0' },
            if self.state.flag_interrupt_disable { '1' } else { '0' },
            if self.state.flag_zero { '1' } else { '0' },
            if self.state.flag_carry { '1' } else { '0' });
        
        // Display current instruction
        let opcode = self.memory.read(self.state.pc);
        match crate::instruction::decode_opcode(opcode) {
            Ok(decoded) => {
                let instruction_str = self.format_instruction(decoded);
                println!("Instruction: {}", instruction_str);
            }
            Err(_) => {
                println!("Instruction: Invalid opcode 0x{:02X}", opcode);
            }
        }
        println!(); // Empty line for readability
    }

    /// Formats an instruction for display with its operands
    /// 
    /// This helper method creates a human-readable string representation
    /// of the instruction at the current PC, including operands based on
    /// the addressing mode.
    /// 
    /// # Arguments
    /// * `decoded` - The decoded instruction with addressing mode and length
    /// 
    /// # Returns
    /// A formatted string representing the instruction
    fn format_instruction(&self, decoded: crate::instruction::DecodedInstruction) -> String {
        use crate::instruction::AddressingMode::*;
        
        // Get the instruction mnemonic
        let mnemonic = format!("{:?}", decoded.instruction);
        
        // Format operands based on addressing mode
        let operands = match decoded.mode {
            Implied => String::new(),
            
            Accumulator => "A".to_string(),
            
            Immediate => {
                let value = self.memory.read(self.state.pc.wrapping_add(1));
                format!("#${:02X}", value)
            }
            
            ZeroPage => {
                let addr = self.memory.read(self.state.pc.wrapping_add(1));
                format!("${:02X}", addr)
            }
            
            ZeroPageX => {
                let addr = self.memory.read(self.state.pc.wrapping_add(1));
                format!("${:02X},X", addr)
            }
            
            ZeroPageY => {
                let addr = self.memory.read(self.state.pc.wrapping_add(1));
                format!("${:02X},Y", addr)
            }
            
            Relative => {
                let offset = self.memory.read(self.state.pc.wrapping_add(1)) as i8;
                let target = self.state.pc.wrapping_add(2).wrapping_add(offset as i16 as u16);
                format!("${:04X}", target)
            }
            
            Absolute => {
                let addr = self.memory.read_word(self.state.pc.wrapping_add(1));
                format!("${:04X}", addr)
            }
            
            AbsoluteX => {
                let addr = self.memory.read_word(self.state.pc.wrapping_add(1));
                format!("${:04X},X", addr)
            }
            
            AbsoluteY => {
                let addr = self.memory.read_word(self.state.pc.wrapping_add(1));
                format!("${:04X},Y", addr)
            }
            
            Indirect => {
                let addr = self.memory.read_word(self.state.pc.wrapping_add(1));
                format!("(${:04X})", addr)
            }
            
            IndexedIndirect => {
                let addr = self.memory.read(self.state.pc.wrapping_add(1));
                format!("(${:02X},X)", addr)
            }
            
            IndirectIndexed => {
                let addr = self.memory.read(self.state.pc.wrapping_add(1));
                format!("(${:02X}),Y", addr)
            }
        };
        
        // Combine mnemonic and operands
        if operands.is_empty() {
            mnemonic
        } else {
            format!("{} {}", mnemonic, operands)
        }
    }
}
