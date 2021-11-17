use credentials_example::{
    BOB_LISTENER_ADDRESS, BOB_TCP_ADDRESS, ECHOER, GITHUB_SSH_CRED_VERIFIER,
};
use ockam::{
    Context, Entity, Result, Routed, TcpTransport, TrustEveryonePolicy, Vault,
    Worker,
};
use ockam_github::GithubSshAuth;

pub struct Echoer;

#[ockam::worker]
impl Worker for Echoer {
    type Context = Context;
    type Message = String;

    async fn handle_message(&mut self, ctx: &mut Context, msg: Routed<String>) -> Result<()> {
        println!("Address: {}, Received: {}", ctx.address(), msg);

        // Echo the message body back on its return_route.
        ctx.send(msg.return_route(), msg.body()).await
    }
}

#[ockam::node]
async fn main(ctx: Context) -> Result<()> {
    let vault = Vault::create(&ctx).await?;
    let mut bob = Entity::create(&ctx, &vault).await?;

    let mut gh_ssh_auth = GithubSshAuth;

    let registry_address = GithubSshAuth::start_registry(&ctx).await?;

    gh_ssh_auth
        .start_verifier(&ctx,GITHUB_SSH_CRED_VERIFIER.into(), registry_address.clone())
        .await?;

    let access_control
        = gh_ssh_auth.create_access_control(vec!["alice".into()], registry_address).await?;
    ctx.start_worker_with_access_control(ECHOER, Echoer, access_control)
        .await?;

    bob.create_secure_channel_listener(BOB_LISTENER_ADDRESS, TrustEveryonePolicy)
        .await?;

    let tcp = TcpTransport::create(&ctx).await?;
    tcp.listen(BOB_TCP_ADDRESS).await?;

    Ok(())
}
