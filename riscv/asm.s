 LUI sp, 327681 #
 ORI sp, sp, 0 #
 ADDI t0, zero, 0 #
 ADDI sp, sp, -4 #literal 0
 SW t0, 0(sp) #
 LW t0, 0(sp) #
 ADDI sp, sp, 4 #
 SW t0, -4(s0) #let a = 0
 LW t0, -4(s0) #While cond
 ADDI sp, sp, -4 #Ident: a
 SW t0, 0(sp) #
 ADDI t0, zero, 5 #
 ADDI sp, sp, -4 #literal 5
 SW t0, 0(sp) #
 LW t0, 0(sp) #
 ADDI sp, sp, 4 #
 LW t1, 0(sp) #
 ADDI sp, sp, 4 #
 SLT t0, t1, t0 #
 ADDI sp, sp, -4 #a < 5
 SW t0, 0(sp) #
 LW t0, 0(sp) #
 ADDI sp, sp, 4 #
 BEQ t0, zero, 72 #branch if cond false
 LW t0, -4(s0) #
 ADDI sp, sp, -4 #Ident: a
 SW t0, 0(sp) #
 ADDI t0, zero, 1 #
 ADDI sp, sp, -4 #literal 1
 SW t0, 0(sp) #
 LW t0, 0(sp) #
 ADDI sp, sp, 4 #
 LW t1, 0(sp) #
 ADDI sp, sp, 4 #
 ADD t0, t1, t0 #
 ADDI sp, sp, -4 #a + 1
 SW t0, 0(sp) #
 LW t0, 0(sp) #
 ADDI sp, sp, 4 #
 SW t0, -4(s0) #
 BEQ zero, zero, -132 #Branch back (while)
 LW t0, -4(s0) #
 ADDI sp, sp, -4 #Ident: a
 SW t0, 0(sp) #
 ADDI sp, sp, -4 #Push block result
 SW t0, 0(sp) #
 EBREAK #
