/* Info for STM32F405 */
MEMORY
{
  FLASH : ORIGIN = 0x08000000, LENGTH = 1024K
  RAM   : ORIGIN = 0x20000000, LENGTH = 112K
}

ENTRY(Reset_Handler);

/* setup stack */
_estack = ORIGIN(RAM) + LENGTH(RAM);

SECTIONS
{
 .vector_table ORIGIN(FLASH) :
  {
    LONG(_estack);
    KEEP(*(.vector_table.reset_vector));
    KEEP(*(.vector_table.exceptions));
  } > FLASH

  .text : ALIGN(4)
  {
    *(.text .text.*);
  } > FLASH

   .rodata : ALIGN(4)
   {
    *(.rodata .rodata.*);
   } > FLASH

   .data : ALIGN(4) /* AT(ADDR(.rodata) + SIZEOF(.rodata)) */
   {
     _sdata = .;
     *(.data .data.*);
     _edata = .;
   }  > RAM AT > FLASH

   _sidata = LOADADDR(.data);

   .bss : ALIGN(4)
   {
      _sbss = .;
      *(.bss .bss.*);
      _ebss = .;
   } > RAM

   . = ALIGN(4);
   _heap_start = .;
}