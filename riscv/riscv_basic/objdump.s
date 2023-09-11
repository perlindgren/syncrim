
riscv_basic:	file format elf32-littleriscv

Disassembly of section .text:

00000000 <_start>:
       0: b7 00 00 00  	lui	ra, 0
       4: 67 80 80 00  	jr	8(ra)

00000008 <_abs_start>:
       8: 73 50 40 30  	csrwi	mie, 0
       c: 73 50 40 34  	csrwi	mip, 0
      10: 93 00 00 00  	li	ra, 0
      14: 13 01 00 00  	li	sp, 0
      18: 93 01 00 00  	li	gp, 0
      1c: 13 02 00 00  	li	tp, 0
      20: 93 02 00 00  	li	t0, 0
      24: 13 03 00 00  	li	t1, 0
      28: 93 03 00 00  	li	t2, 0
      2c: 13 04 00 00  	li	s0, 0
      30: 93 04 00 00  	li	s1, 0
      34: 93 06 00 00  	li	a3, 0
      38: 13 07 00 00  	li	a4, 0
      3c: 93 07 00 00  	li	a5, 0
      40: 13 08 00 00  	li	a6, 0
      44: 93 08 00 00  	li	a7, 0
      48: 13 09 00 00  	li	s2, 0
      4c: 93 09 00 00  	li	s3, 0
      50: 13 0a 00 00  	li	s4, 0
      54: 93 0a 00 00  	li	s5, 0
      58: 13 0b 00 00  	li	s6, 0
      5c: 93 0b 00 00  	li	s7, 0
      60: 13 0c 00 00  	li	s8, 0
      64: 93 0c 00 00  	li	s9, 0
      68: 13 0d 00 00  	li	s10, 0
      6c: 93 0d 00 00  	li	s11, 0
      70: 13 0e 00 00  	li	t3, 0
      74: 93 0e 00 00  	li	t4, 0
      78: 13 0f 00 00  	li	t5, 0
      7c: 93 0f 00 00  	li	t6, 0

00000080 <.Lpcrel_hi0>:
      80: 97 01 00 50  	auipc	gp, 327680
      84: 93 81 01 78  	addi	gp, gp, 1920
      88: f3 23 40 f1  	csrr	t2, mhartid
      8c: b7 02 00 00  	lui	t0, 0
      90: 93 82 02 00  	mv	t0, t0
      94: 63 e6 72 1c  	bltu	t0, t2, 0x260 <.Lline_table_start0+0x140>

00000098 <.Lpcrel_hi1>:
      98: 17 41 00 50  	auipc	sp, 327684
      9c: 13 01 81 f6  	addi	sp, sp, -152
      a0: b7 12 00 00  	lui	t0, 1
      a4: 93 82 02 80  	addi	t0, t0, -2048
      a8: 63 8c 03 00  	beqz	t2, 0xc0 <.Lline_table_start0+0xc0>
      ac: 13 83 03 00  	mv	t1, t2
      b0: 13 8e 02 00  	mv	t3, t0
      b4: b3 82 c2 01  	add	t0, t0, t3
      b8: 13 03 f3 ff  	addi	t1, t1, -1
      bc: e3 1c 03 fe  	bnez	t1, 0xb4 <.Lline_table_start0+0xb4>
      c0: 33 01 51 40  	sub	sp, sp, t0
      c4: 33 04 01 00  	add	s0, sp, zero
      c8: 6f 00 40 00  	j	0xcc <.Lline_table_start0+0xcc>

000000cc <_start_rust>:
      cc: 13 01 01 ff  	addi	sp, sp, -16
      d0: 23 26 11 00  	sw	ra, 12(sp)
      d4: 23 24 81 00  	sw	s0, 8(sp)
      d8: 23 22 91 00  	sw	s1, 4(sp)
      dc: 23 20 21 01  	sw	s2, 0(sp)
      e0: f3 26 40 f1  	csrr	a3, mhartid
      e4: 13 04 06 00  	mv	s0, a2
      e8: 93 84 05 00  	mv	s1, a1
      ec: 13 09 05 00  	mv	s2, a0
      f0: 13 85 06 00  	mv	a0, a3
      f4: 97 00 00 00  	auipc	ra, 0
      f8: e7 80 40 1d  	jalr	468(ra)
      fc: 63 06 05 06  	beqz	a0, 0x168 <.Lline_table_start0+0x48>
     100: 97 00 00 00  	auipc	ra, 0
     104: e7 80 40 1c  	jalr	452(ra)
     108: 37 05 00 50  	lui	a0, 327680
     10c: 13 05 05 00  	mv	a0, a0
     110: b7 05 00 50  	lui	a1, 327680
     114: 93 85 05 00  	mv	a1, a1
     118: 63 f8 a5 00  	bgeu	a1, a0, 0x128 <.Lline_table_start0+0x8>
     11c: 23 a0 05 00  	sw	zero, 0(a1)
     120: 93 85 45 00  	addi	a1, a1, 4
     124: e3 ec a5 fe  	bltu	a1, a0, 0x11c <.Lline_table_start0+0x11c>
     128: 37 05 00 50  	lui	a0, 327680
     12c: 93 05 05 00  	mv	a1, a0
     130: 37 05 00 50  	lui	a0, 327680
     134: 13 05 05 00  	mv	a0, a0
     138: 63 78 b5 02  	bgeu	a0, a1, 0x168 <.Lline_table_start0+0x48>
     13c: 13 06 45 00  	addi	a2, a0, 4
     140: 63 64 b6 00  	bltu	a2, a1, 0x148 <.Lline_table_start0+0x28>
     144: 93 05 06 00  	mv	a1, a2
     148: 13 46 f5 ff  	not	a2, a0
     14c: b3 85 c5 00  	add	a1, a1, a2
     150: 93 f5 c5 ff  	andi	a1, a1, -4
     154: 13 86 45 00  	addi	a2, a1, 4
     158: b7 05 00 00  	lui	a1, 0
     15c: 93 85 85 40  	addi	a1, a1, 1032
     160: 97 00 00 00  	auipc	ra, 0
     164: e7 80 00 27  	jalr	624(ra)
     168: 97 00 00 00  	auipc	ra, 0
     16c: e7 80 c0 0f  	jalr	252(ra)
     170: 13 05 09 00  	mv	a0, s2
     174: 93 85 04 00  	mv	a1, s1
     178: 13 06 04 00  	mv	a2, s0
     17c: 97 00 00 00  	auipc	ra, 0
     180: e7 80 80 11  	jalr	280(ra)
     184: 73 10 00 c0  	unimp

00000188 <default_start_trap>:
     188: 13 01 01 fc  	addi	sp, sp, -64
     18c: 23 20 11 00  	sw	ra, 0(sp)
     190: 23 22 51 00  	sw	t0, 4(sp)
     194: 23 24 61 00  	sw	t1, 8(sp)
     198: 23 26 71 00  	sw	t2, 12(sp)
     19c: 23 28 c1 01  	sw	t3, 16(sp)
     1a0: 23 2a d1 01  	sw	t4, 20(sp)
     1a4: 23 2c e1 01  	sw	t5, 24(sp)
     1a8: 23 2e f1 01  	sw	t6, 28(sp)
     1ac: 23 20 a1 02  	sw	a0, 32(sp)
     1b0: 23 22 b1 02  	sw	a1, 36(sp)
     1b4: 23 24 c1 02  	sw	a2, 40(sp)
     1b8: 23 26 d1 02  	sw	a3, 44(sp)
     1bc: 23 28 e1 02  	sw	a4, 48(sp)
     1c0: 23 2a f1 02  	sw	a5, 52(sp)
     1c4: 23 2c 01 03  	sw	a6, 56(sp)
     1c8: 23 2e 11 03  	sw	a7, 60(sp)
     1cc: 33 05 01 00  	add	a0, sp, zero
     1d0: ef 00 c0 04  	jal	0x21c <.Lline_table_start0+0xfc>
     1d4: 83 20 01 00  	lw	ra, 0(sp)
     1d8: 83 22 41 00  	lw	t0, 4(sp)
     1dc: 03 23 81 00  	lw	t1, 8(sp)
     1e0: 83 23 c1 00  	lw	t2, 12(sp)
     1e4: 03 2e 01 01  	lw	t3, 16(sp)
     1e8: 83 2e 41 01  	lw	t4, 20(sp)
     1ec: 03 2f 81 01  	lw	t5, 24(sp)
     1f0: 83 2f c1 01  	lw	t6, 28(sp)
     1f4: 03 25 01 02  	lw	a0, 32(sp)
     1f8: 83 25 41 02  	lw	a1, 36(sp)
     1fc: 03 26 81 02  	lw	a2, 40(sp)
     200: 83 26 c1 02  	lw	a3, 44(sp)
     204: 03 27 01 03  	lw	a4, 48(sp)
     208: 83 27 41 03  	lw	a5, 52(sp)
     20c: 03 28 81 03  	lw	a6, 56(sp)
     210: 83 28 c1 03  	lw	a7, 60(sp)
     214: 13 01 01 04  	addi	sp, sp, 64
     218: 73 00 20 30  	mret

0000021c <_start_trap_rust>:
     21c: f3 25 20 34  	csrr	a1, mcause
     220: 63 c6 05 00  	bltz	a1, 0x22c <.Lline_table_start0+0x10c>
     224: 17 03 00 00  	auipc	t1, 0
     228: 67 00 83 09  	jr	152(t1)
     22c: 13 95 15 00  	slli	a0, a1, 1
     230: 13 55 15 00  	srli	a0, a0, 1
     234: 93 05 c0 00  	li	a1, 12
     238: 63 70 b5 02  	bgeu	a0, a1, 0x258 <.Lline_table_start0+0x138>
     23c: 13 15 25 00  	slli	a0, a0, 2
     240: b7 05 00 00  	lui	a1, 0
     244: 93 85 85 3d  	addi	a1, a1, 984
     248: 33 85 a5 00  	add	a0, a1, a0
     24c: 03 23 05 00  	lw	t1, 0(a0)
     250: 63 04 03 00  	beqz	t1, 0x258 <.Lline_table_start0+0x138>
     254: 67 00 03 00  	jr	t1
     258: 17 03 00 00  	auipc	t1, 0
     25c: 67 00 83 06  	jr	104(t1)

00000260 <abort>:
     260: 6f 00 00 00  	j	0x260 <.Lline_table_start0+0x140>

00000264 <_setup_interrupts>:
     264: 37 05 00 00  	lui	a0, 0
     268: 13 05 85 27  	addi	a0, a0, 632
     26c: 13 05 15 00  	addi	a0, a0, 1
     270: 73 10 55 30  	csrw	mtvec, a0
     274: 67 80 00 00  	ret

00000278 <handler_0>:
     278: 37 15 00 00  	lui	a0, 1
     27c: 23 00 05 00  	sb	zero, 0(a0)
     280: b7 02 00 05  	lui	t0, 20480
     284: 93 82 12 10  	addi	t0, t0, 257
     288: 23 22 55 00  	sw	t0, 4(a0)
     28c: 73 00 20 30  	mret
     290: 67 80 00 00  	ret

00000294 <main>:
     294: 37 15 00 00  	lui	a0, 1
     298: b7 02 00 05  	lui	t0, 20480
     29c: 93 82 12 10  	addi	t0, t0, 257
     2a0: 37 15 00 00  	lui	a0, 1
     2a4: 23 20 55 00  	sw	t0, 0(a0)
     2a8: 73 50 70 34  	csrwi	839, 0
     2ac: 73 50 04 30  	csrwi	mstatus, 8
     2b0: 33 00 50 00  	add	zero, zero, t0
     2b4: 93 82 92 53  	addi	t0, t0, 1337
     2b8: 6f 00 00 00  	j	0x2b8 <.Lline_table_start0+0x198>

000002bc <ExceptionHandler>:
     2bc: 6f 00 00 00  	j	0x2bc <.Lline_table_start0+0x19c>

000002c0 <UserTimer>:
     2c0: 6f 00 00 00  	j	0x2c0 <.Lline_table_start0+0x1a0>

000002c4 <default_pre_init>:
     2c4: 67 80 00 00  	ret

000002c8 <default_mp_hook>:
     2c8: 63 06 05 00  	beqz	a0, 0x2d4 <.Lline_table_start0+0x1b4>
     2cc: 73 00 50 10  	wfi
     2d0: 6f f0 df ff  	j	0x2cc <.Lline_table_start0+0x1ac>
     2d4: 13 05 10 00  	li	a0, 1
     2d8: 67 80 00 00  	ret

000002dc <default_setup_interrupts>:
     2dc: 37 05 00 00  	lui	a0, 0
     2e0: 13 05 85 18  	addi	a0, a0, 392
     2e4: 73 10 55 30  	csrw	mtvec, a0
     2e8: 67 80 00 00  	ret

000002ec <compiler_builtins::mem::memcpy::he1725a944bfb7565>:
     2ec: 93 06 f0 00  	li	a3, 15
     2f0: 63 f8 c6 08  	bgeu	a3, a2, 0x380 <.Lline_table_start0+0x260>
     2f4: b3 06 a0 40  	neg	a3, a0
     2f8: 93 f6 36 00  	andi	a3, a3, 3
     2fc: 33 07 d5 00  	add	a4, a0, a3
     300: 63 80 06 02  	beqz	a3, 0x320 <.Lline_table_start0+0x200>
     304: 93 07 05 00  	mv	a5, a0
     308: 13 88 05 00  	mv	a6, a1
     30c: 83 48 08 00  	lbu	a7, 0(a6)
     310: 23 80 17 01  	sb	a7, 0(a5)
     314: 93 87 17 00  	addi	a5, a5, 1
     318: 13 08 18 00  	addi	a6, a6, 1
     31c: e3 e8 e7 fe  	bltu	a5, a4, 0x30c <.Lline_table_start0+0x1ec>
     320: b3 85 d5 00  	add	a1, a1, a3
     324: 33 06 d6 40  	sub	a2, a2, a3
     328: 93 77 c6 ff  	andi	a5, a2, -4
     32c: 13 f8 35 00  	andi	a6, a1, 3
     330: b3 06 f7 00  	add	a3, a4, a5
     334: 63 0c 08 04  	beqz	a6, 0x38c <.Lline_table_start0+0x26c>
     338: 13 f3 c5 ff  	andi	t1, a1, -4
     33c: 83 28 03 00  	lw	a7, 0(t1)
     340: 63 54 f0 06  	blez	a5, 0x3a8 <.Lline_table_start0+0x288>
     344: 93 92 35 00  	slli	t0, a1, 3
     348: 13 f8 82 01  	andi	a6, t0, 24
     34c: b3 02 50 40  	neg	t0, t0
     350: 93 f2 82 01  	andi	t0, t0, 24
     354: 13 03 43 00  	addi	t1, t1, 4
     358: 83 23 03 00  	lw	t2, 0(t1)
     35c: b3 d8 08 01  	srl	a7, a7, a6
     360: 33 9e 53 00  	sll	t3, t2, t0
     364: b3 68 1e 01  	or	a7, t3, a7
     368: 23 20 17 01  	sw	a7, 0(a4)
     36c: 13 07 47 00  	addi	a4, a4, 4
     370: 13 03 43 00  	addi	t1, t1, 4
     374: 93 88 03 00  	mv	a7, t2
     378: e3 60 d7 fe  	bltu	a4, a3, 0x358 <.Lline_table_start0+0x238>
     37c: 6f 00 c0 02  	j	0x3a8 <.Lline_table_start0+0x288>
     380: 93 06 05 00  	mv	a3, a0
     384: 63 18 06 02  	bnez	a2, 0x3b4 <.Lline_table_start0+0x294>
     388: 6f 00 40 04  	j	0x3cc <.Lline_table_start0+0x2ac>
     38c: 63 5e f0 00  	blez	a5, 0x3a8 <.Lline_table_start0+0x288>
     390: 13 88 05 00  	mv	a6, a1
     394: 83 28 08 00  	lw	a7, 0(a6)
     398: 23 20 17 01  	sw	a7, 0(a4)
     39c: 13 07 47 00  	addi	a4, a4, 4
     3a0: 13 08 48 00  	addi	a6, a6, 4
     3a4: e3 68 d7 fe  	bltu	a4, a3, 0x394 <.Lline_table_start0+0x274>
     3a8: b3 85 f5 00  	add	a1, a1, a5
     3ac: 13 76 36 00  	andi	a2, a2, 3
     3b0: 63 0e 06 00  	beqz	a2, 0x3cc <.Lline_table_start0+0x2ac>
     3b4: 33 86 c6 00  	add	a2, a3, a2
     3b8: 03 c7 05 00  	lbu	a4, 0(a1)
     3bc: 23 80 e6 00  	sb	a4, 0(a3)
     3c0: 93 86 16 00  	addi	a3, a3, 1
     3c4: 93 85 15 00  	addi	a1, a1, 1
     3c8: e3 e8 c6 fe  	bltu	a3, a2, 0x3b8 <.Lline_table_start0+0x298>
     3cc: 67 80 00 00  	ret

000003d0 <memcpy>:
     3d0: 17 03 00 00  	auipc	t1, 0
     3d4: 67 00 c3 f1  	jr	-228(t1)
