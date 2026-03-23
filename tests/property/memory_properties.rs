// Property-based tests for Memory module
// Tests universal properties across randomized inputs

use cpu_6502_emulator::memory::Memory;
use proptest::prelude::*;

// Property 3: Memory Read Consistency
// For any address and any byte value, after writing the value to that address,
// reading from that address should return the same value.
// **Validates: Requirements 7.1, 7.2**
proptest! {
    #[test]
    fn prop_memory_read_after_write_returns_same_value(
        address in 0u16..=0xFFFF,
        value in 0u8..=0xFF
    ) {
        let mut memory = Memory::new();
        memory.write(address, value);
        let read_value = memory.read(address);
        prop_assert_eq!(read_value, value, 
            "Memory read consistency failed: wrote 0x{:02X} to address 0x{:04X} but read 0x{:02X}",
            value, address, read_value);
    }
}

// Property 4: Little-Endian Word Access
// For any 16-bit address, when reading a word from memory, the byte at address
// should be the low byte and the byte at address+1 should be the high byte.
// **Validates: Requirements 7.4**
proptest! {
    #[test]
    fn prop_little_endian_word_access(
        address in 0u16..=0xFFFF,
        low_byte in 0u8..=0xFF,
        high_byte in 0u8..=0xFF
    ) {
        let mut memory = Memory::new();
        
        // Write low byte at address and high byte at address+1
        memory.write(address, low_byte);
        memory.write(address.wrapping_add(1), high_byte);
        
        // Read word should combine them in little-endian format
        let word = memory.read_word(address);
        let expected_word = ((high_byte as u16) << 8) | (low_byte as u16);
        
        prop_assert_eq!(word, expected_word,
            "Little-endian word access failed: wrote low=0x{:02X} at 0x{:04X} and high=0x{:02X} at 0x{:04X}, \
             expected word 0x{:04X} but got 0x{:04X}",
            low_byte, address, high_byte, address.wrapping_add(1), expected_word, word);
    }
}


// Property 1: File Loading Correctness
// For any binary file, when loaded into memory:
// - If the file is exactly 64KB, all bytes should match the file contents
// - If the file is smaller than 64KB, the first N bytes should match the file and remaining bytes should be zero
// - If the file is larger than 64KB, the first 64KB should match the file's first 64KB
// **Validates: Requirements 1.2, 1.3, 1.4**
proptest! {
    #[test]
    fn prop_file_loading_correctness(
        file_size in 0usize..=70000,
        seed in 0u8..=255
    ) {
        use std::fs::File;
        use std::io::Write;
        
        // Create a temporary file with random data
        let temp_dir = std::env::temp_dir();
        let file_path = temp_dir.join(format!("test_prop_file_{}.bin", seed));
        
        // Generate test data based on seed
        let test_data: Vec<u8> = (0..file_size).map(|i| ((i + seed as usize) % 256) as u8).collect();
        
        {
            let mut file = File::create(&file_path).expect("Failed to create test file");
            file.write_all(&test_data).expect("Failed to write test data");
        }
        
        let mut memory = Memory::new();
        let result = memory.load_from_file(file_path.to_str().unwrap());
        
        prop_assert!(result.is_ok(), "File loading should succeed for valid file");
        
        // Determine how many bytes should be loaded (min of file_size and 64KB)
        let bytes_to_check = std::cmp::min(file_size, 65536);
        
        // Check that loaded bytes match the file
        for i in 0..bytes_to_check {
            let expected = test_data[i];
            let actual = memory.read(i as u16);
            prop_assert_eq!(actual, expected,
                "Memory at address 0x{:04X} should be 0x{:02X} but got 0x{:02X}",
                i, expected, actual);
        }
        
        // If file was smaller than 64KB, check that remaining bytes are zero
        if file_size < 65536 {
            for i in file_size..65536 {
                let actual = memory.read(i as u16);
                prop_assert_eq!(actual, 0,
                    "Memory at address 0x{:04X} should be 0 (padded) but got 0x{:02X}",
                    i, actual);
            }
        }
        
        // Clean up
        std::fs::remove_file(&file_path).ok();
    }
}
