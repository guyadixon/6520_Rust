; Test program to demonstrate loading at offset 0x8000
; This program should be assembled without any header
; and loaded at address 0x8000

.org $8000

start:
    ; Write a test pattern to low memory to verify it was zeroed
    LDA #$AA
    STA $0010       ; Write to zero page
    
    LDA #$55
    STA $0011
    
    ; Write to framebuffer area if enabled
    LDA #$FF
    STA $2000
    
    LDA #$00
    STA $2001
    
    ; Simple loop
    LDX #$00
loop:
    INX
    CPX #$10
    BNE loop
    
    ; Halt
    BRK
