use color_eyre::eyre::Result;
use keyring::set_global_service_name;
use tedbot::{
	auth::{self, load_token},
	chat,
	cli::Config,
	install_tracing, new_client, print_channel_info, BOT_NAME,
};
use tracing::debug;

#[tokio::main]
async fn main() -> Result<()> {
	install_tracing();
	color_eyre::install()?;

	set_global_service_name(BOT_NAME);

	let conf = Config::get()?;

	let client = new_client();

	let token = match load_token(client.as_ref(), &conf).await {
		Ok(token) => token,
		Err(_) => {
			debug!("Failed to load token, authenticating");
			auth::twitch_auth(&conf).await?
		}
	};

	print_channel_info(client.as_ref(), conf.username.as_ref(), &token).await?;

	chat::chat(&token, conf.simple_response.clone()).await?;

	Ok(())
}
