.option arch, rv32i
# ----------------------------------------------------------
#  Group 1's "underlag" for Lab 1
#  Pseudo-instructions may be used for Lab 1.
# ----------------------------------------------------------
 
 
 
# Group 1's Codeword Generator Subroutine (pseudocode)
#  (remember:  "seed" is a global variable, UNSIGNED INTEGER;
#              you may implement local variables in registers or on the stack;
#              result returned in v0; preserve all except t regs)
#
# FUNCTION codgen(): UNSIGNED INTEGER;
#  LOCAL SIGNED INTEGER   n;
#  LOCAL UNSIGNED INTEGER x, y;
#  BEGIN
#    n := [count the number of 0's in word "seed"];
#    x := [rotate "seed" left by 30 bits];
#    y := [shift "seed" right-ARITHMETIC by 6 bits];
#    seed := x XOR y XOR n;
#   RETURN( seed XOR 0x464b713e );
#  END;
# 
# hint:  if "seed" is initialized to 0x3e944b9f,
#        the first five calls will generate these values:
#        0x891432f9, 0x4aa1dccc, 0xc54270fa, 0x9885155f, 0xce83d1b8, ...
# your code is to be written farther down (see comment below).
 
 
# Group 1's Recursive Decoding Subroutine (pseudocode)
#  (for "decode", all four local variables must be implemented ON THE
#              STACK, and NOT in registers; implement the code literally,.
#              no optimizations.  We're trying to teach you something.
#   remember:  result returned in v0; preserve all except t regs)
#
# FUNCTION decode( wordarr, bytearr ): UNSIGNED INTEGER;
#    (wordarr, bytearr passed by reference)
#  LOCAL UNSIGNED INTEGER m, r, x, y;
#  BEGIN
#    x := ONE'S-COMPLEMENT of codgen();
#    IF ([contents of word at "wordarr"] = 0) THEN  
#      [byte pointed to by "bytearr"] := 0;
#      r := x;
#    ELSE
#      y := decode( wordarr+, bytearr+ );  # "k+" means "successor in k"
#      m := ( x - y ) - [contents of word at "wordarr"];
#      [byte pointed to by "bytearr"] := [the eight bits at "m"<20:13>];
#      r := TWO'S-COMPLEMENT OF codgen();
#      r := x + y + m + r + 5;
#    ENDIF;
#    RETURN( r );
#  END;
 
 
# ----------------------------------------------------------
# The following are the ONLY lines that may appear in the
# ".data" section of the code.  You may add NO other lines.
# NO additional global variables.
# ----------------------------------------------------------
 
 
	.data
test_word: .word 0xDEADBEEF
some_string: .string "Hi! :)"
.align 4
abc:	.word	0x9fdd9158	# string "abc", encoded
	.word	0x85715808
	.word	0xac73323a
	.word	    0
plain:	.space	132		# room for 132 characters
 
	.align 4
seed:	.word    0			# 32-bit UNSIGNED INTEGER.
 
coded:	.word	0x015e7a47	# the real encoded data
	.word	0x2ef84ebb
	.word	0x177a8db4
	.word	0x1b722ff9
	.word	0x5dc7cff0
	.word	0x5dc9dea6
	.word	0x1da0c15a
	.word	0xe4c236a2
	.word	0x3d16b0d0
	.word	0x1f397842
	.word	0xaae0d2ba
	.word	0x11246674
	.word	0x0845317f
	.word	0xd5512dad
	.word	0xb6184977
	.word	0xd293a53e
	.word	0x7d9c2716
	.word	0xd917eae6
	.word	0xd8852384
	.word	0x286e46f9
	.word	0xce566029
	.word	0xcefe7daf
	.word	0x62d726d4
	.word	0x0dbaeb2d
	.word	0x95f57c60
	.word	0xed515141
	.word	0x29b77d0f
	.word	0x9f7b8d0c
	.word	0x45a8395a
	.word	0xfead2b72
	.word	0x883d434c
	.word	0xed8ddf60
	.word	0xe51e65e4
	.word	0x19bf6bb1
	.word	0xfeb505ec
	.word	0x662aa23c
	.word	0xf6827cf8
	.word	0xd1dc7a5c
	.word	0x4fa5b066
	.word	0x7ddd25a4
	.word	0xa8ba8e8a
	.word	0x72846227
	.word	0xf8f636fb
	.word	0x2b389a9c
	.word	0xe4038bf6
	.word	0x6e169877
	.word	0xad028132
	.word	0x84dbfe8c
	.word	0x243762ff
	.word	0x59c8f80c
	.word	0xb6e0db4b
	.word	0xedb8cab7
	.word	0xcd4b39f6
	.word	0xaf263741
	.word	0x18d9965f
	.word	0x1ab1f037
	.word	0x5b458792
	.word	0xc94d960d
	.word	0xd45cedea
	.word	0x2160aca3
	.word	0x93c77766
	.word	0x2d66e105
	.word	0x9ff74d4f
	.word	0x6dc22f21
	.word	0x6b03d689
	.word	0x5fc48de0
	.word	0x1138f000
	.word	0xccb58e57
	.word	0xf9c8e200
	.word	0x7ab26e3c
	.word	0xc61dcb3e
	.word	0x6aefccb0
	.word	0x7a452f05
	.word	0xa5cf0731
	.word	0xa249383f
	.word	0x628fe534
	.word	0xcad81710
	.word	0x7f616276
	.word	0x3ce18308
	.word	0xed4857ff
	.word	0xd1e5b1d1
	.word	0xc2e84dc2
	.word	0xaa003742
	.word	0xaf637488
	.word	0x831afc48
	.word	0x287a69a0
	.word	0x6e04546e
	.word	0x13dffa07
	.word	0x3232fb10
	.word	0xd69e2e09
	.word	0x355d8dc7
	.word	0xef902301
	.word	0x9a89ac15
	.word	0x967dc900
	.word	0x08dc2b1c
	.word	0x6b5be690
	.word	0x894b0e02
	.word	0xe26af9af
	.word	0xa6fd3b23
	.word	0xfcf213e5
	.word	0x85217608
	.word	0x7fd3be8b
	.word	0xa2e757fb
	.word	0x3717a341
	.word	0x85ee426d
	.word	0x394bb856
	.word	0x12ac98c3
	.word	0xec7d4ab5
	.word	0x721b6989
	.word	0x30e36360
	.word	0xaa018403
	.word	0x9ee61196
	.word	0xa8697adc
	.word	0x51e9d65a
	.word	0x11023594
	.word	0xc4c4b36b
	.word	0xda80bf7a
	.word	0xbd5a645e
	.word	0x18cea918
	.word	0xa723dda8
	.word	0x0126c05e
	.word	0x2962d48a
	.word	0xd5f7d312
	.word	0xb8947041
	.word	0x7c1e2e9a
	.word	0x945eeac3
	.word	0x7110fb1c
	.word	0xa7bc72cc
	.word	0xdf47dfbb
	.word	0x09a1c6c8
	.word	0xc2e41061
	.word	    0
 
 
# ----------------------------------------------------------
# The following is the main program.  You may not change this.
# You may only add your subroutines AFTER the "infinite end loop" instruction here.
# You MUST have two subroutines named "codgen" and "decode".
# BOTH must adhere to our calling conventions; 
# both MUST preserve all registers except v-regs and t-regs;
# we are going to TEST for this when we run your code.
# ----------------------------------------------------------
 
 
	.text
main:
	addi t0, t1, 0
	la t0, test_word
	li t1, 0xFFFFFFFF
	sw t1, 0(t0)
	sw t1, 4(t0)
	sw t1, 8(t0)
	sw t1, 12(t0)
	li t0, 0x500000F4 # address should be outide of memview range
	sw t1, 0(t0)
	sw t1, 4(t0)
	sw t1, 8(t0)
	sw t1, 12(t0)
	li	s0, 0x0e0657c1	# initialize "seed"
	la	s1, seed	# initialize "seed"
	sw	s0, 0(s1)
	la	a0, coded	# address of start of coded words
	la	a1, plain	# address of start of decoded bytes
	jal ra, codgen
	jal ra, codgen
	jal ra, codgen
	jal ra, codgen
	jal ra, codgen
	jal ra, codgen
	jal ra, decode	# outer call to recursive "decode"
end:
	j       end             # infinite loop; plaintext now in "plain".
 
 
# ----------------------------------------------------------
# Group 1's assembly code for Function CodGen :
# ----------------------------------------------------------

	# your activation record diagram here.

# Group 1's Codeword Generator Subroutine (pseudocode)
#  (remember:  "seed" is a global variable, UNSIGNED INTEGER;
#              you may implement local variables in registers or on the stack;
#              result returned in v0; preserve all except t regs)
#
# FUNCTION codgen(): UNSIGNED INTEGER;
#  LOCAL SIGNED INTEGER   n;
#  LOCAL UNSIGNED INTEGER x, y;
#  BEGIN
#    n := [count the number of 0's in word "seed"];
#    x := [rotate "seed" left by 30 bits];
#    y := [shift "seed" right-ARITHMETIC by 6 bits];
#    seed := x XOR y XOR n;
#   RETURN( seed XOR 0x464b713e );
#  END;
# 
# hint:  if "seed" is initialized to 0x3e944b9f,
#        the first five calls will generate these values:
#        0x891432f9, 0x4aa1dccc, 0xc54270fa, 0x9885155f, 0xce83d1b8, ...
# your code is to be written farther down (see comment below).

#t1 = n
#t2 = x
#t3 = y
codgen:	
	la		s1, seed
	lw		s0, 0(s1)
	li		t0, 1
	li 		t1, 0
loop:	
	beq		t0, x0, continue
	and 	t2, t0, s0
	bne		t2, x0, skip
	addi 	t1, t1, 1
skip:	
	sll		t0, t0, 1
	jal 	x0,	loop
continue:
	slli	t2, s0, 30
	srli	t3, s0, 2
	or		t2, t2, t3 #rotate
	srai	t3, s0, 6  #shift
	addi	s0, t2, 0
	xor		s0, s0, t3
	xor		s0, s0, t1
	sw		s0, 0(s1) #update seed
	jalr	x0, ra






# ----------------------------------------------------------
# Group 1's assembly code for Function DeCode :
# ----------------------------------------------------------

	# your activation record diagram here.

decode:		# your code here.


# end of file.
