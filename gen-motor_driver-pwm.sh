#!/bin/bash -eu
cd "$(realpath "$(dirname "$0")")"
if [[ ! -r pwm-f303d.txt ]]; then
    printf "Error: template pwm-f303d.txt not found"
else
    for i in Tim1Ch{2..4} Tim2Ch{1..4} Tim3Ch{1..4} Tim4Ch{1..4} Tim8Ch{1..4}; do
        if [[ $i =~ Tim2Ch[1-4] ]]; then
            sed "s/Tim1Ch1/$i/g" pwm-f303d.txt | sed -E 's/let duty (.*) as u16;/let duty \1 as u32;/' >>librobof303d/src/motor_driver/pwm.rs
        else
            sed "s/Tim1Ch1/$i/g" pwm-f303d.txt >>librobof303d/src/motor_driver/pwm.rs
        fi
    done
fi
if [[ ! -r pwm-f407.txt ]]; then
    printf "Error: template pwm-f407.txt not found"
else
    for i in TIM{2..4} TIM8; do
        sed "s/TIM1/$i/g" pwm-f407.txt >>librobof407/src/motor_driver/pwm.rs
    done
fi
