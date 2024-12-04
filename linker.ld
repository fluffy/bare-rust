/* Info for STM32F405RGTx */

_Min_Heap_Size = 0x0;
_Min_Stack_Size = 0x800;

MEMORY
{
  FLASH  (RX) : ORIGIN = 0x08000000, LENGTH = 1024K
  RAM    (XRW): ORIGIN = 0x20000000, LENGTH = 128K
  CCRAM2 (XRW): ORIGIN = 0x10000000, LENGTH = 64K
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


   ._heap_start :
   {
    . = ALIGN(4);
    . = . + _Min_Heap_Size;
    . = . + _Min_Stack_Size;
    . = ALIGN(4);
   }
}