arch ?= x86_64

target ?= $(arch)-rost
rost := target/$(target)/debug/librost_nbs.a

kernel := build/kernel-$(arch).bin
iso := build/os-$(arch).iso

grub_cfg := src/arch/$(arch)/boot/grub.cfg

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

$(kernel): kernel 
		@cp $(rost) $(kernel)

kernel:
		@cargo build --lib
