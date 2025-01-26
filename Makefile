ARCH = x86_64

ASM_x86_64 = nasm -f elf64
ASM_EXT_x86_64 = .nasm

LD_x86_64 = ld -n
LD_EXT_x86_64 = .ld

ASM_OBJS_x86_64 = \
	boot.o

ASM = $(ASM_$(ARCH))
ASM_EXT = $(ASM_EXT_$(ARCH))

LD = $(LD_$(ARCH))
LD_EXT = $(LD_EXT_$(ARCH))

ASM_OBJS = $(patsubst %,build/$(ARCH)/asm/%,$(ASM_OBJS_$(ARCH)))

MKDIR_P = mkdir -p

iso: build/$(ARCH)/deywos.iso

clean:
	$(RM) -r build

build/$(ARCH)/deywos.iso: build/$(ARCH)/kernel.bin

build/$(ARCH)/asm/%.o: asm/$(ARCH)/%$(ASM_EXT)
	$(MKDIR_P) build/$(ARCH)/asm/
	$(ASM) $^ -o $@

build/$(ARCH)/kernel.bin: link/$(ARCH)$(LD_EXT) $(ASM_OBJS)
	$(MKDIR_P) build/$(ARCH)
	$(LD) -o $@ -T $< $(ASM_OBJS)
