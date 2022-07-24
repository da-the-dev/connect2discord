# connect2discord
An example webapp built on Rust, Svelte and discord.js with TypeScript
<br>
# Project overview
#### Description:
This project is a webapp that controls settings of a Discord bot remotely. Bot should also have some functionality.

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
```

Once that's done, do this:
1. Run `npm i` to install all dependencies
2. Run `npm start` to start the project`
3. Go to `localhost:8000/` to view the project

To stop, press `Ctrl+C` in the terminal.

#### TODO:
- [ ] Add info about login link in the README.md
- [ ] Define what the project is supposed to do functionality-wise
