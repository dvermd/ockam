use crate::{Address, Message, NodeContext, Result, Route};
use core::time::Duration;

/// Send a delayed event to a worker
pub struct DelayedEvent<C: NodeContext, M: Message> {
    route: Route,
    ctx: C,
    d: Duration,
    msg: M,
}

impl<C: NodeContext, M: Message> DelayedEvent<C, M> {
    /// Create a new 100ms delayed message event
    pub async fn new(ctx: &C, route: Route, msg: M) -> Result<Self> {
        let child_ctx = ctx.new_context(Address::random(0)).await?;

        debug!(
            "Creating a delayed event with address '{}'",
            child_ctx.address()
        );

        Ok(Self {
            route,
            ctx: child_ctx,
            d: Duration::from_millis(100),
            msg,
        })
    }

    /// Adjust the delay time with a [`Duration`](core::time::Duration)
    pub fn with_duration(self, d: Duration) -> Self {
        Self { d, ..self }
    }

    /// Adjust the delay time in milliseconds
    pub fn with_millis(self, millis: u64) -> Self {
        Self {
            d: Duration::from_millis(millis),
            ..self
        }
    }

    /// Adjust the delay time in seconds
    pub fn with_seconds(self, secs: u64) -> Self {
        Self {
            d: Duration::from_secs(secs),
            ..self
        }
    }

    /// Adjust the delay time in minutes
    pub fn with_minutes(self, mins: u64) -> Self {
        Self {
            d: Duration::from_secs(mins * 60),
            ..self
        }
    }

    /// Run this delayed event
    pub fn spawn(self) {
        let Self { route, ctx, d, msg } = self;
        // FIXME: use `ctx.spawn_detached` (can't because we borrow ctx inside
        // but it takes 'static, but also even that should be replaced by async
        // abstraction, so for now this is okay)
        crate::spawn(async move {
            ctx.sleep(d).await;
            if let Err(e) = ctx.send(route, msg).await {
                error!("Failed to send delayed message: {}", e);
            }
        })
    }
}
