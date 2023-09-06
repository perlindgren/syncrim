
./output:     file format elf32-littleriscv


Disassembly of section .text:

00000000 <main>:
   0:	040002b7          	lui	t0,0x4000
   4:	10128293          	addi	t0,t0,257 # 4000101 <stop+0x40000d9>
   8:	00001537          	lui	a0,0x1
   c:	00552023          	sw	t0,0(a0) # 1000 <stop+0xfd8>
  10:	500002b7          	lui	t0,0x50000
  14:	30529073          	.4byte	0x30529073
  18:	34745073          	.4byte	0x34745073
  1c:	30045073          	.4byte	0x30045073
  20:	00110113          	addi	sp,sp,1
  24:	3471d073          	.4byte	0x3471d073

00000028 <stop>:
  28:	0000006f          	j	28 <stop>

Disassembly of section .isr0:

00000000 <isr0>:
   0:	123452b7          	lui	t0,0x12345
   4:	67828293          	addi	t0,t0,1656 # 12345678 <stop+0x12345650>
   8:	0200006f          	j	28 <stop>

Disassembly of section .debug_abbrev:

00000000 <.debug_abbrev>:
   0:	1101                	.2byte	0x1101
   2:	2501                	.2byte	0x2501
   4:	130e                	.2byte	0x130e
   6:	0305                	.2byte	0x305
   8:	100e                	.2byte	0x100e
   a:	b40e1b17          	auipc	s6,0xb40e1
   e:	1942                	.2byte	0x1942
  10:	0111                	.2byte	0x111
  12:	0612                	.2byte	0x612
  14:	0000                	.2byte	0x0
  16:	3902                	.2byte	0x3902
  18:	0301                	.2byte	0x301
  1a:	000e                	.2byte	0xe
  1c:	0300                	.2byte	0x300
  1e:	0104                	.2byte	0x104
  20:	1349                	.2byte	0x1349
  22:	196d                	.2byte	0x196d
  24:	0b0b0e03          	lb	t3,176(s6) # b40e10ba <test_word+0x640e10ba>
  28:	0188                	.2byte	0x188
  2a:	0400000f          	fence	o,unknown
  2e:	0028                	.2byte	0x28
  30:	0f1c0e03          	lb	t3,241(s8)
  34:	0000                	.2byte	0x0
  36:	1305                	.2byte	0x1305
  38:	0301                	.2byte	0x301
  3a:	0b0e                	.2byte	0xb0e
  3c:	0f01880b          	.4byte	0xf01880b
  40:	0000                	.2byte	0x0
  42:	0d06                	.2byte	0xd06
  44:	0300                	.2byte	0x300
  46:	490e                	.2byte	0x490e
  48:	0f018813          	addi	a6,gp,240
  4c:	0b38                	.2byte	0xb38
  4e:	0000                	.2byte	0x0
  50:	15013307          	.4byte	0x15013307
  54:	08000013          	li	zero,128
  58:	000d                	.2byte	0xd
  5a:	1349                	.2byte	0x1349
  5c:	0188                	.2byte	0x188
  5e:	340b380f          	.4byte	0x340b380f
  62:	0019                	.2byte	0x19
  64:	0900                	.2byte	0x900
  66:	0119                	.2byte	0x119
  68:	0b16                	.2byte	0xb16
  6a:	0000                	.2byte	0x0
  6c:	130a                	.2byte	0x130a
  6e:	0300                	.2byte	0x300
  70:	0b0e                	.2byte	0xb0e
  72:	0f01880b          	.4byte	0xf01880b
  76:	0000                	.2byte	0x0
  78:	0001190b          	.4byte	0x1190b
  7c:	0c00                	.2byte	0xc00
  7e:	1349002f          	.4byte	0x1349002f
  82:	00000e03          	lb	t3,0(zero) # 0 <isr0>
  86:	240d                	.2byte	0x240d
  88:	0300                	.2byte	0x300
  8a:	3e0e                	.2byte	0x3e0e
  8c:	000b0b0b          	.4byte	0xb0b0b
  90:	0e00                	.2byte	0xe00
  92:	012e                	.2byte	0x12e
  94:	0111                	.2byte	0x111
  96:	0612                	.2byte	0x612
  98:	1840                	.2byte	0x1840
  9a:	0e6e                	.2byte	0xe6e
  9c:	0b3a0e03          	lb	t3,179(s4)
  a0:	01870b3b          	.4byte	0x1870b3b
  a4:	0019                	.2byte	0x19
  a6:	0f00                	.2byte	0xf00
  a8:	0005                	.2byte	0x5
  aa:	1802                	.2byte	0x1802
  ac:	0b3a0e03          	lb	t3,179(s4)
  b0:	13490b3b          	.4byte	0x13490b3b
  b4:	0000                	.2byte	0x0
  b6:	0f10                	.2byte	0xf10
  b8:	4900                	.2byte	0x4900
  ba:	330e0313          	addi	t1,t3,816
  be:	0006                	.2byte	0x6
  c0:	1100                	.2byte	0x1100
  c2:	1349000f          	.4byte	0x1349000f
  c6:	00000633          	add	a2,zero,zero
  ca:	0112                	.2byte	0x112
  cc:	4901                	.2byte	0x4901
  ce:	13000013          	li	zero,304
  d2:	0021                	.2byte	0x21
  d4:	1349                	.2byte	0x1349
  d6:	0d22                	.2byte	0xd22
  d8:	00000b37          	lui	s6,0x0
  dc:	2414                	.2byte	0x2414
  de:	0300                	.2byte	0x300
  e0:	0b0e                	.2byte	0xb0e
  e2:	000b3e0b          	.4byte	0xb3e0b
  e6:	1500                	.2byte	0x1500
  e8:	0115                	.2byte	0x115
  ea:	1349                	.2byte	0x1349
  ec:	0000                	.2byte	0x0
  ee:	0516                	.2byte	0x516
  f0:	4900                	.2byte	0x4900
  f2:	00000013          	nop

Disassembly of section .debug_info:

00000000 <.debug_info>:
   0:	058c                	.2byte	0x58c
   2:	0000                	.2byte	0x0
   4:	0004                	.2byte	0x4
   6:	0000                	.2byte	0x0
   8:	0000                	.2byte	0x0
   a:	0104                	.2byte	0x104
   c:	0000012f          	.4byte	0x12f
  10:	001c                	.2byte	0x1c
  12:	0211                	.2byte	0x211
  14:	0000                	.2byte	0x0
  16:	0000                	.2byte	0x0
  18:	0000                	.2byte	0x0
  1a:	01c9                	.2byte	0x1c9
  1c:	0000                	.2byte	0x0
  1e:	0000                	.2byte	0x0
  20:	0000                	.2byte	0x0
  22:	0004                	.2byte	0x4
  24:	0000                	.2byte	0x0
  26:	2a02                	.2byte	0x2a02
  28:	0001                	.2byte	0x1
  2a:	0200                	.2byte	0x200
  2c:	0439                	.2byte	0x439
  2e:	0000                	.2byte	0x0
  30:	8a02                	.2byte	0x8a02
  32:	0002                	.2byte	0x2
  34:	0300                	.2byte	0x300
  36:	03c5                	.2byte	0x3c5
  38:	0000                	.2byte	0x0
  3a:	0000                	.2byte	0x0
  3c:	0000                	.2byte	0x0
  3e:	0101                	.2byte	0x101
  40:	1d04                	.2byte	0x1d04
  42:	0004                	.2byte	0x4
  44:	0000                	.2byte	0x0
  46:	3d04                	.2byte	0x3d04
  48:	0004                	.2byte	0x4
  4a:	0100                	.2byte	0x100
  4c:	de04                	.2byte	0xde04
  4e:	0000                	.2byte	0x0
  50:	0200                	.2byte	0x200
  52:	5f04                	.2byte	0x5f04
  54:	0004                	.2byte	0x4
  56:	0300                	.2byte	0x300
  58:	0500                	.2byte	0x500
  5a:	027e                	.2byte	0x27e
  5c:	0000                	.2byte	0x0
  5e:	0420                	.2byte	0x420
  60:	a406                	.2byte	0xa406
  62:	4a000003          	lb	zero,1184(zero) # 4a0 <stop+0x478>
  66:	0004                	.2byte	0x4
  68:	0400                	.2byte	0x400
  6a:	0614                	.2byte	0x614
  6c:	00000467          	jalr	s0,zero # 0 <isr0>
  70:	04e1                	.2byte	0x4e1
  72:	0000                	.2byte	0x0
  74:	1004                	.2byte	0x1004
  76:	5406                	.2byte	0x5406
  78:	0004                	.2byte	0x4
  7a:	3500                	.2byte	0x3500
  7c:	0000                	.2byte	0x0
  7e:	0100                	.2byte	0x100
  80:	061c                	.2byte	0x61c
  82:	00000403          	lb	s0,0(zero) # 0 <isr0>
  86:	0458                	.2byte	0x458
  88:	0000                	.2byte	0x0
  8a:	1804                	.2byte	0x1804
  8c:	8e06                	.2byte	0x8e06
  8e:	0004                	.2byte	0x4
  90:	a300                	.2byte	0xa300
  92:	0000                	.2byte	0x0
  94:	0400                	.2byte	0x400
  96:	0600                	.2byte	0x600
  98:	0344                	.2byte	0x344
  9a:	0000                	.2byte	0x0
  9c:	000000a3          	sb	zero,1(zero) # 1 <isr0+0x1>
  a0:	0804                	.2byte	0x804
  a2:	0500                	.2byte	0x500
  a4:	00000313          	li	t1,0
  a8:	0408                	.2byte	0x408
  aa:	0000af07          	.4byte	0xaf07
  ae:	0800                	.2byte	0x800
  b0:	0458                	.2byte	0x458
  b2:	0000                	.2byte	0x0
  b4:	0004                	.2byte	0x4
  b6:	0009                	.2byte	0x9
  b8:	4106                	.2byte	0x4106
  ba:	e1000003          	lb	zero,-496(zero) # fffffe10 <test_word+0xaffffe10>
  be:	0000                	.2byte	0x0
  c0:	0400                	.2byte	0x400
  c2:	0000                	.2byte	0x0
  c4:	0109                	.2byte	0x109
  c6:	9806                	.2byte	0x9806
  c8:	0004                	.2byte	0x4
  ca:	f400                	.2byte	0xf400
  cc:	0000                	.2byte	0x0
  ce:	0400                	.2byte	0x400
  d0:	0000                	.2byte	0x0
  d2:	0209                	.2byte	0x209
  d4:	7606                	.2byte	0x7606
  d6:	0002                	.2byte	0x2
  d8:	0700                	.2byte	0x700
  da:	0001                	.2byte	0x1
  dc:	0400                	.2byte	0x400
  de:	0000                	.2byte	0x0
  e0:	0500                	.2byte	0x500
  e2:	0341                	.2byte	0x341
  e4:	0000                	.2byte	0x0
  e6:	0408                	.2byte	0x408
  e8:	bf06                	.2byte	0xbf06
  ea:	0002                	.2byte	0x2
  ec:	4a00                	.2byte	0x4a00
  ee:	0004                	.2byte	0x4
  f0:	0400                	.2byte	0x400
  f2:	0004                	.2byte	0x4
  f4:	9805                	.2byte	0x9805
  f6:	0004                	.2byte	0x4
  f8:	0800                	.2byte	0x800
  fa:	0604                	.2byte	0x604
  fc:	000002bf 0000044a 	.8byte	0x44a000002bf
 104:	0404                	.2byte	0x404
 106:	0a00                	.2byte	0xa00
 108:	0276                	.2byte	0x276
 10a:	0000                	.2byte	0x0
 10c:	0408                	.2byte	0x408
 10e:	0500                	.2byte	0x500
 110:	0425                	.2byte	0x425
 112:	0000                	.2byte	0x0
 114:	0408                	.2byte	0x408
 116:	3406                	.2byte	0x3406
 118:	0f000003          	lb	zero,240(zero) # f0 <stop+0xc8>
 11c:	0005                	.2byte	0x5
 11e:	0400                	.2byte	0x400
 120:	0600                	.2byte	0x600
 122:	0110                	.2byte	0x110
 124:	0000                	.2byte	0x0
 126:	051c                	.2byte	0x51c
 128:	0000                	.2byte	0x0
 12a:	0404                	.2byte	0x404
 12c:	0200                	.2byte	0x200
 12e:	049e                	.2byte	0x49e
 130:	0000                	.2byte	0x0
 132:	a10a                	.2byte	0xa10a
 134:	0001                	.2byte	0x1
 136:	0000                	.2byte	0x0
 138:	0001                	.2byte	0x1
 13a:	0500                	.2byte	0x500
 13c:	044a                	.2byte	0x44a
 13e:	0000                	.2byte	0x0
 140:	0418                	.2byte	0x418
 142:	4306                	.2byte	0x4306
 144:	0004                	.2byte	0x4
 146:	6c00                	.2byte	0x6c00
 148:	0004                	.2byte	0x4
 14a:	0400                	.2byte	0x400
 14c:	0600                	.2byte	0x600
 14e:	0439                	.2byte	0x439
 150:	0000                	.2byte	0x0
 152:	00000287          	.4byte	0x287
 156:	1004                	.2byte	0x1004
 158:	0b06                	.2byte	0xb06
 15a:	0001                	.2byte	0x1
 15c:	e800                	.2byte	0xe800
 15e:	0004                	.2byte	0x4
 160:	0400                	.2byte	0x400
 162:	0008                	.2byte	0x8
 164:	ad0a                	.2byte	0xad0a
 166:	00000003          	lb	zero,0(zero) # 0 <isr0>
 16a:	0501                	.2byte	0x501
 16c:	000001bf 03060424 	.8byte	0x3060424000001bf
 174:	0004                	.2byte	0x4
 176:	5800                	.2byte	0x5800
 178:	0004                	.2byte	0x4
 17a:	0400                	.2byte	0x400
 17c:	061c                	.2byte	0x61c
 17e:	00000467          	jalr	s0,zero # 0 <isr0>
 182:	04e1                	.2byte	0x4e1
 184:	0000                	.2byte	0x0
 186:	1004                	.2byte	0x1004
 188:	5406                	.2byte	0x5406
 18a:	0004                	.2byte	0x4
 18c:	3500                	.2byte	0x3500
 18e:	0000                	.2byte	0x0
 190:	0100                	.2byte	0x100
 192:	0620                	.2byte	0x620
 194:	0344                	.2byte	0x344
 196:	0000                	.2byte	0x0
 198:	02e4                	.2byte	0x2e4
 19a:	0000                	.2byte	0x0
 19c:	0004                	.2byte	0x4
 19e:	8e06                	.2byte	0x8e06
 1a0:	0004                	.2byte	0x4
 1a2:	e400                	.2byte	0xe400
 1a4:	0002                	.2byte	0x2
 1a6:	0400                	.2byte	0x400
 1a8:	0608                	.2byte	0x608
 1aa:	01a8                	.2byte	0x1a8
 1ac:	0000                	.2byte	0x0
 1ae:	054d                	.2byte	0x54d
 1b0:	0000                	.2byte	0x0
 1b2:	1404                	.2byte	0x1404
 1b4:	0000                	.2byte	0x0
 1b6:	2e02                	.2byte	0x2e02
 1b8:	02000003          	lb	zero,32(zero) # 20 <isr0+0x20>
 1bc:	0266                	.2byte	0x266
 1be:	0000                	.2byte	0x0
 1c0:	1305                	.2byte	0x1305
 1c2:	0004                	.2byte	0x4
 1c4:	1400                	.2byte	0x1400
 1c6:	0604                	.2byte	0x604
 1c8:	0252                	.2byte	0x252
 1ca:	0000                	.2byte	0x0
 1cc:	0402                	.2byte	0x402
 1ce:	0000                	.2byte	0x0
 1d0:	0004                	.2byte	0x4
 1d2:	0406                	.2byte	0x406
 1d4:	2a000003          	lb	zero,672(zero) # 2a0 <stop+0x278>
 1d8:	0002                	.2byte	0x2
 1da:	0400                	.2byte	0x400
 1dc:	060c                	.2byte	0x60c
 1de:	025d                	.2byte	0x25d
 1e0:	0000                	.2byte	0x0
 1e2:	0000057b          	.4byte	0x57b
 1e6:	0804                	.2byte	0x804
 1e8:	2e06                	.2byte	0x2e06
 1ea:	0004                	.2byte	0x4
 1ec:	8800                	.2byte	0x8800
 1ee:	0005                	.2byte	0x5
 1f0:	0100                	.2byte	0x100
 1f2:	0010                	.2byte	0x10
 1f4:	0200                	.2byte	0x200
 1f6:	025d                	.2byte	0x25d
 1f8:	0000                	.2byte	0x0
 1fa:	2105                	.2byte	0x2105
 1fc:	0001                	.2byte	0x1
 1fe:	1000                	.2byte	0x1000
 200:	0604                	.2byte	0x604
 202:	045a                	.2byte	0x45a
 204:	0000                	.2byte	0x0
 206:	00000493          	li	s1,0
 20a:	0004                	.2byte	0x4
 20c:	7506                	.2byte	0x7506
 20e:	58000003          	lb	zero,1408(zero) # 580 <stop+0x558>
 212:	0004                	.2byte	0x4
 214:	0400                	.2byte	0x400
 216:	0608                	.2byte	0x608
 218:	03ff                	.2byte	0x3ff
 21a:	0000                	.2byte	0x0
 21c:	0458                	.2byte	0x458
 21e:	0000                	.2byte	0x0
 220:	0c04                	.2byte	0xc04
 222:	0000                	.2byte	0x0
 224:	0200                	.2byte	0x200
 226:	030c                	.2byte	0x30c
 228:	0000                	.2byte	0x0
 22a:	e605                	.2byte	0xe605
 22c:	0002                	.2byte	0x2
 22e:	0400                	.2byte	0x400
 230:	0704                	.2byte	0x704
 232:	0236                	.2byte	0x236
 234:	0000                	.2byte	0x0
 236:	5808                	.2byte	0x5808
 238:	0004                	.2byte	0x4
 23a:	0400                	.2byte	0x400
 23c:	0900                	.2byte	0x900
 23e:	0600                	.2byte	0x600
 240:	036c                	.2byte	0x36c
 242:	0000                	.2byte	0x0
 244:	0259                	.2byte	0x259
 246:	0000                	.2byte	0x0
 248:	0004                	.2byte	0x4
 24a:	0b00                	.2byte	0xb00
 24c:	7106                	.2byte	0x7106
 24e:	0002                	.2byte	0x2
 250:	6a00                	.2byte	0x6a00
 252:	0002                	.2byte	0x2
 254:	0400                	.2byte	0x400
 256:	0000                	.2byte	0x0
 258:	0500                	.2byte	0x500
 25a:	036c                	.2byte	0x36c
 25c:	0000                	.2byte	0x0
 25e:	0404                	.2byte	0x404
 260:	5f0c                	.2byte	0x5f0c
 262:	0004                	.2byte	0x4
 264:	2700                	.2byte	0x2700
 266:	0000                	.2byte	0x0
 268:	0000                	.2byte	0x0
 26a:	7105                	.2byte	0x7105
 26c:	0002                	.2byte	0x2
 26e:	0400                	.2byte	0x400
 270:	0c04                	.2byte	0xc04
 272:	045f 0000 0027      	.byte	0x5f, 0x04, 0x00, 0x00, 0x27, 0x00
 278:	0000                	.2byte	0x0
 27a:	bf06                	.2byte	0xbf06
 27c:	0002                	.2byte	0x2
 27e:	5f00                	.2byte	0x5f00
 280:	0004                	.2byte	0x4
 282:	0400                	.2byte	0x400
 284:	0000                	.2byte	0x0
 286:	0500                	.2byte	0x500
 288:	00e5                	.2byte	0xe5
 28a:	0000                	.2byte	0x0
 28c:	0408                	.2byte	0x408
 28e:	00029307          	.4byte	0x29307
 292:	0800                	.2byte	0x800
 294:	0458                	.2byte	0x458
 296:	0000                	.2byte	0x0
 298:	0004                	.2byte	0x4
 29a:	0009                	.2byte	0x9
 29c:	6c06                	.2byte	0x6c06
 29e:	b6000003          	lb	zero,-1184(zero) # fffffb60 <test_word+0xaffffb60>
 2a2:	0002                	.2byte	0x2
 2a4:	0400                	.2byte	0x400
 2a6:	0000                	.2byte	0x0
 2a8:	0271060b          	.4byte	0x271060b
 2ac:	0000                	.2byte	0x0
 2ae:	000002c7          	.4byte	0x2c7
 2b2:	0004                	.2byte	0x4
 2b4:	0000                	.2byte	0x0
 2b6:	6c05                	.2byte	0x6c05
 2b8:	08000003          	lb	zero,128(zero) # 80 <stop+0x58>
 2bc:	0c04                	.2byte	0xc04
 2be:	04ba                	.2byte	0x4ba
 2c0:	0000                	.2byte	0x0
 2c2:	00000027          	.4byte	0x27
 2c6:	0500                	.2byte	0x500
 2c8:	0271                	.2byte	0x271
 2ca:	0000                	.2byte	0x0
 2cc:	0408                	.2byte	0x408
 2ce:	ba0c                	.2byte	0xba0c
 2d0:	0004                	.2byte	0x4
 2d2:	2700                	.2byte	0x2700
 2d4:	0000                	.2byte	0x0
 2d6:	0600                	.2byte	0x600
 2d8:	000002bf 000004ba 	.8byte	0x4ba000002bf
 2e0:	0004                	.2byte	0x4
 2e2:	0000                	.2byte	0x0
 2e4:	0305                	.2byte	0x305
 2e6:	0002                	.2byte	0x2
 2e8:	0800                	.2byte	0x800
 2ea:	0704                	.2byte	0x704
 2ec:	02f0                	.2byte	0x2f0
 2ee:	0000                	.2byte	0x0
 2f0:	5808                	.2byte	0x5808
 2f2:	0004                	.2byte	0x4
 2f4:	0400                	.2byte	0x400
 2f6:	0900                	.2byte	0x900
 2f8:	0600                	.2byte	0x600
 2fa:	036c                	.2byte	0x36c
 2fc:	0000                	.2byte	0x0
 2fe:	0314                	.2byte	0x314
 300:	0000                	.2byte	0x0
 302:	0004                	.2byte	0x4
 304:	0900                	.2byte	0x900
 306:	0601                	.2byte	0x601
 308:	0271                	.2byte	0x271
 30a:	0000                	.2byte	0x0
 30c:	0325                	.2byte	0x325
 30e:	0000                	.2byte	0x0
 310:	0004                	.2byte	0x4
 312:	0000                	.2byte	0x0
 314:	6c05                	.2byte	0x6c05
 316:	08000003          	lb	zero,128(zero) # 80 <stop+0x58>
 31a:	0c04                	.2byte	0xc04
 31c:	044a                	.2byte	0x44a
 31e:	0000                	.2byte	0x0
 320:	00000027          	.4byte	0x27
 324:	0500                	.2byte	0x500
 326:	0271                	.2byte	0x271
 328:	0000                	.2byte	0x0
 32a:	0408                	.2byte	0x408
 32c:	4a0c                	.2byte	0x4a0c
 32e:	0004                	.2byte	0x4
 330:	2700                	.2byte	0x2700
 332:	0000                	.2byte	0x0
 334:	0600                	.2byte	0x600
 336:	000002bf 0000044a 	.8byte	0x44a000002bf
 33e:	0404                	.2byte	0x404
 340:	0000                	.2byte	0x0
 342:	0200                	.2byte	0x200
 344:	011a                	.2byte	0x11a
 346:	0000                	.2byte	0x0
 348:	e205                	.2byte	0xe205
 34a:	01000003          	lb	zero,16(zero) # 10 <isr0+0x10>
 34e:	0701                	.2byte	0x701
 350:	0354                	.2byte	0x354
 352:	0000                	.2byte	0x0
 354:	c508                	.2byte	0xc508
 356:	01000003          	lb	zero,16(zero) # 10 <isr0+0x10>
 35a:	0900                	.2byte	0x900
 35c:	0600                	.2byte	0x600
 35e:	0422                	.2byte	0x422
 360:	0000                	.2byte	0x0
 362:	0378                	.2byte	0x378
 364:	0000                	.2byte	0x0
 366:	0001                	.2byte	0x1
 368:	0900                	.2byte	0x900
 36a:	0601                	.2byte	0x601
 36c:	0371                	.2byte	0x371
 36e:	0000                	.2byte	0x0
 370:	039d                	.2byte	0x39d
 372:	0000                	.2byte	0x0
 374:	0001                	.2byte	0x1
 376:	0000                	.2byte	0x0
 378:	2205                	.2byte	0x2205
 37a:	0004                	.2byte	0x4
 37c:	0100                	.2byte	0x100
 37e:	0c01                	.2byte	0xc01
 380:	0539                	.2byte	0x539
 382:	0000                	.2byte	0x0
 384:	00000027          	.4byte	0x27
 388:	640c                	.2byte	0x640c
 38a:	0001                	.2byte	0x1
 38c:	b800                	.2byte	0xb800
 38e:	0002                	.2byte	0x2
 390:	0600                	.2byte	0x600
 392:	000002bf 00000539 	.8byte	0x539000002bf
 39a:	0101                	.2byte	0x101
 39c:	0500                	.2byte	0x500
 39e:	0371                	.2byte	0x371
 3a0:	0000                	.2byte	0x0
 3a2:	0101                	.2byte	0x101
 3a4:	390c                	.2byte	0x390c
 3a6:	0005                	.2byte	0x5
 3a8:	2700                	.2byte	0x2700
 3aa:	0000                	.2byte	0x0
 3ac:	0c00                	.2byte	0xc00
 3ae:	0164                	.2byte	0x164
 3b0:	0000                	.2byte	0x0
 3b2:	02b8                	.2byte	0x2b8
 3b4:	0000                	.2byte	0x0
 3b6:	bf06                	.2byte	0xbf06
 3b8:	0002                	.2byte	0x2
 3ba:	6400                	.2byte	0x6400
 3bc:	0001                	.2byte	0x1
 3be:	0100                	.2byte	0x100
 3c0:	0001                	.2byte	0x1
 3c2:	0000                	.2byte	0x0
 3c4:	0d00                	.2byte	0xd00
 3c6:	0024                	.2byte	0x24
 3c8:	0000                	.2byte	0x0
 3ca:	09020107          	.4byte	0x9020107
 3ce:	0004                	.2byte	0x4
 3d0:	0e00                	.2byte	0xe00
 3d2:	0000                	.2byte	0x0
 3d4:	0000                	.2byte	0x0
 3d6:	0004                	.2byte	0x4
 3d8:	0000                	.2byte	0x0
 3da:	5201                	.2byte	0x5201
 3dc:	0240                	.2byte	0x240
 3de:	0000                	.2byte	0x0
 3e0:	032e                	.2byte	0x32e
 3e2:	0000                	.2byte	0x0
 3e4:	0801                	.2byte	0x801
 3e6:	705a010f          	.4byte	0x705a010f
 3ea:	0001                	.2byte	0x1
 3ec:	0100                	.2byte	0x100
 3ee:	f508                	.2byte	0xf508
 3f0:	00000003          	lb	zero,0(zero) # 0 <isr0>
 3f4:	1000                	.2byte	0x1000
 3f6:	01c0                	.2byte	0x1c0
 3f8:	0000                	.2byte	0x0
 3fa:	00ba                	.2byte	0xba
 3fc:	0000                	.2byte	0x0
 3fe:	0000                	.2byte	0x0
 400:	0000                	.2byte	0x0
 402:	7605                	.2byte	0x7605
 404:	0001                	.2byte	0x1
 406:	0800                	.2byte	0x800
 408:	0604                	.2byte	0x604
 40a:	000001fb          	.4byte	0x1fb
 40e:	0420                	.2byte	0x420
 410:	0000                	.2byte	0x0
 412:	0004                	.2byte	0x4
 414:	8d06                	.2byte	0x8d06
 416:	0002                	.2byte	0x2
 418:	3000                	.2byte	0x3000
 41a:	0004                	.2byte	0x4
 41c:	0400                	.2byte	0x400
 41e:	0004                	.2byte	0x4
 420:	2911                	.2byte	0x2911
 422:	0004                	.2byte	0x4
 424:	0000                	.2byte	0x0
 426:	0000                	.2byte	0x0
 428:	0a00                	.2byte	0xa00
 42a:	037a                	.2byte	0x37a
 42c:	0000                	.2byte	0x0
 42e:	0100                	.2byte	0x100
 430:	3d10                	.2byte	0x3d10
 432:	0004                	.2byte	0x4
 434:	6000                	.2byte	0x6000
 436:	00000003          	lb	zero,0(zero) # 0 <isr0>
 43a:	0000                	.2byte	0x0
 43c:	1200                	.2byte	0x1200
 43e:	044a                	.2byte	0x44a
 440:	0000                	.2byte	0x0
 442:	00045113          	srli	sp,s0,0x0
 446:	0000                	.2byte	0x0
 448:	940d0003          	lb	zero,-1728(s10)
 44c:	0002                	.2byte	0x2
 44e:	0700                	.2byte	0x700
 450:	1404                	.2byte	0x1404
 452:	03ce                	.2byte	0x3ce
 454:	0000                	.2byte	0x0
 456:	0708                	.2byte	0x708
 458:	f70d                	.2byte	0xf70d
 45a:	0001                	.2byte	0x1
 45c:	0700                	.2byte	0x700
 45e:	1004                	.2byte	0x1004
 460:	0000013b          	.4byte	0x13b
 464:	034a                	.2byte	0x34a
 466:	0000                	.2byte	0x0
 468:	0000                	.2byte	0x0
 46a:	0000                	.2byte	0x0
 46c:	8605                	.2byte	0x8605
 46e:	0004                	.2byte	0x4
 470:	0800                	.2byte	0x800
 472:	0604                	.2byte	0x604
 474:	01b1                	.2byte	0x1b1
 476:	0000                	.2byte	0x0
 478:	048a                	.2byte	0x48a
 47a:	0000                	.2byte	0x0
 47c:	0004                	.2byte	0x4
 47e:	3a06                	.2byte	0x3a06
 480:	4a000003          	lb	zero,1184(zero) # 4a0 <stop+0x478>
 484:	0004                	.2byte	0x4
 486:	0400                	.2byte	0x400
 488:	0004                	.2byte	0x4
 48a:	9311                	.2byte	0x9311
 48c:	0004                	.2byte	0x4
 48e:	0000                	.2byte	0x0
 490:	0000                	.2byte	0x0
 492:	0500                	.2byte	0x500
 494:	01ba                	.2byte	0x1ba
 496:	0000                	.2byte	0x0
 498:	0408                	.2byte	0x408
 49a:	b106                	.2byte	0xb106
 49c:	0001                	.2byte	0x1
 49e:	b100                	.2byte	0xb100
 4a0:	0004                	.2byte	0x4
 4a2:	0400                	.2byte	0x400
 4a4:	0600                	.2byte	0x600
 4a6:	033a                	.2byte	0x33a
 4a8:	0000                	.2byte	0x0
 4aa:	044a                	.2byte	0x44a
 4ac:	0000                	.2byte	0x0
 4ae:	0404                	.2byte	0x404
 4b0:	1100                	.2byte	0x1100
 4b2:	03c5                	.2byte	0x3c5
 4b4:	0000                	.2byte	0x0
 4b6:	0000                	.2byte	0x0
 4b8:	0000                	.2byte	0x0
 4ba:	9a05                	.2byte	0x9a05
 4bc:	0002                	.2byte	0x2
 4be:	0800                	.2byte	0x800
 4c0:	0604                	.2byte	0x604
 4c2:	01b1                	.2byte	0x1b1
 4c4:	0000                	.2byte	0x0
 4c6:	04d8                	.2byte	0x4d8
 4c8:	0000                	.2byte	0x0
 4ca:	0004                	.2byte	0x4
 4cc:	3a06                	.2byte	0x3a06
 4ce:	4a000003          	lb	zero,1184(zero) # 4a0 <stop+0x478>
 4d2:	0004                	.2byte	0x4
 4d4:	0400                	.2byte	0x400
 4d6:	0004                	.2byte	0x4
 4d8:	5911                	.2byte	0x5911
 4da:	0000                	.2byte	0x0
 4dc:	0000                	.2byte	0x0
 4de:	0000                	.2byte	0x0
 4e0:	0d00                	.2byte	0xd00
 4e2:	02ba                	.2byte	0x2ba
 4e4:	0000                	.2byte	0x0
 4e6:	0410                	.2byte	0x410
 4e8:	b305                	.2byte	0xb305
 4ea:	08000003          	lb	zero,128(zero) # 80 <stop+0x58>
 4ee:	0604                	.2byte	0x604
 4f0:	01b1                	.2byte	0x1b1
 4f2:	0000                	.2byte	0x0
 4f4:	0506                	.2byte	0x506
 4f6:	0000                	.2byte	0x0
 4f8:	0004                	.2byte	0x4
 4fa:	3a06                	.2byte	0x3a06
 4fc:	4a000003          	lb	zero,1184(zero) # 4a0 <stop+0x478>
 500:	0004                	.2byte	0x4
 502:	0400                	.2byte	0x400
 504:	0004                	.2byte	0x4
 506:	0f11                	.2byte	0xf11
 508:	0001                	.2byte	0x1
 50a:	0000                	.2byte	0x0
 50c:	0000                	.2byte	0x0
 50e:	1000                	.2byte	0x1000
 510:	0132                	.2byte	0x132
 512:	0000                	.2byte	0x0
 514:	000002c3          	.4byte	0x2c3
 518:	0000                	.2byte	0x0
 51a:	0000                	.2byte	0x0
 51c:	2910                	.2byte	0x2910
 51e:	0005                	.2byte	0x5
 520:	2900                	.2byte	0x2900
 522:	0000                	.2byte	0x0
 524:	0000                	.2byte	0x0
 526:	0000                	.2byte	0x0
 528:	1500                	.2byte	0x1500
 52a:	0348                	.2byte	0x348
 52c:	0000                	.2byte	0x0
 52e:	0f16                	.2byte	0xf16
 530:	0005                	.2byte	0x5
 532:	1600                	.2byte	0x1600
 534:	0540                	.2byte	0x540
 536:	0000                	.2byte	0x0
 538:	0d00                	.2byte	0xd00
 53a:	025a                	.2byte	0x25a
 53c:	0000                	.2byte	0x0
 53e:	6b100007          	.4byte	0x6b100007
 542:	0001                	.2byte	0x1
 544:	6c00                	.2byte	0x6c00
 546:	0004                	.2byte	0x4
 548:	0000                	.2byte	0x0
 54a:	0000                	.2byte	0x0
 54c:	0500                	.2byte	0x500
 54e:	000a                	.2byte	0xa
 550:	0000                	.2byte	0x0
 552:	0408                	.2byte	0x408
 554:	fb06                	.2byte	0xfb06
 556:	0001                	.2byte	0x1
 558:	6b00                	.2byte	0x6b00
 55a:	0005                	.2byte	0x5
 55c:	0400                	.2byte	0x400
 55e:	0600                	.2byte	0x600
 560:	028d                	.2byte	0x28d
 562:	0000                	.2byte	0x0
 564:	0430                	.2byte	0x430
 566:	0000                	.2byte	0x0
 568:	0404                	.2byte	0x404
 56a:	1100                	.2byte	0x1100
 56c:	0574                	.2byte	0x574
 56e:	0000                	.2byte	0x0
 570:	0000                	.2byte	0x0
 572:	0000                	.2byte	0x0
 574:	190a                	.2byte	0x190a
 576:	00000003          	lb	zero,0(zero) # 0 <isr0>
 57a:	1001                	.2byte	0x1001
 57c:	01fa                	.2byte	0x1fa
 57e:	0000                	.2byte	0x0
 580:	0099                	.2byte	0x99
 582:	0000                	.2byte	0x0
 584:	0000                	.2byte	0x0
 586:	0000                	.2byte	0x0
 588:	ac0d                	.2byte	0xac0d
 58a:	0001                	.2byte	0x1
 58c:	0200                	.2byte	0x200
 58e:	0001                	.2byte	0x1

Disassembly of section .debug_aranges:

00000000 <.debug_aranges>:
   0:	001c                	.2byte	0x1c
   2:	0000                	.2byte	0x0
   4:	0002                	.2byte	0x2
   6:	0000                	.2byte	0x0
   8:	0000                	.2byte	0x0
   a:	0004                	.2byte	0x4
   c:	ffff                	.2byte	0xffff
   e:	ffff                	.2byte	0xffff
  10:	0000                	.2byte	0x0
  12:	0000                	.2byte	0x0
  14:	0004                	.2byte	0x4
	...

Disassembly of section .debug_str:

00000000 <.debug_str>:
   0:	6c41                	.2byte	0x6c41
   2:	6769                	.2byte	0x6769
   4:	6d6e                	.2byte	0x6d6e
   6:	6e65                	.2byte	0x6e65
   8:	0074                	.2byte	0x74
   a:	6d26                	.2byte	0x6d26
   c:	7475                	.2byte	0x7475
   e:	6420                	.2byte	0x6420
  10:	6e79                	.2byte	0x6e79
  12:	6320                	.2byte	0x6320
  14:	3a65726f          	jal	tp,573ba <stop+0x57392>
  18:	663a                	.2byte	0x663a
  1a:	746d                	.2byte	0x746d
  1c:	3a3a                	.2byte	0x3a3a
  1e:	74697257          	.4byte	0x74697257
  22:	0065                	.2byte	0x65
  24:	3875                	.2byte	0x3875
  26:	5400                	.2byte	0x5400
  28:	6600                	.2byte	0x6600
  2a:	286e                	.2byte	0x286e
  2c:	6326                	.2byte	0x6326
  2e:	3a65726f          	jal	tp,573d4 <stop+0x573ac>
  32:	663a                	.2byte	0x663a
  34:	746d                	.2byte	0x746d
  36:	3a3a                	.2byte	0x3a3a
  38:	7472                	.2byte	0x7472
  3a:	3a3a                	.2byte	0x3a3a
  3c:	7478657b          	.4byte	0x7478657b
  40:	7265                	.2byte	0x7265
  42:	236e                	.2byte	0x236e
  44:	7d30                	.2byte	0x7d30
  46:	3a3a                	.2byte	0x3a3a
  48:	7161704f          	.4byte	0x7161704f
  4c:	6575                	.2byte	0x6575
  4e:	202c                	.2byte	0x202c
  50:	6d26                	.2byte	0x6d26
  52:	7475                	.2byte	0x7475
  54:	6320                	.2byte	0x6320
  56:	3a65726f          	jal	tp,573fc <stop+0x573d4>
  5a:	663a                	.2byte	0x663a
  5c:	746d                	.2byte	0x746d
  5e:	3a3a                	.2byte	0x3a3a
  60:	6f46                	.2byte	0x6f46
  62:	6d72                	.2byte	0x6d72
  64:	7461                	.2byte	0x7461
  66:	6574                	.2byte	0x6574
  68:	2972                	.2byte	0x2972
  6a:	2d20                	.2byte	0x2d20
  6c:	203e                	.2byte	0x203e
  6e:	65726f63          	bltu	tp,s7,6cc <stop+0x6a4>
  72:	3a3a                	.2byte	0x3a3a
  74:	6572                	.2byte	0x6572
  76:	746c7573          	.4byte	0x746c7573
  7a:	3a3a                	.2byte	0x3a3a
  7c:	6552                	.2byte	0x6552
  7e:	746c7573          	.4byte	0x746c7573
  82:	283c                	.2byte	0x283c
  84:	2c29                	.2byte	0x2c29
  86:	6320                	.2byte	0x6320
  88:	3a65726f          	jal	tp,5742e <stop+0x57406>
  8c:	663a                	.2byte	0x663a
  8e:	746d                	.2byte	0x746d
  90:	3a3a                	.2byte	0x3a3a
  92:	7245                	.2byte	0x7245
  94:	6f72                	.2byte	0x6f72
  96:	3e72                	.2byte	0x3e72
  98:	2600                	.2byte	0x2600
  9a:	65726f63          	bltu	tp,s7,6f8 <stop+0x6d0>
  9e:	3a3a                	.2byte	0x3a3a
  a0:	6170                	.2byte	0x6170
  a2:	696e                	.2byte	0x696e
  a4:	6c3a3a63          	.4byte	0x6c3a3a63
  a8:	7461636f          	jal	t1,167ee <stop+0x167c6>
  ac:	6f69                	.2byte	0x6f69
  ae:	3a6e                	.2byte	0x3a6e
  b0:	4c3a                	.2byte	0x4c3a
  b2:	7461636f          	jal	t1,167f8 <stop+0x167d0>
  b6:	6f69                	.2byte	0x6f69
  b8:	006e                	.2byte	0x6e
  ba:	6326                	.2byte	0x6326
  bc:	3a65726f          	jal	tp,57462 <stop+0x5743a>
  c0:	703a                	.2byte	0x703a
  c2:	6e61                	.2byte	0x6e61
  c4:	6369                	.2byte	0x6369
  c6:	3a3a                	.2byte	0x3a3a
  c8:	6170                	.2byte	0x6170
  ca:	696e                	.2byte	0x696e
  cc:	6e695f63          	bge	s2,t1,7ca <stop+0x7a2>
  d0:	6f66                	.2byte	0x6f66
  d2:	3a3a                	.2byte	0x3a3a
  d4:	6150                	.2byte	0x6150
  d6:	696e                	.2byte	0x696e
  d8:	666e4963          	blt	t3,t1,74a <stop+0x722>
  dc:	6543006f          	j	30730 <stop+0x30708>
  e0:	746e                	.2byte	0x746e
  e2:	7265                	.2byte	0x7265
  e4:	4f00                	.2byte	0x4f00
  e6:	7470                	.2byte	0x7470
  e8:	6f69                	.2byte	0x6f69
  ea:	3c6e                	.2byte	0x3c6e
  ec:	5b26                	.2byte	0x5b26
  ee:	65726f63          	bltu	tp,s7,74c <stop+0x724>
  f2:	3a3a                	.2byte	0x3a3a
  f4:	6d66                	.2byte	0x6d66
  f6:	3a74                	.2byte	0x3a74
  f8:	723a                	.2byte	0x723a
  fa:	3a74                	.2byte	0x3a74
  fc:	503a                	.2byte	0x503a
  fe:	616c                	.2byte	0x616c
 100:	6f686563          	bltu	a6,s6,7ea <stop+0x7c2>
 104:	646c                	.2byte	0x646c
 106:	7265                	.2byte	0x7265
 108:	3e5d                	.2byte	0x3e5d
 10a:	6100                	.2byte	0x6100
 10c:	6772                	.2byte	0x6772
 10e:	6f660073          	.4byte	0x6f660073
 112:	6d72                	.2byte	0x6d72
 114:	7461                	.2byte	0x7461
 116:	6574                	.2byte	0x6574
 118:	0072                	.2byte	0x72
 11a:	6572                	.2byte	0x6572
 11c:	746c7573          	.4byte	0x746c7573
 120:	4c00                	.2byte	0x4c00
 122:	7461636f          	jal	t1,16868 <stop+0x16840>
 126:	6f69                	.2byte	0x6f69
 128:	006e                	.2byte	0x6e
 12a:	65726f63          	bltu	tp,s7,788 <stop+0x760>
 12e:	6300                	.2byte	0x6300
 130:	616c                	.2byte	0x616c
 132:	676e                	.2byte	0x676e
 134:	4c20                	.2byte	0x4c20
 136:	564c                	.2byte	0x564c
 138:	204d                	.2byte	0x204d
 13a:	7228                	.2byte	0x7228
 13c:	7375                	.2byte	0x7375
 13e:	6374                	.2byte	0x6374
 140:	7620                	.2byte	0x7620
 142:	7265                	.2byte	0x7265
 144:	6e6f6973          	.4byte	0x6e6f6973
 148:	3120                	.2byte	0x3120
 14a:	372e                	.2byte	0x372e
 14c:	2d302e33          	.4byte	0x2d302e33
 150:	696e                	.2byte	0x696e
 152:	6c746867          	.4byte	0x6c746867
 156:	2079                	.2byte	0x2079
 158:	3128                	.2byte	0x3128
 15a:	3038                	.2byte	0x3038
 15c:	6664                	.2byte	0x6664
 15e:	6266                	.2byte	0x6266
 160:	3161                	.2byte	0x3161
 162:	3220                	.2byte	0x3220
 164:	3230                	.2byte	0x3230
 166:	38302d33          	.4byte	0x38302d33
 16a:	312d                	.2byte	0x312d
 16c:	2934                	.2byte	0x2934
 16e:	0029                	.2byte	0x29
 170:	695f 666e 006f      	.byte	0x5f, 0x69, 0x6e, 0x66, 0x6f, 0x00
 176:	2826                	.2byte	0x2826
 178:	7964                	.2byte	0x7964
 17a:	206e                	.2byte	0x206e
 17c:	65726f63          	bltu	tp,s7,7da <stop+0x7b2>
 180:	3a3a                	.2byte	0x3a3a
 182:	6e61                	.2byte	0x6e61
 184:	3a79                	.2byte	0x3a79
 186:	413a                	.2byte	0x413a
 188:	796e                	.2byte	0x796e
 18a:	2b20                	.2byte	0x2b20
 18c:	6320                	.2byte	0x6320
 18e:	3a65726f          	jal	tp,57534 <stop+0x5750c>
 192:	6d3a                	.2byte	0x6d3a
 194:	7261                	.2byte	0x7261
 196:	3a72656b          	.4byte	0x3a72656b
 19a:	533a                	.2byte	0x533a
 19c:	6e65                	.2byte	0x6e65
 19e:	2964                	.2byte	0x2964
 1a0:	4f00                	.2byte	0x4f00
 1a2:	6170                	.2byte	0x6170
 1a4:	7571                	.2byte	0x7571
 1a6:	0065                	.2byte	0x65
 1a8:	7562                	.2byte	0x7562
 1aa:	0066                	.2byte	0x66
 1ac:	6f62                	.2byte	0x6f62
 1ae:	64006c6f          	jal	s8,67ee <stop+0x67c6>
 1b2:	7461                	.2byte	0x7461
 1b4:	5f61                	.2byte	0x5f61
 1b6:	7470                	.2byte	0x7470
 1b8:	0072                	.2byte	0x72
 1ba:	7326                	.2byte	0x7326
 1bc:	7274                	.2byte	0x7274
 1be:	4600                	.2byte	0x4600
 1c0:	616d726f          	jal	tp,d77d6 <stop+0xd77ae>
 1c4:	7474                	.2byte	0x7474
 1c6:	7265                	.2byte	0x7265
 1c8:	2f00                	.2byte	0x2f00
 1ca:	6f68                	.2byte	0x6f68
 1cc:	656d                	.2byte	0x656d
 1ce:	7761702f          	.4byte	0x7761702f
 1d2:	6c65                	.2byte	0x6c65
 1d4:	6569682f          	.4byte	0x6569682f
 1d8:	6172                	.2byte	0x6172
 1da:	6372                	.2byte	0x6372
 1dc:	7968                	.2byte	0x7968
 1de:	6e79732f          	.4byte	0x6e79732f
 1e2:	6d697263          	bgeu	s2,s6,8a6 <stop+0x87e>
 1e6:	7369722f          	.4byte	0x7369722f
 1ea:	722f7663          	bgeu	t5,sp,916 <stop+0x8ee>
 1ee:	7369                	.2byte	0x7369
 1f0:	615f7663          	bgeu	t5,s5,7fc <stop+0x7d4>
 1f4:	75006d73          	.4byte	0x75006d73
 1f8:	70003233          	.4byte	0x70003233
 1fc:	746e696f          	jal	s2,e6942 <stop+0xe691a>
 200:	7265                	.2byte	0x7265
 202:	4f00                	.2byte	0x4f00
 204:	7470                	.2byte	0x7470
 206:	6f69                	.2byte	0x6f69
 208:	3c6e                	.2byte	0x3c6e
 20a:	7375                	.2byte	0x7375
 20c:	7a69                	.2byte	0x7a69
 20e:	3e65                	.2byte	0x3e65
 210:	7300                	.2byte	0x7300
 212:	6372                	.2byte	0x6372
 214:	69616d2f          	.4byte	0x69616d2f
 218:	2e6e                	.2byte	0x2e6e
 21a:	7372                	.2byte	0x7372
 21c:	722f402f          	.4byte	0x722f402f
 220:	7369                	.2byte	0x7369
 222:	615f7663          	bgeu	t5,s5,82e <stop+0x806>
 226:	662e6d73          	.4byte	0x662e6d73
 22a:	64386637          	lui	a2,0x64386
 22e:	3339                	.2byte	0x3339
 230:	6638                	.2byte	0x6638
 232:	6532                	.2byte	0x6532
 234:	6665                	.2byte	0x6665
 236:	3161                	.2byte	0x3161
 238:	67632d63          	.4byte	0x67632d63
 23c:	2e75                	.2byte	0x2e75
 23e:	0030                	.2byte	0x30
 240:	7572                	.2byte	0x7572
 242:	625f7473          	.4byte	0x625f7473
 246:	6765                	.2byte	0x6765
 248:	6e69                	.2byte	0x6e69
 24a:	755f 776e 6e69      	.byte	0x5f, 0x75, 0x6e, 0x77, 0x69, 0x6e
 250:	0064                	.2byte	0x64
 252:	6170                	.2byte	0x6170
 254:	6c79                	.2byte	0x6c79
 256:	0064616f          	jal	sp,4625c <stop+0x46234>
 25a:	2928                	.2byte	0x2928
 25c:	6c00                	.2byte	0x6c00
 25e:	7461636f          	jal	t1,169a4 <stop+0x1697c>
 262:	6f69                	.2byte	0x6f69
 264:	006e                	.2byte	0x6e
 266:	6170                	.2byte	0x6170
 268:	696e                	.2byte	0x696e
 26a:	6e695f63          	bge	s2,t1,968 <stop+0x940>
 26e:	6f66                	.2byte	0x6f66
 270:	5300                	.2byte	0x5300
 272:	00656d6f          	jal	s10,56278 <stop+0x56250>
 276:	6d49                	.2byte	0x6d49
 278:	6c70                	.2byte	0x6c70
 27a:	6569                	.2byte	0x6569
 27c:	0064                	.2byte	0x64
 27e:	6c50                	.2byte	0x6c50
 280:	6361                	.2byte	0x6361
 282:	6865                	.2byte	0x6865
 284:	65646c6f          	jal	s8,468da <stop+0x468b2>
 288:	0072                	.2byte	0x72
 28a:	7472                	.2byte	0x7472
 28c:	7600                	.2byte	0x7600
 28e:	6174                	.2byte	0x6174
 290:	6c62                	.2byte	0x6c62
 292:	0065                	.2byte	0x65
 294:	7375                	.2byte	0x7375
 296:	7a69                	.2byte	0x7a69
 298:	0065                	.2byte	0x65
 29a:	5b26                	.2byte	0x5b26
 29c:	65726f63          	bltu	tp,s7,8fa <stop+0x8d2>
 2a0:	3a3a                	.2byte	0x3a3a
 2a2:	6d66                	.2byte	0x6d66
 2a4:	3a74                	.2byte	0x3a74
 2a6:	723a                	.2byte	0x723a
 2a8:	3a74                	.2byte	0x3a74
 2aa:	503a                	.2byte	0x503a
 2ac:	616c                	.2byte	0x616c
 2ae:	6f686563          	bltu	a6,s6,998 <stop+0x970>
 2b2:	646c                	.2byte	0x646c
 2b4:	7265                	.2byte	0x7265
 2b6:	005d                	.2byte	0x5d
 2b8:	0045                	.2byte	0x45
 2ba:	72616863          	bltu	sp,t1,9ea <stop+0x9c2>
 2be:	5f00                	.2byte	0x5f00
 2c0:	305f 2600 6f63      	.byte	0x5f, 0x30, 0x00, 0x26, 0x63, 0x6f
 2c6:	6572                	.2byte	0x6572
 2c8:	3a3a                	.2byte	0x3a3a
 2ca:	6d66                	.2byte	0x6d66
 2cc:	3a74                	.2byte	0x3a74
 2ce:	723a                	.2byte	0x723a
 2d0:	3a74                	.2byte	0x3a74
 2d2:	7b3a                	.2byte	0x7b3a
 2d4:	7865                	.2byte	0x7865
 2d6:	6574                	.2byte	0x6574
 2d8:	6e72                	.2byte	0x6e72
 2da:	3a7d3023          	.4byte	0x3a7d3023
 2de:	4f3a                	.2byte	0x4f3a
 2e0:	6170                	.2byte	0x6170
 2e2:	7571                	.2byte	0x7571
 2e4:	0065                	.2byte	0x65
 2e6:	6974704f          	.4byte	0x6974704f
 2ea:	263c6e6f          	jal	t3,c6d4c <stop+0xc6d24>
 2ee:	65726f63          	bltu	tp,s7,94c <stop+0x924>
 2f2:	3a3a                	.2byte	0x3a3a
 2f4:	6d66                	.2byte	0x6d66
 2f6:	3a74                	.2byte	0x3a74
 2f8:	413a                	.2byte	0x413a
 2fa:	6772                	.2byte	0x6772
 2fc:	6d75                	.2byte	0x6d75
 2fe:	6e65                	.2byte	0x6e65
 300:	7374                	.2byte	0x7374
 302:	003e                	.2byte	0x3e
 304:	656d                	.2byte	0x656d
 306:	67617373          	.4byte	0x67617373
 30a:	0065                	.2byte	0x65
 30c:	6974706f          	j	481a2 <stop+0x4817a>
 310:	43006e6f          	jal	t3,6740 <stop+0x6718>
 314:	746e756f          	jal	a0,e7a5a <stop+0xe7a32>
 318:	6400                	.2byte	0x6400
 31a:	6e79                	.2byte	0x6e79
 31c:	6320                	.2byte	0x6320
 31e:	3a65726f          	jal	tp,576c4 <stop+0x5769c>
 322:	663a                	.2byte	0x663a
 324:	746d                	.2byte	0x746d
 326:	3a3a                	.2byte	0x3a3a
 328:	74697257          	.4byte	0x74697257
 32c:	0065                	.2byte	0x65
 32e:	6170                	.2byte	0x6170
 330:	696e                	.2byte	0x696e
 332:	61760063          	beq	a2,s7,932 <stop+0x90a>
 336:	756c                	.2byte	0x756c
 338:	0065                	.2byte	0x65
 33a:	656c                	.2byte	0x656c
 33c:	676e                	.2byte	0x676e
 33e:	6874                	.2byte	0x6874
 340:	4900                	.2byte	0x4900
 342:	69770073          	.4byte	0x69770073
 346:	7464                	.2byte	0x7464
 348:	0068                	.2byte	0x68
 34a:	6326                	.2byte	0x6326
 34c:	3a65726f          	jal	tp,576f2 <stop+0x576ca>
 350:	663a                	.2byte	0x663a
 352:	746d                	.2byte	0x746d
 354:	3a3a                	.2byte	0x3a3a
 356:	7241                	.2byte	0x7241
 358:	656d7567          	.4byte	0x656d7567
 35c:	746e                	.2byte	0x746e
 35e:	5b260073          	.4byte	0x5b260073
 362:	7375                	.2byte	0x7375
 364:	7a69                	.2byte	0x7a69
 366:	3b65                	.2byte	0x3b65
 368:	3320                	.2byte	0x3320
 36a:	005d                	.2byte	0x5d
 36c:	6f4e                	.2byte	0x6f4e
 36e:	656e                	.2byte	0x656e
 370:	4500                	.2byte	0x4500
 372:	7272                	.2byte	0x7272
 374:	6c00                	.2byte	0x6c00
 376:	6e69                	.2byte	0x6e69
 378:	0065                	.2byte	0x65
 37a:	6428                	.2byte	0x6428
 37c:	6e79                	.2byte	0x6e79
 37e:	6320                	.2byte	0x6320
 380:	3a65726f          	jal	tp,57726 <stop+0x576fe>
 384:	613a                	.2byte	0x613a
 386:	796e                	.2byte	0x796e
 388:	3a3a                	.2byte	0x3a3a
 38a:	6e41                	.2byte	0x6e41
 38c:	2079                	.2byte	0x2079
 38e:	6f63202b          	.4byte	0x6f63202b
 392:	6572                	.2byte	0x6572
 394:	3a3a                	.2byte	0x3a3a
 396:	616d                	.2byte	0x616d
 398:	6b72                	.2byte	0x6b72
 39a:	7265                	.2byte	0x7265
 39c:	3a3a                	.2byte	0x3a3a
 39e:	646e6553          	.4byte	0x646e6553
 3a2:	0029                	.2byte	0x29
 3a4:	6f70                	.2byte	0x6f70
 3a6:	69746973          	.4byte	0x69746973
 3aa:	45006e6f          	jal	t3,67fa <stop+0x67d2>
 3ae:	7272                	.2byte	0x7272
 3b0:	2600726f          	jal	tp,7610 <stop+0x75e8>
 3b4:	726f635b          	.4byte	0x726f635b
 3b8:	3a65                	.2byte	0x3a65
 3ba:	663a                	.2byte	0x663a
 3bc:	746d                	.2byte	0x746d
 3be:	3a3a                	.2byte	0x3a3a
 3c0:	7472                	.2byte	0x7472
 3c2:	3a3a                	.2byte	0x3a3a
 3c4:	7241                	.2byte	0x7241
 3c6:	656d7567          	.4byte	0x656d7567
 3ca:	746e                	.2byte	0x746e
 3cc:	005d                	.2byte	0x5d
 3ce:	5f5f 5241 4152      	.byte	0x5f, 0x5f, 0x41, 0x52, 0x52, 0x41
 3d4:	5f59                	.2byte	0x5f59
 3d6:	455a4953          	.4byte	0x455a4953
 3da:	545f 5059 5f45      	.byte	0x5f, 0x54, 0x59, 0x50, 0x45, 0x5f
 3e0:	005f 6552 7573      	.byte	0x5f, 0x00, 0x52, 0x65, 0x73, 0x75
 3e6:	746c                	.2byte	0x746c
 3e8:	283c                	.2byte	0x283c
 3ea:	2c29                	.2byte	0x2c29
 3ec:	6320                	.2byte	0x6320
 3ee:	3a65726f          	jal	tp,57794 <stop+0x5776c>
 3f2:	663a                	.2byte	0x663a
 3f4:	746d                	.2byte	0x746d
 3f6:	3a3a                	.2byte	0x3a3a
 3f8:	7245                	.2byte	0x7245
 3fa:	6f72                	.2byte	0x6f72
 3fc:	3e72                	.2byte	0x3e72
 3fe:	6300                	.2byte	0x6300
 400:	66006c6f          	jal	s8,6a60 <stop+0x6a38>
 404:	616c                	.2byte	0x616c
 406:	72007367          	.4byte	0x72007367
 40a:	7369                	.2byte	0x7369
 40c:	615f7663          	bgeu	t5,s5,a18 <stop+0x9f0>
 410:	50006d73          	.4byte	0x50006d73
 414:	6e61                	.2byte	0x6e61
 416:	6369                	.2byte	0x6369
 418:	6e49                	.2byte	0x6e49
 41a:	6f66                	.2byte	0x6f66
 41c:	4c00                	.2byte	0x4c00
 41e:	6665                	.2byte	0x6665
 420:	0074                	.2byte	0x74
 422:	41006b4f          	.4byte	0x41006b4f
 426:	6772                	.2byte	0x6772
 428:	6d75                	.2byte	0x6d75
 42a:	6e65                	.2byte	0x6e65
 42c:	0074                	.2byte	0x74
 42e:	5f6e6163          	bltu	t3,s6,a10 <stop+0x9e8>
 432:	6e75                	.2byte	0x6e75
 434:	646e6977          	.4byte	0x646e6977
 438:	6600                	.2byte	0x6600
 43a:	746d                	.2byte	0x746d
 43c:	5200                	.2byte	0x5200
 43e:	6769                	.2byte	0x6769
 440:	7468                	.2byte	0x7468
 442:	7000                	.2byte	0x7000
 444:	6569                	.2byte	0x6569
 446:	00736563          	bltu	t1,t2,450 <stop+0x428>
 44a:	7241                	.2byte	0x7241
 44c:	656d7567          	.4byte	0x656d7567
 450:	746e                	.2byte	0x746e
 452:	6c610073          	.4byte	0x6c610073
 456:	6769                	.2byte	0x6769
 458:	006e                	.2byte	0x6e
 45a:	6966                	.2byte	0x6966
 45c:	656c                	.2byte	0x656c
 45e:	5500                	.2byte	0x5500
 460:	6b6e                	.2byte	0x6b6e
 462:	6f6e                	.2byte	0x6f6e
 464:	66006e77          	.4byte	0x66006e77
 468:	6c69                	.2byte	0x6c69
 46a:	006c                	.2byte	0x6c
 46c:	6d26                	.2byte	0x6d26
 46e:	7475                	.2byte	0x7475
 470:	6320                	.2byte	0x6320
 472:	3a65726f          	jal	tp,57818 <stop+0x577f0>
 476:	663a                	.2byte	0x663a
 478:	746d                	.2byte	0x746d
 47a:	3a3a                	.2byte	0x3a3a
 47c:	6f46                	.2byte	0x6f46
 47e:	6d72                	.2byte	0x6d72
 480:	7461                	.2byte	0x7461
 482:	6574                	.2byte	0x6574
 484:	0072                	.2byte	0x72
 486:	5b26                	.2byte	0x5b26
 488:	7326                	.2byte	0x7326
 48a:	7274                	.2byte	0x7274
 48c:	005d                	.2byte	0x5d
 48e:	7270                	.2byte	0x7270
 490:	6365                	.2byte	0x6365
 492:	7369                	.2byte	0x7369
 494:	6f69                	.2byte	0x6f69
 496:	006e                	.2byte	0x6e
 498:	6150                	.2byte	0x6150
 49a:	6172                	.2byte	0x6172
 49c:	006d                	.2byte	0x6d
 49e:	7478657b          	.4byte	0x7478657b
 4a2:	7265                	.2byte	0x7265
 4a4:	236e                	.2byte	0x236e
 4a6:	7d30                	.2byte	0x7d30
	...

Disassembly of section .debug_pubnames:

00000000 <.debug_pubnames>:
   0:	00a9                	.2byte	0xa9
   2:	0000                	.2byte	0x0
   4:	0002                	.2byte	0x2
   6:	0000                	.2byte	0x0
   8:	0000                	.2byte	0x0
   a:	0590                	.2byte	0x590
   c:	0000                	.2byte	0x0
   e:	0026                	.2byte	0x26
  10:	0000                	.2byte	0x0
  12:	65726f63          	bltu	tp,s7,670 <stop+0x648>
  16:	2b00                	.2byte	0x2b00
  18:	0000                	.2byte	0x0
  1a:	6600                	.2byte	0x6600
  1c:	746d                	.2byte	0x746d
  1e:	3000                	.2byte	0x3000
  20:	0000                	.2byte	0x0
  22:	7200                	.2byte	0x7200
  24:	0074                	.2byte	0x74
  26:	0040                	.2byte	0x40
  28:	0000                	.2byte	0x0
  2a:	654c                	.2byte	0x654c
  2c:	7466                	.2byte	0x7466
  2e:	4600                	.2byte	0x4600
  30:	0000                	.2byte	0x0
  32:	5200                	.2byte	0x5200
  34:	6769                	.2byte	0x6769
  36:	7468                	.2byte	0x7468
  38:	4c00                	.2byte	0x4c00
  3a:	0000                	.2byte	0x0
  3c:	4300                	.2byte	0x4300
  3e:	6e65                	.2byte	0x6e65
  40:	6574                	.2byte	0x6574
  42:	0072                	.2byte	0x72
  44:	0052                	.2byte	0x52
  46:	0000                	.2byte	0x0
  48:	6e55                	.2byte	0x6e55
  4a:	776f6e6b          	.4byte	0x776f6e6b
  4e:	006e                	.2byte	0x6e
  50:	012d                	.2byte	0x12d
  52:	0000                	.2byte	0x0
  54:	7478657b          	.4byte	0x7478657b
  58:	7265                	.2byte	0x7265
  5a:	236e                	.2byte	0x236e
  5c:	7d30                	.2byte	0x7d30
  5e:	b600                	.2byte	0xb600
  60:	0001                	.2byte	0x1
  62:	7000                	.2byte	0x7000
  64:	6e61                	.2byte	0x6e61
  66:	6369                	.2byte	0x6369
  68:	bb00                	.2byte	0xbb00
  6a:	0001                	.2byte	0x1
  6c:	7000                	.2byte	0x7000
  6e:	6e61                	.2byte	0x6e61
  70:	6369                	.2byte	0x6369
  72:	695f 666e 006f      	.byte	0x5f, 0x69, 0x6e, 0x66, 0x6f, 0x00
  78:	01f5                	.2byte	0x1f5
  7a:	0000                	.2byte	0x0
  7c:	6f6c                	.2byte	0x6f6c
  7e:	69746163          	bltu	s0,s7,700 <stop+0x6d8>
  82:	25006e6f          	jal	t3,62d2 <stop+0x62aa>
  86:	0002                	.2byte	0x2
  88:	6f00                	.2byte	0x6f00
  8a:	7470                	.2byte	0x7470
  8c:	6f69                	.2byte	0x6f69
  8e:	006e                	.2byte	0x6e
  90:	00000343          	.4byte	0x343
  94:	6572                	.2byte	0x6572
  96:	746c7573          	.4byte	0x746c7573
  9a:	cc00                	.2byte	0xcc00
  9c:	72000003          	lb	zero,1824(zero) # 720 <stop+0x6f8>
  a0:	7369                	.2byte	0x7369
  a2:	615f7663          	bgeu	t5,s5,6ae <stop+0x686>
  a6:	00006d73          	.4byte	0x6d73
  aa:	0000                	.2byte	0x0
	...

Disassembly of section .debug_pubtypes:

00000000 <.debug_pubtypes>:
   0:	035a                	.2byte	0x35a
   2:	0000                	.2byte	0x0
   4:	0002                	.2byte	0x2
   6:	0000                	.2byte	0x0
   8:	0000                	.2byte	0x0
   a:	0590                	.2byte	0x590
   c:	0000                	.2byte	0x0
   e:	0035                	.2byte	0x35
  10:	0000                	.2byte	0x0
  12:	6c41                	.2byte	0x6c41
  14:	6769                	.2byte	0x6769
  16:	6d6e                	.2byte	0x6d6e
  18:	6e65                	.2byte	0x6e65
  1a:	0074                	.2byte	0x74
  1c:	0059                	.2byte	0x59
  1e:	0000                	.2byte	0x0
  20:	6c50                	.2byte	0x6c50
  22:	6361                	.2byte	0x6361
  24:	6865                	.2byte	0x6865
  26:	65646c6f          	jal	s8,4667c <stop+0x46654>
  2a:	0072                	.2byte	0x72
  2c:	000000a3          	sb	zero,1(zero) # 1 <isr0+0x1>
  30:	6e756f43          	.4byte	0x6e756f43
  34:	0074                	.2byte	0x74
  36:	0000010f          	.4byte	0x10f
  3a:	7241                	.2byte	0x7241
  3c:	656d7567          	.4byte	0x656d7567
  40:	746e                	.2byte	0x746e
  42:	3200                	.2byte	0x3200
  44:	0001                	.2byte	0x1
  46:	4f00                	.2byte	0x4f00
  48:	6170                	.2byte	0x6170
  4a:	7571                	.2byte	0x7571
  4c:	0065                	.2byte	0x65
  4e:	0000013b          	.4byte	0x13b
  52:	7241                	.2byte	0x7241
  54:	656d7567          	.4byte	0x656d7567
  58:	746e                	.2byte	0x746e
  5a:	01640073          	.4byte	0x1640073
  5e:	0000                	.2byte	0x0
  60:	7245                	.2byte	0x7245
  62:	6f72                	.2byte	0x6f72
  64:	0072                	.2byte	0x72
  66:	0000016b          	.4byte	0x16b
  6a:	6f46                	.2byte	0x6f46
  6c:	6d72                	.2byte	0x6d72
  6e:	7461                	.2byte	0x7461
  70:	6574                	.2byte	0x6574
  72:	0072                	.2byte	0x72
  74:	01c0                	.2byte	0x1c0
  76:	0000                	.2byte	0x0
  78:	6150                	.2byte	0x6150
  7a:	696e                	.2byte	0x696e
  7c:	666e4963          	blt	t3,t1,6ee <stop+0x6c6>
  80:	01fa006f          	j	a089e <stop+0xa0876>
  84:	0000                	.2byte	0x0
  86:	6f4c                	.2byte	0x6f4c
  88:	69746163          	bltu	s0,s7,70a <stop+0x6e2>
  8c:	2a006e6f          	jal	t3,632c <stop+0x6304>
  90:	0002                	.2byte	0x2
  92:	4f00                	.2byte	0x4f00
  94:	7470                	.2byte	0x7470
  96:	6f69                	.2byte	0x6f69
  98:	3c6e                	.2byte	0x3c6e
  9a:	6326                	.2byte	0x6326
  9c:	3a65726f          	jal	tp,57442 <stop+0x5741a>
  a0:	663a                	.2byte	0x663a
  a2:	746d                	.2byte	0x746d
  a4:	3a3a                	.2byte	0x3a3a
  a6:	7241                	.2byte	0x7241
  a8:	656d7567          	.4byte	0x656d7567
  ac:	746e                	.2byte	0x746e
  ae:	87003e73          	.4byte	0x87003e73
  b2:	0002                	.2byte	0x2
  b4:	4f00                	.2byte	0x4f00
  b6:	7470                	.2byte	0x7470
  b8:	6f69                	.2byte	0x6f69
  ba:	3c6e                	.2byte	0x3c6e
  bc:	5b26                	.2byte	0x5b26
  be:	65726f63          	bltu	tp,s7,71c <stop+0x6f4>
  c2:	3a3a                	.2byte	0x3a3a
  c4:	6d66                	.2byte	0x6d66
  c6:	3a74                	.2byte	0x3a74
  c8:	723a                	.2byte	0x723a
  ca:	3a74                	.2byte	0x3a74
  cc:	503a                	.2byte	0x503a
  ce:	616c                	.2byte	0x616c
  d0:	6f686563          	bltu	a6,s6,7ba <stop+0x792>
  d4:	646c                	.2byte	0x646c
  d6:	7265                	.2byte	0x7265
  d8:	3e5d                	.2byte	0x3e5d
  da:	e400                	.2byte	0xe400
  dc:	0002                	.2byte	0x2
  de:	4f00                	.2byte	0x4f00
  e0:	7470                	.2byte	0x7470
  e2:	6f69                	.2byte	0x6f69
  e4:	3c6e                	.2byte	0x3c6e
  e6:	7375                	.2byte	0x7375
  e8:	7a69                	.2byte	0x7a69
  ea:	3e65                	.2byte	0x3e65
  ec:	4800                	.2byte	0x4800
  ee:	52000003          	lb	zero,1312(zero) # 520 <stop+0x4f8>
  f2:	7365                	.2byte	0x7365
  f4:	6c75                	.2byte	0x6c75
  f6:	3c74                	.2byte	0x3c74
  f8:	2928                	.2byte	0x2928
  fa:	202c                	.2byte	0x202c
  fc:	65726f63          	bltu	tp,s7,75a <stop+0x732>
 100:	3a3a                	.2byte	0x3a3a
 102:	6d66                	.2byte	0x6d66
 104:	3a74                	.2byte	0x3a74
 106:	453a                	.2byte	0x453a
 108:	7272                	.2byte	0x7272
 10a:	003e726f          	jal	tp,e790c <stop+0xe78e4>
 10e:	03c5                	.2byte	0x3c5
 110:	0000                	.2byte	0x0
 112:	3875                	.2byte	0x3875
 114:	f500                	.2byte	0xf500
 116:	26000003          	lb	zero,608(zero) # 260 <stop+0x238>
 11a:	65726f63          	bltu	tp,s7,778 <stop+0x750>
 11e:	3a3a                	.2byte	0x3a3a
 120:	6170                	.2byte	0x6170
 122:	696e                	.2byte	0x696e
 124:	703a3a63          	.4byte	0x703a3a63
 128:	6e61                	.2byte	0x6e61
 12a:	6369                	.2byte	0x6369
 12c:	695f 666e 3a6f      	.byte	0x5f, 0x69, 0x6e, 0x66, 0x6f, 0x3a
 132:	503a                	.2byte	0x503a
 134:	6e61                	.2byte	0x6e61
 136:	6369                	.2byte	0x6369
 138:	6e49                	.2byte	0x6e49
 13a:	6f66                	.2byte	0x6f66
 13c:	0200                	.2byte	0x200
 13e:	0004                	.2byte	0x4
 140:	2600                	.2byte	0x2600
 142:	6428                	.2byte	0x6428
 144:	6e79                	.2byte	0x6e79
 146:	6320                	.2byte	0x6320
 148:	3a65726f          	jal	tp,574ee <stop+0x574c6>
 14c:	613a                	.2byte	0x613a
 14e:	796e                	.2byte	0x796e
 150:	3a3a                	.2byte	0x3a3a
 152:	6e41                	.2byte	0x6e41
 154:	2079                	.2byte	0x2079
 156:	6f63202b          	.4byte	0x6f63202b
 15a:	6572                	.2byte	0x6572
 15c:	3a3a                	.2byte	0x3a3a
 15e:	616d                	.2byte	0x616d
 160:	6b72                	.2byte	0x6b72
 162:	7265                	.2byte	0x7265
 164:	3a3a                	.2byte	0x3a3a
 166:	646e6553          	.4byte	0x646e6553
 16a:	0029                	.2byte	0x29
 16c:	0429                	.2byte	0x429
 16e:	0000                	.2byte	0x0
 170:	6428                	.2byte	0x6428
 172:	6e79                	.2byte	0x6e79
 174:	6320                	.2byte	0x6320
 176:	3a65726f          	jal	tp,5751c <stop+0x574f4>
 17a:	613a                	.2byte	0x613a
 17c:	796e                	.2byte	0x796e
 17e:	3a3a                	.2byte	0x3a3a
 180:	6e41                	.2byte	0x6e41
 182:	2079                	.2byte	0x2079
 184:	6f63202b          	.4byte	0x6f63202b
 188:	6572                	.2byte	0x6572
 18a:	3a3a                	.2byte	0x3a3a
 18c:	616d                	.2byte	0x616d
 18e:	6b72                	.2byte	0x6b72
 190:	7265                	.2byte	0x7265
 192:	3a3a                	.2byte	0x3a3a
 194:	646e6553          	.4byte	0x646e6553
 198:	0029                	.2byte	0x29
 19a:	0430                	.2byte	0x430
 19c:	0000                	.2byte	0x0
 19e:	5b26                	.2byte	0x5b26
 1a0:	7375                	.2byte	0x7375
 1a2:	7a69                	.2byte	0x7a69
 1a4:	3b65                	.2byte	0x3b65
 1a6:	3320                	.2byte	0x3320
 1a8:	005d                	.2byte	0x5d
 1aa:	044a                	.2byte	0x44a
 1ac:	0000                	.2byte	0x0
 1ae:	7375                	.2byte	0x7375
 1b0:	7a69                	.2byte	0x7a69
 1b2:	0065                	.2byte	0x65
 1b4:	0458                	.2byte	0x458
 1b6:	0000                	.2byte	0x0
 1b8:	3375                	.2byte	0x3375
 1ba:	0032                	.2byte	0x32
 1bc:	045f 0000 6326      	.byte	0x5f, 0x04, 0x00, 0x00, 0x26, 0x63
 1c2:	3a65726f          	jal	tp,57568 <stop+0x57540>
 1c6:	663a                	.2byte	0x663a
 1c8:	746d                	.2byte	0x746d
 1ca:	3a3a                	.2byte	0x3a3a
 1cc:	7241                	.2byte	0x7241
 1ce:	656d7567          	.4byte	0x656d7567
 1d2:	746e                	.2byte	0x746e
 1d4:	046c0073          	.4byte	0x46c0073
 1d8:	0000                	.2byte	0x0
 1da:	5b26                	.2byte	0x5b26
 1dc:	7326                	.2byte	0x7326
 1de:	7274                	.2byte	0x7274
 1e0:	005d                	.2byte	0x5d
 1e2:	00000493          	li	s1,0
 1e6:	7326                	.2byte	0x7326
 1e8:	7274                	.2byte	0x7274
 1ea:	ba00                	.2byte	0xba00
 1ec:	0004                	.2byte	0x4
 1ee:	2600                	.2byte	0x2600
 1f0:	726f635b          	.4byte	0x726f635b
 1f4:	3a65                	.2byte	0x3a65
 1f6:	663a                	.2byte	0x663a
 1f8:	746d                	.2byte	0x746d
 1fa:	3a3a                	.2byte	0x3a3a
 1fc:	7472                	.2byte	0x7472
 1fe:	3a3a                	.2byte	0x3a3a
 200:	6c50                	.2byte	0x6c50
 202:	6361                	.2byte	0x6361
 204:	6865                	.2byte	0x6865
 206:	65646c6f          	jal	s8,4685c <stop+0x46834>
 20a:	5d72                	.2byte	0x5d72
 20c:	e100                	.2byte	0xe100
 20e:	0004                	.2byte	0x4
 210:	6300                	.2byte	0x6300
 212:	6168                	.2byte	0x6168
 214:	0072                	.2byte	0x72
 216:	04e8                	.2byte	0x4e8
 218:	0000                	.2byte	0x0
 21a:	5b26                	.2byte	0x5b26
 21c:	65726f63          	bltu	tp,s7,87a <stop+0x852>
 220:	3a3a                	.2byte	0x3a3a
 222:	6d66                	.2byte	0x6d66
 224:	3a74                	.2byte	0x3a74
 226:	723a                	.2byte	0x723a
 228:	3a74                	.2byte	0x3a74
 22a:	413a                	.2byte	0x413a
 22c:	6772                	.2byte	0x6772
 22e:	6d75                	.2byte	0x6d75
 230:	6e65                	.2byte	0x6e65
 232:	5d74                	.2byte	0x5d74
 234:	0f00                	.2byte	0xf00
 236:	0005                	.2byte	0x5
 238:	2600                	.2byte	0x2600
 23a:	65726f63          	bltu	tp,s7,898 <stop+0x870>
 23e:	3a3a                	.2byte	0x3a3a
 240:	6d66                	.2byte	0x6d66
 242:	3a74                	.2byte	0x3a74
 244:	723a                	.2byte	0x723a
 246:	3a74                	.2byte	0x3a74
 248:	7b3a                	.2byte	0x7b3a
 24a:	7865                	.2byte	0x7865
 24c:	6574                	.2byte	0x6574
 24e:	6e72                	.2byte	0x6e72
 250:	3a7d3023          	.4byte	0x3a7d3023
 254:	4f3a                	.2byte	0x4f3a
 256:	6170                	.2byte	0x6170
 258:	7571                	.2byte	0x7571
 25a:	0065                	.2byte	0x65
 25c:	051c                	.2byte	0x51c
 25e:	0000                	.2byte	0x0
 260:	6e66                	.2byte	0x6e66
 262:	2628                	.2byte	0x2628
 264:	65726f63          	bltu	tp,s7,8c2 <stop+0x89a>
 268:	3a3a                	.2byte	0x3a3a
 26a:	6d66                	.2byte	0x6d66
 26c:	3a74                	.2byte	0x3a74
 26e:	723a                	.2byte	0x723a
 270:	3a74                	.2byte	0x3a74
 272:	7b3a                	.2byte	0x7b3a
 274:	7865                	.2byte	0x7865
 276:	6574                	.2byte	0x6574
 278:	6e72                	.2byte	0x6e72
 27a:	3a7d3023          	.4byte	0x3a7d3023
 27e:	4f3a                	.2byte	0x4f3a
 280:	6170                	.2byte	0x6170
 282:	7571                	.2byte	0x7571
 284:	2c65                	.2byte	0x2c65
 286:	2620                	.2byte	0x2620
 288:	756d                	.2byte	0x756d
 28a:	2074                	.2byte	0x2074
 28c:	65726f63          	bltu	tp,s7,8ea <stop+0x8c2>
 290:	3a3a                	.2byte	0x3a3a
 292:	6d66                	.2byte	0x6d66
 294:	3a74                	.2byte	0x3a74
 296:	463a                	.2byte	0x463a
 298:	616d726f          	jal	tp,d78ae <stop+0xd7886>
 29c:	7474                	.2byte	0x7474
 29e:	7265                	.2byte	0x7265
 2a0:	2029                	.2byte	0x2029
 2a2:	3e2d                	.2byte	0x3e2d
 2a4:	6320                	.2byte	0x6320
 2a6:	3a65726f          	jal	tp,5764c <stop+0x57624>
 2aa:	723a                	.2byte	0x723a
 2ac:	7365                	.2byte	0x7365
 2ae:	6c75                	.2byte	0x6c75
 2b0:	3a74                	.2byte	0x3a74
 2b2:	523a                	.2byte	0x523a
 2b4:	7365                	.2byte	0x7365
 2b6:	6c75                	.2byte	0x6c75
 2b8:	3c74                	.2byte	0x3c74
 2ba:	2928                	.2byte	0x2928
 2bc:	202c                	.2byte	0x202c
 2be:	65726f63          	bltu	tp,s7,91c <stop+0x8f4>
 2c2:	3a3a                	.2byte	0x3a3a
 2c4:	6d66                	.2byte	0x6d66
 2c6:	3a74                	.2byte	0x3a74
 2c8:	453a                	.2byte	0x453a
 2ca:	7272                	.2byte	0x7272
 2cc:	003e726f          	jal	tp,e7ace <stop+0xe7aa6>
 2d0:	0539                	.2byte	0x539
 2d2:	0000                	.2byte	0x0
 2d4:	2928                	.2byte	0x2928
 2d6:	4000                	.2byte	0x4000
 2d8:	0005                	.2byte	0x5
 2da:	2600                	.2byte	0x2600
 2dc:	756d                	.2byte	0x756d
 2de:	2074                	.2byte	0x2074
 2e0:	65726f63          	bltu	tp,s7,93e <stop+0x916>
 2e4:	3a3a                	.2byte	0x3a3a
 2e6:	6d66                	.2byte	0x6d66
 2e8:	3a74                	.2byte	0x3a74
 2ea:	463a                	.2byte	0x463a
 2ec:	616d726f          	jal	tp,d7902 <stop+0xd78da>
 2f0:	7474                	.2byte	0x7474
 2f2:	7265                	.2byte	0x7265
 2f4:	4d00                	.2byte	0x4d00
 2f6:	0005                	.2byte	0x5
 2f8:	2600                	.2byte	0x2600
 2fa:	756d                	.2byte	0x756d
 2fc:	2074                	.2byte	0x2074
 2fe:	7964                	.2byte	0x7964
 300:	206e                	.2byte	0x206e
 302:	65726f63          	bltu	tp,s7,960 <stop+0x938>
 306:	3a3a                	.2byte	0x3a3a
 308:	6d66                	.2byte	0x6d66
 30a:	3a74                	.2byte	0x3a74
 30c:	573a                	.2byte	0x573a
 30e:	6972                	.2byte	0x6972
 310:	6574                	.2byte	0x6574
 312:	7400                	.2byte	0x7400
 314:	0005                	.2byte	0x5
 316:	6400                	.2byte	0x6400
 318:	6e79                	.2byte	0x6e79
 31a:	6320                	.2byte	0x6320
 31c:	3a65726f          	jal	tp,576c2 <stop+0x5769a>
 320:	663a                	.2byte	0x663a
 322:	746d                	.2byte	0x746d
 324:	3a3a                	.2byte	0x3a3a
 326:	74697257          	.4byte	0x74697257
 32a:	0065                	.2byte	0x65
 32c:	0000057b          	.4byte	0x57b
 330:	6326                	.2byte	0x6326
 332:	3a65726f          	jal	tp,576d8 <stop+0x576b0>
 336:	703a                	.2byte	0x703a
 338:	6e61                	.2byte	0x6e61
 33a:	6369                	.2byte	0x6369
 33c:	3a3a                	.2byte	0x3a3a
 33e:	6f6c                	.2byte	0x6f6c
 340:	69746163          	bltu	s0,s7,9c2 <stop+0x99a>
 344:	3a3a6e6f          	jal	t3,a6ee6 <stop+0xa6ebe>
 348:	6f4c                	.2byte	0x6f4c
 34a:	69746163          	bltu	s0,s7,9cc <stop+0x9a4>
 34e:	88006e6f          	jal	t3,fff063ce <test_word+0xaff063ce>
 352:	0005                	.2byte	0x5
 354:	6200                	.2byte	0x6200
 356:	006c6f6f          	jal	t5,c635c <stop+0xc6334>
 35a:	0000                	.2byte	0x0
	...

Disassembly of section .comment:

00000000 <.comment>:
   0:	4c00                	.2byte	0x4c00
   2:	6e69                	.2byte	0x6e69
   4:	3a72656b          	.4byte	0x3a72656b
   8:	4c20                	.2byte	0x4c20
   a:	444c                	.2byte	0x444c
   c:	3120                	.2byte	0x3120
   e:	2e302e37          	lui	t3,0x2e302
  12:	0030                	.2byte	0x30
  14:	7572                	.2byte	0x7572
  16:	20637473          	.4byte	0x20637473
  1a:	6576                	.2byte	0x6576
  1c:	7372                	.2byte	0x7372
  1e:	6f69                	.2byte	0x6f69
  20:	206e                	.2byte	0x206e
  22:	2e31                	.2byte	0x2e31
  24:	302e3337          	lui	t1,0x302e3
  28:	6e2d                	.2byte	0x6e2d
  2a:	6769                	.2byte	0x6769
  2c:	7468                	.2byte	0x7468
  2e:	796c                	.2byte	0x796c
  30:	2820                	.2byte	0x2820
  32:	3831                	.2byte	0x3831
  34:	6430                	.2byte	0x6430
  36:	6666                	.2byte	0x6666
  38:	6162                	.2byte	0x6162
  3a:	2031                	.2byte	0x2031
  3c:	3032                	.2byte	0x3032
  3e:	3332                	.2byte	0x3332
  40:	302d                	.2byte	0x302d
  42:	2d38                	.2byte	0x2d38
  44:	3431                	.2byte	0x3431
  46:	0029                	.2byte	0x29

Disassembly of section .riscv.attributes:

00000000 <.riscv.attributes>:
   0:	1b41                	.2byte	0x1b41
   2:	0000                	.2byte	0x0
   4:	7200                	.2byte	0x7200
   6:	7369                	.2byte	0x7369
   8:	01007663          	bgeu	zero,a6,14 <isr0+0x14>
   c:	0011                	.2byte	0x11
   e:	0000                	.2byte	0x0
  10:	1004                	.2byte	0x1004
  12:	7205                	.2byte	0x7205
  14:	3376                	.2byte	0x3376
  16:	6932                	.2byte	0x6932
  18:	7032                	.2byte	0x7032
  1a:	0031                	.2byte	0x31

Disassembly of section .debug_frame:

00000000 <.debug_frame>:
   0:	0010                	.2byte	0x10
   2:	0000                	.2byte	0x0
   4:	ffff                	.2byte	0xffff
   6:	ffff                	.2byte	0xffff
   8:	0004                	.2byte	0x4
   a:	0004                	.2byte	0x4
   c:	7c01                	.2byte	0x7c01
   e:	0c01                	.2byte	0xc01
  10:	0002                	.2byte	0x2
  12:	0000                	.2byte	0x0
  14:	000c                	.2byte	0xc
	...
  1e:	0000                	.2byte	0x0
  20:	0004                	.2byte	0x4
	...

Disassembly of section .debug_line:

00000000 <.Lline_table_start0>:
   0:	003a                	.2byte	0x3a
   2:	0000                	.2byte	0x0
   4:	0004                	.2byte	0x4
   6:	00000023          	sb	zero,0(zero) # 0 <.Lline_table_start0>
   a:	0101                	.2byte	0x101
   c:	fb01                	.2byte	0xfb01
   e:	0d0e                	.2byte	0xd0e
  10:	0100                	.2byte	0x100
  12:	0101                	.2byte	0x101
  14:	0001                	.2byte	0x1
  16:	0000                	.2byte	0x0
  18:	0001                	.2byte	0x1
  1a:	0100                	.2byte	0x100
  1c:	00637273          	.4byte	0x637273
  20:	6d00                	.2byte	0x6d00
  22:	6961                	.2byte	0x6961
  24:	2e6e                	.2byte	0x2e6e
  26:	7372                	.2byte	0x7372
  28:	0100                	.2byte	0x100
  2a:	0000                	.2byte	0x0
  2c:	0500                	.2byte	0x500
  2e:	0a05                	.2byte	0xa05
  30:	0500                	.2byte	0x500
  32:	0002                	.2byte	0x2
  34:	0000                	.2byte	0x0
  36:	1a00                	.2byte	0x1a00
  38:	0409                	.2byte	0x409
  3a:	0000                	.2byte	0x0
  3c:	0101                	.2byte	0x101

Disassembly of section .data:

50000000 <test_word>:
50000000:	0100                	.2byte	0x100
	...
