#!/bin/bash
set +e

svd2rusty doc/STM32F405.svd | rustfmt > src/hal/cpu/gen_cpu.rs

