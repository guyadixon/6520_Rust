// Integration tests with sample 6502 programs
// Tests end-to-end functionality of the emulator
//
// **Validates: All Requirements**

use cpu_6502_emulator::{cpu::Cpu, memory::Memory};

/// Helper function to create a CPU with a program loaded at a specific address
fn create_cpu_with_program(program: &[u8], start_address: u16) -> Cpu {
    let mut memory = Memory::new();
    
    // Load program into memory at the start address
    for (i, &byte) in program.iter().enumerate() {
        memory.write(start_address.wrapping_add(i as u16), byte);
    }
    
    Cpu::new(memory, start_address)
}

/// Helper function to execute a program for a specified number of steps
fn execute_steps(cpu: &mut Cpu, steps: usize) -> Result<(), String> {
    for _ in 0..steps {
        cpu.step()?;
    }
    Ok(())
}

#[cfg(test)]
mod simple_loop_tests {
    use super::*;

    #[test]
    fn test_simple_counting_loop() {
        // Program: Count from 0 to 5 in accumulator
        // LDX #$00      ; X = 0 (counter)
        // loop:
        // TXA           ; A = X
        // INX           ; X++
        // CPX #$05      ; Compare X with 5
        // BNE loop      ; Branch if not equal
        // NOP           ; End marker
        let program = vec![
            0xA2, 0x00,       // LDX #$00
            0x8A,             // TXA (loop starts here at 0x8002)
            0xE8,             // INX
            0xE0, 0x05,       // CPX #$05
            0xD0, 0xFA,       // BNE loop (offset -6 to go back to 0x8002)
            0xEA,             // NOP
        ];
        
        let mut cpu = create_cpu_with_program(&program, 0x8000);
        
        // Execute the program
        // Initial: LDX = 1 instruction
        // Loop iterations: X=0,1,2,3,4 (5 iterations, stops when X=5)
        // Each iteration: TXA, INX, CPX, BNE = 4 instructions
        // Final: TXA, INX, CPX (no branch), NOP = 4 instructions
        // Total: 1 + (5 * 4) + 4 = 22 instructions (not 25 as previously thought)
        execute_steps(&mut cpu, 22).expect("Program execution failed");
        
        // Verify final state
        assert_eq!(cpu.state.a, 0x04, "Accumulator should be 4 (last value before X=5)");
        assert_eq!(cpu.state.x, 0x05, "X register should be 5");
        assert!(cpu.state.flag_zero, "Zero flag should be set (5-5=0)");
        assert!(!cpu.state.flag_negative, "Negative flag should be clear");
    }

    #[test]
    fn test_countdown_loop() {
        // Program: Count down from 10 to 0
        // LDX #$0A      ; X = 10
        // loop:
        // DEX           ; X--
        // BNE loop      ; Branch if not zero
        // NOP           ; End marker
        let program = vec![
            0xA2, 0x0A,       // LDX #$0A
            0xCA,             // DEX
            0xD0, 0xFD,       // BNE loop (offset -3)
            0xEA,             // NOP
        ];
        
        let mut cpu = create_cpu_with_program(&program, 0x8000);
        
        // Execute: LDX + (10 * (DEX + BNE)) + NOP = 1 + 20 + 1 = 22 instructions
        execute_steps(&mut cpu, 22).expect("Program execution failed");
        
        // Verify final state
        assert_eq!(cpu.state.x, 0x00, "X register should be 0");
        assert!(cpu.state.flag_zero, "Zero flag should be set");
    }
}

#[cfg(test)]
mod arithmetic_tests {
    use super::*;

    #[test]
    fn test_addition_without_carry() {
        // Program: Add two numbers (5 + 3 = 8)
        // CLC           ; Clear carry
        // LDA #$05      ; A = 5
        // ADC #$03      ; A = A + 3
        // NOP           ; End marker
        let program = vec![
            0x18,             // CLC
            0xA9, 0x05,       // LDA #$05
            0x69, 0x03,       // ADC #$03
            0xEA,             // NOP
        ];
        
        let mut cpu = create_cpu_with_program(&program, 0x8000);
        execute_steps(&mut cpu, 4).expect("Program execution failed");
        
        assert_eq!(cpu.state.a, 0x08, "Accumulator should be 8");
        assert!(!cpu.state.flag_carry, "Carry flag should be clear");
        assert!(!cpu.state.flag_zero, "Zero flag should be clear");
        assert!(!cpu.state.flag_overflow, "Overflow flag should be clear");
    }

    #[test]
    fn test_addition_with_carry() {
        // Program: Add numbers that produce carry (255 + 1 = 256, wraps to 0)
        // CLC           ; Clear carry
        // LDA #$FF      ; A = 255
        // ADC #$01      ; A = A + 1 (produces carry)
        // NOP           ; End marker
        let program = vec![
            0x18,             // CLC
            0xA9, 0xFF,       // LDA #$FF
            0x69, 0x01,       // ADC #$01
            0xEA,             // NOP
        ];
        
        let mut cpu = create_cpu_with_program(&program, 0x8000);
        execute_steps(&mut cpu, 4).expect("Program execution failed");
        
        assert_eq!(cpu.state.a, 0x00, "Accumulator should wrap to 0");
        assert!(cpu.state.flag_carry, "Carry flag should be set");
        assert!(cpu.state.flag_zero, "Zero flag should be set");
    }

    #[test]
    fn test_subtraction() {
        // Program: Subtract two numbers (10 - 3 = 7)
        // SEC           ; Set carry (no borrow)
        // LDA #$0A      ; A = 10
        // SBC #$03      ; A = A - 3
        // NOP           ; End marker
        let program = vec![
            0x38,             // SEC
            0xA9, 0x0A,       // LDA #$0A
            0xE9, 0x03,       // SBC #$03
            0xEA,             // NOP
        ];
        
        let mut cpu = create_cpu_with_program(&program, 0x8000);
        execute_steps(&mut cpu, 4).expect("Program execution failed");
        
        assert_eq!(cpu.state.a, 0x07, "Accumulator should be 7");
        assert!(cpu.state.flag_carry, "Carry flag should be set (no borrow)");
    }

    #[test]
    fn test_multi_byte_addition() {
        // Program: Add two 16-bit numbers stored in memory
        // Low bytes at $00 and $02, high bytes at $01 and $03
        // Result stored at $04 (low) and $05 (high)
        // Number 1: $1234 (4660), Number 2: $5678 (22136)
        // Expected: $68AC (26796)
        let program = vec![
            0xA9, 0x34,       // LDA #$34 (low byte of num1)
            0x85, 0x00,       // STA $00
            0xA9, 0x12,       // LDA #$12 (high byte of num1)
            0x85, 0x01,       // STA $01
            0xA9, 0x78,       // LDA #$78 (low byte of num2)
            0x85, 0x02,       // STA $02
            0xA9, 0x56,       // LDA #$56 (high byte of num2)
            0x85, 0x03,       // STA $03
            // Now add: low bytes first
            0x18,             // CLC
            0xA5, 0x00,       // LDA $00 (low byte of num1)
            0x65, 0x02,       // ADC $02 (low byte of num2)
            0x85, 0x04,       // STA $04 (store low byte result)
            // Then high bytes with carry
            0xA5, 0x01,       // LDA $01 (high byte of num1)
            0x65, 0x03,       // ADC $03 (high byte of num2)
            0x85, 0x05,       // STA $05 (store high byte result)
            0xEA,             // NOP
        ];
        
        let mut cpu = create_cpu_with_program(&program, 0x8000);
        execute_steps(&mut cpu, 16).expect("Program execution failed");
        
        // Check result
        let result_low = cpu.memory.read(0x04);
        let result_high = cpu.memory.read(0x05);
        let result = ((result_high as u16) << 8) | (result_low as u16);
        
        assert_eq!(result, 0x68AC, "16-bit addition result should be 0x68AC");
    }
}

#[cfg(test)]
mod subroutine_tests {
    use super::*;

    #[test]
    fn test_simple_subroutine_call() {
        // Program: Call a subroutine that increments A
        // main:
        // LDA #$10      ; A = 16
        // JSR increment ; Call subroutine
        // NOP           ; Return here
        // BRK           ; End
        // increment:
        // CLC           ; Clear carry
        // ADC #$01      ; A = A + 1
        // RTS           ; Return
        let program = vec![
            0xA9, 0x10,       // LDA #$10 (at 0x8000)
            0x20, 0x08, 0x80, // JSR $8008 (at 0x8002)
            0xEA,             // NOP (at 0x8005)
            0x00,             // BRK (at 0x8006)
            0xEA,             // Padding (at 0x8007)
            0x18,             // CLC (at 0x8008 - subroutine)
            0x69, 0x01,       // ADC #$01 (at 0x8009)
            0x60,             // RTS (at 0x800B)
        ];
        
        let mut cpu = create_cpu_with_program(&program, 0x8000);
        
        // Execute: LDA, JSR, CLC, ADC, RTS, NOP = 6 instructions
        execute_steps(&mut cpu, 6).expect("Program execution failed");
        
        assert_eq!(cpu.state.a, 0x11, "Accumulator should be 17 (16+1)");
        // PC should be at NOP instruction (0x8005) + 1 = 0x8006
        assert_eq!(cpu.state.pc, 0x8006, "PC should return to instruction after JSR");
    }

    #[test]
    fn test_nested_subroutine_calls() {
        // Program: Call subroutine A which calls subroutine B
        // main:
        // LDA #$05      ; A = 5
        // JSR subA      ; Call subroutine A
        // NOP           ; Return here
        // BRK           ; End
        // subA:
        // JSR subB      ; Call subroutine B
        // ADC #$01      ; A = A + 1
        // RTS           ; Return to main
        // subB:
        // ADC #$02      ; A = A + 2
        // RTS           ; Return to subA
        let program = vec![
            0xA9, 0x05,       // LDA #$05 (at 0x8000)
            0x20, 0x08, 0x80, // JSR $8008 (subA) (at 0x8002)
            0xEA,             // NOP (at 0x8005)
            0x00,             // BRK (at 0x8006)
            0xEA,             // Padding (at 0x8007)
            // subA at 0x8008:
            0x20, 0x0E, 0x80, // JSR $800E (subB) (at 0x8008)
            0x69, 0x01,       // ADC #$01 (at 0x800B)
            0x60,             // RTS (at 0x800D)
            // subB at 0x800E:
            0x69, 0x02,       // ADC #$02 (at 0x800E)
            0x60,             // RTS (at 0x8010)
        ];
        
        let mut cpu = create_cpu_with_program(&program, 0x8000);
        
        // Execute: LDA, JSR subA, JSR subB, ADC #$02, RTS, ADC #$01, RTS, NOP = 8 instructions
        execute_steps(&mut cpu, 8).expect("Program execution failed");
        
        assert_eq!(cpu.state.a, 0x08, "Accumulator should be 8 (5+2+1)");
        assert_eq!(cpu.state.pc, 0x8006, "PC should return to main after nested calls");
    }

    #[test]
    fn test_subroutine_with_parameters_on_stack() {
        // Program: Pass parameter via accumulator, use stack for local storage
        // main:
        // LDA #$0A      ; A = 10 (parameter)
        // JSR double    ; Call subroutine to double it
        // NOP           ; Return here
        // double:
        // PHA           ; Save A on stack
        // CLC           ; Clear carry
        // PLA           ; Restore A
        // ADC #$0A      ; A = A + 10 (double the original value)
        // RTS           ; Return
        let program = vec![
            0xA9, 0x0A,       // LDA #$0A (at 0x8000)
            0x20, 0x06, 0x80, // JSR $8006 (double) (at 0x8002)
            0xEA,             // NOP (at 0x8005)
            // double at 0x8006:
            0x48,             // PHA (at 0x8006)
            0x18,             // CLC (at 0x8007)
            0x68,             // PLA (at 0x8008)
            0x69, 0x0A,       // ADC #$0A (at 0x8009)
            0x60,             // RTS (at 0x800B)
        ];
        
        let mut cpu = create_cpu_with_program(&program, 0x8000);
        
        // Execute: LDA, JSR, PHA, CLC, PLA, ADC, RTS, NOP = 8 instructions
        execute_steps(&mut cpu, 8).expect("Program execution failed");
        
        assert_eq!(cpu.state.a, 0x14, "Accumulator should be 20 (10*2)");
        assert_eq!(cpu.state.sp, 0xFF, "Stack pointer should be restored to 0xFF");
    }
}

#[cfg(test)]
mod memory_operations_tests {
    use super::*;

    #[test]
    fn test_memory_copy() {
        // Program: Copy 5 bytes from $0010 to $0020
        // Setup source data first
        let program = vec![
            // Initialize source data
            0xA9, 0x11,       // LDA #$11 (at 0x8000)
            0x85, 0x10,       // STA $10 (at 0x8002)
            0xA9, 0x22,       // LDA #$22 (at 0x8004)
            0x85, 0x11,       // STA $11 (at 0x8006)
            0xA9, 0x33,       // LDA #$33 (at 0x8008)
            0x85, 0x12,       // STA $12 (at 0x800A)
            0xA9, 0x44,       // LDA #$44 (at 0x800C)
            0x85, 0x13,       // STA $13 (at 0x800E)
            0xA9, 0x55,       // LDA #$55 (at 0x8010)
            0x85, 0x14,       // STA $14 (at 0x8012)
            // Copy loop
            0xA2, 0x00,       // LDX #$00 (counter) (at 0x8014)
            // loop: (starts at 0x8016)
            0xB5, 0x10,       // LDA $10,X (at 0x8016)
            0x95, 0x20,       // STA $20,X (at 0x8018)
            0xE8,             // INX (at 0x801A)
            0xE0, 0x05,       // CPX #$05 (at 0x801B)
            0xD0, 0xF7,       // BNE loop (at 0x801D, offset -9 to go back to 0x8016)
            0xEA,             // NOP (at 0x801F)
        ];
        
        let mut cpu = create_cpu_with_program(&program, 0x8000);
        
        // Execute all instructions
        // Init: 10 instructions (5 * (LDA + STA))
        // Loop setup: 1 instruction (LDX)
        // Loop: 5 iterations * 4 instructions (LDA, STA, INX, CPX, BNE) = 20
        // Final iteration: LDA, STA, INX, CPX (no branch), NOP = 5
        // Total: 10 + 1 + 20 + 5 = 36 instructions
        execute_steps(&mut cpu, 36).expect("Program execution failed");
        
        // Verify copied data
        for i in 0..5 {
            let source = cpu.memory.read(0x10 + i);
            let dest = cpu.memory.read(0x20 + i);
            assert_eq!(source, dest, "Byte {} should be copied correctly", i);
        }
        
        assert_eq!(cpu.memory.read(0x20), 0x11);
        assert_eq!(cpu.memory.read(0x21), 0x22);
        assert_eq!(cpu.memory.read(0x22), 0x33);
        assert_eq!(cpu.memory.read(0x23), 0x44);
        assert_eq!(cpu.memory.read(0x24), 0x55);
    }

    #[test]
    fn test_array_sum() {
        // Program: Sum an array of 4 bytes
        // Array at $0010-$0013, result in A
        let program = vec![
            // Initialize array
            0xA9, 0x05,       // LDA #$05 (at 0x8000)
            0x85, 0x10,       // STA $10 (at 0x8002)
            0xA9, 0x0A,       // LDA #$0A (at 0x8004)
            0x85, 0x11,       // STA $11 (at 0x8006)
            0xA9, 0x0F,       // LDA #$0F (at 0x8008)
            0x85, 0x12,       // STA $12 (at 0x800A)
            0xA9, 0x14,       // LDA #$14 (at 0x800C)
            0x85, 0x13,       // STA $13 (at 0x800E)
            // Sum loop
            0xA9, 0x00,       // LDA #$00 (accumulator = 0) (at 0x8010)
            0xA2, 0x00,       // LDX #$00 (counter) (at 0x8012)
            0x18,             // CLC (at 0x8014)
            // loop: (starts at 0x8015)
            0x75, 0x10,       // ADC $10,X (at 0x8015)
            0xE8,             // INX (at 0x8017)
            0xE0, 0x04,       // CPX #$04 (at 0x8018)
            0xD0, 0xF9,       // BNE loop (at 0x801A, offset -7 to go back to 0x8015)
            0xEA,             // NOP (at 0x801C)
        ];
        
        let mut cpu = create_cpu_with_program(&program, 0x8000);
        
        // Execute all instructions
        // Init: 8 instructions (4 * (LDA + STA))
        // Setup: 3 instructions (LDA #$00, LDX #$00, CLC)
        // Loop: 4 iterations * 3 instructions (ADC, INX, CPX, BNE) = 12
        // Final: ADC, INX, CPX (no branch), NOP = 4
        // Total: 8 + 3 + 12 + 4 = 27 instructions
        execute_steps(&mut cpu, 27).expect("Program execution failed");
        
        // Verify sum: 5 + 10 + 15 + 20 = 50 (0x32)
        assert_eq!(cpu.state.a, 0x32, "Sum should be 50");
    }
}

#[cfg(test)]
mod complex_program_tests {
    use super::*;

    #[test]
    fn test_fibonacci_sequence() {
        // Program: Calculate first 8 Fibonacci numbers
        // Store results at $0010-$0017
        // F(0)=0, F(1)=1, F(n)=F(n-1)+F(n-2)
        let program = vec![
            // Initialize first two values
            0xA9, 0x00,       // LDA #$00 (F(0) = 0)
            0x85, 0x10,       // STA $10
            0xA9, 0x01,       // LDA #$01 (F(1) = 1)
            0x85, 0x11,       // STA $11
            // Loop to calculate remaining values
            0xA2, 0x02,       // LDX #$02 (start at index 2)
            // loop:
            0x18,             // CLC
            0xB5, 0x0E,       // LDA $0E,X (F(n-2))
            0x75, 0x0F,       // ADC $0F,X (F(n-1))
            0x95, 0x10,       // STA $10,X (F(n))
            0xE8,             // INX
            0xE0, 0x08,       // CPX #$08
            0xD0, 0xF5,       // BNE loop
            0xEA,             // NOP
        ];
        
        let mut cpu = create_cpu_with_program(&program, 0x8000);
        
        // Execute all instructions
        execute_steps(&mut cpu, 43).expect("Program execution failed");
        
        // Verify Fibonacci sequence: 0, 1, 1, 2, 3, 5, 8, 13
        let expected = vec![0x00, 0x01, 0x01, 0x02, 0x03, 0x05, 0x08, 0x0D];
        for (i, &expected_val) in expected.iter().enumerate() {
            let actual = cpu.memory.read(0x10 + i as u16);
            assert_eq!(actual, expected_val, "Fibonacci F({}) should be {}", i, expected_val);
        }
    }

    #[test]
    fn test_bit_manipulation() {
        // Program: Test various bit manipulation operations
        // Set specific bits, clear specific bits, toggle bits
        let program = vec![
            // Start with 0b10101010
            0xA9, 0xAA,       // LDA #$AA
            // Set bit 0 (OR with 0b00000001)
            0x09, 0x01,       // ORA #$01
            0x85, 0x10,       // STA $10 (should be 0xAB)
            // Clear bit 1 (AND with 0b11111101)
            0x29, 0xFD,       // AND #$FD
            0x85, 0x11,       // STA $11 (should be 0xA9)
            // Toggle bit 7 (EOR with 0b10000000)
            0x49, 0x80,       // EOR #$80
            0x85, 0x12,       // STA $12 (should be 0x29)
            0xEA,             // NOP
        ];
        
        let mut cpu = create_cpu_with_program(&program, 0x8000);
        
        execute_steps(&mut cpu, 8).expect("Program execution failed");
        
        assert_eq!(cpu.memory.read(0x10), 0xAB, "After setting bit 0");
        assert_eq!(cpu.memory.read(0x11), 0xA9, "After clearing bit 1");
        assert_eq!(cpu.memory.read(0x12), 0x29, "After toggling bit 7");
    }
}

#[cfg(test)]
mod state_verification_tests {
    use super::*;

    #[test]
    fn test_all_flags_manipulation() {
        // Program: Test all flag manipulation instructions
        let program = vec![
            // Set all flags
            0x38,             // SEC (set carry)
            0xF8,             // SED (set decimal)
            0x78,             // SEI (set interrupt disable)
            // Load a value to set overflow (requires ADC)
            0xA9, 0x7F,       // LDA #$7F (127)
            0x69, 0x01,       // ADC #$01 (128, sets overflow and negative)
            // Now clear flags
            0x18,             // CLC (clear carry)
            0xD8,             // CLD (clear decimal)
            0x58,             // CLI (clear interrupt disable)
            0xB8,             // CLV (clear overflow)
            0xEA,             // NOP
        ];
        
        let mut cpu = create_cpu_with_program(&program, 0x8000);
        
        // Execute first 3 instructions (set flags)
        execute_steps(&mut cpu, 3).expect("Program execution failed");
        assert!(cpu.state.flag_carry, "Carry should be set");
        assert!(cpu.state.flag_decimal, "Decimal should be set");
        assert!(cpu.state.flag_interrupt_disable, "Interrupt disable should be set");
        
        // Execute ADC to set overflow
        execute_steps(&mut cpu, 2).expect("Program execution failed");
        assert!(cpu.state.flag_overflow, "Overflow should be set");
        assert!(cpu.state.flag_negative, "Negative should be set");
        
        // Execute clear instructions
        execute_steps(&mut cpu, 4).expect("Program execution failed");
        assert!(!cpu.state.flag_carry, "Carry should be clear");
        assert!(!cpu.state.flag_decimal, "Decimal should be clear");
        assert!(!cpu.state.flag_interrupt_disable, "Interrupt disable should be clear");
        assert!(!cpu.state.flag_overflow, "Overflow should be clear");
    }

    #[test]
    fn test_register_transfers() {
        // Program: Test all register transfer instructions
        let program = vec![
            0xA9, 0x42,       // LDA #$42
            0xAA,             // TAX (X = 0x42)
            0xA8,             // TAY (Y = 0x42)
            0xA9, 0x00,       // LDA #$00 (clear A)
            0x8A,             // TXA (A = 0x42)
            0xA9, 0x00,       // LDA #$00 (clear A again)
            0x98,             // TYA (A = 0x42)
            0xA2, 0xFF,       // LDX #$FF
            0x9A,             // TXS (SP = 0xFF)
            0xBA,             // TSX (X = 0xFF)
            0xEA,             // NOP
        ];
        
        let mut cpu = create_cpu_with_program(&program, 0x8000);
        
        execute_steps(&mut cpu, 11).expect("Program execution failed");
        
        assert_eq!(cpu.state.a, 0x42, "A should be 0x42 after TYA");
        assert_eq!(cpu.state.x, 0xFF, "X should be 0xFF after TSX");
        assert_eq!(cpu.state.y, 0x42, "Y should be 0x42");
        assert_eq!(cpu.state.sp, 0xFF, "SP should be 0xFF");
    }
}
