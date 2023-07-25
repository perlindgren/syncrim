SECTIONS
{
   . = 0x0;
   .init : { *(.init) }
   . = 0x80000000;
   .some_section : { *(.some_section) }
}