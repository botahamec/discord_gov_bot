#![allow(non_snake_case)]

// IMPORTS

//project imports
mod stdfn; use stdfn::*;

//discord imports
extern crate serenity;
use serenity::{
	model::{
		channel::Message,
		gateway::Ready,
		guild::{
			Guild,
		},
	},
	prelude::{
		EventHandler,
		Context,
	},
	client::Client,
	framework::standard::{
		StandardFramework,
		Args,
		CommandResult,
		macros::{
			command,
			group,
		},
	},
};

use std::fs::File;
use std::io::Read;

// COMMANDS

//ROLES

#[command]
pub fn add_speaker(ctx: &mut Context, msg: &Message, args: Args) -> CommandResult {
	add_speaker_command(ctx, msg, args)?;
    Ok(())
}

//CHANNELS

#[command]
pub fn voting_channel(ctx: &mut Context, msg: &Message, _args: Args) -> CommandResult {
	add_voting_channel_command(ctx, msg)?;
	Ok(())
}

#[command]
pub fn set_title(ctx: &mut Context, msg: &Message, args: Args) -> CommandResult {
	set_title_command(ctx, msg, args)?;
	Ok(())
}

#[command]
pub fn set_abbr(ctx: &mut Context, msg: &Message, args: Args) -> CommandResult {
	set_abbr_command(ctx, msg, args)?;
	Ok(())
}

// SPEAKER

#[command]
pub fn start_vote(ctx: &mut Context, msg: &Message, args: Args) -> CommandResult {
	start_vote_command(ctx, msg, args)?;
	Ok(())
}

#[command]
pub fn set_url(ctx: &mut Context, msg: &Message, args: Args) -> CommandResult {
	set_url_command(ctx, msg, args)?;
	Ok(())
}

// REPORTING

pub fn vote(ctx: &mut Context, msg: &Message, _args: Args) -> CommandResult {
	vote_command(ctx, msg)?;
	Ok(())
}

//GROUPS

group!({
    name: "roles",
    options: {},
    commands: [add_speaker],
});

group!({
	name: "channels",
	options: {},
	commands: [voting_channel, set_title, set_abbr],
});

group!({
	name: "speaker",
	options: {},
	commands: [start_vote, set_url]
});

group!({
	name: "reporting",
	options: {},
	commands: []
});

pub struct Handler;

impl EventHandler for Handler {

	//message saying that the bot is online
	fn ready(&self, _: Context, _ready: Ready) {println!("The government is now open!");}

	fn message(&self, _ctx: Context, msg: Message) {

		//checks for votes in voting channels
		match check_msg_for_votes(msg) {
			Ok(T) => T,
			Err(E) => println!("{}", E)
		};
	}

	fn guild_create(&self, _ctx: Context, guild: Guild, _is_new: bool) {
		let id = guild.id.0;
		match add_server(id) {
			Ok(_T) => println!("The server with id: {}, was successfully added!", id),
			Err(E) => println!("An error occurred when adding a server with id: {}\n{}", id, E)
		};
	}
}

fn main() {

	// Get key from an external file
	let mut DISCORD_TOKEN = String::new();
	let mut key_file = File::open(".key").unwrap();
	key_file.read_to_string(&mut DISCORD_TOKEN).unwrap();
	println!("{}", DISCORD_TOKEN);

	// Login with the key
	let mut client = Client::new(DISCORD_TOKEN, Handler)
		.expect("Error creating client");
	println!("Created client");
	client.with_framework(StandardFramework::new()
        .configure(|c| c.prefix("~")) // set the bot's prefix to "~"
        .group(&ROLES_GROUP)
		.group(&CHANNELS_GROUP)
		.group(&SPEAKER_GROUP)
		.group(&REPORTING_GROUP));
	println!("Created framework");
	
	// start listening for events by starting a single shard
	if let Err(why) = client.start() {
		println!("An error occurred while running the client: {:?}", why);
	}
	println!("Something happened...");
}