# `stm32-usbd-examples`

> A collection of examples for [`stm32-usbd`](https://github.com/Disasm/stm32-usbd).

## Supported targets

### STM32F103C8 ([Blue Pill](https://wiki.stm32duino.com/index.php?title=Blue_Pill) board)

    cd example-stm32f103c8
    cargo run --release

### STM32F042K6 ([NUCLEO-F042K6](https://www.st.com/en/evaluation-tools/nucleo-f042k6.html) board)

    cd example-stm32f042k6
    openocd -f openocd.cfg &
    cargo run --release

### STM32F432KC ([NUCLEO-L432KC](https://www.st.com/en/evaluation-tools/nucleo-l432kc.html) board)

    cd example-stm32l432kc
    openocd -f openocd.cfg &
    cargo run --release
