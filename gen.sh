#!/bin/bash

set +e

# show all reg blocks
# cat doc/STM32F405.json | jq '.device.peripherals.peripheral[].name'

echo "// DO NOT EDIT. This was generated by gen.sh" > src/gen_cpu.rs

echo "#![allow(unused)]" >> src/gen_cpu.rs

cat doc/STM32F405.json \
   | jq '.device.peripherals.peripheral[] | .name as $regname | select( .name == "RCC" ).registers.register.[] | .name as $rname | .fields.field.[] | {  ($regname+"_"+$rname+"_"+.name) :   .bitOffset  } ' \
   | grep "_" \
   | sed -e 's/[":]//g' \
   | awk ' { print "const " $1 ": i32 = " $2 ";" } ' >> src/gen_cpu.rs

cat doc/STM32F405.json \
   | jq '.device.peripherals.peripheral[] | .name as $regname | select( .name == "FLASH" ).registers.register.[] | .name as $rname | .fields.field.[] | {  ($regname+"_"+$rname+"_"+.name) :   .bitOffset  } ' \
   | grep "_" \
   | sed -e 's/[":]//g' \
   | awk ' { print "const " $1 ": i32 = " $2 ";" } ' >> src/gen_cpu.rs

cat doc/STM32F405.json \
   | jq '.device.peripherals.peripheral[] | .name as $regname | select( .name == "GPIOA" ).registers.register.[] | .name as $rname | .fields.field.[] | {  ($regname+"_"+$rname+"_"+.name) :   .bitOffset  } ' \
   | grep "_" \
   | sed -e 's/[":]//g' \
   | awk ' { print "const " $1 ": i32 = " $2 ";" } ' >> src/gen_cpu.rs

cat doc/STM32F405.json \
   | jq '.device.peripherals.peripheral[] | .name as $regname | select( .name == "GPIOA" ).registers.register.[] | .name as $rname | .fields.field.[] | {  ($regname+"_"+$rname+"_"+.name) :   .bitOffset  } ' \
   | grep "_" \
   | sed -e 's/[":]//g' \
   | awk ' { print "const " $1 ": i32 = " $2 ";" } ' \
   | sed -e 's/GPIOA_/GPIOB_/' >> src/gen_cpu.rs

cat doc/STM32F405.json \
   | jq '.device.peripherals.peripheral[] | .name as $regname | select( .name == "GPIOA" ).registers.register.[] | .name as $rname | .fields.field.[] | {  ($regname+"_"+$rname+"_"+.name) :   .bitOffset  } ' \
   | grep "_" \
   | sed -e 's/[":]//g' \
   | awk ' { print "const " $1 ": i32 = " $2 ";" } ' \
   | sed -e 's/GPIOA_/GPIOC_/' >> src/gen_cpu.rs


