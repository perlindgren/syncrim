SECTIONS
{
  . = 0x0;
  .text :
  {
    KEEP(*(.text)); 
  }

  . = 0x1000;
  .data :
  {
    KEEP(*(.data));  
  } 
}