# ----------------------------------------------------------
#  Group 2's "underlag" for Lab 1
#  Pseudo-instructions may be used for Lab 1.
# ----------------------------------------------------------
 
 
 
# Group 2's Codeword Generator Subroutine (pseudocode)
#  (remember:  "seed" is a global variable, UNSIGNED INTEGER;
#              you may implement local variables in registers or on the stack;
#              result returned in v0; preserve all except t regs)
#
 
 
# Group 2's Recursive Decoding Subroutine (pseudocode)
#  (for "decode", all four local variables must be implemented ON THE
#              STACK, and NOT in registers; implement the code literally,.
#              no optimizations.  We're trying to teach you something.
#   remember:  result returned in v0; preserve all except t regs)
#
 
 
# ----------------------------------------------------------
# The following are the ONLY lines that may appear in the
# ".data" section of the code.  You may add NO other lines.
# NO additional global variables.
# ----------------------------------------------------------
 
 
	.data
plain:	.space	111		# room for 111 characters
 
	.align 4
seed:	.word    0			# 32-bit UNSIGNED INTEGER.
 
abc:	.word	0x62ab2a56	# string "abc", encoded
	.word	0xa1cf643c
	.word	0x0c0bd91f
	.word	    0
coded:	.word	0x5544a9f1	# the real encoded data
	.word	0x386f1fb5
	.word	0x6bd5c5b1
	.word	0xef886d1c
	.word	0x561172ce
	.word	0x13e22576
	.word	0x4b0390b5
	.word	0xfacda2ad
	.word	0xcee130b8
	.word	0x29ac9cb7
	.word	0xa8258c56
	.word	0xdce741c3
	.word	0x4cbeae38
	.word	0xf5bc91a9
	.word	0xac281d11
	.word	0x5a10218e
	.word	0x812caa68
	.word	0xcab07b57
	.word	0x65af2c34
	.word	0x4117a997
	.word	0xb3ff2ae1
	.word	0x521d6972
	.word	0x555fee5a
	.word	0x1e77fb36
	.word	0xdbf7cec3
	.word	0x8b658749
	.word	0xeb29b7f7
	.word	0xeb56972e
	.word	0x79f20c4e
	.word	0x5036c13d
	.word	0x15372870
	.word	0xdda2bcfc
	.word	0xc19b7aff
	.word	0x0a2a5a2f
	.word	0x43d11c6f
	.word	0x52ad0b67
	.word	0xe29c2a65
	.word	0xcf3580f4
	.word	0xa8688de2
	.word	0x3cf2ee9c
	.word	0x472b57ae
	.word	0x58003d45
	.word	0xd7ee1305
	.word	0xa46d7481
	.word	0xbc01cee9
	.word	0x63dcd64f
	.word	0x7cd54d8d
	.word	0x0f4bd5c0
	.word	0x42275377
	.word	0xe337ec2f
	.word	0xc1ec063f
	.word	0x98db8d51
	.word	0x7fe97be0
	.word	0xa7e62ab6
	.word	0x488db1e2
	.word	0xa571d3fa
	.word	0xa144d15a
	.word	0x42c8fe01
	.word	0xd4b380d2
	.word	0xd39b07e8
	.word	0xfb3b6e05
	.word	0x70ec0b75
	.word	0xcf3fb946
	.word	0xe1fd07ce
	.word	0xe66bc017
	.word	0x86b77f45
	.word	0x94020298
	.word	0x99fa4fed
	.word	0x935abbb0
	.word	0x9621b3b4
	.word	0x16cba096
	.word	0x8442f7fd
	.word	0x11d3db46
	.word	0x9400c70f
	.word	0x884437c4
	.word	0x613738ad
	.word	0x4a1acd6b
	.word	0xc92a7ac6
	.word	0x6e54c848
	.word	0x17f26190
	.word	0xfc43a849
	.word	0x7e9aa283
	.word	0x82d8564d
	.word	0x521d6327
	.word	0x669f610b
	.word	0x717d55bb
	.word	0x10c1bbf4
	.word	0xb11a0ea1
	.word	0xbf88f4f1
	.word	0x50b6e0f6
	.word	0x8249659b
	.word	0x2dd11935
	.word	0xec0160ff
	.word	0x4a6c930e
	.word	0x56bf5a1a
	.word	0xa8841626
	.word	0x3471228e
	.word	0x81658237
	.word	0xde2f964d
	.word	0xafaf998d
	.word	0x2a33e5e1
	.word	0x2264385f
	.word	0x3fd44413
	.word	0x94935835
	.word	0x009d8e04
	.word	0x1d30d325
	.word	0x88fa11c5
	.word	0xb7c00535
	.word	0x2070f791
	.word	0x574892ce
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
	.set noreorder, norvc #do not reorder, do not generate compressed instructions
main:	
  la  sp, _stack_start
  li	s0, 0x177023a6  # initialize "seed" test 0x51d40b5c real 0x177023a6
	la	s1, seed	# initialize "seed"
	sw	s0, 0(s1)
	la	a0, coded	# address of start of coded words
	la	a1, plain	# address of start of decoded bytes
	jal	   decode	# outer call to recursive "decode"
 // jal codgen
 // jal codgen
 // jal codgen
 // jal codgen
 // jal codgen
end:
	j       end             # infinite loop; plaintext now in "plain".
 
 
# ----------------------------------------------------------
# Group 2's assembly code for Function CodGen :
# ----------------------------------------------------------
# FUNCTION codgen(): UNSIGNED INTEGER;
#  LOCAL SIGNED INTEGER   n;
#  LOCAL UNSIGNED INTEGER x, y;
#  BEGIN
#    n := [count the number of 0's in word "seed"];
#    x := [multiply "seed" by the constant 2];
#    y := [divide "seed" (unsigned !) by the constant 64];
#    seed := x + y + n;  [ignore overflow condition]
#   RETURN( seed XOR 0x290995cf );
#  END;
# 
# hint:  if "seed" is initialized to 0x51d40b5c,
#        the first five calls will generate these values:
#        0x8de6f338, 0x657b1e5b, 0xb31f74a7, 0x1f9f8ba8, 0x470f0099, ...
# your code is to be written farther down (see comment below).
	# your activation record diagram here.

codgen:		# your code here.
  li t0, 0  #result
  li t1, 32 #iteration counter
  add a0, s0, zero
count:
  beqz t1, continue 
  andi t2, a0, 1
  srl a0, a0, 1
  addi t1, t1, -1
  bne t2, zero, count
increment:
  addi t0, t0, 1
  j count
continue:
  sll t1, s0, 1
  srl t2, s0, 6
  add s0, zero, t1
  add s0, s0, t2
  add s0, s0, t0
  li t0, 0x290995cf
  xor a0, s0, t0
  jr ra


# ----------------------------------------------------------
# Group 2's assembly code for Function DeCode :
# ----------------------------------------------------------

	# your activation record diagram here.
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
#      m := ( x XOR y ) + [contents of word at "wordarr"];
#      [byte pointed to by "bytearr"] := [the eight bits at "m"<9:2>];
#      r := TWO'S-COMPLEMENT OF codgen();
#      r := x + y + m + r + 5;
#    ENDIF;
#    RETURN( r );
#  END;
#ptr bytearr -24fp
#ptr wordarr -20fp
#       m     -16fp
#       r     -12fp
#       x     -8fp
#       y     -4fp
#       fp    0fp
#       ra    4fp
decode:		# your code here.
  addi sp, sp, -4
  sw ra, 0(sp)
  addi sp, sp, -4
  sw tp, 0(sp) #use thread poitner as frame pointer for convenience (fp = s0)
  add tp, sp, zero #new frame pointer
  addi sp, sp, -24 #space for 6 word vars
  sw a0, -20(tp)
  sw a1, -24(tp)

  jal codgen
  not a0, a0
  sw a0, -8(tp) # store x
  lw t0, -20(tp) # wordarr
  lw t0, 0(t0)  #  contents wordarr
  bne t0, zero, else
then:
  lw t1, -24(tp) #bytearr
  sb zero, 0(t1) #bytearr = 0
  j return
else:
  lw a0, -20(tp)
  addi a0, a0, 4
  lw a1, -24(tp)
  addi a1, a1, 1
  jal decode
  sw a0, -4(tp) #y onto stack
  lw t0, -8(tp) #t0 = x
  lw t1, -20(tp)#wordarr ptr
  lw t1, 0(t1)  #wordarr contents
  xor t2, a0, t0 #t2 = m = y xor x
  add t2, t2, t1 #t2 = m = (x xor y) + *wordarr
  sw t2, -16(tp)
  lw t1, -24(tp)
  srl t2, t2, 2
  sb t2, 0(t1)
  jal codgen
  not a0, a0
  addi a0, a0, 1
  lw t0, -4(tp)
  lw t1, -8(tp)
  #lw t2, -12(fp)
  lw t3, -16(tp)
  add a0, a0, t1
  add a0, a0, t0
  add a0, a0, t3
  addi a0, a0, 5
return:
  add sp, tp, zero
  lw tp, 0(sp)
  addi sp, sp, 4
  lw ra, 0(sp)
  addi sp, sp, 4
  jr ra


# end of file.

