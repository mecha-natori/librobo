#!/bin/bash -eu
cd $(realpath $(dirname $0))
if [[ ! -r a.txt ]]; then
    printf "Error: template a.txt not found"
fi
for i in Tim1Ch{2..4} Tim2Ch{1..4} Tim3Ch{1..4} Tim4Ch{1..4} Tim8Ch{1..4}; do
    if [[ $i =~ Tim2Ch[1-4] ]]; then
        sed "s/Tim1Ch1/$i/g" a.txt | sed -E 's/let duty (.*) as u16;/let duty \1 as u32;/' >>librobof303d/src/motor_driver/cytron.rs
    else
        sed "s/Tim1Ch1/$i/g" a.txt >>librobof303d/src/motor_driver/cytron.rs
    fi
done
