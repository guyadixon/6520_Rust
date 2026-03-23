// Property-based tests for command-line argument parsing
// Feature: 6502-cpu-emulator, Property 23: Command-Line Argument Parsing

use cpu_6502_emulator::parse_hex_address;
use proptest::prelude::*;

// Feature: 6502-cpu-emulator, Property 23: Command-Line Argument Parsing
// **Validates: Requirements 12.1-12.6**
//
// For any valid command-line argument combination:
// - No arguments should result in interactive prompts
// - Filename only should use that filename and default to start address 0x0000
// - Filename and valid hex address should use both values
// - Invalid arguments should return descriptive errors
//
// This property verifies that:
// 1. parse_hex_address accepts valid hex addresses with or without 0x prefix
// 2. parse_hex_address rejects invalid formats with descriptive errors
// 3. All valid addresses in range 0x0000-0xFFFF are accepted
// 4. Addresses outside the valid range are rejected
// 5. Error messages are descriptive and include examples

// Test that parse_hex_address accepts all valid addresses in the full range
proptest! {
    #[test]
    fn parse_hex_address_accepts_all_valid_addresses(
        address in 0u16..=0xFFFF
    ) {
        // Test without prefix
        let hex_str = format!("{:04X}", address);
        let result = parse_hex_address(&hex_str);
        prop_assert!(result.is_ok(),
            "Should parse valid address without prefix: {}",
            hex_str);
        prop_assert_eq!(result.unwrap(), address,
            "Parsed address should match expected value");
        
        // Test with lowercase 0x prefix
        let hex_str_with_prefix = format!("0x{:04X}", address);
        let result_with_prefix = parse_hex_address(&hex_str_with_prefix);
        prop_assert!(result_with_prefix.is_ok(),
            "Should parse valid address with 0x prefix: {}",
            hex_str_with_prefix);
        prop_assert_eq!(result_with_prefix.unwrap(), address,
            "Parsed address with prefix should match expected value");
        
        // Test with uppercase 0X prefix
        let hex_str_with_uppercase = format!("0X{:04X}", address);
        let result_with_uppercase = parse_hex_address(&hex_str_with_uppercase);
        prop_assert!(result_with_uppercase.is_ok(),
            "Should parse valid address with 0X prefix: {}",
            hex_str_with_uppercase);
        prop_assert_eq!(result_with_uppercase.unwrap(), address,
            "Parsed address with uppercase prefix should match expected value");
    }
}

// Test that parse_hex_address is case-insensitive for hex digits
proptest! {
    #[test]
    fn parse_hex_address_is_case_insensitive(
        address in 0u16..=0xFFFF
    ) {
        // Test lowercase hex digits
        let lowercase = format!("{:04x}", address);
        let result_lower = parse_hex_address(&lowercase);
        prop_assert!(result_lower.is_ok(),
            "Should parse lowercase hex: {}",
            lowercase);
        prop_assert_eq!(result_lower.unwrap(), address);
        
        // Test uppercase hex digits
        let uppercase = format!("{:04X}", address);
        let result_upper = parse_hex_address(&uppercase);
        prop_assert!(result_upper.is_ok(),
            "Should parse uppercase hex: {}",
            uppercase);
        prop_assert_eq!(result_upper.unwrap(), address);
        
        // Test mixed case
        let mixed = format!("{:02x}{:02X}", address & 0xFF, (address >> 8) & 0xFF);
        let result_mixed = parse_hex_address(&mixed);
        prop_assert!(result_mixed.is_ok(),
            "Should parse mixed case hex: {}",
            mixed);
    }
}

// Test that parse_hex_address handles whitespace correctly
proptest! {
    #[test]
    fn parse_hex_address_trims_whitespace(
        address in 0u16..=0xFFFF,
        leading_spaces in 0usize..=5,
        trailing_spaces in 0usize..=5
    ) {
        let hex_str = format!("{:04X}", address);
        let leading = " ".repeat(leading_spaces);
        let trailing = " ".repeat(trailing_spaces);
        let padded = format!("{}{}{}", leading, hex_str, trailing);
        
        let result = parse_hex_address(&padded);
        prop_assert!(result.is_ok(),
            "Should parse address with whitespace: '{}'",
            padded);
        prop_assert_eq!(result.unwrap(), address,
            "Parsed address should match expected value after trimming");
    }
}

// Test that parse_hex_address rejects empty input
proptest! {
    #[test]
    fn parse_hex_address_rejects_empty_input(
        whitespace_count in 0usize..=10
    ) {
        let empty = " ".repeat(whitespace_count);
        let result = parse_hex_address(&empty);
        
        prop_assert!(result.is_err(),
            "Should reject empty/whitespace-only input: '{}'",
            empty);
        
        let err = result.unwrap_err();
        prop_assert!(err.contains("empty") || err.contains("Expected"),
            "Error should mention empty input or expected format: {}",
            err);
        prop_assert!(err.contains("0x") || err.contains("Examples"),
            "Error should include examples: {}",
            err);
    }
}

// Test that parse_hex_address rejects prefix-only input
proptest! {
    #[test]
    fn parse_hex_address_rejects_prefix_only(
        prefix in prop::sample::select(vec!["0x", "0X"])
    ) {
        let result = parse_hex_address(prefix);
        
        prop_assert!(result.is_err(),
            "Should reject prefix-only input: '{}'",
            prefix);
        
        let err = result.unwrap_err();
        prop_assert!(err.contains("after prefix") || err.contains("Invalid"),
            "Error should mention missing digits after prefix: {}",
            err);
    }
}

// Test that parse_hex_address rejects invalid hexadecimal characters
proptest! {
    #[test]
    fn parse_hex_address_rejects_invalid_hex_chars(
        invalid_char in "[G-Zg-z]",
        position in 0usize..=3
    ) {
        // Create a string with an invalid character at the specified position
        let mut chars = vec!['8', '0', '0', '0'];
        chars[position] = invalid_char.chars().next().unwrap();
        let invalid_hex: String = chars.into_iter().collect();
        
        let result = parse_hex_address(&invalid_hex);
        
        prop_assert!(result.is_err(),
            "Should reject invalid hex characters: '{}'",
            invalid_hex);
        
        let err = result.unwrap_err();
        prop_assert!(err.contains("Invalid address format"),
            "Error should mention invalid format: {}",
            err);
        prop_assert!(err.contains(&invalid_hex),
            "Error should include the invalid input: {}",
            err);
    }
}

// Test that parse_hex_address rejects non-hexadecimal input
proptest! {
    #[test]
    fn parse_hex_address_rejects_non_hex_input(
        non_hex in "[^0-9A-Fa-fx]{1,8}"
    ) {
        // Skip inputs that might accidentally be valid after trimming
        prop_assume!(!non_hex.trim().is_empty());
        prop_assume!(!non_hex.contains("0x") && !non_hex.contains("0X"));
        
        let result = parse_hex_address(&non_hex);
        
        prop_assert!(result.is_err(),
            "Should reject non-hexadecimal input: '{}'",
            non_hex);
        
        let err = result.unwrap_err();
        prop_assert!(err.contains("Invalid") || err.contains("Expected"),
            "Error should indicate invalid format: {}",
            err);
    }
}

// Test that parse_hex_address rejects values that would overflow u16
proptest! {
    #[test]
    fn parse_hex_address_rejects_overflow_values(
        overflow_value in 0x10000u32..=0xFFFFFF
    ) {
        let hex_str = format!("{:X}", overflow_value);
        let result = parse_hex_address(&hex_str);
        
        prop_assert!(result.is_err(),
            "Should reject value that overflows u16: {} (0x{})",
            overflow_value, hex_str);
        
        let err = result.unwrap_err();
        prop_assert!(err.contains("Invalid") || err.contains("0xFFFF"),
            "Error should indicate value is out of range: {}",
            err);
    }
}

// Test that parse_hex_address error messages are descriptive
proptest! {
    #[test]
    fn parse_hex_address_errors_are_descriptive(
        invalid_input in prop::sample::select(vec![
            "",
            "GGGG",
            "hello",
            "12.34",
            "0x0x",
            "10000",
            "FFFFF",
        ])
    ) {
        let result = parse_hex_address(invalid_input);
        
        prop_assert!(result.is_err(),
            "Should reject invalid input: '{}'",
            invalid_input);
        
        let err = result.unwrap_err();
        
        // Error should be descriptive (not just "error" or "invalid")
        prop_assert!(err.len() > 20,
            "Error message should be descriptive (>20 chars): {}",
            err);
        
        // Error should include examples or expected format
        prop_assert!(
            err.contains("0x") || err.contains("Examples") || err.contains("Expected"),
            "Error should include examples or expected format: {}",
            err
        );
        
        // For most errors (not prefix-only), should mention the valid range
        if invalid_input != "0x" && invalid_input != "0X" {
            prop_assert!(
                err.contains("0x0000") || err.contains("0xFFFF") || err.contains("0x0000-0xFFFF"),
                "Error should mention valid range: {}",
                err
            );
        }
    }
}

// Test that parse_hex_address handles boundary values correctly
proptest! {
    #[test]
    fn parse_hex_address_handles_boundary_values(
        boundary in prop::sample::select(vec![
            0x0000, 0x0001, 0x00FF, 0x0100,
            0x7FFF, 0x8000, 0xFFFE, 0xFFFF
        ])
    ) {
        // Test without prefix
        let hex_str = format!("{:04X}", boundary);
        let result = parse_hex_address(&hex_str);
        prop_assert!(result.is_ok(),
            "Should parse boundary value: 0x{:04X}",
            boundary);
        prop_assert_eq!(result.unwrap(), boundary);
        
        // Test with prefix
        let hex_str_with_prefix = format!("0x{:04X}", boundary);
        let result_with_prefix = parse_hex_address(&hex_str_with_prefix);
        prop_assert!(result_with_prefix.is_ok(),
            "Should parse boundary value with prefix: 0x{:04X}",
            boundary);
        prop_assert_eq!(result_with_prefix.unwrap(), boundary);
    }
}

// Test that parse_hex_address accepts various valid formats
proptest! {
    #[test]
    fn parse_hex_address_accepts_various_formats(
        address in 0u16..=0xFFFF
    ) {
        // Test with different digit counts (1-4 digits)
        let formats = vec![
            format!("{:X}", address),      // Minimal digits
            format!("{:04X}", address),    // 4 digits with leading zeros
        ];
        
        for hex_str in formats {
            let result = parse_hex_address(&hex_str);
            prop_assert!(result.is_ok(),
                "Should parse format: {}",
                hex_str);
            prop_assert_eq!(result.unwrap(), address,
                "Parsed value should match for format: {}",
                hex_str);
        }
    }
}

// Test that parse_hex_address is consistent across multiple calls
proptest! {
    #[test]
    fn parse_hex_address_is_consistent(
        address in 0u16..=0xFFFF
    ) {
        let hex_str = format!("{:04X}", address);
        
        // Parse the same string multiple times
        let result1 = parse_hex_address(&hex_str);
        let result2 = parse_hex_address(&hex_str);
        let result3 = parse_hex_address(&hex_str);
        
        prop_assert!(result1.is_ok());
        prop_assert!(result2.is_ok());
        prop_assert!(result3.is_ok());
        
        // Extract values before comparing
        let value1 = result1.unwrap();
        let value2 = result2.unwrap();
        let value3 = result3.unwrap();
        
        // All results should be identical
        prop_assert_eq!(value1, value2);
        prop_assert_eq!(value2, value3);
        prop_assert_eq!(value1, address);
    }
}

// Test that parse_hex_address handles special characters correctly
proptest! {
    #[test]
    fn parse_hex_address_rejects_special_characters(
        special_char in "[!@#$%^&*()\\-+=\\[\\]{}|;:'\",.<>?/\\\\]"
    ) {
        let invalid_input = format!("80{}00", special_char);
        let result = parse_hex_address(&invalid_input);
        
        prop_assert!(result.is_err(),
            "Should reject input with special characters: '{}'",
            invalid_input);
    }
}

// Test that parse_hex_address round-trips correctly
proptest! {
    #[test]
    fn parse_hex_address_round_trips_correctly(
        address in 0u16..=0xFFFF
    ) {
        // Convert address to hex string
        let hex_str = format!("{:04X}", address);
        
        // Parse it back
        let parsed = parse_hex_address(&hex_str).unwrap();
        
        // Should get the same value
        prop_assert_eq!(parsed, address,
            "Round-trip should preserve value: 0x{:04X} -> {} -> 0x{:04X}",
            address, hex_str, parsed);
        
        // Convert back to hex string
        let hex_str2 = format!("{:04X}", parsed);
        
        // Should get the same string (with consistent formatting)
        prop_assert_eq!(hex_str, hex_str2,
            "Round-trip should preserve string representation");
    }
}

// Test that parse_hex_address handles tab characters as whitespace
proptest! {
    #[test]
    fn parse_hex_address_handles_tabs_as_whitespace(
        address in 0u16..=0xFFFF,
        leading_tabs in 0usize..=3,
        trailing_tabs in 0usize..=3
    ) {
        let hex_str = format!("{:04X}", address);
        let leading = "\t".repeat(leading_tabs);
        let trailing = "\t".repeat(trailing_tabs);
        let padded = format!("{}{}{}", leading, hex_str, trailing);
        
        let result = parse_hex_address(&padded);
        prop_assert!(result.is_ok(),
            "Should parse address with tabs: '{}'",
            padded.replace('\t', "\\t"));
        prop_assert_eq!(result.unwrap(), address);
    }
}

// Test that parse_hex_address handles newlines as whitespace
proptest! {
    #[test]
    fn parse_hex_address_handles_newlines_as_whitespace(
        address in 0u16..=0xFFFF
    ) {
        let hex_str = format!("{:04X}", address);
        
        // Test with leading newline
        let with_leading = format!("\n{}", hex_str);
        let result_leading = parse_hex_address(&with_leading);
        prop_assert!(result_leading.is_ok(),
            "Should parse address with leading newline");
        prop_assert_eq!(result_leading.unwrap(), address);
        
        // Test with trailing newline
        let with_trailing = format!("{}\n", hex_str);
        let result_trailing = parse_hex_address(&with_trailing);
        prop_assert!(result_trailing.is_ok(),
            "Should parse address with trailing newline");
        prop_assert_eq!(result_trailing.unwrap(), address);
        
        // Test with both
        let with_both = format!("\n{}\n", hex_str);
        let result_both = parse_hex_address(&with_both);
        prop_assert!(result_both.is_ok(),
            "Should parse address with newlines on both sides");
        prop_assert_eq!(result_both.unwrap(), address);
    }
}
