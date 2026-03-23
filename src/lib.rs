// 6502 CPU Emulator Library
// Exposes modules for testing and external use

pub mod memory;
pub mod cpu;
pub mod instruction;

use memory::Memory;
use cpu::Cpu;

/// Execution mode for the emulator
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExecutionMode {
    /// CPU is paused, waiting for user commands
    Paused,
    /// CPU is running continuously
    Running,
    /// CPU is executing one instruction at a time
    Stepping,
}

/// Action to take after program execution completes (CPU halts)
/// 
/// This enum represents the user's choice after the CPU halts, allowing them to:
/// - Restart execution from the current PC
/// - Load a new program
/// - View memory contents
/// - Quit the emulator
/// 
/// # Requirements
/// Validates: Requirement 19.3
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PostExecutionAction {
    /// Start execution again from current PC, preserving CPU state and memory
    StartAgain,
    /// Load a new program, resetting CPU state
    LoadNew,
    /// View memory contents before making a decision
    ViewMemory,
    /// Exit the emulator
    Quit,
}

/// Main emulator structure that wraps the CPU and provides execution control
/// 
/// The Emulator manages the overall execution state and provides the interface
/// for loading programs and controlling execution flow.
/// 
/// # Requirements
/// Validates: Requirements 1.1, 2.1, 13.2, 13.10, 14.2, 14.3, 15.2, 15.11
#[derive(Debug)]
pub struct Emulator {
    /// The CPU instance
    pub cpu: Cpu,
    /// Current execution mode
    pub mode: ExecutionMode,
    /// Start address for memory view display (256 bytes from this address)
    pub memory_view_start: u16,
    /// Optional breakpoint address (None if no breakpoint set)
    pub breakpoint: Option<u16>,
    /// Optional framebuffer base address (None if framebuffer not enabled)
    pub framebuffer_base: Option<u16>,
    /// Total number of instructions executed since program start
    pub instruction_count: u64,
    /// Time when emulator started (for calculating execution speed)
    pub start_time: std::time::Instant,
    /// Time of last status display (for periodic status updates every 5 seconds)
    pub last_status_time: std::time::Instant,
    /// Time of last framebuffer update (for decoupling rendering from execution)
    pub last_frame_time: std::time::Instant,
}

impl Emulator {
    /// Creates a new Emulator by loading a binary file and initializing the CPU
    /// 
    /// This constructor:
    /// 1. Creates a new memory instance
    /// 2. Loads the binary file into memory (padding or truncating to 64KB)
    /// 3. Creates a CPU with the loaded memory and specified start address
    /// 4. Initializes the emulator in Paused mode
    /// 
    /// # Arguments
    /// * `binary_path` - Path to the binary file to load (should be 64KB or will be adjusted)
    /// * `start_address` - The address where execution should begin (0x0000-0xFFFF)
    /// 
    /// # Returns
    /// * `Ok(Emulator)` - Successfully created emulator
    /// * `Err(String)` - Error message if file loading fails
    /// 
    /// # Requirements
    /// Validates: Requirements 1.1, 1.2, 1.3, 1.4, 1.5, 2.1, 2.2
    /// 
    /// # Examples
    /// ```no_run
    /// use cpu_6502_emulator::Emulator;
    /// 
    /// let emulator = Emulator::new("program.bin", 0x8000).expect("Failed to create emulator");
    /// ```
    pub fn new(binary_path: &str, start_address: u16) -> Result<Self, String> {
        // Create a new memory instance
        let mut memory = Memory::new();
        
        // Load the binary file into memory
        // This handles files of any size (padding or truncating to 64KB)
        memory.load_from_file(binary_path)?;
        
        // Create the CPU with the loaded memory and start address
        let cpu = Cpu::new(memory, start_address);
        
        // Create the emulator in Paused mode
        Ok(Emulator {
            cpu,
            mode: ExecutionMode::Paused,
            memory_view_start: 0x0000,
            breakpoint: None,
            framebuffer_base: None,
            instruction_count: 0,
            start_time: std::time::Instant::now(),
            last_status_time: std::time::Instant::now(),
            last_frame_time: std::time::Instant::now(),
        })
    }
    
    /// Checks if the current PC matches the breakpoint address
    /// 
    /// # Returns
    /// * `true` - Breakpoint is set and PC matches the breakpoint address
    /// * `false` - No breakpoint set or PC doesn't match
    /// 
    /// # Requirements
    /// Validates: Requirements 14.4
    pub fn check_breakpoint(&self) -> bool {
        match self.breakpoint {
            Some(addr) => self.cpu.state.pc == addr,
            None => false,
        }
    }
    
    /// Calculates the execution speed in instructions per second
    /// 
    /// Divides the total instruction count by the elapsed time since program start.
    /// Handles zero elapsed time gracefully by returning 0.0.
    /// 
    /// # Returns
    /// * `f64` - Execution speed in instructions per second
    /// 
    /// # Requirements
    /// Validates: Requirements 16.3, 16.6
    pub fn calculate_execution_speed(&self) -> f64 {
        let elapsed = self.start_time.elapsed().as_secs_f64();
        if elapsed > 0.0 {
            self.instruction_count as f64 / elapsed
        } else {
            0.0
        }
    }
    
    /// Checks if enough time has passed since the last framebuffer update
    ///
    /// This method implements the performance optimization for continuous mode by
    /// decoupling framebuffer rendering from instruction execution. It targets a
    /// refresh rate of approximately 30 FPS (33 milliseconds between updates).
    ///
    /// # Returns
    /// * `true` - If at least 33ms have elapsed since the last framebuffer update
    /// * `false` - If less than 33ms have elapsed
    ///
    /// # Requirements
    /// Validates: Requirements 17.2, 17.3
    ///
    /// # Examples
    /// ```
    /// // In Running mode, check before updating framebuffer:
    /// // if emulator.should_update_framebuffer() {
    /// //     emulator.display_framebuffer();
    /// //     emulator.last_frame_time = Instant::now();
    /// // }
    /// ```
    pub fn should_update_framebuffer(&self) -> bool {
        // Target 30 FPS: 1000ms / 30 = 33.33ms per frame
        self.last_frame_time.elapsed().as_millis() >= 33
    }
    
    /// Checks if enough time has elapsed to display status information in Running mode
    ///
    /// This method determines whether status information (CPU state and statistics)
    /// should be displayed based on the time elapsed since the last status display.
    /// Status is displayed every 5 seconds to avoid cluttering the output with too
    /// frequent updates while still providing periodic feedback during continuous execution.
    ///
    /// # Returns
    /// * `true` - If 5 seconds or more have elapsed since last status display
    /// * `false` - If less than 5 seconds have elapsed
    ///
    /// # Requirements
    /// Validates: Requirement 16.4 - Display CPU status every 5 seconds in Running mode
    ///
    /// # Examples
    /// ```
    /// // In Running mode, check before displaying status:
    /// // if emulator.should_display_status() {
    /// //     emulator.display_state();
    /// //     emulator.display_statistics();
    /// //     emulator.last_status_time = Instant::now();
    /// // }
    /// ```
    pub fn should_display_status(&self) -> bool {
        // Display status every 5 seconds (5000 milliseconds)
        self.last_status_time.elapsed().as_secs() >= 5
    }

    /// Parses a hexadecimal string to a u8 value
    ///
    /// This method parses hexadecimal strings with or without "0x" prefix
    /// and validates that the value is in the range 0x00-0xFF (0-255).
    ///
    /// # Arguments
    /// * `input` - Hexadecimal string to parse (e.g., "0x42", "42", "FF")
    ///
    /// # Returns
    /// * `Ok(u8)` - Successfully parsed value in range 0x00-0xFF
    /// * `Err(String)` - Descriptive error message for invalid input
    ///
    /// # Requirements
    /// Validates: Requirement 18.6 - Accept values in hexadecimal format (with or without 0x prefix)
    /// Validates: Requirement 18.7 - Validate that register values are in range 0x00-0xFF (8-bit)
    ///
    /// # Examples
    /// ```
    /// use cpu_6502_emulator::Emulator;
    ///
    /// let emulator = Emulator::new("test.bin", 0x8000).unwrap();
    /// assert_eq!(emulator.parse_hex_u8("0x42").unwrap(), 0x42);
    /// assert_eq!(emulator.parse_hex_u8("42").unwrap(), 0x42);
    /// assert_eq!(emulator.parse_hex_u8("FF").unwrap(), 0xFF);
    /// assert!(emulator.parse_hex_u8("100").is_err()); // Out of range
    /// assert!(emulator.parse_hex_u8("XYZ").is_err()); // Invalid hex
    /// ```
    pub fn parse_hex_u8(&self, input: &str) -> Result<u8, String> {
        // Trim whitespace
        let input = input.trim();

        // Check for empty input
        if input.is_empty() {
            return Err("Value cannot be empty. Expected hexadecimal value (0x00-0xFF). Examples: 0x42, 42, FF".to_string());
        }

        // Strip 0x or 0X prefix if present
        let hex_str = if input.starts_with("0x") || input.starts_with("0X") {
            &input[2..]
        } else {
            input
        };

        // Check if the remaining string is empty after stripping prefix
        if hex_str.is_empty() {
            return Err(format!("Invalid value format '{}'. Expected hexadecimal digits after prefix. Examples: 0x42, 42", input));
        }

        // Parse the hexadecimal string
        match u8::from_str_radix(hex_str, 16) {
            Ok(value) => {
                // u8 automatically ensures the range is 0x00-0xFF
                Ok(value)
            }
            Err(_) => {
                Err(format!(
                    "Invalid 8-bit hex value: {}. Expected 0x00-0xFF",
                    input
                ))
            }
        }
    }

    /// Parses a hexadecimal string into a u16 value
    ///
    /// Accepts values with or without "0x" or "0X" prefix.
    /// Validates that the value is in the valid range 0x0000 to 0xFFFF.
    ///
    /// # Arguments
    /// * `input` - The hexadecimal string to parse (e.g., "8000", "0x8000", "0xFFFF")
    ///
    /// # Returns
    /// * `Ok(u16)` - Successfully parsed value in range 0x0000-0xFFFF
    /// * `Err(String)` - Descriptive error message for invalid input
    ///
    /// # Requirements
    /// Validates: Requirement 18.6 - Accept values in hexadecimal format (with or without 0x prefix)
    /// Validates: Requirement 18.8 - Validate that PC values are in range 0x0000-0xFFFF (16-bit)
    ///
    /// # Examples
    /// ```
    /// use cpu_6502_emulator::Emulator;
    ///
    /// let emulator = Emulator::new("test.bin", 0x8000).unwrap();
    /// assert_eq!(emulator.parse_hex_u16("0x8000").unwrap(), 0x8000);
    /// assert_eq!(emulator.parse_hex_u16("8000").unwrap(), 0x8000);
    /// assert_eq!(emulator.parse_hex_u16("FFFF").unwrap(), 0xFFFF);
    /// assert_eq!(emulator.parse_hex_u16("0x0000").unwrap(), 0x0000);
    /// assert!(emulator.parse_hex_u16("10000").is_err()); // Out of range
    /// assert!(emulator.parse_hex_u16("XYZ").is_err()); // Invalid hex
    /// ```
    pub fn parse_hex_u16(&self, input: &str) -> Result<u16, String> {
        // Trim whitespace
        let input = input.trim();

        // Check for empty input
        if input.is_empty() {
            return Err("Value cannot be empty. Expected hexadecimal value (0x0000-0xFFFF). Examples: 0x8000, 8000, FFFF".to_string());
        }

        // Strip 0x or 0X prefix if present
        let hex_str = if input.starts_with("0x") || input.starts_with("0X") {
            &input[2..]
        } else {
            input
        };

        // Check if the remaining string is empty after stripping prefix
        if hex_str.is_empty() {
            return Err(format!("Invalid value format '{}'. Expected hexadecimal digits after prefix. Examples: 0x8000, 8000", input));
        }

        // Parse the hexadecimal string
        match u16::from_str_radix(hex_str, 16) {
            Ok(value) => {
                // u16 automatically ensures the range is 0x0000-0xFFFF
                Ok(value)
            }
            Err(_) => {
                Err(format!(
                    "Invalid 16-bit hex value: {}. Expected 0x0000-0xFFFF",
                    input
                ))
            }
        }
    }

    /// Edits a CPU register value
    ///
    /// Updates the specified register with the provided value.
    /// Supports registers: A (Accumulator), X, Y, SP (Stack Pointer), PC (Program Counter)
    ///
    /// # Arguments
    /// * `register` - Register name (case-insensitive): "A", "X", "Y", "SP", or "PC"
    /// * `value_str` - Hexadecimal value string (with or without 0x prefix)
    ///
    /// # Returns
    /// * `Ok(())` - Register successfully updated
    /// * `Err(String)` - Descriptive error message for invalid register or value
    ///
    /// # Requirements
    /// Validates: Requirement 18.1 - Parse register name (A, X, Y, SP, PC)
    /// Validates: Requirement 18.4 - Set specified register to provided value
    /// Validates: Requirement 18.6 - Accept values in hexadecimal format
    /// Validates: Requirement 18.7 - Validate register values are in range
    ///
    /// # Examples
    /// ```
    /// use cpu_6502_emulator::Emulator;
    ///
    /// let mut emulator = Emulator::new("test.bin", 0x8000).unwrap();
    /// emulator.edit_register("A", "0x42").unwrap();
    /// assert_eq!(emulator.cpu.state.a, 0x42);
    /// 
    /// emulator.edit_register("PC", "0x9000").unwrap();
    /// assert_eq!(emulator.cpu.state.pc, 0x9000);
    /// ```
    pub fn edit_register(&mut self, register: &str, value_str: &str) -> Result<(), String> {
        // Normalize register name to uppercase for case-insensitive matching
        let register = register.trim().to_uppercase();
        
        // Match register name and parse appropriate value
        match register.as_str() {
            "A" => {
                let value = self.parse_hex_u8(value_str)?;
                self.cpu.state.a = value;
                Ok(())
            }
            "X" => {
                let value = self.parse_hex_u8(value_str)?;
                self.cpu.state.x = value;
                Ok(())
            }
            "Y" => {
                let value = self.parse_hex_u8(value_str)?;
                self.cpu.state.y = value;
                Ok(())
            }
            "SP" => {
                let value = self.parse_hex_u8(value_str)?;
                self.cpu.state.sp = value;
                Ok(())
            }
            "PC" => {
                let value = self.parse_hex_u16(value_str)?;
                self.cpu.state.pc = value;
                Ok(())
            }
            _ => {
                Err(format!(
                    "Unknown register: {}. Valid registers are: A, X, Y, SP, PC",
                    register
                ))
            }
        }
    }

    /// Edits a memory location by writing a value to the specified address
    /// 
    /// Parses the memory address using parse_hex_u16() and the value using parse_hex_u8().
    /// Writes the value to memory at the specified address.
    /// 
    /// # Arguments
    /// * `address_str` - The memory address as a hex string (e.g., "0x0200", "200")
    /// * `value_str` - The value to write as a hex string (e.g., "0xFF", "FF")
    /// 
    /// # Returns
    /// * `Ok(())` - Memory successfully updated
    /// * `Err(String)` - Descriptive error message for invalid address or value
    /// 
    /// # Requirements
    /// Validates: Requirements 18.2, 18.5, 18.6, 18.7
    /// 
    /// # Examples
    /// ```
    /// use cpu_6502_emulator::Emulator;
    /// 
    /// let mut emulator = Emulator::new("test.bin", 0x8000).unwrap();
    /// emulator.edit_memory("0x0200", "0xFF").unwrap();
    /// assert_eq!(emulator.cpu.memory.read(0x0200), 0xFF);
    /// 
    /// emulator.edit_memory("300", "42").unwrap();
    /// assert_eq!(emulator.cpu.memory.read(0x0300), 0x42);
    /// ```
    pub fn edit_memory(&mut self, address_str: &str, value_str: &str) -> Result<(), String> {
        // Parse memory address using parse_hex_u16()
        let address = self.parse_hex_u16(address_str)?;
        
        // Parse value using parse_hex_u8()
        let value = self.parse_hex_u8(value_str)?;
        
        // Write value to memory at specified address
        self.cpu.memory.write(address, value);
        
        Ok(())
    }

    /// Handles the edit command for register and memory modification
    /// 
    /// Parses command format: "e REGISTER VALUE" or "e ADDRESS VALUE"
    /// Determines if first argument is register name or memory address
    /// Calls edit_register() or edit_memory() accordingly
    /// 
    /// # Arguments
    /// * `args` - Command arguments after 'e' command
    /// 
    /// # Returns
    /// * `Ok(())` - Edit successful
    /// * `Err(String)` - Invalid syntax or edit failed
    /// 
    /// # Requirements
    /// Validates: Requirements 18.1, 18.3, 18.4, 18.5, 18.8, 18.9, 18.11, 18.12
    /// 
    /// # Examples
    /// ```
    /// use cpu_6502_emulator::Emulator;
    /// 
    /// let mut emulator = Emulator::new("test.bin", 0x8000).unwrap();
    /// 
    /// // Edit register
    /// emulator.handle_edit_command("A 0x42").unwrap();
    /// assert_eq!(emulator.cpu.state.a, 0x42);
    /// 
    /// // Edit PC
    /// emulator.handle_edit_command("PC 0x9000").unwrap();
    /// assert_eq!(emulator.cpu.state.pc, 0x9000);
    /// 
    /// // Edit memory
    /// emulator.handle_edit_command("0x0200 0xFF").unwrap();
    /// assert_eq!(emulator.cpu.memory.read(0x0200), 0xFF);
    /// ```
    pub fn handle_edit_command(&mut self, args: &str) -> Result<(), String> {
        // Parse command arguments
        let parts: Vec<&str> = args.trim().split_whitespace().collect();
        
        if parts.len() != 2 {
            return Err("Invalid edit command format. Use: e <register|address> <value>".to_string());
        }
        
        let target = parts[0];
        let value_str = parts[1];
        
        // Determine if target is a register or memory address
        let target_upper = target.to_uppercase();
        match target_upper.as_str() {
            "A" | "X" | "Y" | "SP" | "PC" => {
                // Register edit
                self.edit_register(target, value_str)?;
            }
            _ => {
                // Treat as memory address
                let address = self.parse_hex_u16(target)?;
                let value = self.parse_hex_u8(value_str)?;
                self.cpu.memory.write(address, value);
            }
        }
        
        Ok(())
    }

    /// Restarts execution from the current PC
    /// 
    /// This method implements the "start again" post-execution option.
    /// It resets the instruction count and timing information while preserving
    /// the current CPU state (registers, flags, memory).
    /// 
    /// # Behavior
    /// - Resets instruction_count to 0
    /// - Resets start_time to current time
    /// - Resets last_status_time to current time
    /// - Resets last_frame_time to current time
    /// - Preserves CPU state (registers, flags, memory)
    /// - Resumes execution from current PC
    /// - Sets execution mode to Running
    /// - Clears the CPU halted flag
    /// 
    /// # Requirements
    /// Validates: Requirement 19.4
    /// 
    /// # Examples
    /// ```no_run
    /// use cpu_6502_emulator::Emulator;
    /// 
    /// let mut emulator = Emulator::new("program.bin", 0x8000)
    ///     .expect("Failed to create emulator");
    /// 
    /// // ... program executes and halts ...
    /// 
    /// // Restart execution from current PC
    /// emulator.restart_execution();
    /// ```
    pub fn restart_execution(&mut self) {
        // Reset instruction count (Requirement 19.4)
        self.instruction_count = 0;
        
        // Reset timing information (Requirement 19.4)
        let now = std::time::Instant::now();
        self.start_time = now;
        self.last_status_time = now;
        self.last_frame_time = now;
        
        // Clear the CPU halted flag to resume execution
        self.cpu.halted = false;
        
        // Set execution mode to Running (Requirement 19.4)
        self.mode = ExecutionMode::Running;
    }

}

/// Parses a hexadecimal address string into a u16 value
/// 
/// Accepts addresses with or without "0x" or "0X" prefix.
/// Validates that the address is in the valid range 0x0000 to 0xFFFF.
/// 
/// # Arguments
/// * `input` - The address string to parse (e.g., "8000", "0x8000", "0XC000")
/// 
/// # Returns
/// * `Ok(u16)` - Successfully parsed address in range 0x0000-0xFFFF
/// * `Err(String)` - Descriptive error message for invalid input
/// 
/// # Requirements
/// Validates: Requirements 12.6, 2.4
/// 
/// # Examples
/// ```
/// use cpu_6502_emulator::parse_hex_address;
/// 
/// // With 0x prefix
/// assert_eq!(parse_hex_address("0x8000").unwrap(), 0x8000);
/// 
/// // Without prefix
/// assert_eq!(parse_hex_address("8000").unwrap(), 0x8000);
/// 
/// // Uppercase prefix
/// assert_eq!(parse_hex_address("0XC000").unwrap(), 0xC000);
/// 
/// // Invalid format
/// assert!(parse_hex_address("GGGG").is_err());
/// 
/// // Empty string
/// assert!(parse_hex_address("").is_err());
/// ```
pub fn parse_hex_address(input: &str) -> Result<u16, String> {
    // Trim whitespace
    let input = input.trim();
    
    // Check for empty input
    if input.is_empty() {
        return Err("Address cannot be empty. Expected hexadecimal address (0x0000-0xFFFF). Examples: 0x8000, 8000, 0xC000, C000".to_string());
    }
    
    // Strip 0x or 0X prefix if present
    let hex_str = if input.starts_with("0x") || input.starts_with("0X") {
        &input[2..]
    } else {
        input
    };
    
    // Check if the remaining string is empty after stripping prefix
    if hex_str.is_empty() {
        return Err(format!("Invalid address format '{}'. Expected hexadecimal digits after prefix. Examples: 0x8000, 8000", input));
    }
    
    // Parse the hexadecimal string
    match u16::from_str_radix(hex_str, 16) {
        Ok(addr) => {
            // u16 automatically ensures the range is 0x0000-0xFFFF
            Ok(addr)
        }
        Err(_) => {
            Err(format!(
                "Invalid address format '{}'. Expected hexadecimal address (0x0000-0xFFFF). Examples: 0x8000, 8000, 0xC000, C000",
                input
            ))
        }
    }
}
