# Communications
I'm sick of keeping the way everything ties with everything else, so here's this note.

There are three components in this project. 
- **Client** is the browser that the user uses to communicate with server and Discord API
- **Server** is the backend
- **Discord API** is self-explanatory

Client upon loading the website needs to know a few things *(let's consider that we have already have [logged in](Authorization))*
- <abbr title="Human interacting with the website via client">User's</abbr> data *(such as username, id, etc.)*
- User's <abbr title="This is how Discord servers are called by the devs">guilds</abbr> and if they own any of them
- <abbr title="Discord bot that user wants to control via the website">Bot's</abbr> guilds

We need to know whether the bot exist on any of the Discord guilds the user owns to selectively edit the bot's settings in a specific guild. To achieve this, we need to make a few API calls to Discord on behalf of the user and the bot.

## User comms
User-related API calls are easy. Client has [auth token](Authorization##Access) stored in local storage, which it can use to send requests to Discord. Just add `Authorization: Bearer <auth_token>` header to the request and voila.

## Bot comms
### The problem
Bot-related API calls are much trickier. Since it's the client that need info about the bot, data should eventually travel back to it. We cannot make calls directly from the client because these requests require the same `Authorization` header, but with bot's token this time, like this: `Authorization: Bot <token>`. We also **cannot** give the client this token, because it's unsafe. The solution is using the backend.

### The solution
We can make use of the backend as a *"middleman"* for our requests. Client sends a request to the backend, it adds the correct header, makes the request to the Discord API and responds with API's response. By doing this, the server hides the bot's token and makes calls on behalf of it. 

We can implement a generic GET route like `localhost:8000/botapi/{endpoint}` on the backend. `endpoint` can be any endpoint link of Discord's API, so that there won't be any need for separate routes for each required endpoint. 

### A few more tons of problems
But there's more to these requests. There exists an issue of [rate limiting](https://discord.com/developers/docs/topics/rate-limits). TL;DR - Discord limits the amount of requests one IP can send. If the limit is exsided, there is time limit that must pass before another request can be processed. The backend should account for that and the fontend should show the user a loading message as well. On the backend we can make a request and if it failes with a rate limit, we can tell this happend by `429` status code. Inside the body there'll be `retry_after` value, which is an amount of seconds we must wait until we can send a new request. We can resend the request as soon as the timer is over.