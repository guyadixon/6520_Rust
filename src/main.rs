// 6502 CPU Emulator
// A faithful emulation of the MOS Technology 6502 microprocessor

mod memory;
mod cpu;
mod instruction;

use memory::Memory;
use cpu::Cpu;
use cpu_6502_emulator::parse_hex_address;
use std::collections::HashSet;
use std::time::Instant;

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
pub struct Emulator {
    /// The CPU instance
    pub cpu: Cpu,
    /// Current execution mode
    pub mode: ExecutionMode,
    /// Start address for memory view display (256 bytes from this address)
    pub memory_view_start: u16,
    /// Set of breakpoint addresses
    pub breakpoints: HashSet<u16>,
    /// Optional framebuffer base address (None if framebuffer not enabled)
    pub framebuffer_base: Option<u16>,
    /// Optional persistent framebuffer window
    framebuffer_window: Option<minifb::Window>,
    /// Total number of instructions executed since program start
    pub instruction_count: u64,
    /// Time when emulator started (for calculating execution speed)
    pub start_time: Instant,
    /// Time of last status display (for periodic status updates every 5 seconds)
    pub last_status_time: Instant,
    /// Time of last framebuffer update (for decoupling rendering from execution)
    pub last_frame_time: Instant,
}

impl Emulator {
    /// Creates a new Emulator by loading a binary file and initializing the CPU
    /// 
    /// This constructor:
    /// 1. Creates a new memory instance
    /// 2. Loads the binary file into memory at the specified load address
    /// 3. Zeros all memory before the load address
    /// 4. Creates a CPU with the loaded memory, starting execution at the load address
    /// 5. Initializes the emulator in Paused mode
    /// 
    /// # Arguments
    /// * `binary_path` - Path to the binary file to load
    /// * `load_address` - The address where the binary should be loaded and execution should begin (0x0000-0xFFFF)
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
    pub fn new(binary_path: &str, load_address: u16) -> Result<Self, String> {
        // Create a new memory instance
        let mut memory = Memory::new();
        
        // Load the binary file into memory at the specified offset
        // This zeros memory before the offset and loads the file starting at the offset
        memory.load_from_file_at_offset(binary_path, load_address)?;
        
        // Create the CPU with the loaded memory, starting execution at the load address
        let cpu = Cpu::new(memory, load_address);
        
        // Initialize timing for statistics
        let now = Instant::now();
        
        // Create the emulator in Paused mode
        Ok(Emulator {
            cpu,
            mode: ExecutionMode::Paused,
            memory_view_start: 0x0000,
            breakpoints: HashSet::new(),
            framebuffer_base: None,
            framebuffer_window: None,
            instruction_count: 0,
            start_time: now,
            last_status_time: now,
            last_frame_time: now,
        })
    }

    /// Main execution loop for the emulator
    /// 
    /// This method implements the interactive execution control:
    /// - In Paused mode: displays CPU state and waits for user commands
    /// - In Stepping mode: executes one instruction, displays state, then pauses
    /// - In Running mode: executes instructions continuously until paused or error
    /// 
    /// The loop continues until the user quits or an error occurs.
    /// 
    /// # Returns
    /// * `true` - User wants to restart with a new program
    /// * `false` - User wants to quit
    /// 
    /// # Requirements
    /// Validates: Requirements 6.1, 6.2, 6.3, 6.4, 6.5, 6.6, 10.4, 11.4
    pub fn run(&mut self) -> bool {
        use std::io::{self, Write};

        println!("\n6502 CPU Emulator - Interactive Mode");
        println!("Commands: s/n (step), c (continue), r (run), p (pause), m (memory), b (breakpoint), f (framebuffer), q (quit)");

        loop {
            match self.mode {
                ExecutionMode::Paused => {
                    // Check if CPU is halted - if so, handle post-execution options
                    if self.cpu.halted {
                        return self.handle_halt();
                    }
                    
                    // Clear screen before displaying state
                    Self::clear_screen();
                    
                    // Display CPU state when paused
                    self.display_state();
                    
                    // Display statistics after state in Paused mode
                    self.display_statistics();
                    
                    // Display memory view when paused
                    self.display_memory_view();
                    
                    // Display framebuffer when paused (if enabled)
                    if self.framebuffer_base.is_some() {
                        self.display_framebuffer();
                    }
                    
                    // Display breakpoint status
                    if self.breakpoints.is_empty() {
                        println!("\nBreakpoints: None");
                    } else {
                        println!("\nBreakpoints:");
                        let mut sorted_breakpoints: Vec<_> = self.breakpoints.iter().collect();
                        sorted_breakpoints.sort();
                        for (idx, addr) in sorted_breakpoints.iter().enumerate() {
                            println!("  #{}: 0x{:04X}", idx + 1, addr);
                        }
                    }
                    
                    // Display framebuffer status
                    match self.framebuffer_base {
                        Some(addr) => println!("Framebuffer: 0x{:04X}", addr),
                        None => println!("Framebuffer: None"),
                    }
                    
                    // Prompt for user command
                    print!("\n> ");
                    // Gracefully handle flush errors instead of panicking
                    if let Err(e) = io::stdout().flush() {
                        eprintln!("Warning: Failed to flush output: {}", e);
                        // Continue execution - this is not a fatal error
                    }
                    
                    // Read user input
                    let mut input = String::new();
                    match io::stdin().read_line(&mut input) {
                        Ok(0) => {
                            // EOF (Ctrl+D) - treat as quit command
                            println!("\nEnd of input detected. Quitting...");
                            return false;
                        }
                        Ok(_) => {
                            // Process the command
                            let command = input.trim();
                            if !self.handle_command(command) {
                                // handle_command returns false when user wants to quit
                                return false;
                            }
                        }
                        Err(e) => {
                            eprintln!("Error reading input: {}", e);
                            eprintln!("Please try again or press Ctrl+C to exit.");
                            continue;
                        }
                    }
                }
                
                ExecutionMode::Stepping => {
                    // Check for end of code before stepping
                    if self.cpu.is_at_end_of_code() {
                        self.cpu.halted = true;
                        self.cpu.halt_reason = Some(cpu::HaltReason::EndOfCode);
                        self.display_halt_notification("End of code reached");
                        self.mode = ExecutionMode::Paused;
                        continue;
                    }
                    
                    // Execute one instruction
                    match self.cpu.step() {
                        Ok(()) => {
                            // Increment instruction count after successful execution
                            self.instruction_count += 1;
                            
                            // Clear screen before displaying state
                            Self::clear_screen();
                            
                            // Display the new state after stepping
                            self.display_state();
                            
                            // Display statistics after state in Stepping mode
                            self.display_statistics();
                            
                            // Display memory view after stepping
                            self.display_memory_view();
                            // Display framebuffer after stepping (if enabled)
                            if self.framebuffer_base.is_some() {
                                self.display_framebuffer();
                            }
                            // Return to paused mode after one step
                            self.mode = ExecutionMode::Paused;
                        }
                        Err(err) => {
                            // Mark CPU as halted so handle_halt() will be called in Paused mode
                            self.cpu.halted = true;
                            // Display halt notification
                            self.display_halt_notification(&err);
                            self.mode = ExecutionMode::Paused;
                        }
                    }
                }
                
                ExecutionMode::Running => {
                    // Check for end of code before stepping
                    if self.cpu.is_at_end_of_code() {
                        self.cpu.halted = true;
                        self.cpu.halt_reason = Some(cpu::HaltReason::EndOfCode);
                        self.display_halt_notification("End of code reached");
                        self.mode = ExecutionMode::Paused;
                        continue;
                    }
                    
                    // Check for breakpoint before stepping
                    if self.check_breakpoint() {
                        println!("\nBreakpoint hit at 0x{:04X}", self.cpu.state.pc);
                        self.mode = ExecutionMode::Paused;
                        continue;
                    }
                    
                    // Execute instructions continuously without waiting for display updates
                    match self.cpu.step() {
                        Ok(()) => {
                            // Increment instruction count after successful execution
                            self.instruction_count += 1;
                            
                            // Check if enough time has passed to display status (every 5 seconds)
                            if self.should_display_status() {
                                Self::clear_screen();
                                self.display_state();
                                self.display_statistics();
                                self.last_status_time = Instant::now();
                            }
                            
                            // Check if enough time has passed to update framebuffer (~30 FPS)
                            if self.framebuffer_base.is_some() && self.should_update_framebuffer() {
                                self.display_framebuffer();
                                self.last_frame_time = Instant::now();
                            }
                            // Check if user wants to pause (non-blocking check would be better)
                            // For now, we'll just keep running
                            // In a real implementation, we'd use non-blocking I/O or threads
                        }
                        Err(err) => {
                            // Mark CPU as halted so handle_halt() will be called in Paused mode
                            self.cpu.halted = true;
                            // Display halt notification
                            self.display_halt_notification(&err);
                            self.mode = ExecutionMode::Paused;
                        }
                    }
                }
            }
        }
    }

    /// Handles user commands in interactive mode
    /// 
    /// Processes single-character commands:
    /// - 's' or 'n': Step (execute one instruction)
    /// - 'c' or 'r': Continue (resume execution)
    /// - 'p': Pause (if running)
    /// - 'm': Memory (change memory view start address)
    /// - 'b [address]': Add breakpoint at address (e.g., "b 0x0200" or "b $0200")
    /// - 'b -[address]': Remove breakpoint at address (e.g., "b -0x0200" or "b -$0200")
    /// - 'q': Quit
    /// 
    /// # Arguments
    /// * `command` - The user command string (trimmed)
    /// 
    /// # Returns
    /// * `true` - Continue execution loop
    /// * `false` - Exit execution loop (quit command)
    /// 
    /// # Requirements
    /// Validates: Requirements 6.1, 6.3, 6.5, 6.6, 13.7, 13.8
    fn handle_command(&mut self, command: &str) -> bool {
        // Check if command starts with 'b' for breakpoint operations
        if command.starts_with("b ") || command.starts_with("b-") {
            return self.handle_breakpoint_command(command);
        }
        
        match command.to_lowercase().as_str() {
            "s" | "n" | "step" | "next" => {
                // Step: execute one instruction
                self.mode = ExecutionMode::Stepping;
                true
            }
            
            "c" | "r" | "continue" | "run" => {
                // Continue: resume execution
                println!("Running... (pause not yet implemented in continuous mode)");
                self.mode = ExecutionMode::Running;
                true
            }
            
            "p" | "pause" => {
                // Pause: halt execution
                if self.mode == ExecutionMode::Running {
                    println!("Paused.");
                    self.mode = ExecutionMode::Paused;
                } else {
                    println!("Already paused.");
                }
                true
            }
            
            "m" | "memory" => {
                // Memory: change memory view start address
                self.prompt_for_memory_address();
                true
            }
            
            "b" | "breakpoint" => {
                // Breakpoint: show usage
                println!("Breakpoint commands:");
                println!("  b [address]  - Add breakpoint (e.g., 'b 0x0200' or 'b $0200')");
                println!("  b -[address] - Remove breakpoint (e.g., 'b -0x0200' or 'b -$0200')");
                true
            }
            
            "f" | "framebuffer" => {
                // Framebuffer: set framebuffer base address
                self.prompt_for_framebuffer();
                true
            }
            
            _ if command.starts_with("e ") || command.starts_with("edit ") => {
                // Edit: modify register or memory value
                // Only allow in Paused and Stepping modes
                if self.mode == ExecutionMode::Running {
                    println!("Edit command only available in Paused or Stepping mode");
                    return true;
                }
                
                // Extract arguments after 'e' or 'edit'
                let args = if command.starts_with("edit ") {
                    &command[5..]
                } else {
                    &command[2..]
                };
                
                match self.handle_edit_command(args) {
                    Ok(()) => {
                        // Update display after successful edit
                        self.display_state();
                        self.display_memory_view();
                    }
                    Err(err) => {
                        eprintln!("Error: {}", err);
                    }
                }
                true
            }
            
            "q" | "quit" | "exit" => {
                // Quit: exit the emulator
                println!("Quitting...");
                false
            }
            
            "" => {
                // Empty command - just re-prompt
                true
            }
            
            _ => {
                // Unknown command
                println!("Unknown command: '{}'", command);
                println!("Commands: s/n (step), c (continue) r (run), p (pause), m (memory), b [addr] (breakpoint), f (framebuffer), e <target> <value> (edit), q (quit)");
                true
            }
        }
    }
    
    /// Handles breakpoint add/remove commands
    /// 
    /// Supports:
    /// - `b [address]` - Add breakpoint (e.g., "b 0x0200", "b $0200", "b 0200")
    /// - `b -[address]` - Remove breakpoint (e.g., "b -0x0200", "b -$0200", "b -0200")
    /// 
    /// # Arguments
    /// * `command` - The full command string
    /// 
    /// # Returns
    /// * `true` - Always continues execution
    fn handle_breakpoint_command(&mut self, command: &str) -> bool {
        let parts: Vec<&str> = command.split_whitespace().collect();
        
        if parts.len() != 2 {
            println!("Invalid breakpoint command. Usage:");
            println!("  b [address]  - Add breakpoint (e.g., 'b 0x0200')");
            println!("  b -[address] - Remove breakpoint (e.g., 'b -0x0200')");
            return true;
        }
        
        let addr_str = parts[1];
        
        // Check if this is a remove operation
        if addr_str.starts_with('-') {
            // Remove breakpoint
            let addr_part = &addr_str[1..]; // Skip the '-'
            
            // Handle $ prefix for hex addresses
            let addr_to_parse = if addr_part.starts_with('$') {
                &addr_part[1..]
            } else {
                addr_part
            };
            
            match parse_hex_address(addr_to_parse) {
                Ok(addr) => {
                    if self.breakpoints.remove(&addr) {
                        println!("Breakpoint removed at 0x{:04X}", addr);
                    } else {
                        println!("No breakpoint found at 0x{:04X}", addr);
                    }
                }
                Err(err) => {
                    eprintln!("Error: {}", err);
                }
            }
        } else {
            // Add breakpoint
            // Handle $ prefix for hex addresses
            let addr_to_parse = if addr_str.starts_with('$') {
                &addr_str[1..]
            } else {
                addr_str
            };
            
            match parse_hex_address(addr_to_parse) {
                Ok(addr) => {
                    if self.breakpoints.insert(addr) {
                        println!("Breakpoint added at 0x{:04X}", addr);
                    } else {
                        println!("Breakpoint already exists at 0x{:04X}", addr);
                    }
                }
                Err(err) => {
                    eprintln!("Error: {}", err);
                }
            }
        }
        
        true
    }

    /// Displays a halt notification with the reason and final CPU state
    /// 
    /// Shows:
    /// - Clear message indicating execution has stopped
    /// - The reason for halting (invalid opcode, BRK instruction, or end of code)
    /// - Final CPU state with all registers and flags
    /// 
    /// # Arguments
    /// * `error_msg` - The error message from the failed step
    /// 
    /// # Requirements
    /// Validates: Requirements 11.1, 11.2, 11.3
    fn display_halt_notification(&self, error_msg: &str) {
        println!("\n╔════════════════════════════════════════════════════════════╗");
        println!("║              EXECUTION HALTED                              ║");
        println!("╚════════════════════════════════════════════════════════════╝");
        
        // Determine and display the halt reason
        if let Some(reason) = self.cpu.halt_reason {
            println!("\nReason: {}", match reason {
                cpu::HaltReason::InvalidOpcode => "Invalid opcode encountered",
                cpu::HaltReason::BrkInstruction => "BRK instruction executed",
                cpu::HaltReason::EndOfCode => "End of code reached",
            });
        } else {
            println!("\nReason: Unknown");
        }
        
        // Display the error details
        println!("Details: {}", error_msg);
        
        // Display final CPU state
        println!("\n┌─────────────────────────────────────────────────────────┐");
        println!("│                    FINAL CPU STATE                      │");
        println!("└─────────────────────────────────────────────────────────┘");
        self.display_state();
    }
    
    /// Prompts the user to restart with a new program or quit
    /// 
    /// Returns true if the user wants to restart, false if they want to quit.
    /// 
    /// # Requirements
    /// Validates: Requirements 11.4
    /// Handles post-execution options after CPU halts
    /// 
    /// This method orchestrates the complete post-execution workflow:
    /// 1. Calls prompt_post_execution_options() to get user choice
    /// 2. Matches on PostExecutionAction and executes corresponding action:
    ///    - StartAgain: calls restart_execution() and continues
    ///    - LoadNew: calls load_new_program() and continues
    ///    - ViewMemory: calls handle_memory_view_loop() and re-prompts
    ///    - Quit: exits gracefully
    /// 
    /// # Returns
    /// * `true` - Continue execution (StartAgain or LoadNew)
    /// * `false` - Exit emulator (Quit)
    /// 
    /// # Requirements
    /// Validates: Requirements 19.3, 19.4, 19.5, 19.6, 19.7, 19.8
    /// 
    /// # Examples
    /// ```no_run
    /// if self.cpu.halted {
    ///     return self.handle_halt();
    /// }
    /// ```
    pub fn handle_halt(&mut self) -> bool {
        loop {
            // Get user's choice for what to do after halt
            let action = self.prompt_post_execution_options();
            
            // Execute the corresponding action
            match action {
                PostExecutionAction::StartAgain => {
                    // Reset instruction count and resume from current PC
                    // Requirement 19.6: "WHEN the user selects 's' (start), THE Emulator 
                    // SHALL reset the instruction count to 0 and resume execution from the current PC"
                    self.restart_execution();
                    return true; // Continue execution
                }
                PostExecutionAction::LoadNew => {
                    // Load new program and reset CPU state
                    // Requirement 19.7: "WHEN the user selects 'l' (load), THE Emulator 
                    // SHALL prompt for a new binary file path and start address, then load the new program"
                    if self.load_new_program() {
                        return true; // Continue execution with new program
                    }
                    // If load failed or was cancelled, loop back to options menu
                    continue;
                }
                PostExecutionAction::ViewMemory => {
                    // Allow user to view memory, then return to options
                    // Requirement 19.8: "WHEN the user selects 'm' (memory), THE Emulator 
                    // SHALL allow the user to view memory at different addresses, then return 
                    // to the post-execution options"
                    self.handle_memory_view_loop();
                    // Loop back to show options again
                    continue;
                }
                PostExecutionAction::Quit => {
                    // Exit gracefully
                    // Requirement 19.9: "WHEN the user selects 'q' (quit), THE Emulator 
                    // SHALL exit gracefully"
                    println!("\nThank you for using the 6502 CPU Emulator!");
                    return false; // Exit emulator
                }
            }
        }
    }

    /// Prompts the user for post-execution options after program halts
    /// 
    /// This method implements the complete post-execution menu as specified in Requirements 19.
    /// It displays:
    /// - Final framebuffer if enabled (Requirement 19.1)
    /// - Total instruction count (Requirement 19.2)
    /// - Final CPU state (Requirement 19.3)
    /// - Menu of options: s (start again), l (load new), m (view memory), q (quit) (Requirements 19.4, 19.5)
    /// 
    /// The method loops until the user makes a valid selection and returns the chosen action.
    /// 
    /// # Returns
    /// * `PostExecutionAction` - The action chosen by the user
    /// 
    /// # Requirements
    /// Validates: Requirements 19.1, 19.2, 19.3, 19.4, 19.5, 19.9, 19.10
    /// 
    /// # Examples
    /// ```no_run
    /// let action = emulator.prompt_post_execution_options();
    /// match action {
    ///     PostExecutionAction::StartAgain => { /* restart execution */ }
    ///     PostExecutionAction::LoadNew => { /* load new program */ }
    ///     PostExecutionAction::ViewMemory => { /* view memory */ }
    ///     PostExecutionAction::Quit => { /* exit emulator */ }
    /// }
    /// ```
    pub fn prompt_post_execution_options(&mut self) -> PostExecutionAction {
        use std::io::{self, Write};
        
        // Display final framebuffer if enabled (Requirement 19.1)
        if self.framebuffer_base.is_some() {
            self.display_framebuffer();
        }
        
        // Display total instruction count (Requirement 19.2)
        println!("\n╔════════════════════════════════════════════════════════════╗");
        println!("║              PROGRAM EXECUTION COMPLETE                    ║");
        println!("╚════════════════════════════════════════════════════════════╝");
        
        let count_str = format_number_with_separators(self.instruction_count);
        println!("\nTotal Instructions Executed: {}", count_str);
        
        // Display final CPU state (Requirement 19.3)
        println!("\n┌─────────────────────────────────────────────────────────┐");
        println!("│                    FINAL CPU STATE                      │");
        println!("└─────────────────────────────────────────────────────────┘");
        self.display_state();
        
        // Display menu of options (Requirements 19.4, 19.5)
        println!("\n┌─────────────────────────────────────────────────────────┐");
        println!("│                    OPTIONS                              │");
        println!("└─────────────────────────────────────────────────────────┘");
        println!("What would you like to do?");
        println!("  s - Start execution again from current PC");
        println!("  l - Load new program");
        println!("  m - View memory");
        println!("  q - Quit emulator");
        
        // Loop until valid option is selected (Requirement 19.10)
        loop {
            print!("\nSelect option (s/l/m/q): ");
            if let Err(e) = io::stdout().flush() {
                eprintln!("Warning: Failed to flush output: {}", e);
            }
            
            let mut input = String::new();
            match io::stdin().read_line(&mut input) {
                Ok(0) => {
                    // EOF (Ctrl+D) - treat as quit
                    println!("\nEnd of input detected. Quitting...");
                    return PostExecutionAction::Quit;
                }
                Ok(_) => {
                    // Parse user input and return PostExecutionAction
                    match input.trim().to_lowercase().as_str() {
                        "s" | "start" => {
                            return PostExecutionAction::StartAgain;
                        }
                        "l" | "load" => {
                            return PostExecutionAction::LoadNew;
                        }
                        "m" | "memory" => {
                            return PostExecutionAction::ViewMemory;
                        }
                        "q" | "quit" | "exit" => {
                            return PostExecutionAction::Quit;
                        }
                        "" => {
                            // Empty input - re-prompt
                            println!("Please select an option.");
                            continue;
                        }
                        _ => {
                            // Invalid option - display error and re-prompt
                            println!("Invalid option. Please select s, l, m, or q.");
                            continue;
                        }
                    }
                }
                Err(e) => {
                    eprintln!("Error reading input: {}", e);
                    eprintln!("Please try again or press Ctrl+C to exit.");
                    continue;
                }
            }
        }
    }

    /// Clears the terminal screen
    /// 
    /// Uses ANSI escape codes to clear the screen and move cursor to top-left
    fn clear_screen() {
        print!("\x1B[2J\x1B[1;1H");
    }

    /// Displays the current CPU state
    /// 
    /// Shows:
    /// - Program Counter (PC)
    /// - Accumulator (A)
    /// - Index Registers (X, Y)
    /// - Stack Pointer (SP)
    /// - Status flags (NV-BDIZC format)
    /// - Next 5 instructions with addresses
    /// 
    /// # Requirements
    /// Validates: Requirements 5.1, 5.2, 5.3, 5.4, 5.5, 5.6, 5.7, 6.2, 6.4
    fn display_state(&self) {
        let state = &self.cpu.state;
        
        // Display registers
        println!("\n┌─────────────────────────────────────────────────────────┐");
        println!("│ PC: 0x{:04X}  A: 0x{:02X}  X: 0x{:02X}  Y: 0x{:02X}  SP: 0x01{:02X}       │",
                 state.pc, state.a, state.x, state.y, state.sp);
        
        // Display flags
        let status = state.get_status_byte();
        println!("│ Flags: NV-BDIZC                                         │");
        println!("│        {:08b}                                         │", status);
        print!("│        ");
        print!("{}", if state.flag_negative { 'N' } else { '-' });
        print!("{}", if state.flag_overflow { 'V' } else { '-' });
        print!("-");
        print!("{}", if state.flag_break { 'B' } else { '-' });
        print!("{}", if state.flag_decimal { 'D' } else { '-' });
        print!("{}", if state.flag_interrupt_disable { 'I' } else { '-' });
        print!("{}", if state.flag_zero { 'Z' } else { '-' });
        print!("{}", if state.flag_carry { 'C' } else { '-' });
        println!("                                         │");
        //println!("├─────────────────────────────────────────────────────────┤");
        println!("│                                                         │");
        println!("│ Next Instructions:                                      │");
        
        // Display next 5 instructions
        let mut current_addr = state.pc;
        for i in 0..5 {
            let opcode = self.cpu.memory.read(current_addr);
            let (instruction_str, inst_length) = match crate::instruction::decode_opcode(opcode) {
                Ok(decoded) => {
                    // Format the instruction with operands
                    let operand1 = if decoded.length > 1 {
                        self.cpu.memory.read(current_addr.wrapping_add(1))
                    } else {
                        0
                    };
                    let operand2 = if decoded.length > 2 {
                        self.cpu.memory.read(current_addr.wrapping_add(2))
                    } else {
                        0
                    };
                    
                    let formatted = match decoded.length {
                        1 => format!("{:?}", decoded.instruction),
                        2 => format!("{:?} ${:02X}", decoded.instruction, operand1),
                        3 => format!("{:?} ${:02X}{:02X}", decoded.instruction, operand2, operand1),
                        _ => format!("{:?}", decoded.instruction),
                    };
                    (formatted, decoded.length)
                }
                Err(_) => (format!("Invalid opcode: 0x{:02X}", opcode), 1),
            };
            
            // Mark the current PC instruction with an arrow
            let marker = if i == 0 { ">" } else { " " };
            println!("│   {} 0x{:04X}: {:<42}  │", marker, current_addr, instruction_str);
            
            // Move to next instruction
            current_addr = current_addr.wrapping_add(inst_length as u16);
        }
        
        println!("└─────────────────────────────────────────────────────────┘");
    }

    /// Displays a memory view showing 256 bytes in hex viewer format
    /// 
    /// Shows:
    /// - Header with memory range (e.g., "Memory View (0x0000-0x00FF)")
    /// - 16 rows × 16 bytes in hexadecimal format
    /// - ASCII representation for each row (printable chars or '.')
    /// - Row addresses in 4-digit hexadecimal format
    /// 
    /// Format:
    /// ```
    /// Memory View (0x0000-0x00FF):
    /// 0000 A9 42 8D 00 02 A9 FF 8D 01 02 00 00 00 00 00 00  .B..............
    /// 0010 41 00 00 00 42 00 00 00 00 00 00 00 00 00 00 00  A...B...........
    /// ...
    /// ```
    /// 
    /// # Requirements
    /// Validates: Requirements 13.1, 13.3, 13.4, 13.5, 13.6, 13.10, 13.11
    fn display_memory_view(&self) {
        let start = self.memory_view_start;
        let end = start.wrapping_add(0xFF); // 256 bytes (0x100)
        
        // Display header with memory range
        println!("\nMemory View (0x{:04X}-0x{:04X}):", start, end);
        
        // Display 16 rows of 16 bytes each
        for row in 0..16 {
            let row_addr = start.wrapping_add(row * 16);
            
            // Display row address
            print!("{:04X} ", row_addr);
            
            // Display 16 hex bytes
            let mut bytes = [0u8; 16];
            for col in 0..16 {
                let addr = row_addr.wrapping_add(col);
                let byte = self.cpu.memory.read(addr);
                bytes[col as usize] = byte;
                print!("{:02X} ", byte);
            }
            
            // Display ASCII representation
            print!(" ");
            for byte in bytes.iter() {
                // Printable ASCII range is 0x20-0x7E
                if *byte >= 0x20 && *byte <= 0x7E {
                    print!("{}", *byte as char);
                } else {
                    print!(".");
                }
            }
            
            println!();
        }
    }

    /// Prompts the user for a new memory view start address
    /// 
    /// Continues prompting until a valid address is provided.
    /// Accepts addresses with or without "0x" prefix.
    /// Validates that the address is in the range 0x0000 to 0xFFFF.
    /// Updates memory_view_start field and displays the updated memory view.
    /// 
    /// # Requirements
    /// Validates: Requirements 13.7, 13.8
    fn prompt_for_memory_address(&mut self) {
        use std::io::{self, Write};
        
        loop {
            print!("\nEnter memory view start address (hex, e.g., 0x0200 or 0200): ");
            if let Err(e) = io::stdout().flush() {
                eprintln!("Warning: Failed to flush output: {}", e);
            }
            
            let mut addr_input = String::new();
            match io::stdin().read_line(&mut addr_input) {
                Ok(0) => {
                    // EOF (Ctrl+D) - cancel operation
                    println!("\nMemory view change cancelled.");
                    return;
                }
                Ok(_) => {
                    // Use the parse_hex_address function
                    match parse_hex_address(&addr_input) {
                        Ok(addr) => {
                            // Address is valid (u16 automatically ensures 0x0000-0xFFFF range)
                            self.memory_view_start = addr;
                            println!("Memory view start address updated to 0x{:04X}", addr);
                            // Display the updated memory view
                            self.display_memory_view();
                            return;
                        }
                        Err(err) => {
                            eprintln!("Error: {}", err);
                            continue;
                        }
                    }
                }
                Err(err) => {
                    eprintln!("Error reading input: {}", err);
                    eprintln!("Please try again or press Ctrl+D to cancel.");
                    continue;
                }
            }
        }
    }
    
    /// Checks if the current PC matches any breakpoint address
    /// 
    /// # Returns
    /// * `true` - At least one breakpoint is set and PC matches a breakpoint address
    /// * `false` - No breakpoints set or PC doesn't match any breakpoint
    /// 
    /// # Requirements
    /// Validates: Requirements 14.4
    fn check_breakpoint(&self) -> bool {
        self.breakpoints.contains(&self.cpu.state.pc)
    }
    
    /// Prompts the user for a framebuffer base address
    /// 
    /// Continues prompting until a valid address is provided.
    /// Accepts addresses with or without "0x" prefix.
    /// Validates that the address is in the range 0x0000 to 0xFFFF.
    /// If the user presses Enter without providing a value, defaults to 0xE000.
    /// Updates framebuffer_base field and displays confirmation message.
    /// 
    /// # Requirements
    /// Validates: Requirements 15.1, 15.2
    fn prompt_for_framebuffer(&mut self) {
        use std::io::{self, Write};
        
        loop {
            print!("\nEnter framebuffer base address (hex, e.g., 0x2000 or 2000) [default: 0xE000]: ");
            if let Err(e) = io::stdout().flush() {
                eprintln!("Warning: Failed to flush output: {}", e);
            }
            
            let mut addr_input = String::new();
            match io::stdin().read_line(&mut addr_input) {
                Ok(0) => {
                    // EOF (Ctrl+D) - cancel operation
                    println!("\nFramebuffer setting cancelled.");
                    return;
                }
                Ok(_) => {
                    let trimmed = addr_input.trim();
                    
                    // Check if input is empty - use default
                    if trimmed.is_empty() {
                        println!("Using default framebuffer base address: 0xE000");
                        self.framebuffer_base = Some(0xE000);
                        return;
                    }
                    
                    // Use the parse_hex_address function
                    match parse_hex_address(trimmed) {
                        Ok(addr) => {
                            // Address is valid (u16 automatically ensures 0x0000-0xFFFF range)
                            self.framebuffer_base = Some(addr);
                            println!("Framebuffer base address set to 0x{:04X}", addr);
                            return;
                        }
                        Err(err) => {
                            eprintln!("Error: {}", err);
                            continue;
                        }
                    }
                }
                Err(err) => {
                    eprintln!("Error reading input: {}", err);
                    eprintln!("Please try again or press Ctrl+D to cancel.");
                    continue;
                }
            }
        }
    }
    
    /// Displays the framebuffer in a graphical window
    /// 
    /// Creates a 320x240 pixel window (2x scale) and renders the framebuffer contents.
    /// Each bit in memory represents a pixel (1 = white, 0 = black).
    /// Bits are read left-to-right (bit 7 to bit 0).
    /// Memory layout: 20 bytes per row (160 pixels / 8 bits).
    /// Each framebuffer pixel is rendered as a 2x2 block for better visibility.
    /// 
    /// The window is created once and reused for subsequent updates.
    /// Updates are non-blocking - the window remains open and responsive.
    /// 
    /// # Requirements
    /// Validates: Requirements 15.3, 15.4, 15.5, 15.6, 15.8, 15.10
    fn display_framebuffer(&mut self) {
        if let Some(base) = self.framebuffer_base {
            use minifb::{Window, WindowOptions};
            
            const FB_WIDTH: usize = 160;   // Framebuffer logical width
            const FB_HEIGHT: usize = 120;  // Framebuffer logical height
            const SCALE: usize = 2;        // 2x scale for better visibility
            const WINDOW_WIDTH: usize = FB_WIDTH * SCALE;   // 320 pixels
            const WINDOW_HEIGHT: usize = FB_HEIGHT * SCALE; // 240 pixels
            const BYTES_PER_ROW: usize = 20; // 160 pixels / 8 bits
            
            // Create window if it doesn't exist yet
            if self.framebuffer_window.is_none() {
                match Window::new(
                    &format!("6502 Framebuffer - Base: 0x{:04X} (2x scale)", base),
                    WINDOW_WIDTH,
                    WINDOW_HEIGHT,
                    WindowOptions::default(),
                ) {
                    Ok(mut win) => {
                        // Limit update rate to avoid consuming too much CPU
                        win.set_target_fps(60); // 60 FPS
                        self.framebuffer_window = Some(win);
                    }
                    Err(e) => {
                        eprintln!("Failed to create framebuffer window: {}", e);
                        return;
                    }
                }
            }
            
            // Check if window is still open (user might have closed it)
            if let Some(ref mut window) = self.framebuffer_window {
                if !window.is_open() {
                    // Window was closed by user, disable framebuffer
                    self.framebuffer_window = None;
                    self.framebuffer_base = None;
                    println!("\nFramebuffer window closed. Framebuffer disabled.");
                    return;
                }
                
                // Create pixel buffer for scaled window (RGB format for minifb, initialized to black)
                let mut buffer: Vec<u32> = vec![0x000000; WINDOW_WIDTH * WINDOW_HEIGHT];
                
                // Read framebuffer data from memory and convert to pixels
                for row in 0..FB_HEIGHT {
                    for col_byte in 0..BYTES_PER_ROW {
                        let addr = base.wrapping_add((row * BYTES_PER_ROW + col_byte) as u16);
                        let byte = self.cpu.memory.read(addr);
                        
                        // Process each bit in the byte (bit 7 is leftmost)
                        for bit in 0..8 {
                            let pixel_value = (byte >> (7 - bit)) & 1;
                            // Requirement 15.5: 1 = white, 0 = black
                            let color = if pixel_value == 1 {
                                0x00FFFFFF // White (RGB format)
                            } else {
                                0x00000000 // Black (RGB format)
                            };
                            
                            // Calculate framebuffer pixel position
                            let fb_pixel_x = col_byte * 8 + bit;
                            
                            // Render as 2x2 block in the scaled buffer
                            for dy in 0..SCALE {
                                for dx in 0..SCALE {
                                    let window_x = fb_pixel_x * SCALE + dx;
                                    let window_y = row * SCALE + dy;
                                    let pixel_index = window_y * WINDOW_WIDTH + window_x;
                                    buffer[pixel_index] = color;
                                }
                            }
                        }
                    }
                }
                
                // Update the window with the buffer (non-blocking)
                if let Err(e) = window.update_with_buffer(&buffer, WINDOW_WIDTH, WINDOW_HEIGHT) {
                    eprintln!("Failed to update framebuffer window: {}", e);
                    // Window might have been closed, clean up
                    self.framebuffer_window = None;
                    self.framebuffer_base = None;
                }
            }
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

    /// Displays execution statistics including instruction count and execution speed
    ///
    /// Always displays:
    /// - Total instruction count with thousands separators for readability
    ///
    /// In Running mode only:
    /// - Execution speed in instructions per second
    ///
    /// The output is formatted to be concise and readable, appearing near the register display.
    ///
    /// # Requirements
    /// Validates: Requirements 16.1, 16.5, 16.6, 16.7
    ///
    /// # Examples
    /// ```
    /// // In Paused or Stepping mode:
    /// // Instructions: 1,234,567
    ///
    /// // In Running mode:
    /// // Instructions: 1,234,567
    /// // Speed: 2,500,000 instructions/sec
    /// ```
    pub fn display_statistics(&self) {
        // Format instruction count with thousands separators
        let count_str = format_number_with_separators(self.instruction_count);
        println!("\nInstructions: {}", count_str);

        // Display execution speed only in Running mode
        if matches!(self.mode, ExecutionMode::Running) {
            let speed = self.calculate_execution_speed();
            let speed_str = format_number_with_separators(speed as u64);
            println!("Speed: {} instructions/sec", speed_str);
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
    /// if emulator.should_update_framebuffer() {
    ///     emulator.display_framebuffer();
    ///     emulator.last_frame_time = Instant::now();
    /// }
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
    /// if emulator.should_display_status() {
    ///     emulator.display_state();
    ///     emulator.display_statistics();
    ///     emulator.last_status_time = Instant::now();
    /// }
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

    /// Edits a memory location with a new value
    /// 
    /// # Arguments
    /// * `address_str` - Hexadecimal address string (with or without 0x prefix)
    /// * `value_str` - Hexadecimal value string (with or without 0x prefix)
    /// 
    /// # Returns
    /// * `Ok(())` - Memory successfully updated
    /// * `Err(String)` - Invalid address or value
    /// 
    /// # Requirements
    /// Validates: Requirements 18.2, 18.5, 18.6, 18.9, 18.10
    /// 
    /// # Examples
    /// ```
    /// emulator.edit_memory("0x0200", "0xFF")?;  // Write 0xFF to address 0x0200
    /// emulator.edit_memory("200", "42")?;       // Write 0x42 to address 0x0200
    /// ```
    pub fn edit_memory(&mut self, address_str: &str, value_str: &str) -> Result<(), String> {
        // Parse address and value
        let address = self.parse_hex_u16(address_str)?;
        let value = self.parse_hex_u8(value_str)?;
        
        // Write value to memory
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
    /// emulator.handle_edit_command("A 0x42")?;      // Set accumulator to 0x42
    /// emulator.handle_edit_command("PC 0x8000")?;   // Set PC to 0x8000
    /// emulator.handle_edit_command("0x0200 0xFF")?; // Write 0xFF to address 0x0200
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
                println!("Register {} set to {}", target_upper, value_str);
            }
            _ => {
                // Treat as memory address
                let address = self.parse_hex_u16(target)?;
                let value = self.parse_hex_u8(value_str)?;
                self.cpu.memory.write(address, value);
                println!("Memory[0x{:04X}] set to 0x{:02X}", address, value);
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
        let now = Instant::now();
        self.start_time = now;
        self.last_status_time = now;
        self.last_frame_time = now;
        
        // Clear the CPU halted flag to resume execution
        self.cpu.halted = false;
        
        // Set execution mode to Running (Requirement 19.4)
        self.mode = ExecutionMode::Running;
        
        // Display confirmation message
        println!("\n╔════════════════════════════════════════════════════════════╗");
        println!("║              EXECUTION RESTARTED                           ║");
        println!("╚════════════════════════════════════════════════════════════╝");
        println!("\nExecution restarted from PC: 0x{:04X}", self.cpu.state.pc);
        println!("Instruction count reset to 0");
        println!("CPU state and memory preserved");
        println!("Mode: Running\n");
    }

    /// Loads a new program into the emulator
    /// 
    /// This method:
    /// 1. Prompts the user for a new binary file path
    /// 2. Prompts the user for a new start address
    /// 3. Loads the new binary into memory
    /// 4. Resets the CPU state to initial values
    /// 5. Resets the instruction count to 0
    /// 6. Sets the PC to the new start address
    /// 
    /// The framebuffer configuration is preserved so the user doesn't need to reconfigure it.
    /// The breakpoints are cleared since they may not be relevant to the new program.
    /// 
    /// # Returns
    /// * `true` - New program loaded successfully, continue execution
    /// * `false` - User cancelled or error occurred, return to post-execution options
    /// 
    /// # Requirements
    /// Validates: Requirement 19.5
    pub fn load_new_program(&mut self) -> bool {
        use std::io;
        
        println!("\n╔════════════════════════════════════════════════════════════╗");
        println!("║              LOAD NEW PROGRAM                              ║");
        println!("╚════════════════════════════════════════════════════════════╝\n");
        
        // Prompt for new binary file path
        let file_path = match prompt_for_file_path() {
            Some(path) => path,
            None => {
                println!("Load cancelled.");
                return false;
            }
        };
        
        // Prompt for new start address
        let start_address = match prompt_for_load_address() {
            Some(addr) => addr,
            None => {
                println!("Load cancelled.");
                return false;
            }
        };
        
        // Create new memory and load the binary file
        let mut memory = Memory::new();
        if let Err(e) = memory.load_from_file_at_offset(&file_path, start_address) {
            eprintln!("Error loading file: {}", e);
            println!("\nPress Enter to return to post-execution options...");
            let mut _input = String::new();
            let _ = io::stdin().read_line(&mut _input);
            return false;
        }
        
        // Reset CPU with new memory and start address
        self.cpu = Cpu::new(memory, start_address);
        
        // Reset instruction count (Requirement 19.5)
        self.instruction_count = 0;
        
        // Reset timing information
        let now = Instant::now();
        self.start_time = now;
        self.last_status_time = now;
        self.last_frame_time = now;
        
        // Set execution mode to Paused
        self.mode = ExecutionMode::Paused;
        
        // Clear breakpoints (they may not be relevant to the new program)
        self.breakpoints.clear();
        
        // Reset memory view to start of memory
        self.memory_view_start = 0x0000;
        
        // Note: framebuffer_base is preserved so user doesn't need to reconfigure
        
        // Display confirmation message
        println!("\n╔════════════════════════════════════════════════════════════╗");
        println!("║              NEW PROGRAM LOADED                            ║");
        println!("╚════════════════════════════════════════════════════════════╝");
        println!("\nFile: {}", file_path);
        println!("Start address: 0x{:04X}", start_address);
        println!("CPU state reset to initial values");
        println!("Instruction count reset to 0");
        println!("Breakpoints cleared");
        println!("Mode: Paused\n");
        
        true
    }

    /// Handles the memory view loop for post-execution options
    /// 
    /// Allows the user to view memory at different addresses and then return
    /// to the post-execution options menu.
    /// 
    /// This method implements Requirement 19.8: "WHEN the user selects 'm' (memory),
    /// THE Emulator SHALL allow the user to view memory at different addresses,
    /// then return to the post-execution options"
    /// 
    /// # Behavior
    /// - Displays memory view at current memory_view_start address
    /// - Prompts user for new memory address or 'b' to go back
    /// - Updates memory_view_start when valid address is provided
    /// - Loops until user enters 'b' to return to post-execution options
    /// 
    /// # Requirements
    /// Validates: Requirement 19.8
    pub fn handle_memory_view_loop(&mut self) {
        use std::io::{self, Write};
        
        loop {
            // Display current memory view
            println!("\n╔════════════════════════════════════════════════════════════╗");
            println!("║              MEMORY VIEW                                   ║");
            println!("╚════════════════════════════════════════════════════════════╝");
            self.display_memory_view();
            
            // Prompt for new address or back command
            println!("\nEnter memory address to view (hex), or 'b' to go back:");
            print!("> ");
            if let Err(e) = io::stdout().flush() {
                eprintln!("Warning: Failed to flush output: {}", e);
            }
            
            let mut input = String::new();
            match io::stdin().read_line(&mut input) {
                Ok(0) => {
                    // EOF (Ctrl+D) - treat as back command
                    println!("\nReturning to post-execution options...");
                    break;
                }
                Ok(_) => {
                    let input = input.trim().to_lowercase();
                    
                    // Check if user wants to go back
                    if input == "b" || input == "back" {
                        println!("\nReturning to post-execution options...");
                        break;
                    }
                    
                    // Try to parse as hex address
                    match self.parse_hex_u16(&input) {
                        Ok(address) => {
                            self.memory_view_start = address;
                            println!("Memory view updated to start at 0x{:04X}", address);
                        }
                        Err(e) => {
                            println!("Error: {}", e);
                            println!("Please enter a valid hex address (0x0000-0xFFFF) or 'b' to go back.");
                        }
                    }
                }
                Err(e) => {
                    eprintln!("Error reading input: {}", e);
                    eprintln!("Please try again or enter 'b' to go back.");
                }
            }
        }
    }

}

/// Prompts the user for a binary file path
/// 
/// Continues prompting until a valid file path is provided or the user cancels.
/// 
/// # Returns
/// * `Some(String)` - Valid file path
/// * `None` - User cancelled (Ctrl+C or EOF)
/// 
/// # Requirements
/// Validates: Requirements 1.1, 10.3, 10.4
fn prompt_for_file_path() -> Option<String> {
    use std::io::{self, Write};
    
    loop {
        print!("\nEnter binary file path: ");
        // Gracefully handle flush errors instead of panicking
        if let Err(e) = io::stdout().flush() {
            eprintln!("Warning: Failed to flush output: {}", e);
            // Continue - this is not a fatal error
        }
        
        let mut file_path = String::new();
        match io::stdin().read_line(&mut file_path) {
            Ok(0) => {
                // EOF (Ctrl+D)
                println!("\nInput cancelled.");
                return None;
            }
            Ok(_) => {
                let file_path = file_path.trim();
                if file_path.is_empty() {
                    eprintln!("Error: File path cannot be empty. Please enter a valid file path.");
                    continue;
                }
                return Some(file_path.to_string());
            }
            Err(err) => {
                eprintln!("Error reading input: {}", err);
                eprintln!("Please try again or press Ctrl+C to exit.");
                continue;
            }
        }
    }
}

/// Prompts the user for a load address in hexadecimal format
/// 
/// Continues prompting until a valid address is provided or the user cancels.
/// Accepts addresses with or without "0x" prefix.
/// Validates that the address is in the range 0x0000 to 0xFFFF.
/// If the user presses Enter without providing a value, defaults to 0x0200.
/// 
/// # Returns
/// * `Some(u16)` - Valid load address
/// * `None` - User cancelled (Ctrl+C or EOF)
/// 
/// # Requirements
/// Validates: Requirements 2.1, 2.3, 2.4, 10.3, 10.4
fn prompt_for_load_address() -> Option<u16> {
    use std::io::{self, Write};
    
    loop {
        print!("Enter load address (hex, e.g., 0x8000 or 8000) [default: 0x0200]: ");
        // Gracefully handle flush errors instead of panicking
        if let Err(e) = io::stdout().flush() {
            eprintln!("Warning: Failed to flush output: {}", e);
            // Continue - this is not a fatal error
        }
        
        let mut addr_input = String::new();
        match io::stdin().read_line(&mut addr_input) {
            Ok(0) => {
                // EOF (Ctrl+D)
                println!("\nInput cancelled.");
                return None;
            }
            Ok(_) => {
                let trimmed = addr_input.trim();
                
                // Check if input is empty - use default
                if trimmed.is_empty() {
                    println!("Using default load address: 0x0200");
                    return Some(0x0200);
                }
                
                // Use the parse_hex_address function
                match parse_hex_address(trimmed) {
                    Ok(addr) => {
                        // Address is valid (u16 automatically ensures 0x0000-0xFFFF range)
                        return Some(addr);
                    }
                    Err(err) => {
                        eprintln!("Error: {}", err);
                        continue;
                    }
                }
            }
            Err(err) => {
                eprintln!("Error reading input: {}", err);
                eprintln!("Please try again or press Ctrl+C to exit.");
                continue;
            }
        }
    }
}

/// Parses command-line arguments to extract filename and load address
/// 
/// Supports three invocation modes:
/// 1. No arguments: Returns (None, None) - will prompt for both
/// 2. Filename only: Returns (Some(filename), None) - will default to 0x0200
/// 3. Filename and load address: Returns (Some(filename), Some(address))
/// 
/// If invalid arguments are provided, displays usage information and exits.
/// 
/// # Returns
/// * `(Option<String>, Option<u16>)` - Tuple of optional filename and load address
/// 
/// # Requirements
/// Validates: Requirements 12.1, 12.2, 12.3, 12.4, 12.5
/// 
/// # Examples
/// ```no_run
/// // No arguments
/// let (file, addr) = parse_args();
/// assert_eq!(file, None);
/// assert_eq!(addr, None);
/// 
/// // Filename only
/// // $ cpu_6502_emulator program.bin
/// let (file, addr) = parse_args();
/// assert_eq!(file, Some("program.bin".to_string()));
/// assert_eq!(addr, None); // Will default to 0x0200
/// 
/// // Filename and address
/// // $ cpu_6502_emulator program.bin 8000
/// let (file, addr) = parse_args();
/// assert_eq!(file, Some("program.bin".to_string()));
/// assert_eq!(addr, Some(0x8000));
/// ```
fn parse_args() -> (Option<String>, Option<u16>) {
    let args: Vec<String> = std::env::args().collect();
    
    // args[0] is the program name, so we start from args[1]
    match args.len() {
        1 => {
            // No arguments provided - will prompt for both
            (None, None)
        }
        2 => {
            // Filename only - will default to 0x0200
            let filename = args[1].clone();
            (Some(filename), None)
        }
        3 => {
            // Filename and load address provided
            let filename = args[1].clone();
            let addr_str = &args[2];
            
            // Use the new parse_hex_address function
            match parse_hex_address(addr_str) {
                Ok(addr) => (Some(filename), Some(addr)),
                Err(err) => {
                    // Invalid address format - display error and usage, then exit
                    eprintln!("Error: {}", err);
                    eprintln!();
                    display_usage();
                    std::process::exit(1);
                }
            }
        }
        _ => {
            // Too many arguments - display usage and exit
            eprintln!("Error: Too many arguments provided.");
            eprintln!();
            display_usage();
            std::process::exit(1);
        }
    }
}

/// Displays usage information for the emulator
/// 
/// Shows the correct command-line syntax and examples.
/// 
/// # Requirements
/// Validates: Requirements 12.5
fn display_usage() {
    println!("Usage: cpu_6502_emulator [filename] [load_address]");
    println!();
    println!("Arguments:");
    println!("  filename       - Path to the binary file to load");
    println!("  load_address   - Hexadecimal address where binary is loaded and execution starts");
    println!();
    println!("Examples:");
    println!("  cpu_6502_emulator                    # Interactive mode (prompts for both)");
    println!("  cpu_6502_emulator program.bin        # Uses file, defaults to 0x0200");
    println!("  cpu_6502_emulator program.bin 8000   # Loads at 0x8000, starts at 0x8000");
    println!("  cpu_6502_emulator program.bin 0x8000 # Loads at 0x8000, starts at 0x8000");
    println!();
    println!("Note: Default load address is 0x0200 (after zero page and stack)");
}
/// Format a u64 number with thousands separators for readability.
///
/// # Arguments
/// * `n` - The number to format
///
/// # Returns
/// A string with commas as thousands separators (e.g., "1,234,567")
///
/// # Examples
/// ```
/// assert_eq!(format_number_with_separators(0), "0");
/// assert_eq!(format_number_with_separators(123), "123");
/// assert_eq!(format_number_with_separators(1234), "1,234");
/// assert_eq!(format_number_with_separators(1234567), "1,234,567");
/// ```
///
/// # Requirements
/// Validates: Requirements 16.5
fn format_number_with_separators(n: u64) -> String {
    let s = n.to_string();
    let mut result = String::new();

    // Process digits from right to left
    for (i, c) in s.chars().rev().enumerate() {
        // Add comma before every group of 3 digits (except at the start)
        if i > 0 && i % 3 == 0 {
            result.push(',');
        }
        result.push(c);
    }

    // Reverse the string to get the correct order
    result.chars().rev().collect()
}


fn main() {
    println!("6502 CPU Emulator");
    
    // Parse command-line arguments
    let (arg_file, arg_addr) = parse_args();
    
    // Main restart loop
    loop {
        println!("\nWelcome to the 6502 CPU Emulator!");
        println!("This emulator loads a binary file at a specified address and executes 6502 instructions.");
        
        // Get file path from arguments or prompt
        let file_path = match arg_file.clone() {
            Some(path) => {
                println!("Using file from command line: {}", path);
                path
            }
            None => {
                // No filename provided via command line, prompt for it
                match prompt_for_file_path() {
                    Some(path) => path,
                    None => {
                        println!("Emulator startup cancelled.");
                        std::process::exit(0);
                    }
                }
            }
        };
        
        // Get load address from arguments, default to 0x0200, or prompt
        let load_address = match arg_addr {
            Some(addr) => {
                // Load address provided via command line, use it
                println!("Using load address from command line: 0x{:04X}", addr);
                addr
            }
            None => {
                // No load address provided via command line
                if arg_file.is_some() {
                    // Filename was provided but no address, default to 0x0200
                    println!("No load address provided, defaulting to 0x0200");
                    0x0200
                } else {
                    // No command-line arguments at all, prompt for address
                    match prompt_for_load_address() {
                        Some(addr) => addr,
                        None => {
                            println!("Emulator startup cancelled.");
                            std::process::exit(0);
                        }
                    }
                }
            }
        };
        
        // Create the emulator
        let mut emulator = match Emulator::new(&file_path, load_address) {
            Ok(emu) => {
                println!("\nEmulator initialized successfully!");
                println!("Binary file: {}", file_path);
                println!("Load address: 0x{:04X}", load_address);
                println!("Execution starts at: 0x{:04X}", load_address);
                emu
            }
            Err(err) => {
                eprintln!("\n╔════════════════════════════════════════════════════════════╗");
                eprintln!("║                    EMULATOR ERROR                          ║");
                eprintln!("╚════════════════════════════════════════════════════════════╝");
                eprintln!("\nFailed to create emulator:");
                eprintln!("  {}", err);
                eprintln!("\nPossible causes:");
                eprintln!("  • File does not exist at the specified path");
                eprintln!("  • Insufficient permissions to read the file");
                eprintln!("  • File is locked by another process");
                eprintln!("\nPlease check the file path and try again.");
                
                // Display usage information to help the user
                println!();
                display_usage();
                
                std::process::exit(1);
            }
        };
        
        // Run the emulator and check if user wants to restart
        let should_restart = emulator.run();
        
        if !should_restart {
            // User chose to quit
            println!("\nEmulator terminated gracefully.");
            break;
        }
        
        // User chose to restart - loop continues
    }
    
    // Exit gracefully
    std::process::exit(0);
}


#[cfg(test)]
mod tests {
    use super::*;

    /// Test format_number_with_separators with zero
    /// Validates: Requirements 16.5
    #[test]
    fn test_format_number_zero() {
        assert_eq!(format_number_with_separators(0), "0");
    }

    /// Test format_number_with_separators with small numbers (no separators needed)
    /// Validates: Requirements 16.5
    #[test]
    fn test_format_number_small() {
        assert_eq!(format_number_with_separators(1), "1");
        assert_eq!(format_number_with_separators(12), "12");
        assert_eq!(format_number_with_separators(123), "123");
        assert_eq!(format_number_with_separators(999), "999");
    }

    /// Test format_number_with_separators with numbers requiring one separator
    /// Validates: Requirements 16.5
    #[test]
    fn test_format_number_one_separator() {
        assert_eq!(format_number_with_separators(1_000), "1,000");
        assert_eq!(format_number_with_separators(1_234), "1,234");
        assert_eq!(format_number_with_separators(9_999), "9,999");
        assert_eq!(format_number_with_separators(10_000), "10,000");
        assert_eq!(format_number_with_separators(99_999), "99,999");
        assert_eq!(format_number_with_separators(100_000), "100,000");
        assert_eq!(format_number_with_separators(999_999), "999,999");
    }

    /// Test format_number_with_separators with numbers requiring multiple separators
    /// Validates: Requirements 16.5
    #[test]
    fn test_format_number_multiple_separators() {
        assert_eq!(format_number_with_separators(1_000_000), "1,000,000");
        assert_eq!(format_number_with_separators(1_234_567), "1,234,567");
        assert_eq!(format_number_with_separators(12_345_678), "12,345,678");
        assert_eq!(format_number_with_separators(123_456_789), "123,456,789");
        assert_eq!(format_number_with_separators(1_234_567_890), "1,234,567,890");
    }

    /// Test format_number_with_separators with very large numbers
    /// Validates: Requirements 16.5
    #[test]
    fn test_format_number_large() {
        assert_eq!(format_number_with_separators(u64::MAX), "18,446,744,073,709,551,615");
        assert_eq!(format_number_with_separators(1_000_000_000_000), "1,000,000,000,000");
        assert_eq!(format_number_with_separators(999_999_999_999_999), "999,999,999,999,999");
    }

    /// Test format_number_with_separators with typical instruction counts
    /// Validates: Requirements 16.5
    #[test]
    fn test_format_number_typical_instruction_counts() {
        // Typical instruction counts that might be seen during emulation
        assert_eq!(format_number_with_separators(100), "100");
        assert_eq!(format_number_with_separators(1_000), "1,000");
        assert_eq!(format_number_with_separators(10_000), "10,000");
        assert_eq!(format_number_with_separators(100_000), "100,000");
        assert_eq!(format_number_with_separators(1_000_000), "1,000,000");
        assert_eq!(format_number_with_separators(2_500_000), "2,500,000");
    }

    /// Test format_number_with_separators with edge cases
    /// Validates: Requirements 16.5
    #[test]
    fn test_format_number_edge_cases() {
        // Numbers at boundaries
        assert_eq!(format_number_with_separators(999), "999");
        assert_eq!(format_number_with_separators(1_000), "1,000");
        assert_eq!(format_number_with_separators(999_999), "999,999");
        assert_eq!(format_number_with_separators(1_000_000), "1,000,000");
    }
}
