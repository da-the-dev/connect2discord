<script lang="ts">
  import { onMount } from 'svelte'
  import { Col, Container, Row, Styles } from 'sveltestrap'
  import GuildInfo from './components/GuildInfo.svelte'
  import GuildList from './components/GuildList.svelte'
  import Profile from './components/Profile.svelte'
  import User from './modules/discordUser'

  let user = new User()
  let activeGuild: any

  onMount(async () => {
    user.login
    user.getDiscordUser
    user.getUserGuilds
  })
</script>

<Styles />
<main>
  {#await user.login() then _}
    {#if user.loggedIn}
      <Container>
        {#await user.getDiscordUser() then du}
          <Profile {user} />
          <Row>
            <h1>Welcome, {du.username}</h1>
          </Row>
        {/await}
        {#await user.getUserGuilds() then guilds}
          <Row>
            <Col sm="2">
              <GuildList {guilds} bind:activeGuild />
            </Col>
            <Col>
              <GuildInfo bind:activeGuild />
            </Col>
          </Row>
        {/await}
      </Container>
    {:else}
      <a
        href="https://discord.com/api/oauth2/authorize?client_id=997526709148598282&redirect_uri=http%3A%2F%2Flocalhost%3A8000%2F&response_type=code&scope=identify%20guilds"
      >
        <h2>Login</h2>
      </a>
    {/if}
  {/await}
</main>

<style>
  main {
    padding: 1rem;
    text-align: center;

    min-height: 100vh;
  }
</style>
