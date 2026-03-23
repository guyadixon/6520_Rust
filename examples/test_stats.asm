; Simple test program to verify statistics display
; This program just does a few NOPs and then halts

        .org $0000
        LDA #$42    ; Load 0x42 into A
        NOP         ; No operation
        NOP         ; No operation
        NOP         ; No operation
        BRK         ; Break (halt)
