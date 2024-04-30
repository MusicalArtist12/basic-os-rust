.global start

.section .init.bss
  .align 4096
p4_table:
    .skip 4096
p3_table:
    .skip 4096
p2_table:
    .skip 4096
stack_bottom:
    .skip 4096 * 2
stack_top:


// Global Descriptor Table
.section .init.rodata
gdt64:
  .quad 0 // zero entry

gdt64_code_entry:
  .set gdt64_code_seg, gdt64_code_entry - gdt64
  .quad (1<<44) | (1<<47) | (1<<41) | (1<<43) | (1<<53) // code segment

gdt64_data_entry:
  .set gdt64_data_seg, gdt64_data_entry - gdt64
  .quad (1<<44) | (1<<47) | (1<<41) // data segment

gdt64_pointer:
  .set gdt64_limit, gdt64_pointer - gdt64 - 1
  .word gdt64_limit
  .quad gdt64


.section .init.text, "ax", @progbits
.code32
start:
    cli 
    mov esp, stack_top

    call check_multiboot
    call check_long_mode
    call check_cpuid

    call enable_paging
    call enable_compatibility_mode

    lgdt [gdt64_pointer]

    mov eax, [gdt64_data_seg]
    mov ds, eax
    mov es, eax
    mov fs, eax
    mov gs, eax
    mov ss, eax

    jmp long_jump

    hlt

error:
    mov dword ptr [0xb8000], 0x4f524f45
    mov dword ptr [0xb8004], 0x4f3a4f52
    mov dword ptr [0xb8008], 0x4f204f20
    mov byte  ptr [0xb800a], al
    hlt

check_multiboot:
    cmp eax, 0x36d76289
    jne .no_multiboot
    ret

.no_multiboot:
    mov al, 0x30
    jmp error

check_cpuid:
    // Check if CPUID is supported by attempting to flip the ID bit (bit 21)
    // in the FLAGS register. If we can flip it, CPUID is available.

    // Copy FLAGS in to EAX via stack
    pushfd
    pop eax

    // Copy to ECX as well for comparing later on
    mov ecx, eax

    // Flip the ID bit
    xor eax, 1 << 21

    // Copy EAX to FLAGS via the stack
    push eax
    popfd

    // Copy FLAGS back to EAX (with the flipped bit if CPUID is supported)
    pushfd
    pop eax

    // Restore FLAGS from the old version stored in ECX (i.e. flipping the
    // ID bit back if it was ever flipped).
    push ecx
    popfd

    // Compare EAX and ECX. If they are equal then that means the bit
    // wasn't flipped, and CPUID isn't supported.
    cmp eax, ecx
    je .no_cpuid
    ret
.no_cpuid:
    mov al, 0x31
    jmp error

check_long_mode:
    // test if extended processor info in available
    mov eax, 0x80000000    // implicit argument for cpuid
    cpuid                  // get highest supported argument
    cmp eax, 0x80000001    // it needs to be at least 0x80000001
    jb .no_long_mode       // if it's less, the CPU is too old for long mode

    // use extended info to test if long mode is available
    mov eax, 0x80000001    // argument for extended processor info
    cpuid                  // returns various feature bits in ecx and edx
    test edx, 1 << 29      // test if the LM-bit is set in the D-register
    jz .no_long_mode       // If it's not set, there is no long mode

    ret
.no_long_mode:
    mov al, 0x32
    jmp error

// adapted from https://wiki.osdev.org/Setting_Up_Long_Mode
enable_paging:
    mov edi, 0x1000    // Set the destination index to 0x1000.
    mov cr3, edi       // Set control register 3 to the destination index.
    xor eax, eax       // Nullify the A-register.
    mov ecx, 4096      // Set the C-register to 4096.
    rep stosd          // Clear the memory.
    mov edi, cr3       // Set the destination index to control register 3.

    mov dword ptr [edi], 0x2003  // Set the uint32_t at the destination index to 0x2003.
    add edi, 0x1000              // Add 0x1000 to the destination index.
    mov dword ptr [edi], 0x3003  // Set the uint32_t at the destination index to 0x3003.
    add edi, 0x1000              // Add 0x1000 to the destination index.
    mov dword ptr [edi], 0x4003  // Set the uint32_t at the destination index to 0x4003.
    add edi, 0x1000              // Add 0x1000 to the destination index.

    mov ebx, 0x00000003          // Set the B-register to 0x00000003.
    mov ecx, 512                 // Set the C-register to 512.
 
.set_entry:
    mov dword ptr [edi], ebx     // Set the uint32_t at the destination index to the B-register.
    add ebx, 0x1000              // Add 0x1000 to the B-register.
    add edi, 8                   // Add eight to the destination index.
    loop .set_entry              // Set the next entry.

    mov eax, cr4                 // Set the A-register to control register 4.
    or eax, 1 << 5               // Set the PAE-bit, which is the 6th bit (bit 5).
    mov cr4, eax                 // Set control register 4 to the A-register.

    ret

// adapted from https://wiki.osdev.org/Setting_Up_Long_Mode
enable_compatibility_mode:
    mov ecx, 0xC0000080          // Set the C-register to 0xC0000080, which is the EFER MSR.
    rdmsr                        // Read from the model-specific register.
    or eax, 1 << 8               // Set the LM-bit which is the 9th bit (bit 8).
    wrmsr                        // Write to the model-specific register.

    mov eax, cr0                 // Set the A-register to control register 0.
    or eax, 1 << 31              // Set the PG-bit, which is the 32nd bit (bit 31).
    mov cr0, eax                 // Set control register 0 to the A-register.

    ret

.section .init.text, "ax", @progbits
.code64
long_mode_start:
    mov rax, 0x2f592f412f4b2f4f
    mov qword ptr [0xb8000], rax

    hlt
