# How authorization happens

I'm sick of keeping in mind how Discord's API works, so here's this note.

## Authorzation
First, on the main page <abbr title="Human operating the client">user</abbr> presses "Login" button to login *(obvious, I know)*. After that, <abbr title="Usually a web browser">client</abbr> gets redirected to Discord's page for logging in. Once this completes, client gets redirected back to `localhost:8000` with `code` as a query paramenter. This code shall be called ***authorization code***. The name comes from the fact that to get the code user should *authorize* first. It should be saved and passed on in memory for later usage, we will not need it for very long. The auth code will remain in the query params, so it should be [safely removed](https://stackoverflow.com/questions/22753052/remove-url-parameters-without-refreshing-page) *(the page should not reload)*.

## Access
Second, another code must be generated. This code, the ***access code***, is *the* code that will be used to ping Discord API for user info. To generate it, it is required to know Discord client's secret and ID. It would be unsafe to share this info with every client, so it will need to send a request to the <abbr title="Machine that hosts frontend and backend code">server</abbr> to create the access code there instead. The response will contain the code itself

Finally, client can now ping Discord's API directly to retrieve into about the user's Discord's account.

