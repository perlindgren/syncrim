.option norvc 
	.text
main:
	nop
	csrrwi t1, mstatus, 2 #mstatus = 2, t1 = 0
	csrrci t1, mstatus, 3 #mstatus = 0, t1 = 2
	addi t0, t0, 100
	csrrw t1, mstatus, t0 #mstatus = 100, t1 = 0
	csrrsi t1, mstatus, 2 # mstatus = 102, t1 = 100
	li t0, 3
	csrrs t1, mstatus, t0 # mstatus = 103, t1 = 102
	li t0, 2
	csrrc t1, mstatus, t0 # mstatus = 101, t1 = 103
stop: j stop
