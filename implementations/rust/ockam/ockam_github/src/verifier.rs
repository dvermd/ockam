use crate::GithubError;
use ockam_channel::SecureChannelLocalInfo;
use ockam_core::{async_trait, compat::boxed::Box, Address};
use ockam_core::{Message, Result, Routed, Worker};
use ockam_entity::EntitySecureChannelLocalInfo;
use ockam_message_derive::Message;
use ockam_node::Context;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Message)]
pub struct GithubSshVerifierMsg {
    nickname: String,
    proof: Vec<u8>,
}

pub struct GithubSshVerifier {
    registry_address: Address,
}

impl GithubSshVerifier {
    pub fn new(registry_address: Address) -> Self {
        GithubSshVerifier { registry_address }
    }
}

#[async_trait]
impl Worker for GithubSshVerifier {
    type Message = GithubSshVerifierMsg;
    type Context = Context;

    async fn handle_message(
        &mut self,
        ctx: &mut Self::Context,
        msg: Routed<Self::Message>,
    ) -> Result<()> {
        let local_msg = msg.local_message();
        let profile_id = EntitySecureChannelLocalInfo::find_info(local_msg)?
            .their_profile_id()
            .clone();
        let auth_hash = SecureChannelLocalInfo::find_info(local_msg)?.auth_hash();

        // TODO: Spawn a separate task

        let msg = msg.body();
        let nickname = msg.nickname;
        let proof = msg.proof;

        let keys_str = reqwest::get(format!("https://github.com/{}.keys", nickname))
            .await
            .map_err(|_| GithubError::HttpError)?
            .text()
            .await
            .map_err(|_| GithubError::HttpError)?;
        for key_line in keys_str.lines() {
            let mut split = key_line.split_whitespace();
            // Support only ssh-ed25519 for now
            if let Some(kt) = split.next() {
                if kt != "ssh-ed25519" {
                    continue;
                }
            } else {
                continue;
            }

            if let Some(key_str) = split.next() {
            } else {
                continue;
            }
        }

        // TODO: Check signature of auth_hash by one of the keys
        // TODO: Add info that we authenticated profile_id

        Ok(())
    }
}
