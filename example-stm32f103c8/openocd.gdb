set history save on
set confirm off
target extended-remote :3333
set print asm-demangle on
monitor arm semihosting enable
monitor reset halt
load
# monitor verify
# monitor reset
# quit
continue
