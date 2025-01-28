global start
extern long_mode_start

section .multiboot_header

    multiboot_2_magic equ 0xe85250d6
    architecture equ 0
    header_len equ header_end - header_start
    checksum equ (1 << 32) - (multiboot_2_magic + architecture + header_len)
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
    mov eax, 1 << 31
    cpuid
    ; processor should be at least this new
    cmp eax, (1 << 31) | 1
    ; no? jump
    jb .no_long_mode
    ; test for long mode
    mov eax, (1 << 31) | 1
    cpuid
    ; long flag enabled?
    test edx, 1 << 29
    ; no? jump
    jz .no_long_mode
.setup_page_tables:
    ; link p4 to p3
    mov eax, p3_table
    or eax, 0b11
    mov [p4_table], eax
    ; link p3 to p2
    mov eax, p2_table
    or eax, 0b11
    mov [p3_table], eax
    ; counter = 0
    xor ecx, ecx
.map_p2_table:
    ; page size
    mov eax, 2 * 1024 * 1024
    ; make start address of page
    mul ecx
    ; present, writable, huge flags
    or eax, 0b10000011
    ; link this page
    mov [p2_table + ecx * 8], eax
    ; counter++
    inc ecx
    ; stop at 512
    cmp ecx, 512
    ; reached it? no? jump
    jb .map_p2_table
.enable_paging:
    ; cr3 = p4
    mov eax, p4_table
    mov cr3, eax
    ; set PAE flag in cr4
    mov eax, cr4
    or eax, 1 << 5
    mov cr4, eax
    ; set long mode...
    mov ecx, 0xc0000080
    ; read
    rdmsr
    ; modify to enable long mode
    or eax, 1 << 8
    ; write
    wrmsr
    ; enable paging in cr0
    mov eax, cr0
    or eax, 1 << 31
    mov cr0, eax
.setup_gdt64:
    ; setup 64 bit GDT
    lgdt [gdt64.pointer]
.switch_to_long_mode:
    ; long jump to longe mode main
    jmp gdt64.code:long_mode_start
.no_multiboot
    mov al, "0"
    jmp error
.no_cpuid:
    mov al, "1"
    jmp error
.no_long_mode:
    mov al, "2"
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

align 4096

p4_table:
    resb 4096
p3_table
    resb 4096
p2_table:
    resb 4096

stack_bottom:
    resb 4096 * 16
stack_top:

section .rodata

gdt64:
    dq 0
.code: equ $ - gdt64
    dq (1 << 43) | (1 << 44) | (1 << 47) | (1 << 53)
.pointer:
    dw $ - gdt64 - 1
    dq gdt64
