use discord_rich_presence::{activity, DiscordIpc, DiscordIpcClient};

pub fn rpc() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = DiscordIpcClient::new("815532220189376512")?;

    client.connect()?;
    client.set_activity(activity::Activity::new()
        .state("foo")
        .details("bar")
    )?;
    client.close()?;

    Ok(())
}