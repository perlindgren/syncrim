    .option  norvc
    .text
init:
    la       sp, _stack_start    # set stack pointer
    li       t1, 2
l:  addi     t1, t1, -1
    bne      t1, zero, l
s:  j        s




