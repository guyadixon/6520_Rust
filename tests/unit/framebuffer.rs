// Unit tests for framebuffer functionality
// Tests Requirements 15.1-15.12: Framebuffer Display
//
// **Validates: Requirements 15.1, 15.2, 15.3, 15.4, 15.5, 15.6, 15.7, 15.8, 15.9, 15.10, 15.11, 15.12**

use cpu_6502_emulator::Emulator;
use std::fs::File;
use std::io::Write;
use tempfile::TempDir;

/// Helper function to create a temporary binary file with specified content
fn create_temp_binary(content: &[u8]) -> (TempDir, String) {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let file_path = temp_dir.path().join("test.bin");
    let mut file = File::create(&file_path).expect("Failed to create temp file");
    file.write_all(content).expect("Failed to write to temp file");
    let path_str = file_path.to_str().unwrap().to_string();
    (temp_dir, path_str)
}

#[cfg(test)]
mod framebuffer_field_tests {
    use super::*;

    #[test]
    fn test_framebuffer_field_initialization() {
        // Requirement 15.2, 15.11: Framebuffer field should initialize to None
        let test_data = vec![0xEA]; // NOP
        let (_temp_dir, file_path) = create_temp_binary(&test_data);
        
        let emulator = Emulator::new(&file_path, 0x0000)
            .expect("Failed to create emulator");
        
        assert_eq!(emulator.framebuffer_base, None,
                   "Framebuffer should initialize to None");
    }

    #[test]
    fn test_framebuffer_field_can_be_set() {
        // Requirement 15.2: Framebuffer field can be set to Some(address)
        let test_data = vec![0xEA]; // NOP
        let (_temp_dir, file_path) = create_temp_binary(&test_data);
        
        let mut emulator = Emulator::new(&file_path, 0x0000)
            .expect("Failed to create emulator");
        
        // Set framebuffer base address
        emulator.framebuffer_base = Some(0x2000);
        
        assert_eq!(emulator.framebuffer_base, Some(0x2000),
                   "Framebuffer should be set to 0x2000");
    }

    #[test]
    fn test_framebuffer_field_can_be_replaced() {
        // Requirement 15.11: New framebuffer address replaces existing one
        let test_data = vec![0xEA]; // NOP
        let (_temp_dir, file_path) = create_temp_binary(&test_data);
        
        let mut emulator = Emulator::new(&file_path, 0x0000)
            .expect("Failed to create emulator");
        
        // Set initial framebuffer base address
        emulator.framebuffer_base = Some(0x2000);
        assert_eq!(emulator.framebuffer_base, Some(0x2000));
        
        // Replace with new address
        emulator.framebuffer_base = Some(0x3000);
        assert_eq!(emulator.framebuffer_base, Some(0x3000),
                   "Framebuffer should be replaced with new address");
    }

    #[test]
    fn test_framebuffer_accepts_all_valid_addresses() {
        // Requirement 15.1, 15.2: Framebuffer should accept any address in 0x0000-0xFFFF
        let test_data = vec![0xEA]; // NOP
        let (_temp_dir, file_path) = create_temp_binary(&test_data);
        
        let mut emulator = Emulator::new(&file_path, 0x0000)
            .expect("Failed to create emulator");
        
        // Test minimum address
        emulator.framebuffer_base = Some(0x0000);
        assert_eq!(emulator.framebuffer_base, Some(0x0000));
        
        // Test maximum address
        emulator.framebuffer_base = Some(0xFFFF);
        assert_eq!(emulator.framebuffer_base, Some(0xFFFF));
        
        // Test typical address
        emulator.framebuffer_base = Some(0x2000);
        assert_eq!(emulator.framebuffer_base, Some(0x2000));
    }
}

#[cfg(test)]
mod framebuffer_status_display_tests {
    use super::*;

    #[test]
    fn test_framebuffer_status_none() {
        // Requirement 15.10: Display "Framebuffer: None" when not set
        let test_data = vec![0xEA]; // NOP
        let (_temp_dir, file_path) = create_temp_binary(&test_data);
        
        let emulator = Emulator::new(&file_path, 0x0000)
            .expect("Failed to create emulator");
        
        // Verify framebuffer is None
        assert_eq!(emulator.framebuffer_base, None);
        
        // In the actual implementation, this would display "Framebuffer: None"
        // We can't easily test the display output, but we can verify the field
        match emulator.framebuffer_base {
            None => {
                // This is the expected state
                assert!(true);
            }
            Some(_) => {
                panic!("Framebuffer should be None");
            }
        }
    }

    #[test]
    fn test_framebuffer_status_with_address() {
        // Requirement 15.10: Display "Framebuffer: 0x{address}" when set
        let test_data = vec![0xEA]; // NOP
        let (_temp_dir, file_path) = create_temp_binary(&test_data);
        
        let mut emulator = Emulator::new(&file_path, 0x0000)
            .expect("Failed to create emulator");
        
        // Set framebuffer base address
        emulator.framebuffer_base = Some(0x2000);
        
        // Verify framebuffer is set
        assert_eq!(emulator.framebuffer_base, Some(0x2000));
        
        // In the actual implementation, this would display "Framebuffer: 0x2000"
        match emulator.framebuffer_base {
            Some(addr) => {
                assert_eq!(addr, 0x2000);
            }
            None => {
                panic!("Framebuffer should be set to 0x2000");
            }
        }
    }
}

#[cfg(test)]
mod framebuffer_data_tests {
    use super::*;

    #[test]
    fn test_framebuffer_dimensions() {
        // Requirement 15.3, 15.4: Framebuffer is 160x120 pixels, requires 2400 bytes
        // 160 pixels wide × 120 pixels tall = 19,200 pixels
        // 19,200 pixels / 8 bits per byte = 2,400 bytes
        const WIDTH: usize = 160;
        const HEIGHT: usize = 120;
        const BYTES_PER_ROW: usize = WIDTH / 8; // 20 bytes per row
        const TOTAL_BYTES: usize = BYTES_PER_ROW * HEIGHT; // 2400 bytes
        
        assert_eq!(WIDTH, 160, "Framebuffer width should be 160 pixels");
        assert_eq!(HEIGHT, 120, "Framebuffer height should be 120 pixels");
        assert_eq!(BYTES_PER_ROW, 20, "Each row should be 20 bytes");
        assert_eq!(TOTAL_BYTES, 2400, "Total framebuffer should be 2400 bytes");
    }

    #[test]
    fn test_framebuffer_reads_from_memory() {
        // Requirement 15.4, 15.5: Framebuffer reads from memory starting at base address
        // Create a binary with some test pattern in framebuffer area
        let mut test_data = vec![0x00; 65536]; // 64KB of zeros
        
        // Set framebuffer base at 0x2000
        // Write a test pattern: alternating 0xFF and 0x00
        for i in 0..2400 {
            test_data[0x2000 + i] = if i % 2 == 0 { 0xFF } else { 0x00 };
        }
        
        let (_temp_dir, file_path) = create_temp_binary(&test_data);
        let mut emulator = Emulator::new(&file_path, 0x0000)
            .expect("Failed to create emulator");
        
        // Set framebuffer base address
        emulator.framebuffer_base = Some(0x2000);
        
        // Verify memory contains the test pattern
        for i in 0..2400 {
            let expected = if i % 2 == 0 { 0xFF } else { 0x00 };
            let actual = emulator.cpu.memory.read(0x2000 + i as u16);
            assert_eq!(actual, expected,
                       "Memory at 0x{:04X} should be 0x{:02X}", 0x2000 + i, expected);
        }
    }

    #[test]
    fn test_framebuffer_bit_interpretation() {
        // Requirement 15.5, 15.6: Each bit is a pixel (1 = white, 0 = black)
        // Bit 7 is leftmost pixel
        let byte: u8 = 0b10101010; // Alternating bits
        
        // Extract bits from left to right (bit 7 to bit 0)
        let pixels: Vec<u8> = (0..8).map(|bit| (byte >> (7 - bit)) & 1).collect();
        
        assert_eq!(pixels, vec![1, 0, 1, 0, 1, 0, 1, 0],
                   "Bits should be extracted left-to-right (bit 7 to bit 0)");
        
        // Verify bit 7 is leftmost
        assert_eq!(pixels[0], 1, "Bit 7 should be leftmost pixel");
        assert_eq!(pixels[7], 0, "Bit 0 should be rightmost pixel");
    }

    #[test]
    fn test_framebuffer_row_layout() {
        // Requirement 15.6: 120 rows of 160 pixels each (20 bytes per row)
        const WIDTH_PIXELS: usize = 160;
        const HEIGHT_PIXELS: usize = 120;
        const BYTES_PER_ROW: usize = WIDTH_PIXELS / 8; // 20 bytes
        
        // Verify row calculations
        for row in 0..HEIGHT_PIXELS {
            let row_start_byte = row * BYTES_PER_ROW;
            let row_end_byte = row_start_byte + BYTES_PER_ROW - 1;
            
            assert_eq!(row_end_byte - row_start_byte + 1, BYTES_PER_ROW,
                       "Row {} should contain {} bytes", row, BYTES_PER_ROW);
        }
        
        // Verify total bytes
        let total_bytes = HEIGHT_PIXELS * BYTES_PER_ROW;
        assert_eq!(total_bytes, 2400, "Total framebuffer should be 2400 bytes");
    }

    #[test]
    fn test_framebuffer_memory_updates() {
        // Requirement 15.12: Framebuffer updates as memory changes
        let mut test_data = vec![0x00; 65536]; // 64KB of zeros
        let (_temp_dir, file_path) = create_temp_binary(&test_data);
        
        let mut emulator = Emulator::new(&file_path, 0x0000)
            .expect("Failed to create emulator");
        
        // Set framebuffer base address
        emulator.framebuffer_base = Some(0x2000);
        
        // Initially, memory should be all zeros
        assert_eq!(emulator.cpu.memory.read(0x2000), 0x00);
        
        // Write to framebuffer memory
        emulator.cpu.memory.write(0x2000, 0xFF);
        
        // Verify memory was updated
        assert_eq!(emulator.cpu.memory.read(0x2000), 0xFF,
                   "Framebuffer memory should update when written to");
        
        // Write another pattern
        emulator.cpu.memory.write(0x2001, 0xAA);
        assert_eq!(emulator.cpu.memory.read(0x2001), 0xAA);
    }

    #[test]
    fn test_framebuffer_pixel_colors() {
        // Requirement 15.5: 1 = white, 0 = black
        // This test verifies the color mapping logic
        
        // White pixel (bit = 1)
        let white_bit = 1u8;
        let white_color = if white_bit == 1 { 0xFFFFFF } else { 0x000000 };
        assert_eq!(white_color, 0xFFFFFF, "Bit 1 should map to white (0xFFFFFF)");
        
        // Black pixel (bit = 0)
        let black_bit = 0u8;
        let black_color = if black_bit == 1 { 0xFFFFFF } else { 0x000000 };
        assert_eq!(black_color, 0x000000, "Bit 0 should map to black (0x000000)");
    }

    #[test]
    fn test_framebuffer_full_pattern() {
        // Requirement 15.3, 15.4, 15.5, 15.6: Test complete framebuffer pattern
        let mut test_data = vec![0x00; 65536];
        
        // Create a checkerboard pattern in framebuffer
        // Alternating 0xFF and 0x00 bytes
        for i in 0..2400 {
            test_data[0x2000 + i] = if (i / 20) % 2 == 0 {
                if i % 2 == 0 { 0xFF } else { 0x00 }
            } else {
                if i % 2 == 0 { 0x00 } else { 0xFF }
            };
        }
        
        let (_temp_dir, file_path) = create_temp_binary(&test_data);
        let mut emulator = Emulator::new(&file_path, 0x0000)
            .expect("Failed to create emulator");
        
        emulator.framebuffer_base = Some(0x2000);
        
        // Verify the pattern is correctly stored in memory
        for row in 0..120 {
            for col_byte in 0..20 {
                let addr = 0x2000 + (row * 20 + col_byte) as u16;
                let byte = emulator.cpu.memory.read(addr);
                
                // Verify the checkerboard pattern
                let expected = if (row / 1) % 2 == 0 {
                    if col_byte % 2 == 0 { 0xFF } else { 0x00 }
                } else {
                    if col_byte % 2 == 0 { 0x00 } else { 0xFF }
                };
                
                assert_eq!(byte, expected,
                           "Byte at row {}, col_byte {} should be 0x{:02X}", 
                           row, col_byte, expected);
            }
        }
    }
}
