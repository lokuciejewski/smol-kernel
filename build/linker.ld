SECTIONS
{
  . = 0x42000100;
  .text : {
    boot.o(.text)
    *(.text)
  }
  .bss : {
    _bstart1 = . ;
    *(.bss); *(COMMON);
    _bend1 = . ;
  }
  .data :  {
    *(.data)
  }

  __heap_start = .;
  .heap (NOLOAD) :
  {
    . += 64 * 1024 * 1024;
  }
  __heap_end_exclusive = .;

  .uncached 0x50000000 : {
    _bstart2 = . ;
    *(UNCACHED);
    _bend2 = . ;
  }
}
