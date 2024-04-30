OUTPUT_FORMAT("elf64-x86-64")
ENTRY(start)


SECTIONS 
{
    . = 1M;

    .boot : 
    {
      KEEP(*(.boot.multiboot))
    }
    
    .text :
    {
      KEEP(*(.init.text))
    
    }

	  .rodata : 
    {
      *(.rodata .rodata.*)
    }
}