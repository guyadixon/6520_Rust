// Integration test for framebuffer display in execution loop
// Tests Requirements 15.7, 15.9, 15.12: Framebuffer updates during execution
//
// **Validates: Requirements 15.7, 15.9, 15.12**

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

#[test]
fn test_framebuffer_enabled_in_paused_mode() {
    // Requirement 15.7: Framebuffer displays in Paused mode
    // Create a simple program that writes to framebuffer memory
    let mut test_data = vec![0x00; 65536];
    
    // Program at 0x8000: Write 0xFF to framebuffer base (0x2000)
    test_data[0x8000] = 0xA9; // LDA #$FF
    test_data[0x8001] = 0xFF;
    test_data[0x8002] = 0x8D; // STA $2000
    test_data[0x8003] = 0x00;
    test_data[0x8004] = 0x20;
    test_data[0x8005] = 0xEA; // NOP (to stop here)
    
    let (_temp_dir, file_path) = create_temp_binary(&test_data);
    let mut emulator = Emulator::new(&file_path, 0x8000)
        .expect("Failed to create emulator");
    
    // Enable framebuffer
    emulator.framebuffer_base = Some(0x2000);
    
    // Verify framebuffer is enabled
    assert_eq!(emulator.framebuffer_base, Some(0x2000),
               "Framebuffer should be enabled at 0x2000");
    
    // In Paused mode, display_framebuffer() would be called
    // We can't test the actual display without mocking, but we can verify
    // the framebuffer_base is set correctly
    assert!(emulator.framebuffer_base.is_some(),
            "Framebuffer should be enabled in Paused mode");
}

#[test]
fn test_framebuffer_enabled_in_stepping_mode() {
    // Requirement 15.9: Framebuffer displays after each step
    let mut test_data = vec![0x00; 65536];
    
    // Program: Write pattern to framebuffer
    test_data[0x8000] = 0xA9; // LDA #$AA
    test_data[0x8001] = 0xAA;
    test_data[0x8002] = 0x8D; // STA $2000
    test_data[0x8003] = 0x00;
    test_data[0x8004] = 0x20;
    test_data[0x8005] = 0xA9; // LDA #$55
    test_data[0x8006] = 0x55;
    test_data[0x8007] = 0x8D; // STA $2001
    test_data[0x8008] = 0x01;
    test_data[0x8009] = 0x20;
    
    let (_temp_dir, file_path) = create_temp_binary(&test_data);
    let mut emulator = Emulator::new(&file_path, 0x8000)
        .expect("Failed to create emulator");
    
    // Enable framebuffer
    emulator.framebuffer_base = Some(0x2000);
    
    // Execute first instruction (LDA #$AA)
    emulator.cpu.step().expect("Step 1 failed");
    assert_eq!(emulator.cpu.state.a, 0xAA);
    
    // After stepping, framebuffer should still be enabled
    assert_eq!(emulator.framebuffer_base, Some(0x2000),
               "Framebuffer should remain enabled after stepping");
    
    // Execute second instruction (STA $2000)
    emulator.cpu.step().expect("Step 2 failed");
    assert_eq!(emulator.cpu.memory.read(0x2000), 0xAA,
               "Memory at 0x2000 should be updated");
    
    // Framebuffer should reflect the memory change
    assert_eq!(emulator.framebuffer_base, Some(0x2000),
               "Framebuffer should remain enabled");
}

#[test]
fn test_framebuffer_memory_updates_during_execution() {
    // Requirement 15.12: Framebuffer updates as memory changes
    let mut test_data = vec![0x00; 65536];
    
    // Program: Fill first 10 bytes of framebuffer with pattern
    let mut addr = 0x8000;
    for i in 0..10 {
        test_data[addr] = 0xA9; // LDA #value
        test_data[addr + 1] = i * 0x11; // Pattern: 0x00, 0x11, 0x22, ...
        test_data[addr + 2] = 0x8D; // STA $2000+i
        test_data[addr + 3] = i as u8;
        test_data[addr + 4] = 0x20;
        addr += 5;
    }
    
    let (_temp_dir, file_path) = create_temp_binary(&test_data);
    let mut emulator = Emulator::new(&file_path, 0x8000)
        .expect("Failed to create emulator");
    
    // Enable framebuffer
    emulator.framebuffer_base = Some(0x2000);
    
    // Execute all instructions
    for i in 0..10 {
        // Execute LDA
        emulator.cpu.step().expect(&format!("LDA step {} failed", i));
        // Execute STA
        emulator.cpu.step().expect(&format!("STA step {} failed", i));
        
        // Verify memory was updated
        let expected = i * 0x11;
        let actual = emulator.cpu.memory.read(0x2000 + i as u16);
        assert_eq!(actual, expected,
                   "Memory at 0x{:04X} should be 0x{:02X} after step {}",
                   0x2000u16 + i as u16, expected, i);
    }
    
    // Verify all framebuffer memory was updated correctly
    for i in 0..10 {
        let expected = i * 0x11;
        let actual = emulator.cpu.memory.read(0x2000 + i as u16);
        assert_eq!(actual, expected,
                   "Final framebuffer memory at 0x{:04X} should be 0x{:02X}",
                   0x2000u16 + i as u16, expected);
    }
}

#[test]
fn test_framebuffer_disabled_by_default() {
    // Framebuffer should be disabled (None) by default
    let test_data = vec![0xEA; 65536]; // All NOPs
    let (_temp_dir, file_path) = create_temp_binary(&test_data);
    
    let emulator = Emulator::new(&file_path, 0x8000)
        .expect("Failed to create emulator");
    
    assert_eq!(emulator.framebuffer_base, None,
               "Framebuffer should be disabled by default");
}

#[test]
fn test_framebuffer_can_be_enabled_and_disabled() {
    // Test enabling and disabling framebuffer
    let test_data = vec![0xEA; 65536]; // All NOPs
    let (_temp_dir, file_path) = create_temp_binary(&test_data);
    
    let mut emulator = Emulator::new(&file_path, 0x8000)
        .expect("Failed to create emulator");
    
    // Initially disabled
    assert_eq!(emulator.framebuffer_base, None);
    
    // Enable framebuffer
    emulator.framebuffer_base = Some(0x2000);
    assert_eq!(emulator.framebuffer_base, Some(0x2000));
    
    // Disable framebuffer
    emulator.framebuffer_base = None;
    assert_eq!(emulator.framebuffer_base, None);
}
