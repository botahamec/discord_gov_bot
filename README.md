# discord_gov_bot

## About
A bot used to handle and simulate all the functions of a mock government

## Build
### Requirements
* Cargo (The rust compiler)
* A discord bot token (can be obtained [here](https://discordapp.com/developers/applications/))
### Instructions
There are two ways of running this bot
#### Option #1
1. Go to a terminal and in the directory of the project type "cargo build"
2. Go into the newly created target/debug folder and create a file called ".key" which contains the token for the bot
3. Run the executable file. The bot is online once the message "The government is now online!" appears in the terminal
#### Option #2
1. In the main folder for the project create a file called ".key" which contains the token for the bot
2. Go to a terminal and in the directory of the project type "cargo run". The bot is online once the message "The government is now online!" appears in the terminal