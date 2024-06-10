    .option  norvc
    .text
init:
    la      sp, _stack_start    # set stack pointer
    la      a0, b
    # test signed
    lb      t0, 0(a0)
    lb      t1, 1(a0)
    lb      t2, 2(a0)
    lb      t3, 3(a0)
    lh      t4, 0(a0)
    lh      t5, 2(a0)
    lw      t6, 0(a0)
    
    # test unsigned
    lbu     t0, 0(a0)
    lbu     t1, 1(a0)
    lbu     t2, 2(a0)
    lbu     t3, 3(a0)
    lhu     t4, 0(a0)
    lhu     t5, 2(a0)
    sb      t0, 4(a0)
    sb      t1, 5(a0)
    sb      t2, 6(a0)
    sb      t3, 7(a0)
    sh      t4, 8(a0)
    sh      t5, 10(a0)    
s:  j       s
    
.data
b:  .byte   0x01, 0xf2, 0xf3, 0xf4
   
w:  .word   0x01020304
    .word   0x12345678
    .word   0xF1F2F3F4


