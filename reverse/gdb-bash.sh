#!/bin/bash
gdb -q starti rcraps_reverse_c_armv7.elf
set logging enabled on
si
set logging enabled off
quit
