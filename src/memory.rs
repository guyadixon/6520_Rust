// Memory module for the 6502 CPU emulator
// Manages the 64KB RAM address space

/// Represents the 64KB memory address space of the 6502 CPU
#[derive(Debug)]
pub struct Memory {
    ram: [u8; 65536],
}

impl Memory {
    /// Creates a new Memory instance with all bytes initialized to zero
    pub fn new() -> Self {
        Memory {
            ram: [0; 65536],
        }
    }

    /// Reads a single byte from the specified address
    pub fn read(&self, address: u16) -> u8 {
        self.ram[address as usize]
    }

    /// Writes a single byte to the specified address
    pub fn write(&mut self, address: u16, value: u8) {
        self.ram[address as usize] = value;
    }

    /// Reads a 16-bit word from the specified address in little-endian format
    /// The byte at `address` is the low byte, and the byte at `address + 1` is the high byte
    pub fn read_word(&self, address: u16) -> u16 {
        let low = self.read(address) as u16;
        let high = self.read(address.wrapping_add(1)) as u16;
        (high << 8) | low
    }

    /// Loads a binary file into memory at a specific offset address
    /// - Memory before the offset is zeroed
    /// - The binary is loaded starting at the offset address
    /// - If the file would exceed 64KB when loaded at offset, it's truncated
    /// - Memory after the loaded data is left as zero
    /// 
    /// # Arguments
    /// * `path` - Path to the binary file
    /// * `offset` - Address where the binary should be loaded (0x0000-0xFFFF)
    /// 
    /// # Returns
    /// * `Ok(())` - File loaded successfully
    /// * `Err(String)` - Error message if loading fails
    pub fn load_from_file_at_offset(&mut self, path: &str, offset: u16) -> Result<(), String> {
        use std::fs::File;
        use std::io::Read;

        let mut file = File::open(path)
            .map_err(|e| format!("Failed to read file '{}': {}", path, e))?;

        // Calculate how many bytes we can load from the offset
        let offset_usize = offset as usize;
        let max_bytes = 65536 - offset_usize;

        // Read file into a temporary buffer
        let mut file_data = Vec::new();
        file.read_to_end(&mut file_data)
            .map_err(|e| format!("Failed to read file '{}': {}", path, e))?;

        // Determine how many bytes to actually load
        let bytes_to_load = std::cmp::min(file_data.len(), max_bytes);

        // Zero out memory before the offset
        for i in 0..offset_usize {
            self.ram[i] = 0;
        }

        // Load the file data at the offset
        for i in 0..bytes_to_load {
            self.ram[offset_usize + i] = file_data[i];
        }

        // Zero out memory after the loaded data
        for i in (offset_usize + bytes_to_load)..65536 {
            self.ram[i] = 0;
        }

        Ok(())
    }

    /// Loads a binary file into memory
    /// - If the file is smaller than 64KB, remaining bytes are left as zero
    /// - If the file is larger than 64KB, only the first 64KB is loaded
    pub fn load_from_file(&mut self, path: &str) -> Result<(), String> {
        use std::fs::File;
        use std::io::Read;

        let mut file = File::open(path)
            .map_err(|e| format!("Failed to read file '{}': {}", path, e))?;

        let _bytes_read = file.read(&mut self.ram)
            .map_err(|e| format!("Failed to read file '{}': {}", path, e))?;

        // File is automatically padded with zeros (already initialized)
        // or truncated (we only read up to 64KB)
        Ok(())
    }
}

impl Default for Memory {
    fn default() -> Self {
        Self::new()
    }
}
