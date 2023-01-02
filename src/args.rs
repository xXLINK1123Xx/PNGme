use clap::{Parser, command, Subcommand, Args};


#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(next_line_help = true)]
#[command(propagate_version = true)]
pub struct MyArgs {
	#[command(subcommand)]
	pub commands: Commands
}

#[derive(Subcommand)]
pub enum Commands {
	Encode(EncodeCommand),
	Decode(DecodeCommand),
	Remove(RemoveCommand),
	Print(PrintCommand)
}

#[derive(Args, Debug)]
pub struct EncodeCommand {
	pub image_path: String,
	pub chunk_type: String,
	pub message: String,
	pub output_file: Option<String>
}

#[derive(Args, Debug)]
pub struct DecodeCommand {
	pub image_path: String,
	pub chunk_type: String
}

#[derive(Args, Debug)]
pub struct RemoveCommand {
	pub image_path: String,
	pub chunk_type: String
}

#[derive(Args, Debug)]
pub struct PrintCommand {
	pub image_path: String
}