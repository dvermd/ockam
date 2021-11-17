use ockam_core::{async_trait, compat::boxed::Box, Address};
use ockam_core::{AccessControl, LocalMessage, Result};

pub struct GithubSshAccessControl {
    allowed_nicknames: Vec<String>,
    registry_address: Address,
}

impl GithubSshAccessControl {
    pub fn new(allowed_nicknames: Vec<String>, registry_address: Address) -> Self {
        GithubSshAccessControl {
            allowed_nicknames,
            registry_address,
        }
    }
}

#[async_trait]
impl AccessControl for GithubSshAccessControl {
    async fn msg_is_authorized(&mut self, _local_msg: &LocalMessage) -> Result<bool> {
        // TODO: Go to registry and check if this entity has proven possession of one of the nicknames
        // TODO: Cache result respecting expiration date
        unimplemented!()
    }
}
