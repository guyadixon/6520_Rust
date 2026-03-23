; Framebuffer test program
; This program writes a simple pattern to the framebuffer memory
; Framebuffer base: 0x2000
; Size: 160x120 pixels = 2400 bytes

; Start at 0x8000
.org $8000

start:
    ; Initialize X register to 0 (will be our byte counter)
    LDX #$00
    
    ; Initialize Y register to 0 (will be our page counter)
    LDY #$00

fill_loop:
    ; Load alternating pattern (0xFF for even bytes, 0x00 for odd bytes)
    TXA              ; Transfer X to A
    AND #$01         ; Check if X is odd or even
    BEQ load_ff      ; If even, load 0xFF
    
load_00:
    LDA #$00         ; Load 0x00 for odd bytes
    JMP store_byte
    
load_ff:
    LDA #$FF         ; Load 0xFF for even bytes
    
store_byte:
    ; Store to framebuffer memory (0x2000 + Y*256 + X)
    STA $2000,X      ; Store at 0x2000 + X (first page)
    
    ; Increment X
    INX
    BNE fill_loop    ; If X != 0, continue filling current page
    
    ; X wrapped to 0, move to next page
    INY
    CPY #$09         ; Check if we've filled 9 pages (9*256 = 2304 bytes)
    BNE fill_loop
    
    ; Fill remaining 96 bytes (2400 - 2304 = 96)
    LDX #$00
fill_remaining:
    TXA
    AND #$01
    BEQ load_ff2
    LDA #$00
    JMP store_byte2
load_ff2:
    LDA #$FF
store_byte2:
    STA $2900,X      ; Store at 0x2900 + X
    INX
    CPX #$60         ; 96 bytes = 0x60
    BNE fill_remaining

done:
    ; Infinite loop to keep the pattern visible
    JMP done
