    .option  norvc
    .text
init:
    la       sp, _stack_start    # set stack pointer
    csrwi    0x350, 2            # set stack_depth
main:
    csrwi 0x300, 8 # enable global interrupts
    la t1, .clic_vec
    csrw 0x351, t1
    la t1, 0b11110
    csrw 0xB22, t1
    la t1, 0b1
    csrs 0xB22, t1
blink:
    csrs 0x0, t1
    nop
    csrc 0x0, t1
    j blink


isr_2: #interrupt 2
    jr       ra                 # return

    .section .vector_table, "aw"
    .word    0x20212223
    .word    0x24252627
    .word    0x28292A2B
    .word    0x2C2D2E2F
    .word    0x30313233
    .word    0x34353637
    .word    0x38393A3B
    .word    0x3C3D3E3F
    .word    0x40414243
    .word    0x44454647
    .word    0x48494A4B
    .word    0x4C4D4E4F
    .word    0x50515253
    .word    0x54555657
    .word    0x58595A5B
    .word    0x5C5D5E5F
    .word    0x60616263
    .word    0x64656667
    .word    0x68696A6B
    .word    0x6C6D6E6F
    .word    0x70717273
    .word    0x74757677
    .word    0x78797A7B
    .word    0x7C7D7E7F

    .section .clic_vec, "aw"
    .word    0x00000000
    .word    0x00000000
    .word    isr_2
    .word    0x20212223
    .word    0x24252627
    .word    0x28292A2B
    .word    0x2C2D2E2F
    .word    0x30313233
    .word    0x34353637
    .word    0x38393A3B
    .word    0x3C3D3E3F
    .word    0x40414243
    .word    0x44454647
    .word    0x48494A4B
    .word    0x4C4D4E4F
    .word    0x50515253
    .word    0x54555657
    .word    0x58595A5B
    .word    0x5C5D5E5F
    .word    0x60616263
    .word    0x64656667
    .word    0x68696A6B
    .word    0x6C6D6E6F
    .word    0x70717273
    .word    0x74757677
    .word    0x78797A7B
    .word    0x7C7D7E7F
