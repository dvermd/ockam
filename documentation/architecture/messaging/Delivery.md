# Delivery properties

When sending messages between workers depends on the workers and transports in between,
the received sequences may be different from sent, messages may be missing, reordered or compromised,
they may not be received at all.

The longer the pipeline between sender and receiver, harder it becomes to reason about delivery.

To simplify that we are going formalize a set of properties, and describe how they can be combined.

## End-to-end

Delivery properties exist between two workers (sender and receiver) on a route.

For example when worker `A` send messages to worker `D` via route `A->B;B->C;C->D`,
we describe delivery on route `A->D = A->B;B->C;C->D`

Delivery properties on routes `B->C` and `C->D` may be different, but as long as we can validate properties of `A->D`, we don't have to care.

This approach is called **end-to-end**.


## Core combination techniques

In order to combine delivery over multiple steps, we use two main techniques:

1. Pipelining:

If we have a delivery `A->B` and delivery `B->C`, and we know the properties of those deliveries, (we know how the workers forwarding messages work)
Then we can tell how a pipelined delivery `A->B;B->C` would behave.

**TODO: picture**

1. End-to-end coordination:

If we know that a delivery from `A->C` has certain properties, we can extend logic of A and C to improve those properties.

For example if we want to facilitate delivery between devices over cloud services, we would create a pipeline from one device through the cloud service and to the other device `D1->C;C->D2`

Then we would set up end-to-end coordination between devices by adding some messaging logic **in the devices** to make sure delivery properties are respected end-to-end

Special workers can be used to "wrap" unreliable delivery and provide reliable delivery over unreliable message pipelines, e.g. `D1->D1w ; D1w->C;C->D2w ; D2w->D2` where `D1w` and `D2w` are wrapper workers.

**TODO: picture**

Main tool to implement end-to-end wrapping is called Pipe,
more on pipes in [Pipes and Channels](./Pipes_Channels.md)


## Properties

1. [Accessibility](./Accessibility.md)
  Accessibility describes an ability of the route (and workers in this route) to deliver messages to the destination.
  Accessibility is required property in messaging, since there is not delivery without it.

1. [Reliability and uniqness](./Reliability.md)
  Reliability describe how many sent messages were received and whether some messages are lost
  Uniqness describes whether messages received correspond to unique sent messages or are duplicates of the same sent message

1. [Ordering](./Ordering.md)
  Ordefing describes whether messages are received in the same order as they were sent
  Uniqness is related to ordering in the sense that absolute ordering requires no duplicates, hence similar techniques are used to achieve those properties

1. [Integrity](./Integrity.md)
  Integrity describes whether received messages carry the same data as sent messages, that they are not corrupted
  Delivery Integrity of multiple messages is related to ordering and reliability and often requires those properties

1. [Privacy](./Privacy.md)
  Provacy of messages describes which messaging participants have access to the messages data

1. [Autheniticity and authorization](./Trust.md)
   Autheniticity describes whether messages received originated at the specific sender
   Authorization describes messages can be received by a certain receiver if they were send by a certain sender or went through a certain route
