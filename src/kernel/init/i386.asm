.global start

.section .bss
stack_bottom:
    .lcomm stack, 64
stack_top:

.section .text
.code32
start:
    mov esp, stack_top

    mov dword ptr [0xb8000], 0x2f4b2f4f
    hlt

error:
    mov dword ptr [0xb8000], 0x4f524f45
    mov dword ptr [0xb8004], 0x4f3a4f52
    mov dword ptr [0xb8008], 0x4f204f20
    mov byte  ptr [0xb800a], al
    hlt