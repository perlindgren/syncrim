.option norvc
 .text
 main:
  la t0, 0x5000
  li t1, 64
  li t2, 128
  sw t1, 0(t0)
  sw t2, 4(t0)
  sw t1, 8(t0)
  sw t2, 12(t0)

  end:
  j end
