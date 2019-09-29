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

#[command]
pub fn set_role(ctx: &mut Context, msg: &Message, args: Args) -> CommandResult {
	set_role_command(ctx, msg, args)?;
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

#[command]
pub fn set_results(ctx: &mut Context, msg: &Message, args: Args) -> CommandResult {
	set_results_command(ctx, msg, args)?;
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

#[command]
pub fn end_vote(ctx: &mut Context, msg: &Message, _args: Args) -> CommandResult {
	end_vote_command(ctx, msg)?;
	Ok(())
}

// REPORTING

#[command]
pub fn voted(ctx: &mut Context, msg: &Message, args: Args) -> CommandResult {
	vote_embed_command(ctx, msg, args)?;
	Ok(())
}

#[command]
pub fn not_voted(ctx: &mut Context, msg: &Message, args: Args) -> CommandResult {
	not_voted_embed_command(ctx, msg, args)?;
	Ok(())
}

// VOTING

#[command]
pub fn yea(ctx: &mut Context, msg: &Message, _args: Args) -> CommandResult {
	yea_command(ctx, msg)?;
	Ok(())
}

#[command]
pub fn nay(ctx: &mut Context, msg: &Message, _args: Args) -> CommandResult {
	nay_command(ctx, msg)?;
	Ok(())
}

#[command]
pub fn abstain(ctx: &mut Context, msg: &Message, _args: Args) -> CommandResult {
	abs_command(ctx, msg)?;
	Ok(())
}

//GROUPS

group!({
    name: "roles",
    options: {
		checks: [Admin]
	},
    commands: [add_speaker, set_role],
});

group!({
	name: "channels",
	options: {
		checks: [Admin]
	},
	commands: [voting_channel, set_title, set_abbr, set_results],
});

group!({
	name: "speaker",
	options: {
		checks: [Speaker]
	},
	commands: [start_vote, set_url, end_vote]
});

group!({
	name: "reporting",
	options: {},
	commands: [voted, not_voted]
});

group!({
	name: "voting",
	options: {},
	commands: [yea, nay, abstain]
});

pub struct Handler;

impl EventHandler for Handler {

	//message saying that the bot is online
	fn ready(&self, _: Context, _ready: Ready) {println!("The government is now open!");}

	fn message(&self, ctx: Context, msg: Message) {

		//checks for votes in voting channels
		match check_msg_for_votes(msg.clone()) {
			Ok(T) => {if T {vote_report(&ctx, &msg.clone());}},
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
	let mut key_file = File::open("files/.key").unwrap();
	key_file.read_to_string(&mut DISCORD_TOKEN).unwrap();

	// Login with the key
	let mut client = Client::new(DISCORD_TOKEN, Handler)
		.expect("Error creating client");
	client.with_framework(StandardFramework::new()
        .configure(|c| c.prefix("~")) // set the bot's prefix to "~"
        .group(&ROLES_GROUP)
		.group(&CHANNELS_GROUP)
		.group(&SPEAKER_GROUP)
		.group(&REPORTING_GROUP)
		.group(&VOTING_GROUP));
	
	// start listening for events by starting a single shard
	if let Err(why) = client.start() {
		println!("An error occurred while running the client: {:?}", why);
	}
}