#!/bin/bash

qemu-system-x86_64 -drive format=raw,file=target/x86_64_rustos/debug/bootimage-rustos.bin
