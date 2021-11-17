use credentials_example::{BOB_LISTENER_ADDRESS, BOB_TCP_ADDRESS, ECHOER, GITHUB_SSH_CRED_VERIFIER};
use ockam::{route, Context, Entity, Result, TcpTransport, TrustEveryonePolicy, Vault, TCP, Identity};
use ockam_github::GithubSshAuth;

#[ockam::node]
async fn main(mut ctx: Context) -> Result<()> {
    let _tcp = TcpTransport::create(&ctx).await?;

    let vault = Vault::create(&ctx).await?;
    let mut alice = Entity::create(&ctx, &vault).await?;
    alice.create_key("GH_SSH".into()).await?;
    // TODO: Replace with alice.create_key("GH_SSH", "/Users/sanjo/.ssh/sanjomail").await?;

    let channel = alice
        .create_secure_channel(
            route![(TCP, BOB_TCP_ADDRESS), BOB_LISTENER_ADDRESS],
            TrustEveryonePolicy,
        )
        .await?;

    let mut gh_ssh_auth = GithubSshAuth;
    gh_ssh_auth
        .present_credential(
            &mut alice,
            "GH_SSH".into(),
            route![channel, GITHUB_SSH_CRED_VERIFIER],
        )
        .await?;

    ctx.send(route![channel, ECHOER], "Hello, Bob! I'm Alice from github").await?;
    let msg = ctx.receive::<String>().await?.take().body();

    assert_eq!(msg, "Hello, Bob! I'm Alice from github");

    Ok(())
}
