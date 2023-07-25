
output_linked.o:     file format elf32-littleriscv


Disassembly of section .init:

00000000 <_start>:
   0:	00000093          	li	ra,0
   4:	0040016f          	jal	sp,8 <_start+0x8>
   8:	ff9ff16f          	jal	sp,0 <_start>
   c:	00410167          	jalr	sp,4(sp)
  10:	ff1ff16f          	jal	sp,0 <_start>
  14:	01000113          	li	sp,16
  18:	00000093          	li	ra,0
  1c:	00408093          	addi	ra,ra,4
  20:	00208463          	beq	ra,sp,28 <_start+0x28>
  24:	fd9ff06f          	j	fffffffc <_some_symbol+0x7ffffffc>
  28:	00209463          	bne	ra,sp,30 <_start+0x30>
  2c:	fddff06f          	j	8 <_start+0x8>
  30:	fd1ff06f          	j	0 <_start>
  34:	ff800093          	li	ra,-8
  38:	00000113          	li	sp,0
  3c:	00408093          	addi	ra,ra,4
  40:	0020d463          	bge	ra,sp,48 <_start+0x48>
  44:	fb9ff06f          	j	fffffffc <_some_symbol+0x7ffffffc>
  48:	ff800093          	li	ra,-8
  4c:	00000113          	li	sp,0
  50:	00408093          	addi	ra,ra,4
  54:	00117463          	bgeu	sp,ra,5c <_start+0x5c>
  58:	fa5ff06f          	j	fffffffc <_some_symbol+0x7ffffffc>
  5c:	ff800093          	li	ra,-8
  60:	00000113          	li	sp,0
  64:	00408093          	addi	ra,ra,4
  68:	00114463          	blt	sp,ra,70 <_start+0x70>
  6c:	f91ff06f          	j	fffffffc <_some_symbol+0x7ffffffc>
  70:	ff800093          	li	ra,-8
  74:	00800113          	li	sp,8
  78:	00408093          	addi	ra,ra,4
  7c:	0020e463          	bltu	ra,sp,84 <_start+0x84>
  80:	f7dff06f          	j	fffffffc <_some_symbol+0x7ffffffc>

Disassembly of section .some_section:

80000000 <_some_symbol>:
80000000:	00002003          	lw	zero,0(zero) # 0 <_start>
80000004:	0010a023          	sw	ra,0(ra)
