// Property-based tests for opcode decoding
// Tests universal properties across all possible opcode values

use cpu_6502_emulator::instruction::decode_opcode;
use proptest::prelude::*;
use std::collections::HashSet;

// Complete list of all official 6502 opcodes
// Based on the MOS Technology 6502 Programming Manual
const VALID_OPCODES: &[u8] = &[
    // LDA - Load Accumulator
    0xA9, 0xA5, 0xB5, 0xAD, 0xBD, 0xB9, 0xA1, 0xB1,
    // LDX - Load X Register
    0xA2, 0xA6, 0xB6, 0xAE, 0xBE,
    // LDY - Load Y Register
    0xA0, 0xA4, 0xB4, 0xAC, 0xBC,
    // STA - Store Accumulator
    0x85, 0x95, 0x8D, 0x9D, 0x99, 0x81, 0x91,
    // STX - Store X Register
    0x86, 0x96, 0x8E,
    // STY - Store Y Register
    0x84, 0x94, 0x8C,
    // ADC - Add with Carry
    0x69, 0x65, 0x75, 0x6D, 0x7D, 0x79, 0x61, 0x71,
    // SBC - Subtract with Carry
    0xE9, 0xE5, 0xF5, 0xED, 0xFD, 0xF9, 0xE1, 0xF1,
    // INC - Increment Memory
    0xE6, 0xF6, 0xEE, 0xFE,
    // INX - Increment X Register
    0xE8,
    // INY - Increment Y Register
    0xC8,
    // DEC - Decrement Memory
    0xC6, 0xD6, 0xCE, 0xDE,
    // DEX - Decrement X Register
    0xCA,
    // DEY - Decrement Y Register
    0x88,
    // AND - Logical AND
    0x29, 0x25, 0x35, 0x2D, 0x3D, 0x39, 0x21, 0x31,
    // ORA - Logical OR
    0x09, 0x05, 0x15, 0x0D, 0x1D, 0x19, 0x01, 0x11,
    // EOR - Logical Exclusive OR
    0x49, 0x45, 0x55, 0x4D, 0x5D, 0x59, 0x41, 0x51,
    // ASL - Arithmetic Shift Left
    0x0A, 0x06, 0x16, 0x0E, 0x1E,
    // LSR - Logical Shift Right
    0x4A, 0x46, 0x56, 0x4E, 0x5E,
    // ROL - Rotate Left
    0x2A, 0x26, 0x36, 0x2E, 0x3E,
    // ROR - Rotate Right
    0x6A, 0x66, 0x76, 0x6E, 0x7E,
    // CMP - Compare Accumulator
    0xC9, 0xC5, 0xD5, 0xCD, 0xDD, 0xD9, 0xC1, 0xD1,
    // CPX - Compare X Register
    0xE0, 0xE4, 0xEC,
    // CPY - Compare Y Register
    0xC0, 0xC4, 0xCC,
    // BCC - Branch if Carry Clear
    0x90,
    // BCS - Branch if Carry Set
    0xB0,
    // BEQ - Branch if Equal (Zero Set)
    0xF0,
    // BMI - Branch if Minus (Negative Set)
    0x30,
    // BNE - Branch if Not Equal (Zero Clear)
    0xD0,
    // BPL - Branch if Plus (Negative Clear)
    0x10,
    // BVC - Branch if Overflow Clear
    0x50,
    // BVS - Branch if Overflow Set
    0x70,
    // JMP - Jump
    0x4C, 0x6C,
    // JSR - Jump to Subroutine
    0x20,
    // RTS - Return from Subroutine
    0x60,
    // RTI - Return from Interrupt
    0x40,
    // PHA - Push Accumulator
    0x48,
    // PHP - Push Processor Status
    0x08,
    // PLA - Pull Accumulator
    0x68,
    // PLP - Pull Processor Status
    0x28,
    // TSX - Transfer Stack Pointer to X
    0xBA,
    // TXS - Transfer X to Stack Pointer
    0x9A,
    // CLC - Clear Carry Flag
    0x18,
    // CLD - Clear Decimal Flag
    0xD8,
    // CLI - Clear Interrupt Disable Flag
    0x58,
    // CLV - Clear Overflow Flag
    0xB8,
    // SEC - Set Carry Flag
    0x38,
    // SED - Set Decimal Flag
    0xF8,
    // SEI - Set Interrupt Disable Flag
    0x78,
    // TAX - Transfer Accumulator to X
    0xAA,
    // TAY - Transfer Accumulator to Y
    0xA8,
    // TXA - Transfer X to Accumulator
    0x8A,
    // TYA - Transfer Y to Accumulator
    0x98,
    // NOP - No Operation
    0xEA,
    // BRK - Break
    0x00,
];

// Property 5: Opcode Decoding Completeness
// For all official 6502 opcodes (as defined in the 6502 specification),
// the decoder should return a valid instruction and addressing mode without error.
// **Validates: Requirements 4.2, 8.1-8.11**
proptest! {
    #[test]
    fn prop_all_valid_opcodes_decode_successfully(opcode_index in 0usize..VALID_OPCODES.len()) {
        let opcode = VALID_OPCODES[opcode_index];
        let result = decode_opcode(opcode);
        
        prop_assert!(result.is_ok(),
            "Valid opcode 0x{:02X} should decode successfully but got error: {:?}",
            opcode, result.err());
        
        // Verify the decoded instruction has valid properties
        let decoded = result.unwrap();
        
        // Length should be 1, 2, or 3 bytes
        prop_assert!(decoded.length >= 1 && decoded.length <= 3,
            "Opcode 0x{:02X} has invalid length: {} (should be 1-3)",
            opcode, decoded.length);
    }
}

// Property 6: Invalid Opcode Rejection
// For any byte value that is not an official 6502 opcode,
// the decoder should return an error.
// **Validates: Requirements 4.6**
proptest! {
    #[test]
    fn prop_invalid_opcodes_return_error(opcode in 0u8..=0xFF) {
        // Create a HashSet of valid opcodes for fast lookup
        let valid_set: HashSet<u8> = VALID_OPCODES.iter().copied().collect();
        
        let result = decode_opcode(opcode);
        
        if valid_set.contains(&opcode) {
            // Valid opcode should decode successfully
            prop_assert!(result.is_ok(),
                "Valid opcode 0x{:02X} should decode successfully but got error: {:?}",
                opcode, result.err());
        } else {
            // Invalid opcode should return an error
            prop_assert!(result.is_err(),
                "Invalid opcode 0x{:02X} should return an error but decoded successfully",
                opcode);
            
            // Verify the error message contains the opcode value
            let error_msg = result.unwrap_err();
            prop_assert!(error_msg.contains(&format!("0x{:02X}", opcode)),
                "Error message should contain the opcode value 0x{:02X}, got: {}",
                opcode, error_msg);
        }
    }
}

// Additional property: Verify the count of valid opcodes
// This ensures our VALID_OPCODES list is complete
#[test]
fn test_valid_opcode_count() {
    // This implementation supports 149 official 6502 opcodes
    // Note: The full 6502 has 151 official opcodes, but this implementation
    // does not include BIT (0x24, 0x2C) as it's not in the requirements
    assert_eq!(VALID_OPCODES.len(), 149,
        "Expected 149 valid opcodes, found {}", VALID_OPCODES.len());
    
    // Verify no duplicates in the list
    let unique_opcodes: HashSet<u8> = VALID_OPCODES.iter().copied().collect();
    assert_eq!(unique_opcodes.len(), VALID_OPCODES.len(),
        "VALID_OPCODES contains duplicates");
}

// Additional property: Verify all valid opcodes are unique
proptest! {
    #[test]
    fn prop_valid_opcodes_are_unique(
        index1 in 0usize..VALID_OPCODES.len(),
        index2 in 0usize..VALID_OPCODES.len()
    ) {
        if index1 != index2 {
            prop_assert_ne!(VALID_OPCODES[index1], VALID_OPCODES[index2],
                "Found duplicate opcode 0x{:02X} at indices {} and {}",
                VALID_OPCODES[index1], index1, index2);
        }
    }
}
