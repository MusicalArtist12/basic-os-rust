.global stack_top
.global stack_bottom
.global p4_table
.global p3_table
.global p2_table

.section .bss
.align 4096
p4_table:
    .skip 4096
p3_table:
    .skip 4096
p2_table:
    .skip 4096
stack_bottom:
    .skip 512 * 1024 * 1024
stack_top:

