            .option  norvc
            .text
init:       la      sp, _stack_start        # set stack pointer
main:       csrwi   0x300, 8                # enable global interrupts
            la      t1, isr_0
            srl     t1, t1, 2
            csrw    0xB00, t1               # setup isr_0 address

            li      t2, 0b11110000          # interrupt every 15 cycles, cmp value 0b1111 = 15, prescaler 0b0000                                           
            csrw    0x400, t2               # timer.counter_top CSR
            la t1,  0b1110                  # prio 0b11, enable, 0b1, pend 0b0
            csrw    0xB20, t1               # write above to interrupt 0 (timer interrupt)
stop:       j       stop                    # wait for interrupt

isr_0:      la      t0, .toggled            # &static mut toggled state
            lw      t1, 0(t0)               # deref toggled
            xori    t1, t1, 1               # toggle bit 0
            csrw    0x0, t1                 # set bit 0 (t1 = 1) in GPIO CSR (LED on/off)
            sw      t1, 0(t0)               # store toggled value
            csrr    t3, 0xB40               # read captured timestamp
            sw      t3, 4(t0)               # store timestamp
            jr      ra                      # return 

            .data
.toggled:   .word   0x0                     # state
            .word   0x0                     # time-stamp

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
    .word    0x00000000
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
