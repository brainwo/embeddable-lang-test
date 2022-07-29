# This script is effective for Linux/Unix-based OS
# ================================================
# Make sure you have cargo-watch already installed
# This sends a notification on every changes happened,
# make sure you have a notification server installed as well
# Currently the error checker simply check whether there is
# compiled wasm file in target folder

#!/bin/sh

cargo watch -C 'src' -i '/www'\
    -x 'build --target wasm32-unknown-unknown --release'\
    -s '(mv --force ../target/wasm32-unknown-unknown/release/*.wasm ../www/game.wasm && notify-send "✔    Cargo Watch" "All building process have been done successfully") || notify-send "❌    Cargo Watch" "Build not successfully runs; please check the compile error messages"' & 
basic-http-server www &
[ "$1" == "f" ] && firefox --new-window "127.0.0.1:4000"
[ "$1" == "g" ] && google-chrome-stable --new-window "127.0.0.1:4000"

