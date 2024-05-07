.section .rodata
.align 4
gdt64:
    .quad 0 // zero entry

gdt64_code:
    .set gdt64_code_offset, gdt64_code - gdt64
    .quad (1<<44) | (1<<47) | (1<<41) | (1<<43) | (1<<53) // code segment

gdt64_data:
    .set gdt64_data_offset, gdt64_data - gdt64
    .quad (1<<44) | (1<<47) | (1<<41) // data segment

gdt64_pointer:
    .set gdt64_pointer_offset, gdt64_pointer - gdt64 - 1
    .word gdt64_pointer_offset
    .quad gdt64

.section .bss
.align 4096
p4_table:
    .skip 4096
p3_table:
    .skip 4096
p2_table:
    .skip 4096
stack_bottom:
    .skip 4096 * 4
stack_top:

