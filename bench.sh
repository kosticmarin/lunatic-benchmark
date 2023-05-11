#!/bin/bash
echo -ne '\033c'
set -e

trap_ctrlc()
{
        killall lunatic
        exit
}

trap trap_ctrlc SIGHUP SIGINT SIGTERM

cargo b --release

# export RUST_LOG=lunatic=trace
echo "Starting control node"
lunatic control --bind-socket 127.0.0.1:10410 &
sleep 1
echo "Starting $1 no-entry clients"
CLIENTS=$1
for (( i=0; i<CLIENTS; i++));
do
    lunatic node http://127.0.0.1:10410/ --bind-socket "127.0.0.1:1000$i" &
done
sleep 1

BINARY=$2
lunatic node http://127.0.0.1:10410/ --bind-socket 127.0.0.1:20000 --wasm "target/wasm32-wasi/release/$BINARY.wasm" &
PID=$!
wait $PID
killall lunatic
