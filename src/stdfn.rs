//project imports
mod filefn; use filefn::*;

//discord imports
extern crate serenity;
use serenity::{
	model::channel::Message,
	model::id::ChannelId,
	prelude::Context,
	framework::standard::{
		CheckResult,
		CommandOptions,
		Args,
		macros::check
	},
};

//stdlib imports
use std::{
	io::{
		Result, 
		Error,
		ErrorKind,
	}
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

pub fn set_role_command(ctx: &mut Context, msg: &Message, args: Args) -> Result<()> {
	let potential_role_name = args.rest();

	let guild_id = match msg.guild_id {
		Some(i) => i.0,
		None => return Err(Error::new(ErrorKind::NotFound, "No Guild ID found"))
	};
	let channel_id = msg.channel_id.0;
	
    if let Some(guild) = msg.guild(&ctx.cache) {
        // `role_by_name()` allows us to attempt attaining a reference to a role
        // via its name.
        if let Some(role) = guild.read().role_by_name(&potential_role_name) {
			write_to_file(voting_channel_file(guild_id, channel_id, "role"), format!("{}", role.id.0))?;
            if let Err(e) = msg.channel_id.say(&ctx.http, &format!("The role, {} is now a member of this channel", potential_role_name)) {
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

	// creates directory for the channel
	create_dir(guild_id, format!("votes/{}", channel_id))?;

	create_vote_channel_file(guild_id, channel_id, "abbr")?;
	create_vote_channel_file(guild_id, channel_id, "yeas")?;
	create_vote_channel_file(guild_id, channel_id, "nays")?;
	create_vote_channel_file(guild_id, channel_id, "abst")?;
	create_vote_channel_file(guild_id, channel_id, "novs")?;
	create_vote_channel_file(guild_id, channel_id, "bill")?;
	create_vote_channel_file(guild_id, channel_id, "url")?;
	create_vote_channel_file(guild_id, channel_id, "title")?;
	create_vote_channel_file(guild_id, channel_id, "role")?;
	create_vote_channel_file(guild_id, channel_id, "results")?;

	add_to_file(guild_file(guild_id, "voting_channels"), format!("{}", channel_id))?; //add to list of voting channels

	//report that it's done
	if let Err(e) = msg.channel_id.say(&ctx.http, "The channel now is a voting channel".to_string()) {
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

	write_to_file(voting_channel_file(guild_id, channel_id, "title"), String::from(title))?;
	if let Err(e) = msg.channel_id.say(&ctx.http, "The title for members of this channel has changed".to_string()) {
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
	if let Err(e) = msg.channel_id.say(&ctx.http, "The abbreviation for this channel has changed".to_string()) {
		println!("Couldn't send message, \n {}", e);
	};
	Ok(())
}

pub fn set_results_command(ctx: &mut Context, msg: &Message, args: Args) -> Result<()> {
	let new_results = args.rest();
	let guild_id = match msg.guild_id {
		Some(i) => i.0,
		None => return Err(Error::new(ErrorKind::NotFound, "No Guild ID found"))
	};
	let channel_id = msg.channel_id.0;

	match new_results.parse::<u64>() {
		Ok(_i) => {
			write_to_file(voting_channel_file(guild_id, channel_id, "results"), String::from(new_results))?;
			if let Err(e) = msg.channel_id.say(&ctx.http, &format!("The results of votes in this channel will be posted to <#{}>", new_results)) {
				println!("Couldn't send message, \n {}", e);
			};
		},
		Err(_e) => {
			if let Err(e) = msg.channel_id.say(&ctx.http, "You must provide a channel id for which channel will be the results channel for this voting channel.".to_string()) {
				println!("Couldn't send message, \n {}", e);
			};
		}
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
	let role_id = str_from_file(voting_channel_file(guild_id, channel_id, "role")).unwrap();

	let guild = msg.guild(&ctx.cache).unwrap();
	let guild = guild.read();
	let members = &guild.members;

	for member in members.values() {
		if member.roles.iter().any(|r| role_id == format!("{}", r.0)) {
			let user = &member.user.read();
			add_to_file(voting_channel_file(guild_id, channel_id, "novs"), user.name.clone())?;
		}
	}

	write_to_file(voting_channel_file(guild_id, channel_id, "bill"), String::from(vote))?;
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
	if let Err(e) = msg.channel_id.say(&ctx.http, "The URL has changed".to_string()) {
		println!("Couldn't send message, \n {}", e);
	};
	Ok(())
}

pub fn end_vote_command(ctx: &mut Context, msg: &Message) -> Result<()> {

	let guild_id = match msg.guild_id {
		Some(i) => i.0,
		None => return Err(Error::new(ErrorKind::NotFound, "No Guild ID found"))
	};
	let channel_id = msg.channel_id.0;

	let yeas = match vec_from_file(voting_channel_file(guild_id, channel_id, "yeas")) {
		Ok(i) => i,
		Err(e) => return Err(Error::new(ErrorKind::InvalidData, e))
	};
	let nays = match vec_from_file(voting_channel_file(guild_id, channel_id, "nays")) {
		Ok(i) => i,
		Err(e) => return Err(Error::new(ErrorKind::InvalidData, e))
	};
	let abst = match vec_from_file(voting_channel_file(guild_id, channel_id, "abst")) {
		Ok(i) => i,
		Err(e) => return Err(Error::new(ErrorKind::InvalidData, e))
	};
	let novs = match vec_from_file(voting_channel_file(guild_id, channel_id, "novs")) {
		Ok(i) => i,
		Err(e) => return Err(Error::new(ErrorKind::InvalidData, e))
	};

	let mut yeas_str = yeas.join("\n");
	let mut nays_str = nays.join("\n");
	let mut abst_str = abst.join("\n");
	let mut novs_str = novs.join("\n");

	if yeas.is_empty() {yeas_str = String::from("N/A");}
	if nays.is_empty() {nays_str = String::from("N/A");}
	if abst.is_empty() {abst_str = String::from("N/A");}
	if novs.is_empty() {novs_str = String::from("N/A");}

	let results_str = str_from_file(voting_channel_file(guild_id, channel_id, "results"))?;
	let results = ChannelId::from(results_str.parse::<u64>().unwrap());

	let _msg = results.send_message(&ctx.http, |m| {
		m.content("");
		m.embed(|e| {
			e.title(str_from_file(voting_channel_file(guild_id, channel_id, "bill")).unwrap());
			e.description(str_from_file(voting_channel_file(guild_id, channel_id, "title")).unwrap());
			e.url(str_from_file(voting_channel_file(guild_id, channel_id, "url")).unwrap());
			e.fields(vec![
				(format!("Yeas - {}", yeas.len()), yeas_str, true),
				(format!("Nays - {}", nays.len()), nays_str, true),
				(format!("Abstains - {}", abst.len()), abst_str, true),
				(format!("Not Voting - {}", novs.len()), novs_str, true)
			]);
			e.footer(|f| {
				f.text("Consider supporting Botahamec by donating either to this project or Patreon");
				f
			});
			e
		});
		m
	});

	write_to_file(voting_channel_file(guild_id, channel_id, "yeas"), String::from(""))?;
	write_to_file(voting_channel_file(guild_id, channel_id, "nays"), String::from(""))?;
	write_to_file(voting_channel_file(guild_id, channel_id, "abst"), String::from(""))?;
	write_to_file(voting_channel_file(guild_id, channel_id, "novs"), String::from(""))?;
	write_to_file(voting_channel_file(guild_id, channel_id, "bill"), String::from(""))?;
	write_to_file(voting_channel_file(guild_id, channel_id, "url"), String::from(""))?;

	Ok(())
}

pub fn vote_embed_command(ctx: &mut Context, msg: &Message, args: Args) -> Result<()> {
	let guild_id = match msg.guild_id {
		Some(i) => i.0,
		None => return Err(Error::new(ErrorKind::NotFound, "No Guild ID found"))
	};
	let channel_id = channel_from_abbr(guild_id, String::from(args.rest()))?;

	let yeas = match vec_from_file(voting_channel_file(guild_id, channel_id, "yeas")) {
		Ok(i) => i,
		Err(e) => return Err(Error::new(ErrorKind::InvalidData, e))
	};
	let nays = match vec_from_file(voting_channel_file(guild_id, channel_id, "nays")) {
		Ok(i) => i,
		Err(e) => return Err(Error::new(ErrorKind::InvalidData, e))
	};
	let abst = match vec_from_file(voting_channel_file(guild_id, channel_id, "abst")) {
		Ok(i) => i,
		Err(e) => return Err(Error::new(ErrorKind::InvalidData, e))
	};

	let mut yeas_str = yeas.join("\n");
	let mut nays_str = nays.join("\n");
	let mut abst_str = abst.join("\n");

	if yeas.is_empty() {yeas_str = String::from("N/A");}
	if nays.is_empty() {nays_str = String::from("N/A");}
	if abst.is_empty() {abst_str = String::from("N/A");}

	let msg = msg.channel_id.send_message(&ctx.http, |m| {
		m.content("");
		m.embed(|e| {
			e.title(str_from_file(voting_channel_file(guild_id, channel_id, "bill")).unwrap());
			e.description(str_from_file(voting_channel_file(guild_id, channel_id, "title")).unwrap());
			e.url(str_from_file(voting_channel_file(guild_id, channel_id, "url")).unwrap());
			e.fields(vec![
				(format!("Yeas - {}", yeas.len()), yeas_str, true),
				(format!("Nays - {}", nays.len()), nays_str, true),
				(format!("Abstains - {}", abst.len()), abst_str, true),
			]);
			e.footer(|f| {
				f.text("Consider supporting Botahamec by donating either to this project or Patreon");
				f
			});
			e
		});
		m
	});

	if let Err(why) = msg {
    	println!("Error sending message: {:?}", why);
	}

	Ok(())
}

pub fn not_voted_embed_command(ctx: &mut Context, msg: &Message, args: Args) -> Result<()> {
	let guild_id = match msg.guild_id {
		Some(i) => i.0,
		None => return Err(Error::new(ErrorKind::NotFound, "No Guild ID found"))
	};
	let channel_id = channel_from_abbr(guild_id, String::from(args.rest()))?;

	let novs = match vec_from_file(voting_channel_file(guild_id, channel_id, "novs")) {
		Ok(i) => i,
		Err(e) => return Err(Error::new(ErrorKind::InvalidData, e))
	};
	let novs_str = if novs.is_empty() {String::from("N/A")} else {novs.join("\n")};

	let msg = msg.channel_id.send_message(&ctx.http, |m| {
		m.content("");
		m.embed(|e| {
			e.title(str_from_file(voting_channel_file(guild_id, channel_id, "bill")).unwrap());
			e.description(str_from_file(voting_channel_file(guild_id, channel_id, "title")).unwrap());
			e.url(str_from_file(voting_channel_file(guild_id, channel_id, "url")).unwrap());
			e.fields(vec![(format!("Not Voting - {}", novs.len()), novs_str, true)]);
			e.footer(|f| {
				f.text("Consider supporting Botahamec by donating either to this project or Patreon");
				f
			});
			e
		});
		m
	});

	if let Err(why) = msg {
    	println!("Error sending message: {:?}", why);
	}

	Ok(())
}

pub fn vote_report(ctx: &Context, msg: &Message) -> Result<()> {

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

pub fn vote_yea(name: String, channel: u64, guild: u64) -> Result<()> {
	remove_from_file(voting_channel_file(guild, channel, "yeas"), name.clone())?;
	remove_from_file(voting_channel_file(guild, channel, "nays"), name.clone())?;
	remove_from_file(voting_channel_file(guild, channel, "abst"), name.clone())?;
	remove_from_file(voting_channel_file(guild, channel, "novs"), name.clone())?;
	add_to_file(voting_channel_file(guild, channel, "yeas"), name.clone())?;
	Ok(())
}

pub fn vote_nay(name: String, channel: u64, guild: u64) -> Result<()> {
	remove_from_file(voting_channel_file(guild, channel, "yeas"), name.clone())?;
	remove_from_file(voting_channel_file(guild, channel, "nays"), name.clone())?;
	remove_from_file(voting_channel_file(guild, channel, "abst"), name.clone())?;
	remove_from_file(voting_channel_file(guild, channel, "novs"), name.clone())?;
	add_to_file(voting_channel_file(guild, channel, "nays"), name.clone())?;
	Ok(())
}

pub fn vote_abs(name: String, channel: u64, guild: u64) -> Result<()> {
	remove_from_file(voting_channel_file(guild, channel, "yeas"), name.clone())?;
	remove_from_file(voting_channel_file(guild, channel, "nays"), name.clone())?;
	remove_from_file(voting_channel_file(guild, channel, "abst"), name.clone())?;
	remove_from_file(voting_channel_file(guild, channel, "novs"), name.clone())?;
	add_to_file(voting_channel_file(guild, channel, "abst"), name.clone())?;
	Ok(())
}

pub fn yea_command(ctx: &mut Context, msg: &Message) -> Result<()> {
	let channel = msg.channel_id.0;
	let guild = msg.guild_id.unwrap().0;
	let name = msg.author.name.clone();
	vote_yea(name, channel, guild)?;
	vote_report(ctx, msg)?;
	Ok(())
}

pub fn nay_command(ctx: &mut Context, msg: &Message) -> Result<()> {
	let channel = msg.channel_id.0;
	let guild = msg.guild_id.unwrap().0;
	let name = msg.author.name.clone();
	vote_nay(name, channel, guild)?;
	vote_report(ctx, msg)?;
	Ok(())
}

pub fn abs_command(ctx: &mut Context, msg: &Message) -> Result<()> {
	let channel = msg.channel_id.0;
	let guild = msg.guild_id.unwrap().0;
	let name = msg.author.name.clone();
	vote_abs(name, channel, guild)?;
	vote_report(ctx, msg)?;
	Ok(())
}

///looks at a message to see if it a vote and processes the vote if necessary
pub fn check_msg_for_votes(msg: Message) -> Result<bool> {

	let guild_id = msg.guild_id.unwrap().0;
	let mut boolean : bool = false; //used to determine whether a vote was sent

	//checks if the channel is a voting channel
	if file_contains(guild_file(guild_id, "voting_channels"), format!("{}", msg.channel_id.0)).unwrap() {
		let vote = msg.content;
		if file_contains(guild_file(guild_id, "yeas"), vote.clone()).unwrap() {
			vote_yea(msg.author.name.clone(), msg.channel_id.0, guild_id)?;
			boolean = true;
		}
		if file_contains(guild_file(guild_id, "nays"), vote.clone()).unwrap() {
			vote_nay(msg.author.name.clone(), msg.channel_id.0, guild_id)?;
			boolean = true;
		}
		if file_contains(guild_file(guild_id, "abst"), vote.clone()).unwrap() {
			vote_abs(msg.author.name.clone(), msg.channel_id.0, guild_id)?;
			boolean = true
		}
	}

	Ok(boolean)
}

///adds the data of a new server to the database
pub fn add_server(guild: u64) -> Result<()> {

	if !file_contains(String::from("data/servers.txt"), format!("{}", guild)).unwrap() {
		
		//sets up directory structure
		make_dir(format!("data/servers/{}", guild))?; //creates the folder for the server
		create_dir(guild, String::from("votes"))?; //information on current votes

		//creates the necessary files
		create_file(guild, "voting_channels")?; //list of voting channels
		create_file(guild, "speaker_roles")?;
		copy_file(guild, "yeas")?;
		copy_file(guild, "nays")?;
		copy_file(guild, "abst")?;

		add_to_file(String::from("data/servers.txt"), format!("{}", guild))?;
	}

	Ok(())
}

#[check]
#[name = "Admin"]
fn admin_check(ctx: &mut Context, msg: &Message, _: &mut Args, _: &CommandOptions) -> CheckResult {
    if let Some(member) = msg.member(&ctx.cache) {

        if let Ok(permissions) = member.permissions(&ctx.cache) {
            return permissions.administrator().into();
        }
    }

    false.into()
}

#[check]
#[name = "Speaker"]
fn speacker_check(ctx: &mut Context, msg: &Message, _: &mut Args, _: &CommandOptions) -> CheckResult {
    if let Some(member) = msg.member(&ctx.cache) {

        let speaker_roles = vec_from_file(guild_file(msg.guild_id.unwrap().0, "speaker_roles")).unwrap();
		for role in speaker_roles {
			if member.roles.iter().any(|r| role == format!("{}", r.0)) {
				return true.into(); 
			}
		}
    }

    false.into()
}

#[check]
#[name = "Developer"]
fn dev_check(ctx: &mut Context, msg: &Message, _: &mut Args, _: &CommandOptions) -> CheckResult {
	if let Some(member) = msg.member(&ctx.cache) {
		if member.user.read().id.0 == 345_338_635_148_591_107 {
			return true.into();
		}
	}

	false.into()
}