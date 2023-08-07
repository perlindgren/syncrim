.section .text

start:
    lw x0, 0(x0)
    sw x1, 0(x1)
l:  beq zero, zero, l

.section .data 
a:  .byte 0,1,2,3
b:  .word 0,1,2,3
