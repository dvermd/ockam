use crate::change_history::ProfileChangeHistory;
use crate::{
    authentication::Authentication,
    profile::Profile,
    AuthenticationProof, Changes, Contact, Contacts, EntityError,
    EntityError::{ContactVerificationFailed, InvalidInternalState},
    EventIdentifier, Identity, KeyAttributes, Lease, MetaKeyAttributes, ProfileChangeEvent,
    ProfileEventAttributes, ProfileIdentifier, ProfileVault, TTL,
};
use cfg_if::cfg_if;
use ockam_core::compat::rand::{thread_rng, CryptoRng, RngCore};
use ockam_core::compat::{
    string::{String, ToString},
    vec::Vec,
};
use ockam_core::{allow, deny, Address, AsyncTryClone, Result, Route};
use ockam_core::{async_trait, compat::boxed::Box};
use ockam_vault::{KeyIdVault, PublicKey, Secret, SecretAttributes};
use ockam_vault_core::{SecretPersistence, SecretType, CURVE25519_SECRET_LENGTH};
use ockam_vault_sync_core::VaultSync;

cfg_if! {
    if #[cfg(feature = "credentials")] {
        use signature_core::message::Message;
        use crate::credential::EntityCredential;
    }
}

/// Profile implementation
#[derive(AsyncTryClone)]
pub struct ProfileState {
    id: ProfileIdentifier,
    change_history: ProfileChangeHistory,
    contacts: Contacts,
    pub(crate) vault: VaultSync,
    #[cfg(feature = "credentials")]
    pub(crate) rand_msg: Message,
    #[cfg(feature = "credentials")]
    pub(crate) credentials: Vec<EntityCredential>,
    lease: Option<Lease>,
}

impl ProfileState {
    /// Profile constructor
    pub fn new(
        identifier: ProfileIdentifier,
        change_events: Changes,
        contacts: Contacts,
        vault: VaultSync,
        rng: impl RngCore + CryptoRng + Clone,
    ) -> Self {
        // Avoid warning
        let _ = rng;
        Self {
            id: identifier,
            change_history: ProfileChangeHistory::new(change_events),
            contacts,
            vault,
            #[cfg(feature = "credentials")]
            rand_msg: Message::random(rng),
            #[cfg(feature = "credentials")]
            credentials: vec![],
            lease: None,
        }
    }

    pub(crate) fn change_history(&self) -> &ProfileChangeHistory {
        &self.change_history
    }

    pub(crate) fn vault_address(&self) -> Address {
        self.vault.address()
    }

    /// Create ProfileState
    pub(crate) async fn create(mut vault: VaultSync) -> Result<Self> {
        let initial_event_id = EventIdentifier::initial(&mut vault).await;

        let key_attribs = KeyAttributes::with_attributes(
            Profile::PROFILE_UPDATE.to_string(),
            MetaKeyAttributes::SecretAttributes(SecretAttributes::new(
                SecretType::Curve25519,
                SecretPersistence::Persistent,
                CURVE25519_SECRET_LENGTH,
            )),
        );

        let create_key_event = Self::create_key_static(
            initial_event_id,
            key_attribs.clone(),
            ProfileEventAttributes::new(),
            None,
            &mut vault,
        )
        .await?;

        let create_key_change =
            ProfileChangeHistory::find_key_change_in_event(&create_key_event, &key_attribs)
                .ok_or(InvalidInternalState)?;

        let public_key = ProfileChangeHistory::get_change_public_key(create_key_change)?;
        let public_key_id = vault.compute_key_id_for_public_key(&public_key).await?;
        let public_key_id = ProfileIdentifier::from_key_id(public_key_id);

        let profile = Self::new(
            public_key_id,
            vec![create_key_event],
            Default::default(),
            vault,
            thread_rng(),
        );

        Ok(profile)
    }

    pub(crate) async fn get_secret_key_from_event(
        key_attributes: &KeyAttributes,
        event: &ProfileChangeEvent,
        vault: &mut impl ProfileVault,
    ) -> Result<Secret> {
        let public_key = ProfileChangeHistory::get_public_key_from_event(key_attributes, event)?;

        let public_key_id = vault.compute_key_id_for_public_key(&public_key).await?;

        vault.get_secret_by_key_id(&public_key_id).await
    }

    pub async fn get_root_secret(&mut self) -> Result<Secret> {
        let public_key = ProfileChangeHistory::get_current_profile_update_public_key(
            self.change_history().as_ref(),
        )?;

        let key_id = self
            .vault
            .compute_key_id_for_public_key(&public_key)
            .await?;
        self.vault.get_secret_by_key_id(&key_id).await
    }

    pub fn has_lease(&self) -> bool {
        self.lease.is_some()
    }

    pub fn lease(&self) -> Option<&Lease> {
        self.lease.as_ref()
    }
}

#[async_trait]
impl Identity for ProfileState {
    async fn identifier(&self) -> Result<ProfileIdentifier> {
        Ok(self.id.clone())
    }

    async fn create_key(&mut self, label: String) -> Result<()> {
        let key_attribs = KeyAttributes::new(label);

        let event = {
            self.create_key(key_attribs, ProfileEventAttributes::new())
                .await?
        };
        self.add_change(event).await
    }

    async fn rotate_profile_key(&mut self) -> Result<()> {
        let event = {
            self.rotate_key(
                KeyAttributes::new(Profile::PROFILE_UPDATE.to_string()),
                ProfileEventAttributes::new(),
            )
            .await?
        };
        self.add_change(event).await
    }

    /// Get [`Secret`] key. Key is uniquely identified by label in [`KeyAttributes`]
    async fn get_profile_secret_key(&self) -> Result<Secret> {
        self.get_secret_key(Profile::PROFILE_UPDATE.to_string())
            .await
    }

    async fn get_secret_key(&self, label: String) -> Result<Secret> {
        let key_attributes = KeyAttributes::new(label);
        let event = ProfileChangeHistory::find_last_key_event(
            self.change_history().as_ref(),
            &key_attributes,
        )?
        .clone();
        Self::get_secret_key_from_event(
            &key_attributes,
            &event,
            &mut self.vault.async_try_clone().await?,
        )
        .await
    }

    async fn get_profile_public_key(&self) -> Result<PublicKey> {
        self.get_public_key(Profile::PROFILE_UPDATE.to_string())
            .await
    }

    async fn get_public_key(&self, label: String) -> Result<PublicKey> {
        self.change_history
            .get_public_key(&KeyAttributes::new(label))
    }

    /// Generate Proof of possession of [`Profile`].
    /// channel_state should be tied to channel's cryptographical material (e.g. h value for Noise XX)
    async fn create_auth_proof(&mut self, channel_state: &[u8]) -> Result<AuthenticationProof> {
        let root_secret = self.get_root_secret().await?;

        Authentication::generate_proof(channel_state, &root_secret, &mut self.vault).await
    }
    /// Verify Proof of possession of [`Profile`] with given [`ProfileIdentifier`].
    /// channel_state should be tied to channel's cryptographical material (e.g. h value for Noise XX)
    async fn verify_auth_proof(
        &mut self,
        channel_state: &[u8],
        responder_contact_id: &ProfileIdentifier,
        proof: &[u8],
    ) -> Result<bool> {
        let contact = self
            .get_contact(responder_contact_id)
            .await?
            .ok_or(EntityError::ContactNotFound)?;

        Authentication::verify_proof(
            channel_state,
            &contact.get_profile_update_public_key()?,
            proof,
            &mut self.vault,
        )
        .await
    }

    async fn add_change(&mut self, change_event: ProfileChangeEvent) -> Result<()> {
        let slice = core::slice::from_ref(&change_event);
        if ProfileChangeHistory::check_consistency(self.change_history.as_ref(), slice) {
            self.change_history.push_event(change_event);
        }
        Ok(())
    }

    async fn get_changes(&self) -> Result<Changes> {
        Ok(self.change_history.as_ref().to_vec())
    }

    /// Verify whole event chain of current [`Profile`]
    async fn verify_changes(&mut self) -> Result<bool> {
        if !ProfileChangeHistory::check_consistency(&[], self.change_history().as_ref()) {
            return deny();
        }

        if !self
            .change_history
            .verify_all_existing_events(&mut self.vault)
            .await?
        {
            return deny();
        }

        let root_public_key = self.change_history.get_first_root_public_key()?;

        let root_key_id = self
            .vault
            .compute_key_id_for_public_key(&root_public_key)
            .await?;
        let profile_id = ProfileIdentifier::from_key_id(root_key_id);

        if profile_id != self.identifier().await? {
            return Err(EntityError::ProfileIdDoesNotMatch.into());
        }

        allow()
    }

    async fn get_contacts(&self) -> Result<Vec<Contact>> {
        Ok(self.contacts.values().cloned().collect())
    }

    async fn as_contact(&mut self) -> Result<Contact> {
        Ok(Contact::new(
            self.id.clone(),
            self.change_history.as_ref().to_vec(),
        ))
    }

    async fn get_contact(&mut self, id: &ProfileIdentifier) -> Result<Option<Contact>> {
        Ok(self.contacts.get(id).cloned())
    }

    async fn verify_contact(&mut self, contact: Contact) -> Result<bool> {
        contact.verify(&mut self.vault).await?;

        allow()
    }

    async fn verify_and_add_contact(&mut self, contact: Contact) -> Result<bool> {
        if !self.verify_contact(contact.clone()).await? {
            return Err(ContactVerificationFailed.into());
        }

        self.contacts.insert(contact.identifier().clone(), contact);

        allow()
    }

    async fn verify_and_update_contact(
        &mut self,
        contact_id: &ProfileIdentifier,
        change_events: &[ProfileChangeEvent],
    ) -> Result<bool> {
        let contact = self
            .contacts
            .get_mut(contact_id)
            .ok_or(EntityError::ContactNotFound)
            .expect("contact not found");

        contact
            .verify_and_update(change_events, &mut self.vault)
            .await
    }

    async fn get_lease(
        &self,
        _lease_manager_route: &Route,
        _org_id: String,
        _bucket: String,
        _ttl: TTL,
    ) -> Result<Lease> {
        if let Some(lease) = self.lease.clone() {
            Ok(lease)
        } else {
            Err(InvalidInternalState.into())
        }
    }

    async fn revoke_lease(&mut self, _lease_manager_route: &Route, lease: Lease) -> Result<()> {
        if let Some(existing_lease) = &self.lease {
            if existing_lease == &lease {
                self.lease = None
            }
        }
        Ok(())
    }
}
