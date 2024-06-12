.global start
.section .init, "ax", @progbits
.code32
.extern stack_top

start:
    cli 
    mov %ebx, %edi          // pass multiboot address information to _start
    movl $stack_top, %esp
    
    call check_multiboot
    call check_cpuid
    call check_longmode

    call setup_page_tables
    call enable_paging

    lgdt (gdt64_pointer)
    
    movw $gdt64_data_offset, %ax
    movw %ax, %ds
    movw %ax, %es
    movw %ax, %fs
    movw %ax, %gs
    movw %ax, %ss

    ljmp $gdt64_code_offset, $fix_cs

fix_cs:    


    jmp _start

    hlt

check_multiboot:
    cmp $0x36d76289, %eax 
    jne multiboot_fail

    ret

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
    movl $p3_table, %eax
    or $0b11, %eax
    movl %eax, (p4_table)

    movl $p2_table, %eax
    or $0b11, %eax
    movl %eax, (p3_table)

    movl $0, %ecx
    .map_p2_table:
        movl $0x200000, %eax
        mul %ecx
        orl $0b10000011, %eax
        movl %eax, p2_table(,%ecx, 8)

        inc %ecx
        cmp $512, %ecx
    jne .map_p2_table

    movl $p4_table, %eax
    or $0b11, %eax
    // movl %eax, p4_table + 511 * 8
    movl %eax, [p4_table + 511 * 8]


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

multiboot_fail:
    movl $0x4f554f4d, vga_start
    movl $0x4f544f4c, vga_start + 4
    movl $0x4f424f49, vga_start + 8
    movl $0x4f4f4f4f, vga_start + 12
    movl $0x4f0a4f54, vga_start + 16

    hlt
