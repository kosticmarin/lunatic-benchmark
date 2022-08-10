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
 ./bench.sh 1 single --message-size 500k --requests 1024
```

Output

```
Client 0 stats:
Overall upload stats:

Transferred  500.00 MiB in 1024 messages in 16.28s (30.72 MiB/s)

Message upload metrics:

      │  Throughput   │ Duration
──────┼───────────────┼──────────
 AVG  │  152.83 MiB/s │    2.00ms
 P0   │   84.94 MiB/s │    2.00ms
 P10  │  118.94 MiB/s │    2.00ms
 P50  │  147.75 MiB/s │    3.00ms
 P90  │  194.87 MiB/s │    4.00ms
 P100 │  225.12 MiB/s │    5.00ms
Overall download stats:

Transferred  500.00 MiB in 1024 messages in 16.28s (30.72 MiB/s)

Message download metrics:

      │  Throughput   │ Duration
──────┼───────────────┼──────────
 AVG  │  339.33 MiB/s │    1.00ms
 P0   │  113.25 MiB/s │    0.00ns
 P10  │  225.87 MiB/s │    1.00ms
 P50  │  300.25 MiB/s │    1.00ms
 P90  │  460.00 MiB/s │    2.00ms
 P100 │  695.00 MiB/s │    4.00ms
```

## Multiple

On a local nodes creates N "client" processes which then send messages to all distributed nodes in the network requests. Takes measurements for each message between all distributed nodes.

Note: each client talks to all distributed nodes in a random order.

Run example:

```
./bench.sh 10 multiple --clients 2 --message-size 500k --requests 1024
```

Output

```
Client 0 stats:
Overall upload stats:

Transferred  500.49 MiB in 1025 messages in 15.78s (31.72 MiB/s)

Message upload metrics:

      │  Throughput   │ Duration
──────┼───────────────┼──────────
 AVG  │  150.74 MiB/s │    2.00ms
 P0   │   66.81 MiB/s │    2.00ms
 P10  │  116.06 MiB/s │    2.00ms
 P50  │  150.12 MiB/s │    3.00ms
 P90  │  184.87 MiB/s │    4.00ms
 P100 │  227.12 MiB/s │    7.00ms
Overall download stats:

Transferred  500.49 MiB in 1025 messages in 15.78s (31.72 MiB/s)

Message download metrics:

      │  Throughput   │ Duration
──────┼───────────────┼──────────
 AVG  │  333.56 MiB/s │    1.00ms
 P0   │   16.08 MiB/s │    0.00ns
 P10  │  195.50 MiB/s │    0.00ns
 P50  │  300.50 MiB/s │    1.00ms
 P90  │  548.50 MiB/s │    2.00ms
 P100 │ 2626.00 MiB/s │   30.00ms

Client 1 stats:
Overall upload stats:

Transferred  500.49 MiB in 1025 messages in 15.85s (31.58 MiB/s)

Message upload metrics:

      │  Throughput   │ Duration
──────┼───────────────┼──────────
 AVG  │  152.58 MiB/s │    2.00ms
 P0   │   82.88 MiB/s │    2.00ms
 P10  │  115.75 MiB/s │    2.00ms
 P50  │  151.75 MiB/s │    3.00ms
 P90  │  188.87 MiB/s │    4.00ms
 P100 │  219.25 MiB/s │    5.00ms
Overall download stats:

Transferred  500.49 MiB in 1025 messages in 15.85s (31.58 MiB/s)

Message download metrics:

      │  Throughput   │ Duration
──────┼───────────────┼──────────
 AVG  │  336.40 MiB/s │    1.00ms
 P0   │   16.34 MiB/s │    0.00ns
 P10  │  195.00 MiB/s │    0.00ns
 P50  │  293.25 MiB/s │    1.00ms
 P90  │  552.00 MiB/s │    2.00ms
 P100 │ 3944.00 MiB/s │   29.00ms
```
