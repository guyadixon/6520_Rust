// Property-based tests for the 6502 CPU emulator
// Tests universal properties across randomized inputs using proptest

mod memory_properties;
mod flag_properties;
mod opcode_properties;
mod addressing_mode_properties;
mod load_instruction_properties;
mod store_instruction_properties;
mod arithmetic_instruction_properties;
mod logical_instruction_properties;
mod branch_instruction_properties;
mod control_flow_properties;
mod stack_properties;
mod pc_advancement_properties;
mod step_execution_properties;
mod error_handling_properties;
mod command_line_parsing_properties;
mod execution_statistics_properties;
mod framebuffer_properties;
