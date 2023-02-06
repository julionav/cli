use anyhow::{bail, ensure, Context, Result};
use clap::Parser;
use console::Term;
use std::path::Path;
use tokio::{fs, io::AsyncWriteExt};

use crate::util::{git, secrets, term};

const RIVET_VERSION_TOML: &'static str = include_str!("../../tpl/rivet.version.toml");

#[derive(Parser)]
pub struct Opts {
	/// Use recommended settings
	#[clap(long)]
	recommended: bool,

	/// Update gitignore
	#[clap(long)]
	gitignore: bool,

	/// Create Rivet config
	#[clap(long)]
	rivet_config: bool,

	/// Setup development token
	#[clap(long)]
	dev: bool,

	#[clap(flatten)]
	dev_opts: crate::commands::dev::InitOpts,
}

impl Opts {
	pub async fn execute(
		&self,
		cloud_token: Option<&str>,
		term: &Term,
		override_api_url: Option<String>,
	) -> Result<()> {
		// Check if token already exists
		eprintln!();
		let cloud_token = if let Some(cloud_token) = cloud_token.clone() {
			Some(cloud_token.to_string())
		} else {
			secrets::read_cloud_token().await?
		};
		let ctx = if let Some(cloud_token) = cloud_token {
			let ctx = cli_core::ctx::init(override_api_url.clone(), cloud_token).await?;

			let game_res = ctx
				.client()
				.get_game_by_id()
				.game_id(&ctx.game_id)
				.send()
				.await
				.context("client.get_game_by_id")?;
			let game = game_res.game().context("game_res.game")?;
			let display_name = game.display_name().context("game.display_name")?;

			term::status::success("Found existing token", display_name);

			ctx
		} else {
			read_cloud_token(term, override_api_url.clone()).await?
		};

		// Update .gitignore
		eprintln!();
		if !git::check_ignore(Path::new(".rivet/")).await? {
			if self.recommended
				|| self.gitignore
				|| term::input::bool(term, "Add .rivet/ to .gitignore?").await?
			{
				let mut file = fs::OpenOptions::new()
					.write(true)
					.append(true)
					.open(".gitignore")
					.await?;
				file.write_all(b"\n### Rivet ###\n.rivet/\n.env\n").await?;

				ensure!(
					git::check_ignore(Path::new(".rivet/")).await?,
					"updated gitignore does not ignore Rivet files"
				);

				term::status::success("Finished", "Git will now ignore the .rivet/ folder.");
			}
		} else {
			term::status::success(
				".gitignore already configured",
				"The .rivet/ folder is already ignored by Git.",
			);
		}

		// Create rivet.version.toml
		eprintln!();
		let config_path = std::env::current_dir()?.join("rivet.version.toml");
		let config_needs_creation = match fs::read_to_string(&config_path).await {
			Ok(_) => false,
			Err(err) if err.kind() == std::io::ErrorKind::NotFound => true,
			Err(err) => {
				return Err(err.into());
			}
		};
		if config_needs_creation {
			if self.recommended
				|| self.rivet_config
				|| term::input::bool(term, "Create default config at rivet.version.toml?").await?
			{
				fs::write(config_path, RIVET_VERSION_TOML).await?;

				term::status::success(
					"Finished",
					"Rivet Matchmaker and Rivet CDN will be enabled next time you deploy.",
				);
			}
		} else {
			term::status::success(
				"Version already configured",
				"Your game is already configured with rivet.version.toml.",
			);
		}

		// Development flow
		eprintln!();
		if self.recommended
			|| self.dev || term::input::bool(term, "Setup development environment?").await?
		{
			self.dev_opts.execute(term, &ctx).await?
		}

		eprintln!();
		term::status::success(
			"What's next?",
			"https://docs.rivet.gg/general/guides/crash-course",
		);

		Ok(())
	}
}

async fn read_cloud_token(term: &Term, override_api_url: Option<String>) -> Result<cli_core::Ctx> {
	let token = term::input::secure(term, "Rivet cloud token?").await?;

	// Create new context
	let new_ctx = cli_core::ctx::init(
		override_api_url,
		// Exclude overridden access token to check the token
		token.clone(),
	)
	.await?;
	let inspect = new_ctx
		.client()
		.inspect()
		.send()
		.await
		.context("client.inspect()")?;

	let game_id = match inspect.agent.as_ref().context("inspect.agent")? {
		cli_core::rivet_cloud::model::AuthAgent::GameCloud(game_cloud) => {
			game_cloud.game_id.clone().context("game_cloud.game_id")?
		}
		_ => bail!("invalid agent kind"),
	};

	let game_res = new_ctx
		.client()
		.get_game_by_id()
		.game_id(game_id)
		.send()
		.await
		.context("client.get_game_by_id()")?;
	let game = game_res.game().context("game_res.game")?;
	let display_name = game.display_name().context("game.display_name")?;

	// Write the token
	secrets::write_cloud_token(&token).await?;

	term::status::success("Token Saved", display_name);

	Ok(new_ctx)
}

pub fn dashboard_api_url(game_id: &str) -> String {
	format!("https://hub.rivet.gg/developer/games/{game_id}/api")
}
