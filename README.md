# connect2discord
An example web app built on Rust, Svelte and discord.js with TypeScript
<br>
# Project overview
#### Description:
This project is a web app that controls settings of a Discord bot remotely. Bot should also have some functionality.

#### Tech stack:
- **Frontend:** Svelte on Vite with Typescript.
- **Backend:** Rust on actix-web and ~~mongodb~~ for DB.
- **Bot:** discord.js with TypeScript.

### How to use
This project relies on some private keys. The keys in question are:
- Discord app's client secret
- Discord app's bot's token
- Discord app's client ID *(not a private value, but you'll understand why I mention it here)*

These values are set in a `.env` file at the root of the project, which you'll have to populate yourself if you are planning to use this project. The template is this:

##### .env
```
CLIENT_ID=abunchofnumbers
CLIENT_SECRET=bigstring
TOKEN=AvEryBigstRing
CDB_PASS=your_couchdb_password
```

Once that done, you'll need to install [CouchDB](https://couchdb.apache.org/#download) binary and set it up. Create a new database with the name `connect2discord` and in this database's "Permissions" tab and remove the "\_admin" role under "Members". Add the password you've setup in `.env` under `CDB_PASS`.

After that:
1. Run `npm i` to install all dependencies
2. Run `npm start` to start the project`
3. Go to `localhost:8000/` to view the project

To stop, press `Ctrl+C` in the terminal.

### Why do `/docs` suck so much?
Those `.md`'s with crazy maniac's rambles are not as much as documentation, but more so the real life [Rubber duck debugging](https://en.wikipedia.org/wiki/Rubber_duck_debugging). I use these files to describe the problem I'm having at the moment, which helps me to come up with a solution. I write things down, so I don't forget anything. Plus, I want to practice touch typing. Don't rely on those files too much lol.

#### TODO:
- [ ] Add info about login link in the README.md
- [x] Define what the project is supposed to do functionality-wise
- [x] Look into the "nano" npm package and why it doesn't work in the browser.
		*It just isn't built to support usage in browsers.*
- [x] Figure out how to omit fields in Rust's structs
        *You cannot.*
- [ ] Rewrite frontend async UI. Again
