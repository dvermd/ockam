use ockam_core::{async_trait, compat::boxed::Box};
use ockam_core::{Routed, Worker};
use ockam_entity::EntityIdentifier;
use ockam_node::Context;

pub struct Entry {
    entity_identifier: EntityIdentifier,
    gh_nicknames: Vec<String>,
    // TODO: expiration date
}

pub struct CredentialsRegistry {
    registry: Vec<Entry>,
}

impl Default for CredentialsRegistry {
    fn default() -> Self {
        Self { registry: vec![] }
    }
}

pub struct CredentialsRegistryWorker {
    registry: CredentialsRegistry,
}

impl CredentialsRegistryWorker {
    pub fn new(registry: CredentialsRegistry) -> Self {
        Self { registry }
    }
}

#[async_trait]
impl Worker for CredentialsRegistryWorker {
    type Message = ();
    type Context = Context;

    async fn handle_message(
        &mut self,
        _context: &mut Self::Context,
        _msg: Routed<Self::Message>,
    ) -> ockam_core::Result<()> {
        unimplemented!()
    }
}
