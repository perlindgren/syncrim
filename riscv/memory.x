SECTIONS
{
   . = 0x0;
   .text : { *(.init) }
   . = 0x80000000;
   .data : { *(.data) }
}
