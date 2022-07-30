# Database interactions
Whenever the Discord bot join a guild, a new entry to the database should be added. This entry should contain the default settings. For example, let's say we need to control the color of embeds' border. Let's also say that the default color is `#cc00cc`. The default couch db entry would be:
```json
{
	"_id": "12654346532",      // Guild ID
	"_rev": "5-123245123fg212" // Revision
	"embedColor": "#cc00cc"    // Color to use for embeds
}
```
We cannot directly access the database via the client, because the is no reliable and up-to-date JS module to use in the browser. There is [`nano`](https://www.npmjs.com/package/nano) package, but it doesn't support browser usage and only works under NodeJS. To achieve our goal we'll once again use the backend. Let's create route `localhost:8000/db/getSettings/{id}` to get the database settings. We can sanitize the data and remove `_id` and `_rev` fields. The rest will go into the response.

Client is not the only one who'll need to access the database. Because it will also be hosted on the same machine as the backend code, we can directly connect to the database. We will need to create an entry each time a bot is added to a new, never-before-visited guild, read settings specific to a particular server and maybe even delete very old entries.  
