SECTIONS
{
  . = 0x0;
  .text :
  {
    KEEP(*(.text)); 
  }
  . = 0x50000000;
  .data :
  {
    KEEP(*(.data));  
  }
  . = 0x50000100;
  .vector_table : 
  {
    KEEP(*(.vector_table));  
  }
}
PROVIDE(_stack_start = 0x50002000);
