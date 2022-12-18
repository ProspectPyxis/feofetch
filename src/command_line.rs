use clap::Parser;

#[derive(Parser)]
#[command(version, about)]
pub struct Args {
	/// Override default config location with the specified path
	#[arg(short, long)]
	pub config_path: Option<String>,
	/// Set x-offset to this number, overriding config
	#[arg(short, long)]
	pub x_offset: Option<usize>,
	/// Set y-offset to this number, overriding config
	#[arg(short, long)]
	pub y_offset: Option<usize>,
}
