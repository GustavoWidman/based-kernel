section .multiboot
align 8
mb2_header_start:
    dd 0xE85250D6                                               ; Multiboot2 magic
    dd 0                                                        ; arch: i386
    dd mb2_header_end - mb2_header_start                        ; header length
    dd -(0xE85250D6 + 0 + (mb2_header_end - mb2_header_start))  ; checksum

    ; End tag
    dw 0
    dw 0
    dd 8
mb2_header_end:

section .bss
align 4096
pml4:    resb 4096
pdpt:    resb 4096
pd:      resb 4096
stack:   resb 16384
stack_top:

section .text
bits 32
global _start
extern main

_start:
    mov esp, stack_top

    ; Build page tables
    ; PML4[0] -> PDPT
    mov eax, pdpt
    or eax, 0x03        ; present + writable
    mov [pml4], eax

    ; PDPT[0] -> PD
    mov eax, pd
    or eax, 0x03
    mov [pdpt], eax

    ; Identity map first 1GiB using 2MiB pages
    mov ecx, 0
    mov eax, 0x83       ; present + writable + huge page
.map_pd:
    mov [pd + ecx * 8], eax
    add eax, 0x200000   ; 2MiB
    inc ecx
    cmp ecx, 512
    jne .map_pd

    ; Enable long mode
    mov eax, pml4
    mov cr3, eax        ; load page table

    mov eax, cr4
    or eax, 0x20        ; enable PAE
    mov cr4, eax

    mov ecx, 0xC0000080 ; EFER MSR
    rdmsr
    or eax, 0x100       ; enable long mode
    wrmsr

    mov eax, cr0
    or eax, 0x80000001  ; enable paging + protected mode
    mov cr0, eax

    lgdt [gdt.ptr]      ; load GDT
    jmp 0x08:long_mode  ; far jump to 64-bit code

section .rodata
gdt:
    dq 0                    ; null descriptor
    dq 0x00209A0000000000   ; code segment
.ptr:
    dw $ - gdt - 1
    dq gdt

section .text
bits 64
long_mode:
    xor ax, ax
    mov ss, ax
    mov ds, ax
    mov es, ax

    call main

.halt:
    hlt
    jmp .halt
