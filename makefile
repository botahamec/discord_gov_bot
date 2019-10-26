
target/debug: src
	cargo build

target/debug/files: src/files
	cp src/files target/debug/files

target/debug/data:
	mkdir target/debug/data
	mkdir target/debug/data/servers

run-win: target/debug target/debug/files target/debug/data
	cd target/debug; discord_gov_bot.exe