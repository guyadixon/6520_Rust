; Test program to verify framebuffer updates without blocking
; This program writes a simple pattern to the framebuffer at 0x2000
; and then loops, allowing the user to step through and see updates

; Start at 0x8000
.org $8000

start:
    ; Initialize framebuffer base at 0x2000
    ; Write a checkerboard pattern
    
    LDA #$AA        ; Load alternating bit pattern 10101010
    STA $2000       ; Write to first byte of framebuffer
    
    LDA #$55        ; Load alternating bit pattern 01010101
    STA $2001       ; Write to second byte
    
    LDA #$AA
    STA $2002
    
    LDA #$55
    STA $2003
    
    ; Write a few more bytes to create visible pattern
    LDA #$FF        ; All white
    STA $2014       ; Write to row 1, byte 0
    
    LDA #$00        ; All black
    STA $2015       ; Write to row 1, byte 1
    
    ; Now loop and modify the pattern
loop:
    LDA $2000       ; Read current value
    EOR #$FF        ; Invert all bits
    STA $2000       ; Write back
    
    ; Add a small delay by doing some NOPs
    NOP
    NOP
    NOP
    NOP
    NOP
    
    JMP loop        ; Loop forever
