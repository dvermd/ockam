# Ordering

Ordering of a delivery describes order in which messages are received in relation to the order in which messages were sent.

### Types of ordering

1. Monotonic ordering
  A sequence is monotonically ordered when:
  In a delivery containing messages `m1, m2 ...`
  A message `m1` which was sent before message `m2`,
  cannot be received after `m2`

1. Strict monotonic ordering
  A sequence is strictly ordered when:
  In a delivery `m1, m2, ...`
  A message `m1` which was sent before message `m2`,
  cannot be received after `m2` or after `m1`

Difference between monotonic and strict ordering is that
**strict ordering doesn't allow duplicates**

1. Continuous ordering
  A sequence is continously monotonic ordered when:
  In a delivery `m1, m2, ...`
  A message `m2` which was sent after `m1`,
  cannot be received before `m1`

Difference between monotonic and continuous ordering is that
**continuous ordering doesn't allow message loss**

1. Continuous strict ordering (Absolute ordering)
  A sequence is absolute ordered when:
  In a delivery `m1, m2, ...`
  A message `m2` which was sent after `m1`
  can only be received after `m1`

**Absolute ordering doesn't allow neither message loss or duplicates**

**TODO: pictures**

**NOTE: all ordering properties exist per delivery, which only applies for messages sent via same route**

**TODO: picture**

## Local delivery

Local delivery (delivery to local routes `[0#A]`) is **absolutely** ordered

**This is a requirement for node implementations**

However, workers **may** process messages in different order.

## Ordering in complex routes

When forwarding messages through an intermediate worker, it can process messages in different order, which might reorder messages in delivery.

### Pipelining ordering

Similar to reliability, ordering guarantees are weakening when pipelining

Pipelining monotonic and continous delivery results in monotonic delivery

**TODO: picture**

Pipelining strict and non-strict delivery results in non-strict delivery

**TODO: picture**

### Ordered processors

**Ordered processor** is a worker which upon receiving a sequence of messages,
sends a sequence of processing results in order.

Processors ordering has the same [ordering types](#Types_of_ordering) as delivery

Delivery via local routes through an ordered processor has the same properties as the processor

**TODO: picture**


### Ordered pipes

A pipe in which receiver sends messages in the same order as sender receives them,
can be viewed as an ordered processor

More on pipes [here](./Pipes_Channels.md)

Ordered pipes can be injected to turn unordered delivery on a route (or multiple routes)
into an ordered delivery

**TODO: picture**

## Implementing ordered pipes

### Sequential processing pipe:

Also known as **receive queue approach**

`P1` does not process a message before `P2` sends a previous message

To enforce continuity, sender may re-send messages which were not confirmed.

To enforce strictness, receiver may confirm duplicate messages without sending them

**TODO: picture**

### Indexed processing pipe:

Also known as **send queue approach**

`P1` assigns each message a monotonic continous index
`P2` sends messages in index order

To enforce continuity, receiver may request missing lower index messages when receiving non-consecutive index

To enforce strictness, receiver doesn't send messages with the same index to already sent messages


**TODO: picture**

More on pipes and channels: [Pipes and Channels](./Pipes_Channels.md)

Back to: [Delivery properties](Delivery.md)

Up next: [Integrity](./Integrity.md)

