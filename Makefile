arch ?= x86_64

target ?= $(arch)-rost
rost := target/$(target)/debug/librost_nbs.a

kernel := build/kernel-$(arch).bin
iso := build/os-$(arch).iso


linker_script := src/arch/$(arch)/boot/linker.ld
grub_cfg := src/arch/$(arch)/boot/grub.cfg

assembly_source_files := $(wildcard src/arch/$(arch)/boot/*.asm)
assembly_object_files := $(patsubst src/arch/$(arch)/boot/%.asm, \
		build/arch/$(arch)/boot/%.o, $(assembly_source_files))

.PHONY: all clean run iso kernel

all: $(kernel) 

clean:
	@rm -r build

run: $(iso) 
		@qemu-system-x86_64 -cdrom $(iso)

debug: $(iso)
		@qemu -S -gdb tcp::3333
iso: $(iso)



$(iso): $(kernel) $(grub_cfg)
		@mkdir -p build/isofiles/boot/grub
		@cp $(kernel) build/isofiles/boot/kernel.bin
		@cp $(grub_cfg) build/isofiles/boot/grub
		@grub-mkrescue -o $(iso) build/isofiles 2> /dev/null
		@rm -r build/isofiles

$(kernel): kernel $(rost) $(assembly_object_files) $(linker_script)
		@ld -n -T $(linker_script) -o $(kernel) \
				$(assembly_object_files) $(rost)

kernel:
		@cargo build --lib

# Compile assembly files
build/arch/$(arch)/boot/%.o: src/arch/$(arch)/boot/%.asm 
		@mkdir -p $(shell dirname $@)
		@nasm -felf64 $< -o $@
	