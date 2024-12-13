#!/bin/bash
set +e

# show all reg blocks
# cat doc/STM32F405.json | jq '.device.peripherals.peripheral[].name'

echo "// DO NOT EDIT. This was generated by gen.sh" > src/cpu/gen_cpu.rs

echo "#![allow(unused)]" >> src/cpu/gen_cpu.rs
echo "#![allow(non_snake_case)]" >> src/cpu/gen_cpu.rs
echo "#![allow(non_upper_case_globals)]" >> src/cpu/gen_cpu.rs

for nm in "RCC" "FLASH" "GPIOA" "GPIOB"; do
cat doc/STM32F405.json \
   | jq ".device.peripherals.peripheral[] | .name as \$regname | select( .name == \"$nm\" ).registers.register.[] | .name as \$rname | .fields.field.[] | {  (\$regname+\" \"+\$rname+\" \"+.name) :   .bitOffset  } " \
   | grep ":" \
   | sed -e 's/[":]//g' \
   | awk ' { print "pub const " $1 "_" tolower($2) "_" $3 ": i32 = " $4 ";" } ' \
   >> src/cpu/gen_cpu.rs
done


