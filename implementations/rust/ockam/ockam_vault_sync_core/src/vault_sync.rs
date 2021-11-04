use crate::{Vault, VaultRequestMessage, VaultResponseMessage, VaultTrait};
use ockam_core::compat::{boxed::Box, rand::random};
use ockam_core::NodeContext;
use ockam_core::{async_trait::async_trait, Address, AsyncTryClone, Handle, Result, ResultMessage};
use tracing::debug;

mod asymmetric_vault;
mod hasher;
mod key_id_vault;
mod secret_vault;
mod signer;
mod symmetric_vault;
mod verifier;

pub use asymmetric_vault::*;
pub use hasher::*;
pub use key_id_vault::*;
pub use secret_vault::*;
pub use signer::*;
pub use symmetric_vault::*;
pub use verifier::*;

/// Vault sync wrapper
pub struct VaultSync<C> {
    handle: Handle<C>,
}

#[async_trait]
impl<C: NodeContext> AsyncTryClone for VaultSync<C> {
    async fn async_try_clone(&self) -> Result<Self> {
        self.start_another().await
    }
}

impl<C: NodeContext> VaultSync<C> {
    pub(crate) async fn call(&mut self, msg: VaultRequestMessage) -> Result<VaultResponseMessage> {
        self.handle
            .call::<VaultRequestMessage, ResultMessage<VaultResponseMessage>>(msg)
            .await?
            .into()
    }
    /// Start another Vault at the same address.
    pub async fn start_another(&self) -> Result<Self> {
        let vault_worker_address = self.handle.address().clone();

        let clone = VaultSync::create_with_worker(self.handle.ctx(), &vault_worker_address).await?;

        Ok(clone)
    }

    /// Create and start a new Vault using Worker.
    pub async fn create_with_worker(ctx: &C, vault: &Address) -> Result<Self> {
        let address: Address = random();

        debug!("Starting VaultSync at {}", &address);

        let child_ctx = ctx.new_context(address).await?;

        Ok(Self {
            handle: Handle::new(child_ctx, vault.clone()),
        })
    }

    /// Start a Vault.
    pub async fn create<T: VaultTrait>(ctx: &C, vault: T) -> Result<Self> {
        let vault_address = Vault::create_with_inner(ctx, vault).await?;

        Self::create_with_worker(ctx, &vault_address).await
    }

    /// Return the Vault worker address
    pub fn address(&self) -> Address {
        self.handle.address().clone()
    }
}
