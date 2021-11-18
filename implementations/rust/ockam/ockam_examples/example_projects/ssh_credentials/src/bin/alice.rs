use credentials_example::{
    BOB_LISTENER_ADDRESS, BOB_TCP_ADDRESS, ECHOER, GITHUB_SSH_CRED_VERIFIER,
};
use ockam::{
    route, Context, Entity, Result, SoftwareVault, TcpTransport, TrustEveryonePolicy, VaultSync,
    TCP,
};
use ockam_github::{GithubSshAuth, Keys};
use ockam_vault_core::{SecretAttributes, SecretPersistence, SecretType, SecretVault};
use std::{env, fs};

#[ockam::node]
async fn main(mut ctx: Context) -> Result<()> {
    let _tcp = TcpTransport::create(&ctx).await?;

    let mut vault = VaultSync::create(&ctx, SoftwareVault::default()).await?;
    let vault_address = vault.address();
    let mut alice = Entity::create(&ctx, &vault_address).await?;

    let gh_nickname = env::var("GH_NICKNAME").unwrap();

    let key_path = env::var("SSH_KEY").unwrap();
    let ssh_key = fs::read_to_string(key_path).unwrap();

    let ssh_key = Keys::extract_raw_ed25519_secret_key(&ssh_key)?;
    let ssh_key = vault
        .secret_import(
            &ssh_key,
            SecretAttributes::new(SecretType::Ed25519, SecretPersistence::Ephemeral, 32),
        )
        .await?;

    // TODO: Should we add such key to the Entity?
    // alice.add_key("GH_SSH".into(), &ssh_key).await?;

    let channel = alice
        .create_secure_channel(
            route![(TCP, BOB_TCP_ADDRESS), BOB_LISTENER_ADDRESS],
            TrustEveryonePolicy,
        )
        .await?;

    let mut gh_ssh_auth = GithubSshAuth::new(&ctx, vault).await?;

    gh_ssh_auth
        .present_credential(
            gh_nickname,
            &ssh_key,
            route![channel.clone(), GITHUB_SSH_CRED_VERIFIER],
        )
        .await?;

    ctx.send(
        route![channel, ECHOER],
        "Hello, Bob! I'm Alice from github".to_string(),
    )
    .await?;
    let msg = ctx.receive::<String>().await?.take().body();
    println!("Echo back: {}", &msg);

    Ok(())
}
