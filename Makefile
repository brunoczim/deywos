ARCH = x86_64

RUST_OPT = debug

ASM_x86_64 = nasm -f elf64
ASM_EXT_x86_64 = .nasm

LD_x86_64 = ld -n
LD_EXT_x86_64 = .ld

ASM_OBJS_x86_64 = \
	boot_protected.o \
	boot_long.o \


QEMU_x86_64 = qemu-system-x86_64

ASM = $(ASM_$(ARCH))
ASM_EXT = $(ASM_EXT_$(ARCH))

LD = $(LD_$(ARCH))
LD_EXT = $(LD_EXT_$(ARCH))

ASM_OBJS = $(patsubst %,build/$(ARCH)/asm/%,$(ASM_OBJS_$(ARCH)))

QEMU = $(QEMU_$(ARCH))

RUST_FLAGS_debug =

RUST_FLAGS_release = --release

RUST_FLAGS = $(RUST_FLAGS_$(RUST_OPT))

CARGO = cargo +nightly

MKDIR_P = mkdir -p

GRUB_MKISO = grub-mkrescue

CP = cp -r

RUST_SRC_FILES = $(shell find src/ -type f -name '*.rs')

iso: build/$(ARCH)/deywos.iso

qemu: iso
	$(QEMU) -cdrom build/$(ARCH)/deywos.iso

clean:
	$(RM) -r build
	$(CARGO) clean

build/$(ARCH)/deywos.iso: \
		build/$(ARCH)/iso/boot/kernel.bin \
		build/$(ARCH)/iso/boot/grub/grub.cfg
	$(GRUB_MKISO) -o $@ build/$(ARCH)/iso/

build/$(ARCH)/asm/%.o: asm/$(ARCH)/%$(ASM_EXT)
	$(MKDIR_P) build/$(ARCH)/asm/
	$(ASM) $^ -o $@

build/$(ARCH)/iso/boot/kernel.bin: \
		linker/$(ARCH)$(LD_EXT) \
		$(ASM_OBJS) \
		build/$(ARCH)/libdeywos.a
	$(MKDIR_P) build/$(ARCH)/iso/boot/
	$(LD) -o $@ -T $< $(ASM_OBJS) build/$(ARCH)/libdeywos.a

build/$(ARCH)/iso/boot/grub/grub.cfg: iso/boot/grub/grub.cfg
	$(MKDIR_P) build/$(ARCH)/iso/boot/grub/
	$(CP) $^ $@

build/$(ARCH)/libdeywos.a: target/$(ARCH)-deywos/$(RUST_OPT)/libdeywos.a
	$(CP) $< $@

target/$(ARCH)-deywos/$(RUST_OPT)/libdeywos.a: \
		target-triple/$(ARCH)-deywos.json \
		Cargo.toml \
		Cargo.lock \
		$(RUST_SRC_FILES)
	$(CARGO) build $(RUST_FLAGS) --target target-triple/$(ARCH)-deywos.json
