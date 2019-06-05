[![Build Status](https://travis-ci.org/Disasm/stm32-usbd-examples.svg?branch=master)](https://travis-ci.org/Disasm/stm32-usbd-examples)

# `stm32-usbd-examples`

> A collection of examples for [`stm32-usbd`](https://github.com/Disasm/stm32-usbd).

## Cloning the repository

    git clone --recursive https://github.com/Disasm/stm32-usbd-examples
    cd stm32-usbd-examples

## Supported targets

### STM32F103C8 ([Blue Pill](https://wiki.stm32duino.com/index.php?title=Blue_Pill) board)

    rustup target add thumbv7m-none-eabi
    cd example-stm32f103c8
    # [programmer-specific setup goes here]
    cargo run --release

### STM32F042K6 ([NUCLEO-F042K6](https://www.st.com/en/evaluation-tools/nucleo-f042k6.html) board)

    rustup target add thumbv6m-none-eabi
    cd example-stm32f042k6
    openocd -f openocd.cfg &
    cargo run --release

### STM32F432KC ([NUCLEO-L432KC](https://www.st.com/en/evaluation-tools/nucleo-l432kc.html) board)

    rustup target add thumbv7em-none-eabihf
    cd example-stm32l432kc
    openocd -f openocd.cfg &
    cargo run --release
