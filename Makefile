arch ?= x86_64

target ?= $(arch)-rost
target_dir := target/$(target)/debug
rost := $(target_dir)/rost_nbs
rost_stripped := $(target_dir)/rost_nbs_sd

kernel := build/kernel-$(arch).bin
iso := build/os-$(arch).iso

grub_cfg := src/arch/$(arch)/boot/grub.cfg

.PHONY: all clean run iso kernel debug r d 

all: $(kernel) 

clean:
	@# When changing the link script we want
	@# to remove the target directory without having
	@# to recompile the core and other std crate.
	@rm -rf build
	@rm -rf $(target_dir)/deps/librost*
	@rm -rf $(target_dir)/deps/rost*
	@rm -rf $(target_dir)/incremental
	@rm -f $(target_dir)/rost* $(target_dir)/librost*
	

r: run
run: $(iso) 
		@qemu-system-x86_64 -cdrom $(iso)
d: debug
debug: $(iso)
		@qemu-system-x86_64 -S -gdb tcp::3333 -cdrom $(iso)

iso: $(iso)



$(iso): $(kernel) $(grub_cfg)
		@mkdir -p build/isofiles/boot/grub
		@cp $(kernel) build/isofiles/boot/kernel.bin
		@cp $(grub_cfg) build/isofiles/boot/grub
		@grub-mkrescue -o $(iso) build/isofiles 2> /dev/null
		@rm -r build/isofiles

$(kernel): kernel 
		@mkdir -p build
		@cp $(rost_stripped) $(kernel)

kernel:
		@cargo build 
		@objcopy --strip-debug $(rost) $(rost_stripped) 