This repository outlines a basic CLI tool to generate notes and the outputs  are stored in a JSON for simplicity.

To run this, for security/best practices purposes the binary executable has not been shared. Thus you need cargo to run this which can be achieved by following ([these](https://www.rust-lang.org/tools/install)) rust installation steps. Then at the root of the directory run the following commands as you wish:

- To add note:`cargo run -- add "<title>" "<body>`
- To delete note: `cargo run -- remove <id>`
- To modify note: `cargo run -- modify <id> "<title>" "<body>"`
- To list notes in the JSON file: `cargo run --list`

Alternatively, if you have cargo installed on your system you can run:
1) `cargo build --release`
2) `cd target/release`
3)  `notetaking_cli.exe -- <pattern>`

