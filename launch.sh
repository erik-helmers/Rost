#!/bin/sh

qemu-system-x86_64 -serial stdio -drive format=raw,file=target/x86_64-rost/debug/bootimage-rost.bin
