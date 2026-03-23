// Unit tests for Memory module
// Tests basic memory operations and edge cases

use cpu_6502_emulator::memory::Memory;
use std::fs::File;
use std::io::Write;

#[test]
fn test_memory_new_initializes_to_zero() {
    let memory = Memory::new();
    // Check a few random addresses to ensure they're zero
    assert_eq!(memory.read(0x0000), 0);
    assert_eq!(memory.read(0x1234), 0);
    assert_eq!(memory.read(0xFFFF), 0);
}

#[test]
fn test_memory_read_write() {
    let mut memory = Memory::new();
    
    // Write and read a single byte
    memory.write(0x1000, 0x42);
    assert_eq!(memory.read(0x1000), 0x42);
    
    // Write and read at different addresses
    memory.write(0x0000, 0xFF);
    memory.write(0xFFFF, 0xAA);
    assert_eq!(memory.read(0x0000), 0xFF);
    assert_eq!(memory.read(0xFFFF), 0xAA);
}

#[test]
fn test_memory_read_word_little_endian() {
    let mut memory = Memory::new();
    
    // Write two bytes and read as word
    memory.write(0x1000, 0x34); // Low byte
    memory.write(0x1001, 0x12); // High byte
    
    let word = memory.read_word(0x1000);
    assert_eq!(word, 0x1234, "Expected 0x1234 but got 0x{:04X}", word);
}

#[test]
fn test_memory_read_word_wrapping() {
    let mut memory = Memory::new();
    
    // Test reading word at the end of memory (should wrap)
    memory.write(0xFFFF, 0x34); // Low byte
    memory.write(0x0000, 0x12); // High byte (wraps to 0x0000)
    
    let word = memory.read_word(0xFFFF);
    assert_eq!(word, 0x1234, "Expected 0x1234 but got 0x{:04X}", word);
}

#[test]
fn test_memory_read_word_all_zeros() {
    let memory = Memory::new();
    
    // Reading from uninitialized memory should return 0x0000
    let word = memory.read_word(0x5000);
    assert_eq!(word, 0x0000);
}

#[test]
fn test_memory_read_word_all_ones() {
    let mut memory = Memory::new();
    
    memory.write(0x2000, 0xFF);
    memory.write(0x2001, 0xFF);
    
    let word = memory.read_word(0x2000);
    assert_eq!(word, 0xFFFF);
}

#[test]
fn test_memory_multiple_writes_same_address() {
    let mut memory = Memory::new();
    
    // Overwrite the same address multiple times
    memory.write(0x1000, 0x11);
    assert_eq!(memory.read(0x1000), 0x11);
    
    memory.write(0x1000, 0x22);
    assert_eq!(memory.read(0x1000), 0x22);
    
    memory.write(0x1000, 0x33);
    assert_eq!(memory.read(0x1000), 0x33);
}

#[test]
fn test_memory_boundary_addresses() {
    let mut memory = Memory::new();
    
    // Test first address
    memory.write(0x0000, 0xAA);
    assert_eq!(memory.read(0x0000), 0xAA);
    
    // Test last address
    memory.write(0xFFFF, 0xBB);
    assert_eq!(memory.read(0xFFFF), 0xBB);
}

// File loading tests

#[test]
fn test_load_from_file_empty_file() {
    // Create a temporary empty file
    let temp_dir = std::env::temp_dir();
    let file_path = temp_dir.join("test_empty.bin");
    
    {
        File::create(&file_path).expect("Failed to create test file");
    }
    
    let mut memory = Memory::new();
    let result = memory.load_from_file(file_path.to_str().unwrap());
    
    assert!(result.is_ok(), "Loading empty file should succeed");
    
    // All memory should be zero (padded)
    for i in 0..=0xFFFF {
        assert_eq!(memory.read(i), 0, "Memory at address 0x{:04X} should be 0", i);
    }
    
    // Clean up
    std::fs::remove_file(&file_path).ok();
}

#[test]
fn test_load_from_file_small_file() {
    // Create a temporary file with 256 bytes
    let temp_dir = std::env::temp_dir();
    let file_path = temp_dir.join("test_small.bin");
    
    {
        let mut file = File::create(&file_path).expect("Failed to create test file");
        let data: Vec<u8> = (0..=255).collect();
        file.write_all(&data).expect("Failed to write test data");
    }
    
    let mut memory = Memory::new();
    let result = memory.load_from_file(file_path.to_str().unwrap());
    
    assert!(result.is_ok(), "Loading small file should succeed");
    
    // First 256 bytes should match the file
    for i in 0..=255 {
        assert_eq!(memory.read(i), i as u8, "Memory at address 0x{:04X} should be 0x{:02X}", i, i);
    }
    
    // Remaining bytes should be zero (padded)
    for i in 256..=0xFFFF {
        assert_eq!(memory.read(i), 0, "Memory at address 0x{:04X} should be 0 (padded)", i);
    }
    
    // Clean up
    std::fs::remove_file(&file_path).ok();
}

#[test]
fn test_load_from_file_exact_64kb() {
    // Create a temporary file with exactly 64KB
    let temp_dir = std::env::temp_dir();
    let file_path = temp_dir.join("test_exact_64kb.bin");
    
    {
        let mut file = File::create(&file_path).expect("Failed to create test file");
        let data: Vec<u8> = (0..65536).map(|i| (i % 256) as u8).collect();
        file.write_all(&data).expect("Failed to write test data");
    }
    
    let mut memory = Memory::new();
    let result = memory.load_from_file(file_path.to_str().unwrap());
    
    assert!(result.is_ok(), "Loading exact 64KB file should succeed");
    
    // All bytes should match the file
    for i in 0..=0xFFFF {
        let expected = (i % 256) as u8;
        assert_eq!(memory.read(i), expected, "Memory at address 0x{:04X} should be 0x{:02X}", i, expected);
    }
    
    // Clean up
    std::fs::remove_file(&file_path).ok();
}

#[test]
fn test_load_from_file_oversized_file() {
    // Create a temporary file larger than 64KB (65KB)
    let temp_dir = std::env::temp_dir();
    let file_path = temp_dir.join("test_oversized.bin");
    
    {
        let mut file = File::create(&file_path).expect("Failed to create test file");
        // Write 65KB of data (64KB + 1KB)
        let data: Vec<u8> = (0..66560).map(|i| (i % 256) as u8).collect();
        file.write_all(&data).expect("Failed to write test data");
    }
    
    let mut memory = Memory::new();
    let result = memory.load_from_file(file_path.to_str().unwrap());
    
    assert!(result.is_ok(), "Loading oversized file should succeed");
    
    // Only the first 64KB should be loaded
    for i in 0..=0xFFFF {
        let expected = (i % 256) as u8;
        assert_eq!(memory.read(i), expected, "Memory at address 0x{:04X} should be 0x{:02X}", i, expected);
    }
    
    // Clean up
    std::fs::remove_file(&file_path).ok();
}

#[test]
fn test_load_from_file_invalid_path() {
    let mut memory = Memory::new();
    let result = memory.load_from_file("/nonexistent/path/to/file.bin");
    
    assert!(result.is_err(), "Loading from invalid path should fail");
    
    let error_msg = result.unwrap_err();
    assert!(error_msg.contains("Failed to read file"), "Error message should mention file read failure");
    assert!(error_msg.contains("/nonexistent/path/to/file.bin"), "Error message should include the file path");
}

#[test]
fn test_load_from_file_preserves_existing_data_on_error() {
    let mut memory = Memory::new();
    
    // Write some data to memory
    memory.write(0x1000, 0x42);
    memory.write(0x2000, 0xAA);
    
    // Try to load from invalid path
    let result = memory.load_from_file("/nonexistent/file.bin");
    
    assert!(result.is_err(), "Loading from invalid path should fail");
    
    // Memory should still have the original data
    assert_eq!(memory.read(0x1000), 0x42, "Original data should be preserved on error");
    assert_eq!(memory.read(0x2000), 0xAA, "Original data should be preserved on error");
}

#[test]
fn test_load_from_file_with_specific_pattern() {
    // Create a file with a specific pattern to verify correct loading
    let temp_dir = std::env::temp_dir();
    let file_path = temp_dir.join("test_pattern.bin");
    
    {
        let mut file = File::create(&file_path).expect("Failed to create test file");
        // Write a pattern: 0xAA, 0xBB, 0xCC, 0xDD repeated
        let pattern = vec![0xAA, 0xBB, 0xCC, 0xDD];
        let data: Vec<u8> = pattern.iter().cycle().take(1024).copied().collect();
        file.write_all(&data).expect("Failed to write test data");
    }
    
    let mut memory = Memory::new();
    let result = memory.load_from_file(file_path.to_str().unwrap());
    
    assert!(result.is_ok(), "Loading pattern file should succeed");
    
    // Verify the pattern in the first 1024 bytes
    let pattern = vec![0xAA, 0xBB, 0xCC, 0xDD];
    for i in 0u16..1024 {
        let expected = pattern[(i % 4) as usize];
        assert_eq!(memory.read(i), expected, "Memory at address 0x{:04X} should be 0x{:02X}", i, expected);
    }
    
    // Remaining bytes should be zero
    for i in 1024..=0xFFFF {
        assert_eq!(memory.read(i), 0, "Memory at address 0x{:04X} should be 0 (padded)", i);
    }
    
    // Clean up
    std::fs::remove_file(&file_path).ok();
}
