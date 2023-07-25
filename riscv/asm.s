.option norvc
.section .init, "ax"
.global _start

_start:
    addi x1, x0, 0 #x1=0
    jal x2, 8
    jal x2, 0 #this is to be jumped over or we will get stuck
    jalr x2, x2, 4
    jal x2, 0 #this is to be jumped over or we will get stuck
    addi x2, x0, 16
    addi x1, x0, 0
    addi x1, x1, 4
    bne x1, x2, -4
    beq x1, x2, 8
    jal x0, 0
    addi x1, x0, -8
    addi x2, x0, 0
    addi x1, x1, 4
    blt x1, x2, -4
    addi x1, x0, -8
    addi x2, x0, 0
    addi x1, x1, 4
    bltu x2, x1, -4
    addi x1, x0, -8
    addi x2, x0, 0
    addi x1, x1, 4
    bge x2, x1, -4
    addi x1, x0, -8
    addi x2, x0, 8
    addi x1, x1, 4
    bgeu x1, x2, -4

.section .some_section , "ax"
.global _some_symbol

_some_symbol:
    lw x0, 0(x0)
    sw x1, 0(x1)
