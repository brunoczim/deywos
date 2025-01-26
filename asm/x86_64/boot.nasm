global start

section .multiboot_header

    multiboot_2_magic equ 0xe85250d6
    architecture equ 0
    header_len equ header_end - header_start
    checksum equ 0x100000000 - (multiboot_2_magic + architecture + header_len)
    end_type equ 0
    flags equ 0
    size equ 8

header_start:
    dd multiboot_2_magic
    dd architecture
    dd header_len
    dd checksum
    dw end_type
    dw flags
    dd size
header_end:

section .text
bits 32

    multiboot_loaded equ 0x36d76289

start:
    ; is multiboot loaded?
    cmp eax, multiboot_loaded
    ; no? jump
    jne .no_multiboot
    ; load stack
    mov esp, stack_top
.check_cpuid:
    ; load flags
    pushfd
    pop eax
    ; backup flags
    mov ecx, eax
    ; flip cpuid bit
    xor eax, 1 << 21
    ; store modified flags
    push eax
    popfd
    ; reload flags
    pushfd
    pop eax
    ; restore original flags
    push ecx
    popfd
    ; are reloaded changed flags different from original ones?
    cmp eax, ecx
    ; no? jump
    je .no_cpuid
.check_long_mode:
    ; test for extended info
    mov eax, 0x80000000
    cpuid
    ; processor should be at least this new
    cmp eax, 0x80000001
    ; no? jump
    jb .no_long_mode
    ; test for long mode
    mov eax, 0x80000001
    cpuid
    ; long flag enabled?
    test edx, 1 << 29
    ; no? jump
    jz .no_long_mode
.success:
    jmp error
    ; 'OK'
    mov dword [0xb8000], 0x2f4b2f4f
    hlt
.no_multiboot
    mov al, '0'
    jmp error
.no_cpuid:
    mov al, '1'
    jmp error
.no_long_mode:
    mov al, '2'
    jmp error

error:
    ; 'ER'
    mov dword [0xb8000], 0x4f524f45
    ; 'R:'
    mov dword [0xb8004], 0x4f3a4f52
    ; '  '
    mov dword [0xb8008], 0x4f204f20
    mov byte  [0xb800a], al
    hlt

section .bss

stack_bottom:
    resb 64
stack_top:
