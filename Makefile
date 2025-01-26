ARCH = x86_64

ASM_x86_64 = nasm -f elf64
ASM_EXT_x86_64 = .nasm

LD_x86_64 = ld -n
LD_EXT_x86_64 = .ld

ASM_OBJS_x86_64 = \
	boot.o

QEMU_x86_64 = qemu-system-x86_64

ASM = $(ASM_$(ARCH))
ASM_EXT = $(ASM_EXT_$(ARCH))

LD = $(LD_$(ARCH))
LD_EXT = $(LD_EXT_$(ARCH))

ASM_OBJS = $(patsubst %,build/$(ARCH)/asm/%,$(ASM_OBJS_$(ARCH)))

QEMU = $(QEMU_$(ARCH))

MKDIR_P = mkdir -p

GRUB_MKISO = grub-mkrescue

CP = cp -r

iso: build/$(ARCH)/deywos.iso

qemu: iso
	$(QEMU) -cdrom build/$(ARCH)/deywos.iso

clean:
	$(RM) -r build

build/$(ARCH)/deywos.iso: \
		build/$(ARCH)/iso/boot/kernel.bin \
		build/$(ARCH)/iso/boot/grub/grub.cfg
	$(GRUB_MKISO) -o $@ build/$(ARCH)/iso/

build/$(ARCH)/asm/%.o: asm/$(ARCH)/%$(ASM_EXT)
	$(MKDIR_P) build/$(ARCH)/asm/
	$(ASM) $^ -o $@

build/$(ARCH)/iso/boot/kernel.bin: linker/$(ARCH)$(LD_EXT) $(ASM_OBJS)
	$(MKDIR_P) build/$(ARCH)/iso/boot/
	$(LD) -o $@ -T $< $(ASM_OBJS)

build/$(ARCH)/iso/boot/grub/grub.cfg: iso/boot/grub/grub.cfg
	$(MKDIR_P) build/$(ARCH)/iso/boot/grub/
	$(CP) $^ $@
