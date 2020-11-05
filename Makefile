arch ?= x86_64

target ?= $(arch)-rost
target_dir := target/$(target)/debug

kern_default ?= $(target_dir)/rost_nbs


# This is the executable 
kern_elf ?= $(kern_default)
kern_dir := $(dir $(kern_elf))
# kernel Stripped Debug
kern_elf_stripped := $(kern_elf)_sd

iso ?= $(kern_elf).iso

grub_cfg := src/arch/$(arch)/boot/grub.cfg

.PHONY: all clean run iso kernel debug r d 

all: kernel 

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
		@qemu-system-x86_64 -cdrom $(iso) -serial stdio
d: debug
debug: $(iso)
		@qemu-system-x86_64 -S -gdb tcp::3333 -cdrom $(iso)


iso: $(iso)
	

$(iso): $(kern_elf)
	@echo Building ISO for: $(notdir $(kern_elf)) to $(iso)
	@mkdir -p $(kern_dir)/isofiles/boot/grub
	@cp $(kern_elf) $(kern_dir)/isofiles/boot/kernel.bin
	@cp $(grub_cfg) $(kern_dir)/isofiles/boot/grub
	@grub-mkrescue -o $(iso) $(kern_dir)/isofiles 2> /dev/null
	@rm -r $(kern_dir)/isofiles

$(kern_default): kernel

kernel:
		@cargo build 
		@objcopy --strip-debug $(kern_elf) $(kern_elf_stripped) 
