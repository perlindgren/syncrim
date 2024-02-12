MEMORY
{
  RAM : ORIGIN = 0x50000000, LENGTH = 16K
  FLASH : ORIGIN = 0x00000000, LENGTH = 16M
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
