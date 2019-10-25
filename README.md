<p align="center">
  <a href="" rel="noopener">
 <img width=200px height=200px src="https://cdn.discordapp.com/attachments/533798667358830592/628085799291977728/capitol.png" alt="Capitol logo"></a>
</p>

<h3 align="center">Capitol</h3>

---

<p align="center"> This is a bot to simulate all of the functions of a mock-government. Using this bot, you can turn your own server into a virtual Congress.
    <br> 
</p>

## 📝 Table of Contents
+ [About](#about)
+ [Demo / Working](#demo)
+ [How it works](#working)
+ [Getting Started](#getting_started)
+ [Built Using](#built_using)
+ [Authors](#authors)
+ [Acknowledgments](#acknowledgement)

## 🧐 About <a name = "about"></a>
Do you want a great government discord server like Let's Talk Elections, American Government Simulator, and the like? With Capitol, you can achieve exactly this. Just plop the bot into the site, set all of the settings, and let it go.

The bot is not a replacement for a Speaker of the House, but is a helper. It allows the members of Congress to vote in specific channels using commans, and the bot will count the votes as it goes. Once the voting is finished, Capitol will put the result into a channel for everyone to see.

## 🎥 Demo / Working <a name = "demo"></a>
![Working](https://media.giphy.com/media/20NLMBm0BkUOwNljwv/giphy.gif)

*This will stay here until I've made a GIF of my bot in action*

## 💭 How it works <a name = "working"></a>

To start, set your channel as a voting channel using the ~voting_channel command. Change all the settings using the ~add_speaker, ~set_role, ~set_abbr, ~set_title, and ~set_results commands. The ~add_speaker command gives a particular role speaker permission in the channel. The ~set_role command gives which role will be voting in the channel (Anyone with access to the channel can vote, this is used to check who hasn't voted yet). The ~set_abbr command sets the abbreviation of the channel. This will be used to check the results during the vote. The ~set_title command is used to set the name of the voting channel. This will be displayed in the results. The ~set_results command sets the channel where results should be posted once a vote is finished.

When you're ready to begin voting, the speaker must use the ~start_vote command. Optionally, they may also use the ~set_url command to give a link to the thing they are voting on. Voters will vote using the yea, nay, and abstain commands, or by just typing one of those three words (among others) to the chat.

Currently, the bot is running on a Raspberry Pi 4, and it uses the Serenity crate to interact with Discord. The bot is written in Rust.

## 🏁 Getting Started <a name = "getting_started"></a>
These instructions will get you a copy of the project up and running on your local machine for development and testing purposes.

### Prerequisites

1. Cargo (The Rust Compiler)
2. A discord bot token (Can be obtained [here](https://discordapp.com/developers/applications/))

### Installing

1. In the files directory, you'll need to add a file called ".key" and add your discord bot token to it. Make sure it is the only thing in the file.
2. Run 'cargo build'
3. Navigate to the target directory and run the new application. Congratulations, the bot is online.

To test it, run the commands as I described in the [How it Works](#working) section

## ⛏️ Built Using <a name = "built_using"></a>
+ [Serenity](https://www.github.com/serenity-rs/serenity) - Rust Discord API Wrapper

## ✍️ Authors <a name = "authors"></a>
+ [@Botahamec](https://github.com/Botahamec) - Idea & Initial work

See also the list of [contributors](https://github.com/Botahamec/discord_gov_bot/contributors) who participated in this project.

## 🎉 Acknowledgements <a name = "acknowledgement"></a>
Thanks to [kylelobo](https://www.github.com/kylelobo) for the [amazing README template](https://github.com/kylelobo/The-Documentation-Compendium/blob/master/en/README_TEMPLATES/Bot.md) used here
+ Inspiration
+ References
