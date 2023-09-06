SECTIONS
{
  . = 0x0;
  .text :
  {
    KEEP(*(.text)); 
  }

  . = 0x100;
  .isr0 :
  {
    KEEP(*(.isr0));
  }
  . = 0x200;
  .isr1 :
  {
    KEEP(*(.isr1));
  }
  . = 0x50000000;
  .data :
  {
    KEEP(*(.data));  
  } 
}

PROVIDE(_stack_start = 0x50000500);