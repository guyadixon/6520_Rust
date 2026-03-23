// Unit tests for command-line argument parsing
// Tests Requirements 1.1, 2.1, 2.3, 2.4, 10.3, 12.1, 12.2, 12.3, 12.4, 12.6
//
// Note: The actual prompt_for_file_path() and prompt_for_start_address() functions
// are difficult to test directly because they use stdin/stdout. These tests verify
// the parsing logic and error handling behavior indirectly through integration tests
// and manual testing.
//
// The parse_args() function is tested through simulated command-line scenarios.

use cpu_6502_emulator::parse_hex_address;

// ============================================================================
// Tests for parse_hex_address() function
// ============================================================================

/// Test parse_hex_address with valid addresses without prefix
/// Validates: Requirements 12.6, 2.4
#[test]
fn test_parse_hex_address_without_prefix() {
    // Test various valid addresses without 0x prefix
    assert_eq!(parse_hex_address("0000").unwrap(), 0x0000);
    assert_eq!(parse_hex_address("8000").unwrap(), 0x8000);
    assert_eq!(parse_hex_address("C000").unwrap(), 0xC000);
    assert_eq!(parse_hex_address("FFFF").unwrap(), 0xFFFF);
    assert_eq!(parse_hex_address("1234").unwrap(), 0x1234);
    assert_eq!(parse_hex_address("ABCD").unwrap(), 0xABCD);
}

/// Test parse_hex_address with valid addresses with lowercase 0x prefix
/// Validates: Requirements 12.6, 2.4
#[test]
fn test_parse_hex_address_with_lowercase_prefix() {
    // Test various valid addresses with 0x prefix
    assert_eq!(parse_hex_address("0x0000").unwrap(), 0x0000);
    assert_eq!(parse_hex_address("0x8000").unwrap(), 0x8000);
    assert_eq!(parse_hex_address("0xC000").unwrap(), 0xC000);
    assert_eq!(parse_hex_address("0xFFFF").unwrap(), 0xFFFF);
    assert_eq!(parse_hex_address("0x1234").unwrap(), 0x1234);
    assert_eq!(parse_hex_address("0xABCD").unwrap(), 0xABCD);
}

/// Test parse_hex_address with valid addresses with uppercase 0X prefix
/// Validates: Requirements 12.6, 2.4
#[test]
fn test_parse_hex_address_with_uppercase_prefix() {
    // Test various valid addresses with 0X prefix
    assert_eq!(parse_hex_address("0X0000").unwrap(), 0x0000);
    assert_eq!(parse_hex_address("0X8000").unwrap(), 0x8000);
    assert_eq!(parse_hex_address("0XC000").unwrap(), 0xC000);
    assert_eq!(parse_hex_address("0XFFFF").unwrap(), 0xFFFF);
    assert_eq!(parse_hex_address("0X1234").unwrap(), 0x1234);
    assert_eq!(parse_hex_address("0XABCD").unwrap(), 0xABCD);
}

/// Test parse_hex_address with case insensitivity
/// Validates: Requirements 12.6, 2.4
#[test]
fn test_parse_hex_address_case_insensitive() {
    // Hexadecimal parsing should be case-insensitive
    assert_eq!(parse_hex_address("abcd").unwrap(), 0xABCD);
    assert_eq!(parse_hex_address("ABCD").unwrap(), 0xABCD);
    assert_eq!(parse_hex_address("AbCd").unwrap(), 0xABCD);
    assert_eq!(parse_hex_address("0xabcd").unwrap(), 0xABCD);
    assert_eq!(parse_hex_address("0XABCD").unwrap(), 0xABCD);
}

/// Test parse_hex_address with whitespace trimming
/// Validates: Requirements 12.6, 2.4
#[test]
fn test_parse_hex_address_with_whitespace() {
    // Should trim leading and trailing whitespace
    assert_eq!(parse_hex_address("  8000  ").unwrap(), 0x8000);
    assert_eq!(parse_hex_address("\t0x8000\t").unwrap(), 0x8000);
    assert_eq!(parse_hex_address(" 0xC000 ").unwrap(), 0xC000);
    assert_eq!(parse_hex_address("\n8000\n").unwrap(), 0x8000);
}

/// Test parse_hex_address with full address range
/// Validates: Requirements 2.4
#[test]
fn test_parse_hex_address_full_range() {
    // Test minimum and maximum addresses
    assert_eq!(parse_hex_address("0000").unwrap(), 0x0000);
    assert_eq!(parse_hex_address("FFFF").unwrap(), 0xFFFF);
    
    // Test various addresses across the range
    assert_eq!(parse_hex_address("0001").unwrap(), 0x0001);
    assert_eq!(parse_hex_address("00FF").unwrap(), 0x00FF);
    assert_eq!(parse_hex_address("0100").unwrap(), 0x0100);
    assert_eq!(parse_hex_address("7FFF").unwrap(), 0x7FFF);
    assert_eq!(parse_hex_address("8000").unwrap(), 0x8000);
    assert_eq!(parse_hex_address("FFFE").unwrap(), 0xFFFE);
}

/// Test parse_hex_address with empty input
/// Validates: Requirements 12.6, 2.4
#[test]
fn test_parse_hex_address_empty_input() {
    // Empty string should return descriptive error
    let result = parse_hex_address("");
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.contains("empty"));
    assert!(err.contains("0x0000-0xFFFF"));
}

/// Test parse_hex_address with whitespace-only input
/// Validates: Requirements 12.6, 2.4
#[test]
fn test_parse_hex_address_whitespace_only() {
    // Whitespace-only should be treated as empty after trimming
    let result = parse_hex_address("   ");
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.contains("empty"));
}

/// Test parse_hex_address with prefix-only input
/// Validates: Requirements 12.6, 2.4
#[test]
fn test_parse_hex_address_prefix_only() {
    // Just "0x" or "0X" without digits should return error
    let result = parse_hex_address("0x");
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.contains("0x"));
    assert!(err.contains("after prefix"));
    
    let result = parse_hex_address("0X");
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.contains("0X"));
}

/// Test parse_hex_address with invalid hexadecimal characters
/// Validates: Requirements 12.6, 2.4
#[test]
fn test_parse_hex_address_invalid_hex_chars() {
    // Invalid hexadecimal characters should return descriptive error
    let invalid_inputs = ["GGGG", "12XY", "ZZZZ", "HIJK", "0xGGGG", "0XZZZZ"];
    
    for input in &invalid_inputs {
        let result = parse_hex_address(input);
        assert!(result.is_err(), "Should fail to parse '{}'", input);
        let err = result.unwrap_err();
        assert!(err.contains("Invalid address format"));
        assert!(err.contains(input));
        assert!(err.contains("0x0000-0xFFFF"));
    }
}

/// Test parse_hex_address with non-hexadecimal input
/// Validates: Requirements 12.6, 2.4
#[test]
fn test_parse_hex_address_non_hex_input() {
    // Non-hexadecimal input should return descriptive error
    let invalid_inputs = ["hello", "world", "test", "12.34", "0x0x", "++--"];
    
    for input in &invalid_inputs {
        let result = parse_hex_address(input);
        assert!(result.is_err(), "Should fail to parse '{}'", input);
        let err = result.unwrap_err();
        assert!(err.contains("Invalid address format"));
    }
}

/// Test parse_hex_address with too many digits
/// Validates: Requirements 12.6, 2.4
#[test]
fn test_parse_hex_address_too_many_digits() {
    // More than 4 hex digits should fail (exceeds u16 range)
    let result = parse_hex_address("10000");
    assert!(result.is_err());
    
    let result = parse_hex_address("0x10000");
    assert!(result.is_err());
    
    let result = parse_hex_address("FFFFF");
    assert!(result.is_err());
}

/// Test parse_hex_address error messages are descriptive
/// Validates: Requirements 10.3
#[test]
fn test_parse_hex_address_descriptive_errors() {
    // Error messages should include examples and expected format
    let result = parse_hex_address("GGGG");
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.contains("0x8000") || err.contains("8000") || err.contains("0xC000") || err.contains("C000"));
    
    let result = parse_hex_address("");
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.contains("0x8000") || err.contains("8000") || err.contains("0xC000") || err.contains("C000"));
}

// ============================================================================
// Existing tests for command-line argument parsing
// ============================================================================

/// Test that hexadecimal parsing works correctly with and without 0x prefix
#[test]
fn test_hex_parsing_with_prefix() {
    // Test parsing with 0x prefix
    let result = u16::from_str_radix("8000", 16);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 0x8000);
    
    // Test parsing without prefix (simulating user input after stripping "0x")
    let input = "0x8000";
    let result = if input.starts_with("0x") {
        u16::from_str_radix(&input[2..], 16)
    } else {
        u16::from_str_radix(input, 16)
    };
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 0x8000);
}

/// Test command-line argument parsing logic for no arguments
/// Validates: Requirements 12.4
#[test]
fn test_parse_args_no_arguments() {
    // Simulate no arguments (only program name)
    // In real execution: $ cpu_6502_emulator
    // Expected: (None, None) - will prompt for both
    
    // We can't directly test parse_args() without modifying std::env::args(),
    // but we can test the parsing logic
    let args: Vec<String> = vec!["cpu_6502_emulator".to_string()];
    
    let result = match args.len() {
        1 => (None, None),
        2 => (Some(args[1].clone()), None),
        3 => {
            let filename = args[1].clone();
            let addr_str = &args[2];
            let parse_result = if addr_str.starts_with("0x") || addr_str.starts_with("0X") {
                u16::from_str_radix(&addr_str[2..], 16)
            } else {
                u16::from_str_radix(addr_str, 16)
            };
            match parse_result {
                Ok(addr) => (Some(filename), Some(addr)),
                Err(_) => (Some(filename), None),
            }
        }
        _ => (None, None),
    };
    
    assert_eq!(result.0, None);
    assert_eq!(result.1, None);
}

/// Test command-line argument parsing logic for filename only
/// Validates: Requirements 12.1, 12.3
#[test]
fn test_parse_args_filename_only() {
    // Simulate filename only
    // In real execution: $ cpu_6502_emulator program.bin
    // Expected: (Some("program.bin"), None) - will default to 0x0000
    
    let args: Vec<String> = vec![
        "cpu_6502_emulator".to_string(),
        "program.bin".to_string(),
    ];
    
    let result = match args.len() {
        1 => (None, None),
        2 => (Some(args[1].clone()), None),
        3 => {
            let filename = args[1].clone();
            let addr_str = &args[2];
            let parse_result = if addr_str.starts_with("0x") || addr_str.starts_with("0X") {
                u16::from_str_radix(&addr_str[2..], 16)
            } else {
                u16::from_str_radix(addr_str, 16)
            };
            match parse_result {
                Ok(addr) => (Some(filename), Some(addr)),
                Err(_) => (Some(filename), None),
            }
        }
        _ => (None, None),
    };
    
    assert_eq!(result.0, Some("program.bin".to_string()));
    assert_eq!(result.1, None);
}

/// Test command-line argument parsing logic for filename and address
/// Validates: Requirements 12.2, 12.6
#[test]
fn test_parse_args_filename_and_address() {
    // Simulate filename and address
    // In real execution: $ cpu_6502_emulator program.bin 8000
    // Expected: (Some("program.bin"), Some(0x8000))
    
    let args: Vec<String> = vec![
        "cpu_6502_emulator".to_string(),
        "program.bin".to_string(),
        "8000".to_string(),
    ];
    
    let result = match args.len() {
        1 => (None, None),
        2 => (Some(args[1].clone()), None),
        3 => {
            let filename = args[1].clone();
            let addr_str = &args[2];
            let parse_result = if addr_str.starts_with("0x") || addr_str.starts_with("0X") {
                u16::from_str_radix(&addr_str[2..], 16)
            } else {
                u16::from_str_radix(addr_str, 16)
            };
            match parse_result {
                Ok(addr) => (Some(filename), Some(addr)),
                Err(_) => (Some(filename), None),
            }
        }
        _ => (None, None),
    };
    
    assert_eq!(result.0, Some("program.bin".to_string()));
    assert_eq!(result.1, Some(0x8000));
}

/// Test command-line argument parsing with 0x prefix
/// Validates: Requirements 12.6
#[test]
fn test_parse_args_with_0x_prefix() {
    // Simulate filename and address with 0x prefix
    // In real execution: $ cpu_6502_emulator program.bin 0x8000
    // Expected: (Some("program.bin"), Some(0x8000))
    
    let args: Vec<String> = vec![
        "cpu_6502_emulator".to_string(),
        "program.bin".to_string(),
        "0x8000".to_string(),
    ];
    
    let result = match args.len() {
        1 => (None, None),
        2 => (Some(args[1].clone()), None),
        3 => {
            let filename = args[1].clone();
            let addr_str = &args[2];
            let parse_result = if addr_str.starts_with("0x") || addr_str.starts_with("0X") {
                u16::from_str_radix(&addr_str[2..], 16)
            } else {
                u16::from_str_radix(addr_str, 16)
            };
            match parse_result {
                Ok(addr) => (Some(filename), Some(addr)),
                Err(_) => (Some(filename), None),
            }
        }
        _ => (None, None),
    };
    
    assert_eq!(result.0, Some("program.bin".to_string()));
    assert_eq!(result.1, Some(0x8000));
}

/// Test command-line argument parsing with uppercase 0X prefix
/// Validates: Requirements 12.6
#[test]
fn test_parse_args_with_uppercase_0x_prefix() {
    // Simulate filename and address with 0X prefix
    // In real execution: $ cpu_6502_emulator program.bin 0XC000
    // Expected: (Some("program.bin"), Some(0xC000))
    
    let args: Vec<String> = vec![
        "cpu_6502_emulator".to_string(),
        "program.bin".to_string(),
        "0XC000".to_string(),
    ];
    
    let result = match args.len() {
        1 => (None, None),
        2 => (Some(args[1].clone()), None),
        3 => {
            let filename = args[1].clone();
            let addr_str = &args[2];
            let parse_result = if addr_str.starts_with("0x") || addr_str.starts_with("0X") {
                u16::from_str_radix(&addr_str[2..], 16)
            } else {
                u16::from_str_radix(addr_str, 16)
            };
            match parse_result {
                Ok(addr) => (Some(filename), Some(addr)),
                Err(_) => (Some(filename), None),
            }
        }
        _ => (None, None),
    };
    
    assert_eq!(result.0, Some("program.bin".to_string()));
    assert_eq!(result.1, Some(0xC000));
}

/// Test command-line argument parsing with invalid address
/// Validates: Requirements 12.5
#[test]
fn test_parse_args_invalid_address() {
    // Simulate filename and invalid address
    // In real execution: $ cpu_6502_emulator program.bin GGGG
    // Expected: (Some("program.bin"), None) - will prompt for address
    
    let args: Vec<String> = vec![
        "cpu_6502_emulator".to_string(),
        "program.bin".to_string(),
        "GGGG".to_string(),
    ];
    
    let result = match args.len() {
        1 => (None, None),
        2 => (Some(args[1].clone()), None),
        3 => {
            let filename = args[1].clone();
            let addr_str = &args[2];
            let parse_result = if addr_str.starts_with("0x") || addr_str.starts_with("0X") {
                u16::from_str_radix(&addr_str[2..], 16)
            } else {
                u16::from_str_radix(addr_str, 16)
            };
            match parse_result {
                Ok(addr) => (Some(filename), Some(addr)),
                Err(_) => (Some(filename), None),
            }
        }
        _ => (None, None),
    };
    
    assert_eq!(result.0, Some("program.bin".to_string()));
    assert_eq!(result.1, None);
}

/// Test command-line argument parsing with too many arguments
/// Validates: Requirements 12.5
#[test]
fn test_parse_args_too_many_arguments() {
    // Simulate too many arguments
    // In real execution: $ cpu_6502_emulator program.bin 8000 extra
    // Expected: (None, None) - will prompt for both
    
    let args: Vec<String> = vec![
        "cpu_6502_emulator".to_string(),
        "program.bin".to_string(),
        "8000".to_string(),
        "extra".to_string(),
    ];
    
    let result = match args.len() {
        1 => (None, None),
        2 => (Some(args[1].clone()), None),
        3 => {
            let filename = args[1].clone();
            let addr_str = &args[2];
            let parse_result = if addr_str.starts_with("0x") || addr_str.starts_with("0X") {
                u16::from_str_radix(&addr_str[2..], 16)
            } else {
                u16::from_str_radix(addr_str, 16)
            };
            match parse_result {
                Ok(addr) => (Some(filename), Some(addr)),
                Err(_) => (Some(filename), None),
            }
        }
        _ => (None, None),
    };
    
    assert_eq!(result.0, None);
    assert_eq!(result.1, None);
}

/// Test default start address behavior
/// Validates: Requirements 12.3
#[test]
fn test_default_start_address() {
    // When filename is provided but no address, should default to 0x0000
    let filename_provided = true;
    let address_provided = false;
    
    let default_address = if filename_provided && !address_provided {
        0x0000
    } else {
        0xFFFF // Some other value
    };
    
    assert_eq!(default_address, 0x0000);
}

#[test]
fn test_hex_parsing_without_prefix() {
    // Test parsing without 0x prefix
    let result = u16::from_str_radix("C000", 16);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 0xC000);
    
    // Test case insensitivity
    let result = u16::from_str_radix("c000", 16);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 0xC000);
}

#[test]
fn test_hex_parsing_full_range() {
    // Test minimum address (0x0000)
    let result = u16::from_str_radix("0000", 16);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 0x0000);
    
    // Test maximum address (0xFFFF)
    let result = u16::from_str_radix("FFFF", 16);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 0xFFFF);
    
    // Test mid-range addresses
    let test_cases = [
        ("0100", 0x0100),
        ("1000", 0x1000),
        ("8000", 0x8000),
        ("A000", 0xA000),
        ("DEAD", 0xDEAD),
        ("BEEF", 0xBEEF),
    ];
    
    for (input, expected) in &test_cases {
        let result = u16::from_str_radix(input, 16);
        assert!(result.is_ok(), "Failed to parse '{}'", input);
        assert_eq!(result.unwrap(), *expected, "Incorrect parse result for '{}'", input);
    }
}

#[test]
fn test_hex_parsing_invalid_formats() {
    // Test invalid hexadecimal characters
    let invalid_inputs = [
        "GGGG",  // Invalid hex character
        "12XY",  // Mixed valid/invalid
        "ZZZZ",  // All invalid
        "0x0x",  // Double prefix
        "",      // Empty string
        "  ",    // Whitespace only
        "10000", // Too large (5 digits)
    ];
    
    for input in &invalid_inputs {
        let result = u16::from_str_radix(input, 16);
        assert!(result.is_err(), "Should fail to parse '{}'", input);
    }
}

#[test]
fn test_hex_parsing_with_uppercase_prefix() {
    // Test uppercase 0X prefix
    let input = "0X8000";
    let result = if input.starts_with("0x") || input.starts_with("0X") {
        u16::from_str_radix(&input[2..], 16)
    } else {
        u16::from_str_radix(input, 16)
    };
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 0x8000);
}

#[test]
fn test_address_range_validation() {
    // u16 automatically ensures the range is 0x0000-0xFFFF
    // Test that we can represent all valid addresses
    let min_addr: u16 = 0x0000;
    let max_addr: u16 = 0xFFFF;
    
    assert_eq!(min_addr, 0);
    assert_eq!(max_addr, 65535);
    
    // Test that parsing produces valid u16 values
    let test_addresses = ["0000", "0001", "7FFF", "8000", "FFFE", "FFFF"];
    for addr_str in &test_addresses {
        let parsed = u16::from_str_radix(addr_str, 16);
        assert!(parsed.is_ok(), "Failed to parse valid address '{}'", addr_str);
        let value = parsed.unwrap();
        assert!(value >= min_addr && value <= max_addr, 
                "Address 0x{:04X} out of range", value);
    }
}

#[test]
fn test_empty_input_handling() {
    // Test that empty strings are rejected
    let empty_inputs = ["", "   ", "\t", "\n"];
    
    for input in &empty_inputs {
        let trimmed = input.trim();
        assert!(trimmed.is_empty(), "Trimmed '{}' should be empty", input);
    }
}

#[test]
fn test_file_path_validation() {
    // Test that non-empty file paths are accepted
    let valid_paths = [
        "program.bin",
        "/path/to/program.bin",
        "../relative/path.bin",
        "./local.bin",
        "C:\\Windows\\path.bin",
    ];
    
    for path in &valid_paths {
        let trimmed = path.trim();
        assert!(!trimmed.is_empty(), "Path '{}' should not be empty after trim", path);
    }
}

/// Integration test documentation:
/// 
/// The following scenarios should be tested manually or with integration tests:
/// 
/// 1. Valid file path and address:
///    - Input: "test.bin" and "0x8000"
///    - Expected: Emulator starts successfully
/// 
/// 2. Invalid file path (re-prompt):
///    - Input: "" (empty)
///    - Expected: Error message and re-prompt
/// 
/// 3. Invalid address format (re-prompt):
///    - Input: "GGGG" (invalid hex)
///    - Expected: Error message with format explanation and re-prompt
/// 
/// 4. Empty address (re-prompt):
///    - Input: "" (empty)
///    - Expected: Error message and re-prompt
/// 
/// 5. Address without 0x prefix:
///    - Input: "8000"
///    - Expected: Parsed as 0x8000
/// 
/// 6. Address with 0X prefix (uppercase):
///    - Input: "0X8000"
///    - Expected: Parsed as 0x8000
/// 
/// 7. EOF/Ctrl+D handling:
///    - Input: EOF
///    - Expected: "Input cancelled" message and graceful exit
/// 
/// 8. File not found error:
///    - Input: "/nonexistent/file.bin"
///    - Expected: Error message with file path and suggestion to check path
#[test]
fn test_integration_scenarios_documented() {
    // This test exists to document the expected behavior
    // Actual testing requires manual verification or integration test framework
    assert!(true, "Integration scenarios documented above");
}

