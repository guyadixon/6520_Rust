; Simple 6502 test program
; This will be assembled to binary

; Start at 0x0200
.org $0200

start:
    LDA #$42    ; Load 0x42 into A
    LDX #$10    ; Load 0x10 into X
    LDY #$20    ; Load 0x20 into Y
    TAX         ; Transfer A to X (X = 0x42)
    TAY         ; Transfer A to Y (Y = 0x42)
    INX         ; Increment X (X = 0x43)
    INY         ; Increment Y (Y = 0x43)
    DEX         ; Decrement X (X = 0x42)
    DEY         ; Decrement Y (Y = 0x42)
    NOP         ; No operation
    PHP         ; push status flags to stack
    
; Pad to 64KB
.org $FFFF
    .byte $00
