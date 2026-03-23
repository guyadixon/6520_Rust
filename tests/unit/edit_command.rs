// Unit tests for edit command functionality
// Tests the parse_hex_u8 helper function used for parsing 8-bit hexadecimal values

use cpu_6502_emulator::Emulator;

/// Test parse_hex_u8 with valid inputs (with 0x prefix)
#[test]
fn test_parse_hex_u8_with_prefix() {
    let emulator = Emulator::new("test.bin", 0x8000).unwrap();
    
    // Test various valid values with 0x prefix
    assert_eq!(emulator.parse_hex_u8("0x00").unwrap(), 0x00);
    assert_eq!(emulator.parse_hex_u8("0x42").unwrap(), 0x42);
    assert_eq!(emulator.parse_hex_u8("0xFF").unwrap(), 0xFF);
    assert_eq!(emulator.parse_hex_u8("0xAB").unwrap(), 0xAB);
    assert_eq!(emulator.parse_hex_u8("0x7F").unwrap(), 0x7F);
}

/// Test parse_hex_u8 with valid inputs (without 0x prefix)
#[test]
fn test_parse_hex_u8_without_prefix() {
    let emulator = Emulator::new("test.bin", 0x8000).unwrap();
    
    // Test various valid values without 0x prefix
    assert_eq!(emulator.parse_hex_u8("00").unwrap(), 0x00);
    assert_eq!(emulator.parse_hex_u8("42").unwrap(), 0x42);
    assert_eq!(emulator.parse_hex_u8("FF").unwrap(), 0xFF);
    assert_eq!(emulator.parse_hex_u8("AB").unwrap(), 0xAB);
    assert_eq!(emulator.parse_hex_u8("7F").unwrap(), 0x7F);
}

/// Test parse_hex_u8 with uppercase 0X prefix
#[test]
fn test_parse_hex_u8_uppercase_prefix() {
    let emulator = Emulator::new("test.bin", 0x8000).unwrap();
    
    // Test with uppercase 0X prefix
    assert_eq!(emulator.parse_hex_u8("0X42").unwrap(), 0x42);
    assert_eq!(emulator.parse_hex_u8("0XFF").unwrap(), 0xFF);
}

/// Test parse_hex_u8 with lowercase hex digits
#[test]
fn test_parse_hex_u8_lowercase() {
    let emulator = Emulator::new("test.bin", 0x8000).unwrap();
    
    // Test lowercase hex digits
    assert_eq!(emulator.parse_hex_u8("0xab").unwrap(), 0xAB);
    assert_eq!(emulator.parse_hex_u8("ff").unwrap(), 0xFF);
    assert_eq!(emulator.parse_hex_u8("0xcd").unwrap(), 0xCD);
}

/// Test parse_hex_u8 with mixed case hex digits
#[test]
fn test_parse_hex_u8_mixed_case() {
    let emulator = Emulator::new("test.bin", 0x8000).unwrap();
    
    // Test mixed case hex digits
    assert_eq!(emulator.parse_hex_u8("0xAb").unwrap(), 0xAB);
    assert_eq!(emulator.parse_hex_u8("Ff").unwrap(), 0xFF);
    assert_eq!(emulator.parse_hex_u8("0XcD").unwrap(), 0xCD);
}

/// Test parse_hex_u8 with boundary values
#[test]
fn test_parse_hex_u8_boundary_values() {
    let emulator = Emulator::new("test.bin", 0x8000).unwrap();
    
    // Test minimum value (0x00)
    assert_eq!(emulator.parse_hex_u8("0x00").unwrap(), 0x00);
    assert_eq!(emulator.parse_hex_u8("00").unwrap(), 0x00);
    assert_eq!(emulator.parse_hex_u8("0").unwrap(), 0x00);
    
    // Test maximum value (0xFF)
    assert_eq!(emulator.parse_hex_u8("0xFF").unwrap(), 0xFF);
    assert_eq!(emulator.parse_hex_u8("FF").unwrap(), 0xFF);
}

/// Test parse_hex_u8 with single digit values
#[test]
fn test_parse_hex_u8_single_digit() {
    let emulator = Emulator::new("test.bin", 0x8000).unwrap();
    
    // Test single digit values
    assert_eq!(emulator.parse_hex_u8("0").unwrap(), 0x00);
    assert_eq!(emulator.parse_hex_u8("1").unwrap(), 0x01);
    assert_eq!(emulator.parse_hex_u8("9").unwrap(), 0x09);
    assert_eq!(emulator.parse_hex_u8("A").unwrap(), 0x0A);
    assert_eq!(emulator.parse_hex_u8("F").unwrap(), 0x0F);
    assert_eq!(emulator.parse_hex_u8("0x5").unwrap(), 0x05);
}

/// Test parse_hex_u8 with whitespace
#[test]
fn test_parse_hex_u8_with_whitespace() {
    let emulator = Emulator::new("test.bin", 0x8000).unwrap();
    
    // Test with leading/trailing whitespace (should be trimmed)
    assert_eq!(emulator.parse_hex_u8(" 42 ").unwrap(), 0x42);
    assert_eq!(emulator.parse_hex_u8("  0xFF  ").unwrap(), 0xFF);
    assert_eq!(emulator.parse_hex_u8("\t0x7F\t").unwrap(), 0x7F);
}

/// Test parse_hex_u8 with out-of-range values (> 0xFF)
#[test]
fn test_parse_hex_u8_out_of_range() {
    let emulator = Emulator::new("test.bin", 0x8000).unwrap();
    
    // Test values that are too large for u8
    assert!(emulator.parse_hex_u8("100").is_err());
    assert!(emulator.parse_hex_u8("0x100").is_err());
    assert!(emulator.parse_hex_u8("FFF").is_err());
    assert!(emulator.parse_hex_u8("0xFFFF").is_err());
    assert!(emulator.parse_hex_u8("1000").is_err());
}

/// Test parse_hex_u8 with invalid hex characters
#[test]
fn test_parse_hex_u8_invalid_characters() {
    let emulator = Emulator::new("test.bin", 0x8000).unwrap();
    
    // Test invalid hex characters
    assert!(emulator.parse_hex_u8("XY").is_err());
    assert!(emulator.parse_hex_u8("0xGH").is_err());
    assert!(emulator.parse_hex_u8("ZZ").is_err());
    assert!(emulator.parse_hex_u8("12G").is_err());
    assert!(emulator.parse_hex_u8("0x!@").is_err());
}

/// Test parse_hex_u8 with empty input
#[test]
fn test_parse_hex_u8_empty_input() {
    let emulator = Emulator::new("test.bin", 0x8000).unwrap();
    
    // Test empty string
    let result = emulator.parse_hex_u8("");
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("cannot be empty"));
}

/// Test parse_hex_u8 with only prefix
#[test]
fn test_parse_hex_u8_only_prefix() {
    let emulator = Emulator::new("test.bin", 0x8000).unwrap();
    
    // Test with only 0x prefix (no digits)
    let result = emulator.parse_hex_u8("0x");
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("after prefix"));
    
    // Test with only 0X prefix (no digits)
    let result = emulator.parse_hex_u8("0X");
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("after prefix"));
}

/// Test parse_hex_u8 error messages are descriptive
#[test]
fn test_parse_hex_u8_error_messages() {
    let emulator = Emulator::new("test.bin", 0x8000).unwrap();
    
    // Test that error messages contain helpful information
    let result = emulator.parse_hex_u8("XYZ");
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.contains("Invalid") || err.contains("Expected"));
    
    // Test empty input error message
    let result = emulator.parse_hex_u8("");
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.contains("empty") || err.contains("Expected"));
    assert!(err.contains("0x00-0xFF") || err.contains("Examples"));
}

/// Test parse_hex_u8 with all valid single-byte hex values
#[test]
fn test_parse_hex_u8_comprehensive() {
    let emulator = Emulator::new("test.bin", 0x8000).unwrap();
    
    // Test a comprehensive set of valid values
    let test_cases = vec![
        ("0x00", 0x00),
        ("0x01", 0x01),
        ("0x10", 0x10),
        ("0x7F", 0x7F),
        ("0x80", 0x80),
        ("0xFE", 0xFE),
        ("0xFF", 0xFF),
        ("00", 0x00),
        ("01", 0x01),
        ("10", 0x10),
        ("7F", 0x7F),
        ("80", 0x80),
        ("FE", 0xFE),
        ("FF", 0xFF),
    ];
    
    for (input, expected) in test_cases {
        let result = emulator.parse_hex_u8(input);
        assert!(result.is_ok(), "Failed to parse '{}'", input);
        assert_eq!(result.unwrap(), expected, "Incorrect parse result for '{}'", input);
    }
}

// ============================================================================
// Tests for parse_hex_u16 helper function
// ============================================================================

/// Test parse_hex_u16 with valid inputs (with 0x prefix)
#[test]
fn test_parse_hex_u16_with_prefix() {
    let emulator = Emulator::new("test.bin", 0x8000).unwrap();
    
    // Test various valid values with 0x prefix
    assert_eq!(emulator.parse_hex_u16("0x0000").unwrap(), 0x0000);
    assert_eq!(emulator.parse_hex_u16("0x8000").unwrap(), 0x8000);
    assert_eq!(emulator.parse_hex_u16("0xFFFF").unwrap(), 0xFFFF);
    assert_eq!(emulator.parse_hex_u16("0xABCD").unwrap(), 0xABCD);
    assert_eq!(emulator.parse_hex_u16("0x1234").unwrap(), 0x1234);
}

/// Test parse_hex_u16 with valid inputs (without 0x prefix)
#[test]
fn test_parse_hex_u16_without_prefix() {
    let emulator = Emulator::new("test.bin", 0x8000).unwrap();
    
    // Test various valid values without 0x prefix
    assert_eq!(emulator.parse_hex_u16("0000").unwrap(), 0x0000);
    assert_eq!(emulator.parse_hex_u16("8000").unwrap(), 0x8000);
    assert_eq!(emulator.parse_hex_u16("FFFF").unwrap(), 0xFFFF);
    assert_eq!(emulator.parse_hex_u16("ABCD").unwrap(), 0xABCD);
    assert_eq!(emulator.parse_hex_u16("1234").unwrap(), 0x1234);
}

/// Test parse_hex_u16 with uppercase 0X prefix
#[test]
fn test_parse_hex_u16_uppercase_prefix() {
    let emulator = Emulator::new("test.bin", 0x8000).unwrap();
    
    // Test with uppercase 0X prefix
    assert_eq!(emulator.parse_hex_u16("0X8000").unwrap(), 0x8000);
    assert_eq!(emulator.parse_hex_u16("0XFFFF").unwrap(), 0xFFFF);
    assert_eq!(emulator.parse_hex_u16("0X1234").unwrap(), 0x1234);
}

/// Test parse_hex_u16 with lowercase hex digits
#[test]
fn test_parse_hex_u16_lowercase() {
    let emulator = Emulator::new("test.bin", 0x8000).unwrap();
    
    // Test lowercase hex digits
    assert_eq!(emulator.parse_hex_u16("0xabcd").unwrap(), 0xABCD);
    assert_eq!(emulator.parse_hex_u16("ffff").unwrap(), 0xFFFF);
    assert_eq!(emulator.parse_hex_u16("0x1234").unwrap(), 0x1234);
}

/// Test parse_hex_u16 with mixed case hex digits
#[test]
fn test_parse_hex_u16_mixed_case() {
    let emulator = Emulator::new("test.bin", 0x8000).unwrap();
    
    // Test mixed case hex digits
    assert_eq!(emulator.parse_hex_u16("0xAbCd").unwrap(), 0xABCD);
    assert_eq!(emulator.parse_hex_u16("FfFf").unwrap(), 0xFFFF);
    assert_eq!(emulator.parse_hex_u16("0X12aB").unwrap(), 0x12AB);
}

/// Test parse_hex_u16 with boundary values
#[test]
fn test_parse_hex_u16_boundary_values() {
    let emulator = Emulator::new("test.bin", 0x8000).unwrap();
    
    // Test minimum value (0x0000)
    assert_eq!(emulator.parse_hex_u16("0x0000").unwrap(), 0x0000);
    assert_eq!(emulator.parse_hex_u16("0000").unwrap(), 0x0000);
    assert_eq!(emulator.parse_hex_u16("0").unwrap(), 0x0000);
    
    // Test maximum value (0xFFFF)
    assert_eq!(emulator.parse_hex_u16("0xFFFF").unwrap(), 0xFFFF);
    assert_eq!(emulator.parse_hex_u16("FFFF").unwrap(), 0xFFFF);
}

/// Test parse_hex_u16 with short values (1-3 digits)
#[test]
fn test_parse_hex_u16_short_values() {
    let emulator = Emulator::new("test.bin", 0x8000).unwrap();
    
    // Test short values (should be zero-padded)
    assert_eq!(emulator.parse_hex_u16("0").unwrap(), 0x0000);
    assert_eq!(emulator.parse_hex_u16("1").unwrap(), 0x0001);
    assert_eq!(emulator.parse_hex_u16("F").unwrap(), 0x000F);
    assert_eq!(emulator.parse_hex_u16("10").unwrap(), 0x0010);
    assert_eq!(emulator.parse_hex_u16("FF").unwrap(), 0x00FF);
    assert_eq!(emulator.parse_hex_u16("100").unwrap(), 0x0100);
    assert_eq!(emulator.parse_hex_u16("FFF").unwrap(), 0x0FFF);
    assert_eq!(emulator.parse_hex_u16("0x5").unwrap(), 0x0005);
    assert_eq!(emulator.parse_hex_u16("0x42").unwrap(), 0x0042);
    assert_eq!(emulator.parse_hex_u16("0x123").unwrap(), 0x0123);
}

/// Test parse_hex_u16 with whitespace
#[test]
fn test_parse_hex_u16_with_whitespace() {
    let emulator = Emulator::new("test.bin", 0x8000).unwrap();
    
    // Test with leading/trailing whitespace (should be trimmed)
    assert_eq!(emulator.parse_hex_u16(" 8000 ").unwrap(), 0x8000);
    assert_eq!(emulator.parse_hex_u16("  0xFFFF  ").unwrap(), 0xFFFF);
    assert_eq!(emulator.parse_hex_u16("\t0x1234\t").unwrap(), 0x1234);
}

/// Test parse_hex_u16 with out-of-range values (> 0xFFFF)
#[test]
fn test_parse_hex_u16_out_of_range() {
    let emulator = Emulator::new("test.bin", 0x8000).unwrap();
    
    // Test values that are too large for u16
    assert!(emulator.parse_hex_u16("10000").is_err());
    assert!(emulator.parse_hex_u16("0x10000").is_err());
    assert!(emulator.parse_hex_u16("FFFFF").is_err());
    assert!(emulator.parse_hex_u16("0xFFFFF").is_err());
    assert!(emulator.parse_hex_u16("100000").is_err());
}

/// Test parse_hex_u16 with invalid hex characters
#[test]
fn test_parse_hex_u16_invalid_characters() {
    let emulator = Emulator::new("test.bin", 0x8000).unwrap();
    
    // Test invalid hex characters
    assert!(emulator.parse_hex_u16("XYZW").is_err());
    assert!(emulator.parse_hex_u16("0xGHIJ").is_err());
    assert!(emulator.parse_hex_u16("ZZZZ").is_err());
    assert!(emulator.parse_hex_u16("1234G").is_err());
    assert!(emulator.parse_hex_u16("0x!@#$").is_err());
}

/// Test parse_hex_u16 with empty input
#[test]
fn test_parse_hex_u16_empty_input() {
    let emulator = Emulator::new("test.bin", 0x8000).unwrap();
    
    // Test empty string
    let result = emulator.parse_hex_u16("");
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("cannot be empty"));
}

/// Test parse_hex_u16 with only prefix
#[test]
fn test_parse_hex_u16_only_prefix() {
    let emulator = Emulator::new("test.bin", 0x8000).unwrap();
    
    // Test with only 0x prefix (no digits)
    let result = emulator.parse_hex_u16("0x");
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("after prefix"));
    
    // Test with only 0X prefix (no digits)
    let result = emulator.parse_hex_u16("0X");
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("after prefix"));
}

/// Test parse_hex_u16 error messages are descriptive
#[test]
fn test_parse_hex_u16_error_messages() {
    let emulator = Emulator::new("test.bin", 0x8000).unwrap();
    
    // Test that error messages contain helpful information
    let result = emulator.parse_hex_u16("XYZW");
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.contains("Invalid") || err.contains("Expected"));
    
    // Test empty input error message
    let result = emulator.parse_hex_u16("");
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.contains("empty") || err.contains("Expected"));
    assert!(err.contains("0x0000-0xFFFF") || err.contains("Examples"));
}

/// Test parse_hex_u16 with comprehensive set of valid values
#[test]
fn test_parse_hex_u16_comprehensive() {
    let emulator = Emulator::new("test.bin", 0x8000).unwrap();
    
    // Test a comprehensive set of valid values
    let test_cases = vec![
        ("0x0000", 0x0000),
        ("0x0001", 0x0001),
        ("0x0010", 0x0010),
        ("0x0100", 0x0100),
        ("0x1000", 0x1000),
        ("0x7FFF", 0x7FFF),
        ("0x8000", 0x8000),
        ("0xFFFE", 0xFFFE),
        ("0xFFFF", 0xFFFF),
        ("0000", 0x0000),
        ("0001", 0x0001),
        ("0010", 0x0010),
        ("0100", 0x0100),
        ("1000", 0x1000),
        ("7FFF", 0x7FFF),
        ("8000", 0x8000),
        ("FFFE", 0xFFFE),
        ("FFFF", 0xFFFF),
    ];
    
    for (input, expected) in test_cases {
        let result = emulator.parse_hex_u16(input);
        assert!(result.is_ok(), "Failed to parse '{}'", input);
        assert_eq!(result.unwrap(), expected, "Incorrect parse result for '{}'", input);
    }
}

/// Test parse_hex_u16 with typical PC and memory addresses
#[test]
fn test_parse_hex_u16_typical_addresses() {
    let emulator = Emulator::new("test.bin", 0x8000).unwrap();
    
    // Test typical 6502 addresses
    assert_eq!(emulator.parse_hex_u16("0x0200").unwrap(), 0x0200); // Zero page stack area
    assert_eq!(emulator.parse_hex_u16("0x0600").unwrap(), 0x0600); // Common program start
    assert_eq!(emulator.parse_hex_u16("0x8000").unwrap(), 0x8000); // ROM start
    assert_eq!(emulator.parse_hex_u16("0xC000").unwrap(), 0xC000); // Common ROM location
    assert_eq!(emulator.parse_hex_u16("0xFFFC").unwrap(), 0xFFFC); // Reset vector
    assert_eq!(emulator.parse_hex_u16("0xFFFE").unwrap(), 0xFFFE); // IRQ vector
}

// ============================================================================
// Tests for edit_register method
// ============================================================================

/// Test edit_register with accumulator (A)
#[test]
fn test_edit_register_accumulator() {
    let mut emulator = Emulator::new("test.bin", 0x8000).unwrap();
    
    // Initial value should be 0x00
    assert_eq!(emulator.cpu.state.a, 0x00);
    
    // Edit accumulator to 0x42
    let result = emulator.edit_register("A", "0x42");
    assert!(result.is_ok());
    assert_eq!(emulator.cpu.state.a, 0x42);
    
    // Edit accumulator to 0xFF
    let result = emulator.edit_register("A", "FF");
    assert!(result.is_ok());
    assert_eq!(emulator.cpu.state.a, 0xFF);
    
    // Edit accumulator to 0x00
    let result = emulator.edit_register("A", "0");
    assert!(result.is_ok());
    assert_eq!(emulator.cpu.state.a, 0x00);
}

/// Test edit_register with X register
#[test]
fn test_edit_register_x() {
    let mut emulator = Emulator::new("test.bin", 0x8000).unwrap();
    
    // Initial value should be 0x00
    assert_eq!(emulator.cpu.state.x, 0x00);
    
    // Edit X register to 0x10
    let result = emulator.edit_register("X", "0x10");
    assert!(result.is_ok());
    assert_eq!(emulator.cpu.state.x, 0x10);
    
    // Edit X register to 0xAB
    let result = emulator.edit_register("X", "AB");
    assert!(result.is_ok());
    assert_eq!(emulator.cpu.state.x, 0xAB);
}

/// Test edit_register with Y register
#[test]
fn test_edit_register_y() {
    let mut emulator = Emulator::new("test.bin", 0x8000).unwrap();
    
    // Initial value should be 0x00
    assert_eq!(emulator.cpu.state.y, 0x00);
    
    // Edit Y register to 0x20
    let result = emulator.edit_register("Y", "0x20");
    assert!(result.is_ok());
    assert_eq!(emulator.cpu.state.y, 0x20);
    
    // Edit Y register to 0xCD
    let result = emulator.edit_register("Y", "CD");
    assert!(result.is_ok());
    assert_eq!(emulator.cpu.state.y, 0xCD);
}

/// Test edit_register with stack pointer (SP)
#[test]
fn test_edit_register_sp() {
    let mut emulator = Emulator::new("test.bin", 0x8000).unwrap();
    
    // Initial value should be 0xFF
    assert_eq!(emulator.cpu.state.sp, 0xFF);
    
    // Edit SP to 0xFD
    let result = emulator.edit_register("SP", "0xFD");
    assert!(result.is_ok());
    assert_eq!(emulator.cpu.state.sp, 0xFD);
    
    // Edit SP to 0x00
    let result = emulator.edit_register("SP", "00");
    assert!(result.is_ok());
    assert_eq!(emulator.cpu.state.sp, 0x00);
}

/// Test edit_register with program counter (PC)
#[test]
fn test_edit_register_pc() {
    let mut emulator = Emulator::new("test.bin", 0x8000).unwrap();
    
    // Initial value should be 0x8000
    assert_eq!(emulator.cpu.state.pc, 0x8000);
    
    // Edit PC to 0x9000
    let result = emulator.edit_register("PC", "0x9000");
    assert!(result.is_ok());
    assert_eq!(emulator.cpu.state.pc, 0x9000);
    
    // Edit PC to 0xFFFF
    let result = emulator.edit_register("PC", "FFFF");
    assert!(result.is_ok());
    assert_eq!(emulator.cpu.state.pc, 0xFFFF);
    
    // Edit PC to 0x0000
    let result = emulator.edit_register("PC", "0");
    assert!(result.is_ok());
    assert_eq!(emulator.cpu.state.pc, 0x0000);
}

/// Test edit_register with case-insensitive register names
#[test]
fn test_edit_register_case_insensitive() {
    let mut emulator = Emulator::new("test.bin", 0x8000).unwrap();
    
    // Test lowercase register names
    assert!(emulator.edit_register("a", "0x42").is_ok());
    assert_eq!(emulator.cpu.state.a, 0x42);
    
    assert!(emulator.edit_register("x", "0x10").is_ok());
    assert_eq!(emulator.cpu.state.x, 0x10);
    
    assert!(emulator.edit_register("y", "0x20").is_ok());
    assert_eq!(emulator.cpu.state.y, 0x20);
    
    assert!(emulator.edit_register("sp", "0xFD").is_ok());
    assert_eq!(emulator.cpu.state.sp, 0xFD);
    
    assert!(emulator.edit_register("pc", "0x9000").is_ok());
    assert_eq!(emulator.cpu.state.pc, 0x9000);
    
    // Test mixed case register names
    assert!(emulator.edit_register("A", "0x43").is_ok());
    assert_eq!(emulator.cpu.state.a, 0x43);
    
    assert!(emulator.edit_register("Sp", "0xFC").is_ok());
    assert_eq!(emulator.cpu.state.sp, 0xFC);
    
    assert!(emulator.edit_register("Pc", "0xA000").is_ok());
    assert_eq!(emulator.cpu.state.pc, 0xA000);
}

/// Test edit_register with invalid register name
#[test]
fn test_edit_register_invalid_register() {
    let mut emulator = Emulator::new("test.bin", 0x8000).unwrap();
    
    // Test invalid register names
    let result = emulator.edit_register("Z", "0x42");
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.contains("Unknown register"));
    assert!(err.contains("Z"));
    
    let result = emulator.edit_register("ABC", "0x42");
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("Unknown register"));
    
    let result = emulator.edit_register("P", "0x42");
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("Unknown register"));
    
    let result = emulator.edit_register("", "0x42");
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("Unknown register"));
}

/// Test edit_register with invalid value for 8-bit registers
#[test]
fn test_edit_register_invalid_8bit_value() {
    let mut emulator = Emulator::new("test.bin", 0x8000).unwrap();
    
    // Test out-of-range values for 8-bit registers
    let result = emulator.edit_register("A", "0x100");
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("Invalid"));
    
    let result = emulator.edit_register("X", "256");
    assert!(result.is_err());
    
    let result = emulator.edit_register("Y", "0xFFFF");
    assert!(result.is_err());
    
    let result = emulator.edit_register("SP", "1000");
    assert!(result.is_err());
}

/// Test edit_register with invalid value for PC
#[test]
fn test_edit_register_invalid_pc_value() {
    let mut emulator = Emulator::new("test.bin", 0x8000).unwrap();
    
    // Test out-of-range values for PC (16-bit)
    let result = emulator.edit_register("PC", "0x10000");
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("Invalid"));
    
    let result = emulator.edit_register("PC", "65536");
    assert!(result.is_err());
    
    let result = emulator.edit_register("PC", "FFFFF");
    assert!(result.is_err());
}

/// Test edit_register with invalid hex characters
#[test]
fn test_edit_register_invalid_hex() {
    let mut emulator = Emulator::new("test.bin", 0x8000).unwrap();
    
    // Test invalid hex characters
    let result = emulator.edit_register("A", "XY");
    assert!(result.is_err());
    
    let result = emulator.edit_register("X", "0xGH");
    assert!(result.is_err());
    
    let result = emulator.edit_register("PC", "XYZW");
    assert!(result.is_err());
}

/// Test edit_register with empty value
#[test]
fn test_edit_register_empty_value() {
    let mut emulator = Emulator::new("test.bin", 0x8000).unwrap();
    
    // Test empty value string
    let result = emulator.edit_register("A", "");
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("cannot be empty"));
    
    let result = emulator.edit_register("PC", "");
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("cannot be empty"));
}

/// Test edit_register with whitespace in value
#[test]
fn test_edit_register_whitespace_value() {
    let mut emulator = Emulator::new("test.bin", 0x8000).unwrap();
    
    // Test values with whitespace (should be trimmed)
    let result = emulator.edit_register("A", " 0x42 ");
    assert!(result.is_ok());
    assert_eq!(emulator.cpu.state.a, 0x42);
    
    let result = emulator.edit_register("PC", "  0x9000  ");
    assert!(result.is_ok());
    assert_eq!(emulator.cpu.state.pc, 0x9000);
}

/// Test edit_register with whitespace in register name
#[test]
fn test_edit_register_whitespace_register() {
    let mut emulator = Emulator::new("test.bin", 0x8000).unwrap();
    
    // Test register names with whitespace (should be trimmed)
    let result = emulator.edit_register(" A ", "0x42");
    assert!(result.is_ok());
    assert_eq!(emulator.cpu.state.a, 0x42);
    
    let result = emulator.edit_register("  X  ", "0x10");
    assert!(result.is_ok());
    assert_eq!(emulator.cpu.state.x, 0x10);
}

/// Test edit_register preserves other registers
#[test]
fn test_edit_register_preserves_others() {
    let mut emulator = Emulator::new("test.bin", 0x8000).unwrap();
    
    // Set initial values
    emulator.edit_register("A", "0x11").unwrap();
    emulator.edit_register("X", "0x22").unwrap();
    emulator.edit_register("Y", "0x33").unwrap();
    emulator.edit_register("SP", "0x44").unwrap();
    emulator.edit_register("PC", "0x5555").unwrap();
    
    // Verify all values are set
    assert_eq!(emulator.cpu.state.a, 0x11);
    assert_eq!(emulator.cpu.state.x, 0x22);
    assert_eq!(emulator.cpu.state.y, 0x33);
    assert_eq!(emulator.cpu.state.sp, 0x44);
    assert_eq!(emulator.cpu.state.pc, 0x5555);
    
    // Edit A and verify others are unchanged
    emulator.edit_register("A", "0xFF").unwrap();
    assert_eq!(emulator.cpu.state.a, 0xFF);
    assert_eq!(emulator.cpu.state.x, 0x22);
    assert_eq!(emulator.cpu.state.y, 0x33);
    assert_eq!(emulator.cpu.state.sp, 0x44);
    assert_eq!(emulator.cpu.state.pc, 0x5555);
    
    // Edit PC and verify others are unchanged
    emulator.edit_register("PC", "0xAAAA").unwrap();
    assert_eq!(emulator.cpu.state.a, 0xFF);
    assert_eq!(emulator.cpu.state.x, 0x22);
    assert_eq!(emulator.cpu.state.y, 0x33);
    assert_eq!(emulator.cpu.state.sp, 0x44);
    assert_eq!(emulator.cpu.state.pc, 0xAAAA);
}

/// Test edit_register with boundary values
#[test]
fn test_edit_register_boundary_values() {
    let mut emulator = Emulator::new("test.bin", 0x8000).unwrap();
    
    // Test minimum values (0x00 for 8-bit, 0x0000 for PC)
    assert!(emulator.edit_register("A", "0x00").is_ok());
    assert_eq!(emulator.cpu.state.a, 0x00);
    
    assert!(emulator.edit_register("X", "0").is_ok());
    assert_eq!(emulator.cpu.state.x, 0x00);
    
    assert!(emulator.edit_register("PC", "0x0000").is_ok());
    assert_eq!(emulator.cpu.state.pc, 0x0000);
    
    // Test maximum values (0xFF for 8-bit, 0xFFFF for PC)
    assert!(emulator.edit_register("A", "0xFF").is_ok());
    assert_eq!(emulator.cpu.state.a, 0xFF);
    
    assert!(emulator.edit_register("Y", "FF").is_ok());
    assert_eq!(emulator.cpu.state.y, 0xFF);
    
    assert!(emulator.edit_register("PC", "0xFFFF").is_ok());
    assert_eq!(emulator.cpu.state.pc, 0xFFFF);
}

/// Test edit_register error messages are descriptive
#[test]
fn test_edit_register_error_messages() {
    let mut emulator = Emulator::new("test.bin", 0x8000).unwrap();
    
    // Test invalid register error message
    let result = emulator.edit_register("Z", "0x42");
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.contains("Unknown register"));
    assert!(err.contains("Z"));
    assert!(err.contains("A, X, Y, SP, PC"));
    
    // Test invalid value error message
    let result = emulator.edit_register("A", "XYZ");
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.contains("Invalid"));
}

/// Test edit_register with all valid registers
#[test]
fn test_edit_register_all_registers() {
    let mut emulator = Emulator::new("test.bin", 0x8000).unwrap();
    
    // Test A register
    let result = emulator.edit_register("A", "0x42");
    assert!(result.is_ok(), "Failed to edit register A");
    assert_eq!(emulator.cpu.state.a, 0x42, "Register A has incorrect value");
    
    // Test X register
    let result = emulator.edit_register("X", "0x10");
    assert!(result.is_ok(), "Failed to edit register X");
    assert_eq!(emulator.cpu.state.x, 0x10, "Register X has incorrect value");
    
    // Test Y register
    let result = emulator.edit_register("Y", "0x20");
    assert!(result.is_ok(), "Failed to edit register Y");
    assert_eq!(emulator.cpu.state.y, 0x20, "Register Y has incorrect value");
    
    // Test SP register
    let result = emulator.edit_register("SP", "0xFD");
    assert!(result.is_ok(), "Failed to edit register SP");
    assert_eq!(emulator.cpu.state.sp, 0xFD, "Register SP has incorrect value");
    
    // Test PC register (16-bit)
    let result = emulator.edit_register("PC", "0x9000");
    assert!(result.is_ok(), "Failed to edit register PC");
    assert_eq!(emulator.cpu.state.pc, 0x9000, "Register PC has incorrect value");
}

// ============================================================================
// Tests for edit_memory method
// ============================================================================

/// Test edit_memory with valid address and value
#[test]
fn test_edit_memory_basic() {
    let mut emulator = Emulator::new("test.bin", 0x8000).unwrap();
    
    // Write 0xFF to address 0x0200
    let result = emulator.edit_memory("0x0200", "0xFF");
    assert!(result.is_ok());
    assert_eq!(emulator.cpu.memory.read(0x0200), 0xFF);
    
    // Write 0x42 to address 0x0300
    let result = emulator.edit_memory("0x0300", "0x42");
    assert!(result.is_ok());
    assert_eq!(emulator.cpu.memory.read(0x0300), 0x42);
    
    // Write 0x00 to address 0x0400
    let result = emulator.edit_memory("0x0400", "0x00");
    assert!(result.is_ok());
    assert_eq!(emulator.cpu.memory.read(0x0400), 0x00);
}

/// Test edit_memory without 0x prefix
#[test]
fn test_edit_memory_without_prefix() {
    let mut emulator = Emulator::new("test.bin", 0x8000).unwrap();
    
    // Write without 0x prefix
    let result = emulator.edit_memory("200", "42");
    assert!(result.is_ok());
    assert_eq!(emulator.cpu.memory.read(0x0200), 0x42);
    
    let result = emulator.edit_memory("300", "FF");
    assert!(result.is_ok());
    assert_eq!(emulator.cpu.memory.read(0x0300), 0xFF);
}

/// Test edit_memory with uppercase prefix
#[test]
fn test_edit_memory_uppercase_prefix() {
    let mut emulator = Emulator::new("test.bin", 0x8000).unwrap();
    
    // Write with uppercase 0X prefix
    let result = emulator.edit_memory("0X0200", "0XFF");
    assert!(result.is_ok());
    assert_eq!(emulator.cpu.memory.read(0x0200), 0xFF);
}

/// Test edit_memory with lowercase hex digits
#[test]
fn test_edit_memory_lowercase() {
    let mut emulator = Emulator::new("test.bin", 0x8000).unwrap();
    
    // Write with lowercase hex digits
    let result = emulator.edit_memory("0x0200", "0xab");
    assert!(result.is_ok());
    assert_eq!(emulator.cpu.memory.read(0x0200), 0xAB);
    
    let result = emulator.edit_memory("300", "cd");
    assert!(result.is_ok());
    assert_eq!(emulator.cpu.memory.read(0x0300), 0xCD);
}

/// Test edit_memory with mixed case hex digits
#[test]
fn test_edit_memory_mixed_case() {
    let mut emulator = Emulator::new("test.bin", 0x8000).unwrap();
    
    // Write with mixed case hex digits
    let result = emulator.edit_memory("0x0200", "0xAb");
    assert!(result.is_ok());
    assert_eq!(emulator.cpu.memory.read(0x0200), 0xAB);
    
    let result = emulator.edit_memory("0X0300", "Cd");
    assert!(result.is_ok());
    assert_eq!(emulator.cpu.memory.read(0x0300), 0xCD);
}

/// Test edit_memory with boundary addresses
#[test]
fn test_edit_memory_boundary_addresses() {
    let mut emulator = Emulator::new("test.bin", 0x8000).unwrap();
    
    // Test minimum address (0x0000)
    let result = emulator.edit_memory("0x0000", "0x42");
    assert!(result.is_ok());
    assert_eq!(emulator.cpu.memory.read(0x0000), 0x42);
    
    let result = emulator.edit_memory("0", "0x43");
    assert!(result.is_ok());
    assert_eq!(emulator.cpu.memory.read(0x0000), 0x43);
    
    // Test maximum address (0xFFFF)
    let result = emulator.edit_memory("0xFFFF", "0xFF");
    assert!(result.is_ok());
    assert_eq!(emulator.cpu.memory.read(0xFFFF), 0xFF);
    
    let result = emulator.edit_memory("FFFF", "0xFE");
    assert!(result.is_ok());
    assert_eq!(emulator.cpu.memory.read(0xFFFF), 0xFE);
}

/// Test edit_memory with boundary values
#[test]
fn test_edit_memory_boundary_values() {
    let mut emulator = Emulator::new("test.bin", 0x8000).unwrap();
    
    // Test minimum value (0x00)
    let result = emulator.edit_memory("0x0200", "0x00");
    assert!(result.is_ok());
    assert_eq!(emulator.cpu.memory.read(0x0200), 0x00);
    
    let result = emulator.edit_memory("0x0300", "0");
    assert!(result.is_ok());
    assert_eq!(emulator.cpu.memory.read(0x0300), 0x00);
    
    // Test maximum value (0xFF)
    let result = emulator.edit_memory("0x0400", "0xFF");
    assert!(result.is_ok());
    assert_eq!(emulator.cpu.memory.read(0x0400), 0xFF);
    
    let result = emulator.edit_memory("0x0500", "FF");
    assert!(result.is_ok());
    assert_eq!(emulator.cpu.memory.read(0x0500), 0xFF);
}

/// Test edit_memory with whitespace
#[test]
fn test_edit_memory_with_whitespace() {
    let mut emulator = Emulator::new("test.bin", 0x8000).unwrap();
    
    // Test with leading/trailing whitespace (should be trimmed)
    let result = emulator.edit_memory(" 0x0200 ", " 0x42 ");
    assert!(result.is_ok());
    assert_eq!(emulator.cpu.memory.read(0x0200), 0x42);
    
    let result = emulator.edit_memory("  300  ", "  FF  ");
    assert!(result.is_ok());
    assert_eq!(emulator.cpu.memory.read(0x0300), 0xFF);
}

/// Test edit_memory with invalid address (out of range)
#[test]
fn test_edit_memory_invalid_address() {
    let mut emulator = Emulator::new("test.bin", 0x8000).unwrap();
    
    // Test addresses that are too large for u16
    let result = emulator.edit_memory("0x10000", "0x42");
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("Invalid"));
    
    let result = emulator.edit_memory("65536", "0x42");
    assert!(result.is_err());
    
    let result = emulator.edit_memory("FFFFF", "0x42");
    assert!(result.is_err());
}

/// Test edit_memory with invalid value (out of range)
#[test]
fn test_edit_memory_invalid_value() {
    let mut emulator = Emulator::new("test.bin", 0x8000).unwrap();
    
    // Test values that are too large for u8
    let result = emulator.edit_memory("0x0200", "0x100");
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("Invalid"));
    
    let result = emulator.edit_memory("0x0200", "256");
    assert!(result.is_err());
    
    let result = emulator.edit_memory("0x0200", "0xFFFF");
    assert!(result.is_err());
}

/// Test edit_memory with invalid hex characters in address
#[test]
fn test_edit_memory_invalid_address_hex() {
    let mut emulator = Emulator::new("test.bin", 0x8000).unwrap();
    
    // Test invalid hex characters in address
    let result = emulator.edit_memory("XYZW", "0x42");
    assert!(result.is_err());
    
    let result = emulator.edit_memory("0xGHIJ", "0x42");
    assert!(result.is_err());
    
    let result = emulator.edit_memory("ZZZZ", "0x42");
    assert!(result.is_err());
}

/// Test edit_memory with invalid hex characters in value
#[test]
fn test_edit_memory_invalid_value_hex() {
    let mut emulator = Emulator::new("test.bin", 0x8000).unwrap();
    
    // Test invalid hex characters in value
    let result = emulator.edit_memory("0x0200", "XY");
    assert!(result.is_err());
    
    let result = emulator.edit_memory("0x0200", "0xGH");
    assert!(result.is_err());
    
    let result = emulator.edit_memory("0x0200", "ZZ");
    assert!(result.is_err());
}

/// Test edit_memory with empty address
#[test]
fn test_edit_memory_empty_address() {
    let mut emulator = Emulator::new("test.bin", 0x8000).unwrap();
    
    // Test empty address string
    let result = emulator.edit_memory("", "0x42");
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("cannot be empty"));
}

/// Test edit_memory with empty value
#[test]
fn test_edit_memory_empty_value() {
    let mut emulator = Emulator::new("test.bin", 0x8000).unwrap();
    
    // Test empty value string
    let result = emulator.edit_memory("0x0200", "");
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("cannot be empty"));
}

/// Test edit_memory preserves other memory locations
#[test]
fn test_edit_memory_preserves_others() {
    let mut emulator = Emulator::new("test.bin", 0x8000).unwrap();
    
    // Set initial values at different addresses
    emulator.edit_memory("0x0200", "0x11").unwrap();
    emulator.edit_memory("0x0201", "0x22").unwrap();
    emulator.edit_memory("0x0202", "0x33").unwrap();
    emulator.edit_memory("0x0300", "0x44").unwrap();
    
    // Verify all values are set
    assert_eq!(emulator.cpu.memory.read(0x0200), 0x11);
    assert_eq!(emulator.cpu.memory.read(0x0201), 0x22);
    assert_eq!(emulator.cpu.memory.read(0x0202), 0x33);
    assert_eq!(emulator.cpu.memory.read(0x0300), 0x44);
    
    // Edit one address and verify others are unchanged
    emulator.edit_memory("0x0200", "0xFF").unwrap();
    assert_eq!(emulator.cpu.memory.read(0x0200), 0xFF);
    assert_eq!(emulator.cpu.memory.read(0x0201), 0x22);
    assert_eq!(emulator.cpu.memory.read(0x0202), 0x33);
    assert_eq!(emulator.cpu.memory.read(0x0300), 0x44);
    
    // Edit another address and verify others are unchanged
    emulator.edit_memory("0x0300", "0xAA").unwrap();
    assert_eq!(emulator.cpu.memory.read(0x0200), 0xFF);
    assert_eq!(emulator.cpu.memory.read(0x0201), 0x22);
    assert_eq!(emulator.cpu.memory.read(0x0202), 0x33);
    assert_eq!(emulator.cpu.memory.read(0x0300), 0xAA);
}

/// Test edit_memory does not affect CPU registers
#[test]
fn test_edit_memory_preserves_registers() {
    let mut emulator = Emulator::new("test.bin", 0x8000).unwrap();
    
    // Set initial register values
    emulator.edit_register("A", "0x11").unwrap();
    emulator.edit_register("X", "0x22").unwrap();
    emulator.edit_register("Y", "0x33").unwrap();
    emulator.edit_register("SP", "0x44").unwrap();
    emulator.edit_register("PC", "0x5555").unwrap();
    
    // Verify all registers are set
    assert_eq!(emulator.cpu.state.a, 0x11);
    assert_eq!(emulator.cpu.state.x, 0x22);
    assert_eq!(emulator.cpu.state.y, 0x33);
    assert_eq!(emulator.cpu.state.sp, 0x44);
    assert_eq!(emulator.cpu.state.pc, 0x5555);
    
    // Edit memory and verify registers are unchanged
    emulator.edit_memory("0x0200", "0xFF").unwrap();
    assert_eq!(emulator.cpu.state.a, 0x11);
    assert_eq!(emulator.cpu.state.x, 0x22);
    assert_eq!(emulator.cpu.state.y, 0x33);
    assert_eq!(emulator.cpu.state.sp, 0x44);
    assert_eq!(emulator.cpu.state.pc, 0x5555);
    
    // Edit multiple memory locations and verify registers are unchanged
    emulator.edit_memory("0x0300", "0xAA").unwrap();
    emulator.edit_memory("0x0400", "0xBB").unwrap();
    assert_eq!(emulator.cpu.state.a, 0x11);
    assert_eq!(emulator.cpu.state.x, 0x22);
    assert_eq!(emulator.cpu.state.y, 0x33);
    assert_eq!(emulator.cpu.state.sp, 0x44);
    assert_eq!(emulator.cpu.state.pc, 0x5555);
}

/// Test edit_memory with typical 6502 memory addresses
#[test]
fn test_edit_memory_typical_addresses() {
    let mut emulator = Emulator::new("test.bin", 0x8000).unwrap();
    
    // Test typical 6502 memory locations
    
    // Zero page
    let result = emulator.edit_memory("0x0000", "0x42");
    assert!(result.is_ok());
    assert_eq!(emulator.cpu.memory.read(0x0000), 0x42);
    
    // Stack area (0x0100-0x01FF)
    let result = emulator.edit_memory("0x0100", "0x43");
    assert!(result.is_ok());
    assert_eq!(emulator.cpu.memory.read(0x0100), 0x43);
    
    let result = emulator.edit_memory("0x01FF", "0x44");
    assert!(result.is_ok());
    assert_eq!(emulator.cpu.memory.read(0x01FF), 0x44);
    
    // Common program area
    let result = emulator.edit_memory("0x0200", "0x45");
    assert!(result.is_ok());
    assert_eq!(emulator.cpu.memory.read(0x0200), 0x45);
    
    let result = emulator.edit_memory("0x0600", "0x46");
    assert!(result.is_ok());
    assert_eq!(emulator.cpu.memory.read(0x0600), 0x46);
    
    // ROM area
    let result = emulator.edit_memory("0x8000", "0x47");
    assert!(result.is_ok());
    assert_eq!(emulator.cpu.memory.read(0x8000), 0x47);
    
    let result = emulator.edit_memory("0xC000", "0x48");
    assert!(result.is_ok());
    assert_eq!(emulator.cpu.memory.read(0xC000), 0x48);
    
    // Interrupt vectors
    let result = emulator.edit_memory("0xFFFC", "0x49");
    assert!(result.is_ok());
    assert_eq!(emulator.cpu.memory.read(0xFFFC), 0x49);
    
    let result = emulator.edit_memory("0xFFFE", "0x4A");
    assert!(result.is_ok());
    assert_eq!(emulator.cpu.memory.read(0xFFFE), 0x4A);
}

/// Test edit_memory with consecutive addresses
#[test]
fn test_edit_memory_consecutive_addresses() {
    let mut emulator = Emulator::new("test.bin", 0x8000).unwrap();
    
    // Write consecutive bytes
    for i in 0..16 {
        let addr = format!("0x{:04X}", 0x0200 + i);
        let value = format!("0x{:02X}", i);
        let result = emulator.edit_memory(&addr, &value);
        assert!(result.is_ok(), "Failed to write to address {}", addr);
    }
    
    // Verify consecutive bytes
    for i in 0..16 {
        let addr = 0x0200 + i;
        assert_eq!(emulator.cpu.memory.read(addr), i as u8, "Incorrect value at address 0x{:04X}", addr);
    }
}

/// Test edit_memory error messages are descriptive
#[test]
fn test_edit_memory_error_messages() {
    let mut emulator = Emulator::new("test.bin", 0x8000).unwrap();
    
    // Test invalid address error message
    let result = emulator.edit_memory("XYZW", "0x42");
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.contains("Invalid"));
    
    // Test invalid value error message
    let result = emulator.edit_memory("0x0200", "XYZ");
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.contains("Invalid"));
    
    // Test empty address error message
    let result = emulator.edit_memory("", "0x42");
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.contains("cannot be empty"));
    
    // Test empty value error message
    let result = emulator.edit_memory("0x0200", "");
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.contains("cannot be empty"));
}

/// Test edit_memory with all valid byte values
#[test]
fn test_edit_memory_all_byte_values() {
    let mut emulator = Emulator::new("test.bin", 0x8000).unwrap();
    
    // Test a comprehensive set of valid byte values
    let test_cases = vec![
        ("0x00", 0x00),
        ("0x01", 0x01),
        ("0x10", 0x10),
        ("0x7F", 0x7F),
        ("0x80", 0x80),
        ("0xFE", 0xFE),
        ("0xFF", 0xFF),
        ("00", 0x00),
        ("01", 0x01),
        ("10", 0x10),
        ("7F", 0x7F),
        ("80", 0x80),
        ("FE", 0xFE),
        ("FF", 0xFF),
    ];
    
    for (value_str, expected) in test_cases {
        let result = emulator.edit_memory("0x0200", value_str);
        assert!(result.is_ok(), "Failed to write value '{}'", value_str);
        assert_eq!(emulator.cpu.memory.read(0x0200), expected, "Incorrect value for '{}'", value_str);
    }
}

/// Test edit_memory can overwrite existing values
#[test]
fn test_edit_memory_overwrite() {
    let mut emulator = Emulator::new("test.bin", 0x8000).unwrap();
    
    // Write initial value
    emulator.edit_memory("0x0200", "0x42").unwrap();
    assert_eq!(emulator.cpu.memory.read(0x0200), 0x42);
    
    // Overwrite with new value
    emulator.edit_memory("0x0200", "0xFF").unwrap();
    assert_eq!(emulator.cpu.memory.read(0x0200), 0xFF);
    
    // Overwrite again
    emulator.edit_memory("0x0200", "0x00").unwrap();
    assert_eq!(emulator.cpu.memory.read(0x0200), 0x00);
    
    // Overwrite one more time
    emulator.edit_memory("0x0200", "0xAB").unwrap();
    assert_eq!(emulator.cpu.memory.read(0x0200), 0xAB);
}

// ============================================================================
// Tests for handle_edit_command method
// ============================================================================

/// Test handle_edit_command with register edit (accumulator)
#[test]
fn test_handle_edit_command_register_a() {
    let mut emulator = Emulator::new("test.bin", 0x8000).unwrap();
    
    // Set accumulator to 0x42
    let result = emulator.handle_edit_command("A 0x42");
    assert!(result.is_ok());
    assert_eq!(emulator.cpu.state.a, 0x42);
    
    // Set accumulator to 0xFF
    let result = emulator.handle_edit_command("a FF");
    assert!(result.is_ok());
    assert_eq!(emulator.cpu.state.a, 0xFF);
}

/// Test handle_edit_command with register edit (X register)
#[test]
fn test_handle_edit_command_register_x() {
    let mut emulator = Emulator::new("test.bin", 0x8000).unwrap();
    
    // Set X register to 0x10
    let result = emulator.handle_edit_command("X 0x10");
    assert!(result.is_ok());
    assert_eq!(emulator.cpu.state.x, 0x10);
}

/// Test handle_edit_command with register edit (Y register)
#[test]
fn test_handle_edit_command_register_y() {
    let mut emulator = Emulator::new("test.bin", 0x8000).unwrap();
    
    // Set Y register to 0x20
    let result = emulator.handle_edit_command("Y 0x20");
    assert!(result.is_ok());
    assert_eq!(emulator.cpu.state.y, 0x20);
}

/// Test handle_edit_command with register edit (SP register)
#[test]
fn test_handle_edit_command_register_sp() {
    let mut emulator = Emulator::new("test.bin", 0x8000).unwrap();
    
    // Set SP register to 0xFD
    let result = emulator.handle_edit_command("SP 0xFD");
    assert!(result.is_ok());
    assert_eq!(emulator.cpu.state.sp, 0xFD);
}

/// Test handle_edit_command with register edit (PC register)
#[test]
fn test_handle_edit_command_register_pc() {
    let mut emulator = Emulator::new("test.bin", 0x8000).unwrap();
    
    // Set PC register to 0x9000
    let result = emulator.handle_edit_command("PC 0x9000");
    assert!(result.is_ok());
    assert_eq!(emulator.cpu.state.pc, 0x9000);
}

/// Test handle_edit_command with memory edit
#[test]
fn test_handle_edit_command_memory() {
    let mut emulator = Emulator::new("test.bin", 0x8000).unwrap();
    
    // Write 0xFF to address 0x0200
    let result = emulator.handle_edit_command("0x0200 0xFF");
    assert!(result.is_ok());
    assert_eq!(emulator.cpu.memory.read(0x0200), 0xFF);
    
    // Write 0x42 to address 0x0300
    let result = emulator.handle_edit_command("0x0300 0x42");
    assert!(result.is_ok());
    assert_eq!(emulator.cpu.memory.read(0x0300), 0x42);
}

/// Test handle_edit_command with memory edit (no prefix)
#[test]
fn test_handle_edit_command_memory_no_prefix() {
    let mut emulator = Emulator::new("test.bin", 0x8000).unwrap();
    
    // Write without 0x prefix
    let result = emulator.handle_edit_command("200 42");
    assert!(result.is_ok());
    assert_eq!(emulator.cpu.memory.read(0x0200), 0x42);
}

/// Test handle_edit_command with invalid syntax (too few arguments)
#[test]
fn test_handle_edit_command_too_few_args() {
    let mut emulator = Emulator::new("test.bin", 0x8000).unwrap();
    
    // Only one argument
    let result = emulator.handle_edit_command("A");
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("Invalid edit command format"));
}

/// Test handle_edit_command with invalid syntax (too many arguments)
#[test]
fn test_handle_edit_command_too_many_args() {
    let mut emulator = Emulator::new("test.bin", 0x8000).unwrap();
    
    // Three arguments
    let result = emulator.handle_edit_command("A 0x42 0x43");
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("Invalid edit command format"));
}

/// Test handle_edit_command with invalid syntax (no arguments)
#[test]
fn test_handle_edit_command_no_args() {
    let mut emulator = Emulator::new("test.bin", 0x8000).unwrap();
    
    // Empty string
    let result = emulator.handle_edit_command("");
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("Invalid edit command format"));
}

/// Test handle_edit_command with invalid register value
#[test]
fn test_handle_edit_command_invalid_register_value() {
    let mut emulator = Emulator::new("test.bin", 0x8000).unwrap();
    
    // Value too large for 8-bit register
    let result = emulator.handle_edit_command("A 0x100");
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("Invalid"));
}

/// Test handle_edit_command with invalid PC value
#[test]
fn test_handle_edit_command_invalid_pc_value() {
    let mut emulator = Emulator::new("test.bin", 0x8000).unwrap();
    
    // Value too large for 16-bit PC
    let result = emulator.handle_edit_command("PC 0x10000");
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("Invalid"));
}

/// Test handle_edit_command with invalid memory address
#[test]
fn test_handle_edit_command_invalid_memory_address() {
    let mut emulator = Emulator::new("test.bin", 0x8000).unwrap();
    
    // Address too large
    let result = emulator.handle_edit_command("0x10000 0x42");
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("Invalid"));
}

/// Test handle_edit_command with invalid memory value
#[test]
fn test_handle_edit_command_invalid_memory_value() {
    let mut emulator = Emulator::new("test.bin", 0x8000).unwrap();
    
    // Value too large for 8-bit memory
    let result = emulator.handle_edit_command("0x0200 0x100");
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("Invalid"));
}

/// Test handle_edit_command with case insensitive register names
#[test]
fn test_handle_edit_command_case_insensitive() {
    let mut emulator = Emulator::new("test.bin", 0x8000).unwrap();
    
    // Test lowercase
    let result = emulator.handle_edit_command("a 0x42");
    assert!(result.is_ok());
    assert_eq!(emulator.cpu.state.a, 0x42);
    
    // Test uppercase
    let result = emulator.handle_edit_command("X 0x10");
    assert!(result.is_ok());
    assert_eq!(emulator.cpu.state.x, 0x10);
    
    // Test mixed case
    let result = emulator.handle_edit_command("Sp 0xFD");
    assert!(result.is_ok());
    assert_eq!(emulator.cpu.state.sp, 0xFD);
}

/// Test handle_edit_command with whitespace
#[test]
fn test_handle_edit_command_with_whitespace() {
    let mut emulator = Emulator::new("test.bin", 0x8000).unwrap();
    
    // Test with extra whitespace
    let result = emulator.handle_edit_command("  A   0x42  ");
    assert!(result.is_ok());
    assert_eq!(emulator.cpu.state.a, 0x42);
    
    // Test with tabs
    let result = emulator.handle_edit_command("X\t0x10");
    assert!(result.is_ok());
    assert_eq!(emulator.cpu.state.x, 0x10);
}

/// Test handle_edit_command preserves other registers when editing one
#[test]
fn test_handle_edit_command_preserves_other_registers() {
    let mut emulator = Emulator::new("test.bin", 0x8000).unwrap();
    
    // Set initial values
    emulator.handle_edit_command("A 0x11").unwrap();
    emulator.handle_edit_command("X 0x22").unwrap();
    emulator.handle_edit_command("Y 0x33").unwrap();
    
    // Edit A, verify others unchanged
    emulator.handle_edit_command("A 0xFF").unwrap();
    assert_eq!(emulator.cpu.state.a, 0xFF);
    assert_eq!(emulator.cpu.state.x, 0x22);
    assert_eq!(emulator.cpu.state.y, 0x33);
}

/// Test handle_edit_command preserves registers when editing memory
#[test]
fn test_handle_edit_command_memory_preserves_registers() {
    let mut emulator = Emulator::new("test.bin", 0x8000).unwrap();
    
    // Set initial register values
    emulator.handle_edit_command("A 0x42").unwrap();
    emulator.handle_edit_command("X 0x10").unwrap();
    
    // Edit memory
    emulator.handle_edit_command("0x0200 0xFF").unwrap();
    
    // Verify registers unchanged
    assert_eq!(emulator.cpu.state.a, 0x42);
    assert_eq!(emulator.cpu.state.x, 0x10);
    assert_eq!(emulator.cpu.memory.read(0x0200), 0xFF);
}

/// Test handle_edit_command with multiple edits
#[test]
fn test_handle_edit_command_multiple_edits() {
    let mut emulator = Emulator::new("test.bin", 0x8000).unwrap();
    
    // Perform multiple edits
    emulator.handle_edit_command("A 0x11").unwrap();
    emulator.handle_edit_command("X 0x22").unwrap();
    emulator.handle_edit_command("Y 0x33").unwrap();
    emulator.handle_edit_command("SP 0xFD").unwrap();
    emulator.handle_edit_command("PC 0x9000").unwrap();
    emulator.handle_edit_command("0x0200 0xFF").unwrap();
    emulator.handle_edit_command("0x0300 0xAA").unwrap();
    
    // Verify all edits
    assert_eq!(emulator.cpu.state.a, 0x11);
    assert_eq!(emulator.cpu.state.x, 0x22);
    assert_eq!(emulator.cpu.state.y, 0x33);
    assert_eq!(emulator.cpu.state.sp, 0xFD);
    assert_eq!(emulator.cpu.state.pc, 0x9000);
    assert_eq!(emulator.cpu.memory.read(0x0200), 0xFF);
    assert_eq!(emulator.cpu.memory.read(0x0300), 0xAA);
}

// ============================================================================
// Tests for execution mode restrictions
// ============================================================================

/// Test that edit command is available in Paused mode
#[test]
fn test_edit_command_available_in_paused_mode() {
    let mut emulator = Emulator::new("test.bin", 0x8000).unwrap();
    
    // Emulator starts in Paused mode by default
    assert_eq!(emulator.mode, cpu_6502_emulator::ExecutionMode::Paused);
    
    // Edit commands should work in Paused mode
    let result = emulator.handle_edit_command("A 0x42");
    assert!(result.is_ok(), "Edit command should work in Paused mode");
    assert_eq!(emulator.cpu.state.a, 0x42);
    
    let result = emulator.handle_edit_command("0x0200 0xFF");
    assert!(result.is_ok(), "Memory edit should work in Paused mode");
    assert_eq!(emulator.cpu.memory.read(0x0200), 0xFF);
}

/// Test that edit command is available in Stepping mode
#[test]
fn test_edit_command_available_in_stepping_mode() {
    let mut emulator = Emulator::new("test.bin", 0x8000).unwrap();
    
    // Set mode to Stepping
    emulator.mode = cpu_6502_emulator::ExecutionMode::Stepping;
    
    // Edit commands should work in Stepping mode
    let result = emulator.handle_edit_command("X 0x10");
    assert!(result.is_ok(), "Edit command should work in Stepping mode");
    assert_eq!(emulator.cpu.state.x, 0x10);
    
    let result = emulator.handle_edit_command("0x0300 0xAA");
    assert!(result.is_ok(), "Memory edit should work in Stepping mode");
    assert_eq!(emulator.cpu.memory.read(0x0300), 0xAA);
}

/// Test that edit command works in all non-Running modes
#[test]
fn test_edit_command_works_in_all_allowed_modes() {
    let mut emulator = Emulator::new("test.bin", 0x8000).unwrap();
    
    // Test in Paused mode
    emulator.mode = cpu_6502_emulator::ExecutionMode::Paused;
    assert!(emulator.handle_edit_command("A 0x11").is_ok());
    assert_eq!(emulator.cpu.state.a, 0x11);
    
    // Test in Stepping mode
    emulator.mode = cpu_6502_emulator::ExecutionMode::Stepping;
    assert!(emulator.handle_edit_command("X 0x22").is_ok());
    assert_eq!(emulator.cpu.state.x, 0x22);
}

/// Test that mode restriction applies to all register edits
#[test]
fn test_mode_restriction_applies_to_all_registers() {
    let mut emulator = Emulator::new("test.bin", 0x8000).unwrap();
    
    // All register edits should work in Paused mode
    emulator.mode = cpu_6502_emulator::ExecutionMode::Paused;
    
    assert!(emulator.handle_edit_command("A 0x11").is_ok());
    assert!(emulator.handle_edit_command("X 0x22").is_ok());
    assert!(emulator.handle_edit_command("Y 0x33").is_ok());
    assert!(emulator.handle_edit_command("SP 0x44").is_ok());
    assert!(emulator.handle_edit_command("PC 0x5555").is_ok());
    
    // Verify all edits succeeded
    assert_eq!(emulator.cpu.state.a, 0x11);
    assert_eq!(emulator.cpu.state.x, 0x22);
    assert_eq!(emulator.cpu.state.y, 0x33);
    assert_eq!(emulator.cpu.state.sp, 0x44);
    assert_eq!(emulator.cpu.state.pc, 0x5555);
}

/// Test that mode restriction applies to memory edits
#[test]
fn test_mode_restriction_applies_to_memory_edits() {
    let mut emulator = Emulator::new("test.bin", 0x8000).unwrap();
    
    // Memory edits should work in Paused mode
    emulator.mode = cpu_6502_emulator::ExecutionMode::Paused;
    
    assert!(emulator.handle_edit_command("0x0200 0xFF").is_ok());
    assert!(emulator.handle_edit_command("0x0300 0xAA").is_ok());
    assert!(emulator.handle_edit_command("0x0400 0x55").is_ok());
    
    // Verify all edits succeeded
    assert_eq!(emulator.cpu.memory.read(0x0200), 0xFF);
    assert_eq!(emulator.cpu.memory.read(0x0300), 0xAA);
    assert_eq!(emulator.cpu.memory.read(0x0400), 0x55);
}

/// Test that edit command can be used multiple times in allowed modes
#[test]
fn test_multiple_edits_in_allowed_modes() {
    let mut emulator = Emulator::new("test.bin", 0x8000).unwrap();
    
    // Perform multiple edits in Paused mode
    emulator.mode = cpu_6502_emulator::ExecutionMode::Paused;
    
    for i in 0..10 {
        let value = format!("0x{:02X}", i * 10);
        let result = emulator.handle_edit_command(&format!("A {}", value));
        assert!(result.is_ok(), "Edit {} should succeed in Paused mode", i);
    }
    
    // Switch to Stepping mode and continue editing
    emulator.mode = cpu_6502_emulator::ExecutionMode::Stepping;
    
    for i in 0..10 {
        let addr = format!("0x{:04X}", 0x0200 + i);
        let value = format!("0x{:02X}", i);
        let result = emulator.handle_edit_command(&format!("{} {}", addr, value));
        assert!(result.is_ok(), "Memory edit {} should succeed in Stepping mode", i);
    }
}

/// Test that edit command preserves mode after execution
#[test]
fn test_edit_command_preserves_mode() {
    let mut emulator = Emulator::new("test.bin", 0x8000).unwrap();
    
    // Start in Paused mode
    emulator.mode = cpu_6502_emulator::ExecutionMode::Paused;
    
    // Execute edit command
    emulator.handle_edit_command("A 0x42").unwrap();
    
    // Mode should still be Paused
    assert_eq!(emulator.mode, cpu_6502_emulator::ExecutionMode::Paused);
    
    // Switch to Stepping mode
    emulator.mode = cpu_6502_emulator::ExecutionMode::Stepping;
    
    // Execute edit command
    emulator.handle_edit_command("X 0x10").unwrap();
    
    // Mode should still be Stepping
    assert_eq!(emulator.mode, cpu_6502_emulator::ExecutionMode::Stepping);
}

/// Test that edit command works immediately after mode change
#[test]
fn test_edit_command_works_after_mode_change() {
    let mut emulator = Emulator::new("test.bin", 0x8000).unwrap();
    
    // Start in Paused mode
    emulator.mode = cpu_6502_emulator::ExecutionMode::Paused;
    assert!(emulator.handle_edit_command("A 0x11").is_ok());
    
    // Change to Stepping mode
    emulator.mode = cpu_6502_emulator::ExecutionMode::Stepping;
    assert!(emulator.handle_edit_command("A 0x22").is_ok());
    
    // Change back to Paused mode
    emulator.mode = cpu_6502_emulator::ExecutionMode::Paused;
    assert!(emulator.handle_edit_command("A 0x33").is_ok());
    
    // Verify final value
    assert_eq!(emulator.cpu.state.a, 0x33);
}

/// Test comprehensive edit command functionality in allowed modes
#[test]
fn test_comprehensive_edit_in_allowed_modes() {
    let mut emulator = Emulator::new("test.bin", 0x8000).unwrap();
    
    // Test all register types in Paused mode
    emulator.mode = cpu_6502_emulator::ExecutionMode::Paused;
    
    assert!(emulator.handle_edit_command("A 0x42").is_ok());
    assert!(emulator.handle_edit_command("X 0x10").is_ok());
    assert!(emulator.handle_edit_command("Y 0x20").is_ok());
    assert!(emulator.handle_edit_command("SP 0xFD").is_ok());
    assert!(emulator.handle_edit_command("PC 0x9000").is_ok());
    
    // Test memory edits in Stepping mode
    emulator.mode = cpu_6502_emulator::ExecutionMode::Stepping;
    
    assert!(emulator.handle_edit_command("0x0200 0xFF").is_ok());
    assert!(emulator.handle_edit_command("0x0300 0xAA").is_ok());
    assert!(emulator.handle_edit_command("0x0400 0x55").is_ok());
    
    // Verify all edits
    assert_eq!(emulator.cpu.state.a, 0x42);
    assert_eq!(emulator.cpu.state.x, 0x10);
    assert_eq!(emulator.cpu.state.y, 0x20);
    assert_eq!(emulator.cpu.state.sp, 0xFD);
    assert_eq!(emulator.cpu.state.pc, 0x9000);
    assert_eq!(emulator.cpu.memory.read(0x0200), 0xFF);
    assert_eq!(emulator.cpu.memory.read(0x0300), 0xAA);
    assert_eq!(emulator.cpu.memory.read(0x0400), 0x55);
}
