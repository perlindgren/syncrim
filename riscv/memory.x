SECTIONS
{
  . = 0x0;
  .text :
  {
    KEEP(*(.text)); 
  }

  . = 0x200;
  .trap :
  {
    KEEP(*(.trap));
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
  . = 0x50000200;
  .clic_vec : 
  {
    KEEP(*(.clic_vec));  
  }
}
PROVIDE(_stack_start = 0x50000500);
