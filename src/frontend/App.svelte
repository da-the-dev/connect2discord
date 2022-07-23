<script lang="ts">
import { onMount } from 'svelte'
import User from './modules/discordUser'

let user = new User()

onMount(async () => {
	user.login
	user.getDiscordUser
	user.getUserGuilds
})
</script>

<main>
	{#await user.login() then _}
		{#if user.loggedIn}
			{#await user.getDiscordUser() then du}
				<h1>Welcome {du.username}</h1>
			{/await}

			{#await user.getUserGuilds() then ug}
				<ul>
					{#each ug as g}
						{#if g.owner}
							<li>{g.name}</li>
						{/if}
					{/each}
				</ul>
			{/await}
		{:else}
			<a
				href="https://discord.com/api/oauth2/authorize?client_id=997526709148598282&redirect_uri=http%3A%2F%2Flocalhost%3A8000%2F&response_type=code&scope=identify%20guilds"
			>
				<h2>Login</h2>
			</a>
		{/if}
	{/await}
</main>

<footer>
	<p>
		This is a demo project licenced under MIT licence. Feel free to use it
		in any shape or form
	</p>
	<p>Copyright OLOLOLOLO 2022</p>
</footer>

<style>
main {
	padding: 1rem;
	text-align: center;

	min-height: 100vh;
}
ul {
	text-align: left;
}
h1 {
	font-size: xxx-large;
}

:global(html),
:global(body) {
	padding: 0;
}

footer {
	padding: 1rem;
	text-align: center;
	height: 10%;
	background: #eee;

	display: flex;
	align-items: center;
	justify-content: center;
	flex-direction: column;
}
</style>
