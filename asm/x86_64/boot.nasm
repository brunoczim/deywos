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

start:
    mov dword [0xb8000], 0x2f4b2f4f
    hlt
