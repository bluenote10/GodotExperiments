#!/bin/bash

cd $(dirname "$0")

cargo watch --clear --why --delay 0.2 -x "build --release" -s "notify-send 'Compilation done'"
