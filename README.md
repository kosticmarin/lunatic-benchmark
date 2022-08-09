# Lunatic runtime bechmark

This is a WIP repository

Custom benchmark suite similar to [QUINN bench crate](https://github.com/quinn-rs/quinn)

## Single

A most simple case where on two distributed nodes messages are exchanged between two processes.

There are two command line arguments:

- message-size - controls size of a single message in bytes (e.g. 100, 1k, 1M)
- requests - number of requests

Run example

```
./bench.sh 1 single --message-size 1k --requests 1024
```

Output

```
Client 0 stats:
Overall upload stats:

Transferred    1.00 MiB in 1024 messages in 124.94ms (8.00 MiB/s)

Message upload metrics:

      │  Throughput   │ Duration
──────┼───────────────┼──────────
 AVG  │  142.46 MiB/s │    0.00ns
 P0   │   19.91 MiB/s │    0.00ns
 P10  │  113.87 MiB/s │    0.00ns
 P50  │  146.62 MiB/s │    0.00ns
 P90  │  163.62 MiB/s │    0.00ns
 P100 │  258.75 MiB/s │    0.00ns
Overall download stats:

Transferred    1.00 MiB in 1024 messages in 124.94ms (8.00 MiB/s)

Message download metrics:

      │  Throughput   │ Duration
──────┼───────────────┼──────────
 AVG  │   11.33 MiB/s │    0.00ns
 P0   │    1.10 MiB/s │    0.00ns
 P10  │    9.31 MiB/s │    0.00ns
 P50  │   11.48 MiB/s │    0.00ns
 P90  │   13.01 MiB/s │    0.00ns
 P100 │   18.53 MiB/s │    0.00ns
```

## Multiple

On a local nodes creates N "client" processes which then send messages to all distributed nodes in the network requests. Takes measurements for each message between all distributed nodes.

Note: each client talks to all distributed nodes in a random order.

Run example:

```
./bench.sh 10 multiple --clients 2 --message-size 1k --requests 1024
```

Output

```
Client 0 stats:
Overall upload stats:

Transferred    1.00 MiB in 1025 messages in 335.13ms (2.99 MiB/s)

Message upload metrics:

      │  Throughput   │ Duration
──────┼───────────────┼──────────
 AVG  │  139.44 MiB/s │    0.00ns
 P0   │   27.69 MiB/s │    0.00ns
 P10  │   90.19 MiB/s │    0.00ns
 P50  │  135.75 MiB/s │    0.00ns
 P90  │  201.00 MiB/s │    0.00ns
 P100 │  286.00 MiB/s │    0.00ns
Overall download stats:

Transferred    1.00 MiB in 1025 messages in 335.13ms (2.99 MiB/s)

Message download metrics:

      │  Throughput   │ Duration
──────┼───────────────┼──────────
 AVG  │   10.62 MiB/s │    0.00ns
 P0   │    0.80 MiB/s │    0.00ns
 P10  │    7.81 MiB/s │    0.00ns
 P50  │    9.95 MiB/s │    0.00ns
 P90  │   14.86 MiB/s │    0.00ns
 P100 │   19.59 MiB/s │    1.00ms

Client 1 stats:
Overall upload stats:

Transferred    1.00 MiB in 1025 messages in 335.61ms (2.98 MiB/s)

Message upload metrics:

      │  Throughput   │ Duration
──────┼───────────────┼──────────
 AVG  │  139.25 MiB/s │    0.00ns
 P0   │   39.62 MiB/s │    0.00ns
 P10  │   89.31 MiB/s │    0.00ns
 P50  │  132.00 MiB/s │    0.00ns
 P90  │  206.12 MiB/s │    0.00ns
 P100 │  334.00 MiB/s │    0.00ns
Overall download stats:

Transferred    1.00 MiB in 1025 messages in 335.61ms (2.98 MiB/s)

Message download metrics:

      │  Throughput   │ Duration
──────┼───────────────┼──────────
 AVG  │   10.60 MiB/s │    0.00ns
 P0   │    0.66 MiB/s │    0.00ns
 P10  │    7.79 MiB/s │    0.00ns
 P50  │   10.05 MiB/s │    0.00ns
 P90  │   14.73 MiB/s │    0.00ns
 P100 │   19.20 MiB/s │    1.00ms
```
