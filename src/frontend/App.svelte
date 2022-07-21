<script lang="ts">
import { onMount } from 'svelte'
import User from './models/User'
import { getDiscordCode } from './modules/discordRequests'

let user = {} as User
let code = ''

onMount(async () => {
	code = getDiscordCode()
})
</script>

<main>
	{#if code}
		<h1>Welcome: {user.username}</h1>
		<p>{code}</p>
	{:else}
		<a
			href="https://discord.com/api/oauth2/authorize?client_id=997526709148598282&redirect_uri=http%3A%2F%2Flocalhost%3A8000%2F&response_type=code&scope=identify"
		>
			<h2>Login</h2>
		</a>
	{/if}
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

	min-height: 100%;
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
