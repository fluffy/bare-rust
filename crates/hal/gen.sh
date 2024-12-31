#!/bin/bash
set +e

svd2rusty doc/STM32F405.svd | rustfmt > crates/hal/src/svd.rs

