/* Code Sources
    - https://wiki.osdev.org/Setting_Up_Long_Mode

*/

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

.global start
.section .init.text, "ax", @progbits
.code32

start:
    movl $stack_top, %esp
    mov %ebx, %edi          // pass multiboot address information to _start

    call check_cpuid
    call check_longmode

    call setup_page_tables
    call enable_paging

    lgdt (gdt64_pointer)
    movw $gdt64_data_offset, %ax
    movw %ax, %ss
    movw %ax, %ds
    movw %ax, %es

    ljmp $gdt64_code_offset, $_start

    hlt

check_cpuid:
    pushfd
    pop %eax
    
    // make a copy and store into %ecx
    mov %eax, %ecx

    xor $1 << 21, %eax
    push %eax
    popfd

    pushfd
    pop %eax
    
    push %ecx
    popfd
    
    xor %eax, %ecx
    jz cpuid_fail  // jump if eax == ecx
    
    ret

check_longmode:
    mov 0x80000000, %eax
    cpuid 
    cmp 0x80000001, %eax
    jb longmode_fail // jump if eax < ecx

    ret 

setup_page_tables:
    mov $p4_table, %eax
    orl $0b11, %eax
    movl %eax, (p4_table + 511 * 8)

    movl $p3_table, %eax
    or $0b11, %eax
    movl %eax, (p4_table)

    movl $p2_table, %eax
    or $0b11, %eax
    movl %eax, (p3_table)

    movl $0, %ecx
    .map_p2_table:
        movl $0x200000, %eax
        mul %eax
        orl $0b10000011, %eax
        movl %eax, p2_table(,%ecx, 8)

        inc %ecx
        cmp $512, %ecx
    jne .map_p2_table

    ret

enable_paging:
    movl $p4_table, %eax
    movl %eax, %cr3

    movl %cr4, %eax
    orl $1 << 5, %eax
    mov %eax, %cr4

    mov $0xC0000080, %ecx
    rdmsr
    orl $1 << 8, %eax
    wrmsr

    movl %cr0, %eax
    orl $1 << 31, %eax
    mov %eax, %cr0

    ret

.equ vga_start, 0xb8000

cpuid_fail:
    movl $0x4f504f43, vga_start
    movl $0x4f494f55, vga_start + 4
    movl $0x00004f44, vga_start + 8

    hlt
longmode_fail:
    movl $0x4f4f4f4c, vga_start
    movl $0x4f474f4e, vga_start + 4
    movl $0x4f444f4d, vga_start + 8

    hlt

