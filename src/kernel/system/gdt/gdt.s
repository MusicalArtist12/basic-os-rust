.section .rodata
.align 4
.global gdt64_code_offset
.global gdt64_data_offset
.global gdt64_pointer

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
