//project imports
mod filefn; use filefn::*;

//discord imports
extern crate serenity;
use serenity::{
	model::channel::Message,
	prelude::Context,
	framework::standard::Args,
};

//stdlib imports
use std::io::{
	Result, 
	Error,
	ErrorKind,
};

pub fn add_speaker_command(ctx: &mut Context, msg: &Message, args: Args) -> Result<()> {

	let potential_role_name = args.rest();

    if let Some(guild) = msg.guild(&ctx.cache) {
        // `role_by_name()` allows us to attempt attaining a reference to a role
        // via its name.
        if let Some(role) = guild.read().role_by_name(&potential_role_name) {
			add_to_file(guild_file(guild.read().id.0, "speaker_roles"), format!("{}", role.id.0))?;
            if let Err(e) = msg.channel_id.say(&ctx.http, &format!("The role, {} now has speaker permissions", potential_role_name)) {
				println!("Couldn't send message, \n {}", e);
			}
            return Ok(());
        }

		//if let Some(role) = guild.read().roles.keys().map(|r| r.0) {
		//	msg.channel_id.say(&ctx.http, &format!("The role, <@{}> now has speaker permissions", potential_role_name))?;
		//	return Ok(())
		//}
    }

    if let Err(why) = msg.channel_id.say(&ctx.http, format!("Could not find role named: {:?}", potential_role_name)) {
        println!("Error sending message: {:?}", why);
    }

    Ok(())
}

pub fn add_voting_channel_command(ctx: &mut Context, msg: &Message) -> Result<()> {
	
	//find id's
	let guild_id = match msg.guild_id {
		Some(i) => i.0,
		None => return Err(Error::new(ErrorKind::NotFound, "No Guild ID found"))
	};
	let channel_id = msg.channel_id.0;

	create_vote_channel_file(guild_id, channel_id, "abbr")?;
	create_vote_channel_file(guild_id, channel_id, "yeas")?;
	create_vote_channel_file(guild_id, channel_id, "nays")?;
	create_vote_channel_file(guild_id, channel_id, "abst")?;
	create_vote_channel_file(guild_id, channel_id, "novs")?;
	create_vote_channel_file(guild_id, channel_id, "bill")?;
	create_vote_channel_file(guild_id, channel_id, "url")?;
	create_vote_channel_file(guild_id, channel_id, "title")?;

	add_to_file(guild_file(guild_id, "voting_channels"), format!("{}", channel_id))?; //add to list of voting channels

	//report that it's done
	if let Err(e) = msg.channel_id.say(&ctx.http, &format!("The channel now is a voting channel",)) {
		println!("Couldn't send message, \n {}", e);
	};
	Ok(())
}

pub fn set_title_command(ctx: &mut Context, msg: &Message, args: Args) -> Result<()> {

	let title = args.rest();

	let guild_id = match msg.guild_id {
		Some(i) => i.0,
		None => return Err(Error::new(ErrorKind::NotFound, "No Guild ID found"))
	};
	let channel_id = msg.channel_id.0;

	add_to_file(voting_channel_file(guild_id, channel_id, "title"), String::from(title))?;
	if let Err(e) = msg.channel_id.say(&ctx.http, &format!("The title for members of this channel has changed")) {
		println!("Couldn't send message, \n {}", e);
	};
	Ok(())
}

pub fn set_abbr_command(ctx: &mut Context, msg: &Message, args: Args) -> Result<()> {
	let abbr = args.rest();
	let guild_id = match msg.guild_id {
		Some(i) => i.0,
		None => return Err(Error::new(ErrorKind::NotFound, "No Guild ID found"))
	};
	let channel_id = msg.channel_id.0;
	write_to_file(voting_channel_file(guild_id, channel_id, "abbr"), String::from(abbr))?;
	if let Err(e) = msg.channel_id.say(&ctx.http, &format!("The abbreviation for this channel has changed")) {
		println!("Couldn't send message, \n {}", e);
	};
	Ok(())
}

pub fn start_vote_command(ctx: &mut Context, msg: &Message, args: Args) -> Result<()> {

	let vote = args.rest();

	let guild_id = match msg.guild_id {
		Some(i) => i.0,
		None => return Err(Error::new(ErrorKind::NotFound, "No Guild ID found"))
	};
	let channel_id = msg.channel_id.0;

	write_to_file(voting_channel_file(guild_id, channel_id, "vote"), String::from(vote))?;
	if let Err(e) = msg.channel_id.say(&ctx.http, &format!("Voting has started on {}", vote)) {
		println!("Couldn't send message, \n {}", e);
	};
	Ok(())
}

pub fn set_url_command(ctx: &mut Context, msg: &Message, args: Args) -> Result<()> {
	
	let url = args.rest();

	let guild_id = match msg.guild_id {
		Some(i) => i.0,
		None => return Err(Error::new(ErrorKind::NotFound, "No Guild ID found"))
	};
	let channel_id = msg.channel_id.0;

	write_to_file(voting_channel_file(guild_id, channel_id, "url"), String::from(url))?;
	if let Err(e) = msg.channel_id.say(&ctx.http, &format!("The URL has changed")) {
		println!("Couldn't send message, \n {}", e);
	};
	Ok(())
}

pub fn vote_command(ctx: &mut Context, msg: &Message) -> Result<()> {
	let guild_id = match msg.guild_id {
		Some(i) => i.0,
		None => return Err(Error::new(ErrorKind::NotFound, "No Guild ID found"))
	};
	let channel_id = msg.channel_id.0;

	let yeas = match vec_from_file(voting_channel_file(guild_id, channel_id, "yeas")) {
		Ok(i) => i.len(),
		Err(e) => return Err(Error::new(ErrorKind::InvalidData, e))
	};
	let nays = match vec_from_file(voting_channel_file(guild_id, channel_id, "nays")) {
		Ok(i) => i.len(),
		Err(e) => return Err(Error::new(ErrorKind::InvalidData, e))
	};
	let abst = match vec_from_file(voting_channel_file(guild_id, channel_id, "abst")) {
		Ok(i) => i.len(),
		Err(e) => return Err(Error::new(ErrorKind::InvalidData, e))
	};

	let report = format!("{}-{}-{}", yeas, nays, abst);
	if let Err(e) = msg.channel_id.say(&ctx.http, &report) {
		println!("Couldn't send message, \n {}", e);
	};
	Ok(())
}

///looks at a message to see if it a vote and processes the vote if necessary
//TODO: add logic to handle the vote
pub fn check_msg_for_votes(msg: Message) -> Result<()> {

	let guild_id = msg.guild_id.unwrap().0;

	//checks if the channel is a voting channel
	if file_contains(guild_file(guild_id, "vote_channels"), format!("{}", msg.channel_id.0)).unwrap() {
		let vote = msg.content;
		if file_contains(guild_file(guild_id, "yeas"), vote.clone()).unwrap() {unimplemented!("Yea cast");}
		if file_contains(guild_file(guild_id, "nays"), vote.clone()).unwrap() {unimplemented!("Nay cast");}
		if file_contains(guild_file(guild_id, "abst"), vote.clone()).unwrap() {unimplemented!("Abstain!");}
	}

	Ok(())
}

///adds the data of a new server to the database
pub fn add_server(guild: u64) -> Result<()> {

	//sets up directory structure
	make_dir(format!("data/{}", guild))?; //creates the folder for the server
	create_dir(guild, "votes")?; //information on current votes

	//creates the necessary files
	create_file(guild, "voting_channels")?; //list of voting channels
	create_file(guild, "speaker_roles")?;
	copy_file(guild, "yeas");
	copy_file(guild, "nays");
	copy_file(guild, "abst");

	Ok(())
}