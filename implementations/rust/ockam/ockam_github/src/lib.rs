use ockam_core::{Address, Result, Route};
use ockam_entity::Entity;
use ockam_node::Context;

mod access_control;
mod credentials_registry;
mod error;
mod verifier;

pub use access_control::*;
pub use credentials_registry::*;
pub use error::*;
pub use verifier::*;

pub struct GithubSshAuth;

impl GithubSshAuth {
    pub async fn start_registry(ctx: &Context) -> Result<Address> {
        let worker = CredentialsRegistryWorker::new(Default::default());
        let address = Address::random(0);

        ctx.start_worker(address.clone(), worker).await?;

        Ok(address)
    }

    pub async fn start_verifier(
        &mut self,
        ctx: &Context,
        address: Address,
        registry_address: Address,
    ) -> Result<()> {
        let verifier = GithubSshVerifier::new(registry_address);

        ctx.start_worker(address, verifier).await
    }

    pub async fn create_access_control(
        &mut self,
        allowed_nicknames: Vec<String>,
        registry_address: Address,
    ) -> Result<GithubSshAccessControl> {
        Ok(GithubSshAccessControl::new(
            allowed_nicknames,
            registry_address,
        ))
    }

    pub async fn present_credential(
        &mut self,
        entity: &mut Entity,
        key_label: String,
        verifier_route: Route,
    ) -> Result<()> {
        unimplemented!()
    }
}
