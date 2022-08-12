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

ADDR=$(ip -4 -br a | grep UP | awk '{ split($3,a,"/"); print a[1] }')
echo "Starting server on $ADDR"

echo "Starting control node - $ADDR:10410"
lunatic --control "$ADDR:10410" --control-server --test-ca --no-entry &
sleep 1
echo "Starting $1 no-entry clients"
CLIENTS=$1
for (( i=0; i<CLIENTS; i++));
do
    lunatic --control "$ADDR:10410" --node "127.0.0.1:1000$i" --test-ca --no-entry &
done
sleep 1

wait
killall lunatic
