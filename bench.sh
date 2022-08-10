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

echo "Starting control node"
lunatic --control 127.0.0.1:10410 --control-server --test-ca --no-entry &
sleep 1
echo "Starting $1 no-entry clients"
CLIENTS=$1
for (( i=0; i<CLIENTS; i++));
do
    lunatic --control 127.0.0.1:10410 --node "127.0.0.1:1000$i" --test-ca --no-entry &
done
sleep 1

BINARY=$2
lunatic --control 127.0.0.1:10410 --node 127.0.0.1:20000 --test-ca "target/wasm32-wasi/release/$BINARY.wasm" -- ${@:3} &
PID=$!
wait $PID
killall lunatic
