# Ockam messaging building blocks

## Problem statement

Ockam Routing provides a framework to send messages between Ockam Workers and allows workers to forward messages using onward routes and trace return routes.

However, since it's up to the workers and transports to forward messages, Routing protocol does not guarantee that messages sent will be delivered.

Different transports have different delivery properties, routes may forward a message through multiple machines over the network which makes it hard to reason about delivery over complex routes.


## Definitions and symbols

### Message
A piece of information which can be sent and received by Workers.

Message names use lower case, usually starting with `m`, e.g. `m1`, `msg1`, `m_create`

### Worker
A stateful system which can receive and send messages.

Workers are named staring with uppercase letters, e.g. `worker A`, `worker Sender`, `worker C1`

Worker addresses use either worker names, shorter worker names or `<number>#<worker_name>` format

Worker addresses use uppercase letters numbers
For example:
`A` is an address of `worker A`
`0#B` is an address of `worker B` (with address type `0`)
`S` is an address for `worker Sender`

Workers can have multiple addresses:
`C1` is an address for `worker C1`
`C1'` is also an address for `worker C1`

Format with address type, e.g `0#A`, `1#B` is used to distinguish
transport addresses and local node addresses

More on local addresses in [Accessibility](./Accessibility.md#local_routes)

### Route

A path that message can be sent to

In Ockam Routing context messages workers and routes have a more specific definition.
This guide is using wider notion of those for abstraction purposes.

E.g. UDP datagram is not implemented as Ockam Routing Message and return address in the datagram is not implemented as Ockam Route, but they can be called Message and Route.

Routes are used to send messages from one worker to another

A route to `worker B` is written as `->B`, if `worker A` sends messages to this route,
it can be written as `A->B`

Routes can be combined together, combination of `A->B` and `B->C` is written as `A->B ; B->C`
`;` is a route combination operator

Routes are lists of addresses and can be written as such `[A, B, C]`

We can use both `->` and list notation together, e.g. `[A] ; A->C`

Routes without specific addresses are written with lowercase names starting with `r`, e.g `r1`, `r_onward`, `r_return`


### Delivery

A sequence of messages `m1,m2,m3...` send from `worker A` to route `A->B` and received by `worker B`
Delivery might have multiple dynamic **delivery properties**, usually statistically measured

More on delivery properties in [Delivery properties](./Delivery.md)

Delivery exists per route, if messages `m1,m3` are sent over route `A->B` and messages `m2,m4` sent over route `A'->B'`, those are two different deliveries.
Because of that, we can use routes to identify deliveries, e.g. delivery `A->B`

### Pipe

A pair of coordinated workers which provide certain end-to-end delivery with certain properties

### Channel

A pair of coordinated workers which provide certain end-to-end delivery with certain properties in both directions

More on pipes and channels: [Pipes and Channels](./Pipes_Channels.md)



Up next: [Delivery properties overview](Delivery.md)


