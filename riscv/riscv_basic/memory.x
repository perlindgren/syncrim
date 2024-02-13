MEMORY
{
  FLASH : ORIGIN = 0x00000000, LENGTH = 2M
  RAM : ORIGIN = 0x50000000, LENGTH = 16K
}

REGION_ALIAS("REGION_TEXT", FLASH);
REGION_ALIAS("REGION_RODATA", RAM);
REGION_ALIAS("REGION_DATA", RAM);
REGION_ALIAS("REGION_BSS", RAM);
REGION_ALIAS("REGION_HEAP", RAM);
REGION_ALIAS("REGION_STACK", RAM);

PROVIDE(Interrupt0 = DefaultInterruptHandler);
PROVIDE(Interrupt1 = DefaultInterruptHandler);
PROVIDE(Interrupt2 = DefaultInterruptHandler);
PROVIDE(Interrupt3 = DefaultInterruptHandler);
PROVIDE(Interrupt4 = DefaultInterruptHandler);
PROVIDE(Interrupt5 = DefaultInterruptHandler);
PROVIDE(Interrupt6 = DefaultInterruptHandler);
PROVIDE(Interrupt7 = DefaultInterruptHandler);
PROVIDE(Interrupt8 = DefaultInterruptHandler);
PROVIDE(Interrupt9 = DefaultInterruptHandler);
SECTIONS{
  /* ### .uninit */
  .uninit (NOLOAD) : ALIGN(4)
  {
    . = ALIGN(4);
    __suninit = .;
    *(.uninit .uninit.*);
    . = ALIGN(4);
    __euninit = .;
  } > RAM
}
