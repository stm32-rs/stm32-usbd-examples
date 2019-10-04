[![Build Status](https://travis-ci.org/stm32-rs/stm32-usbd-examples.svg?branch=master)](https://travis-ci.org/stm32-rs/stm32-usbd-examples)

# `stm32-usbd-examples`

> A collection of examples for [`stm32-usbd`](https://github.com/stm32-rs/stm32-usbd).

## Cloning the repository

    git clone --recursive https://github.com/stm32-rs/stm32-usbd-examples
    cd stm32-usbd-examples

## Supported targets

### STM32F103C8 ([Blue Pill](https://wiki.stm32duino.com/index.php?title=Blue_Pill) board)

    rustup target add thumbv7m-none-eabi
    cd example-stm32f103c8
    openocd -f openocd.cfg &
    cargo run --release

### STM32F303VC ([STM32F3DISCOVERY](https://www.st.com/en/evaluation-tools/stm32f3discovery.html) board)

    rustup target add thumbv7em-none-eabihf
    cd example-stm32f303vc
    openocd -f board/stm32f3discovery.cfg &
    cargo run --release

### STM32F042K6 ([NUCLEO-F042K6](https://www.st.com/en/evaluation-tools/nucleo-f042k6.html) board)

    rustup target add thumbv6m-none-eabi
    cd example-stm32f042k6
    openocd -f openocd.cfg &
    cargo run --release

### STM32F072RB ([32F072BDISCOVERY](https://www.st.com/en/evaluation-tools/32f072bdiscovery.html) board)

    rustup target add thumbv6m-none-eabi
    cd example-stm32f072rb
    openocd -f openocd.cfg &
    cargo run --release

### STM32L432KC ([NUCLEO-L432KC](https://www.st.com/en/evaluation-tools/nucleo-l432kc.html) board)

    rustup target add thumbv7em-none-eabihf
    cd example-stm32l432kc
    openocd -f openocd.cfg &
    cargo run --release
