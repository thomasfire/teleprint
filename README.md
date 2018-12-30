# Teleprint
Prints files got from Telegram and IMAP on *nix systems

## Building
Install [Rust](https://www.rust-lang.org/) (needed Rust v 1.31 and higher) first. Also you need to install
`openssl-devel` on RedHat based systems or `libssl-dev` on Debian based ones. 

After that, run
```bash
$ cargo build --release
```
The executable is located at `target/release/teleprint`.

Build the documentation:
```bash
$ cargo doc
```

Open `target/doc/teleprint/index.html` via any browser to read the documentation.


## Running the bots

### Running setup

Install `lp` and setup printers manually.

Copy the executable where you want and run it:
```bash
$ ./teleprint --setup
```

Now you need to provide Telegram Bot API's token, name of the printer (they will be shown),
 IMAP server address, port, username and password.
 
 You also need to create the `users.toml` and fill it with data:
 ```toml
users = []
admin = 0
mail_tokens = []
```
Where admin is your Telegram ID (it's integer one).

### Running bots

Just run it:
```bash
$ ./teleprint
```

If no errors appear, bots are started. Now you can use it.

Users can only use `/auth` command and send files if they are authorized. After they 
send that command, you will see a message with his/her ID and you'll need to add the users manually.
But admin has a wide range of commands.

### Administrating

Use the commands below to manage users, tokens, files and printer:
* `/adduser <user_id>` -  add user to the access list
* `/addtoken <token>` - add token to the access list
* `/gentoken <name>` - generate token
* `/deluser <user_id>` - delete user from the access list
* `/deltoken <token>` - delete token from the access list
* `/print <filename>` - print the file
* `/users` - get users list
* `/tokens` - get tokens list
* `/files` - get files list
* `/getfile <filename>` - get file  
* `/delfile <filename>` - delete file
* `/lpstat` - see lpstat output
* `/cancel <job ID or name>` - cancel the job
* `/help` - print the list of commands above

### Sending and printing files

You or user just need to send you file in Telegram, and you will see that somebody wants to print something,
after that you can view that file by `/getfile <filename>`.

To print the file by email user should send token (only that) in the text and attach the file to the letter.
After that almost everything goes like if it was from Telegram.