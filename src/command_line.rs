use clap::Parser;

#[derive(Parser)]
#[command(version, about)]
pub struct Args {
	/// Override default config location with the specified path
	#[arg(short, long)]
	pub config_path: Option<String>,
}
