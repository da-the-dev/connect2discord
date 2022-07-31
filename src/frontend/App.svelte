<script lang="ts">
import { Col, Container, ListGroup, ListGroupItem, Row, Styles } from 'sveltestrap'
import Profile from './components/Profile.svelte'
import Bot from './modules/bot'
import DB from './modules/database'
import User from './modules/user'
import { activeGuild, activeGuildSettings } from './stores/activeGuild'

const user = new User()
const bot = new Bot()
const db = new DB()

async function selectGuild(i: number) {
    $activeGuild = user.guilds[i]
    if (bot.guilds.find(g => g.id == $activeGuild.id)) $activeGuildSettings = await db.getSettings($activeGuild.id)
}
</script>

<Styles />

{#await Promise.all([user.login(), bot.getGuilds()]) then _}
    {#if user.loggedIn}
        <Container fluid>
            <Row><h1>Hi, {user.discordUser.username}</h1></Row>
            <Profile {user} />
            <Row>
                <Col xs="3">
                    <ListGroup>
                        {#each user.guilds as guild, i}
                            <ListGroupItem on:click={() => selectGuild(i)}>{guild.name} {guild.id}</ListGroupItem>
                        {/each}
                    </ListGroup>
                </Col>
                <Col>
                    {$activeGuildSettings.embedColor}
                </Col>
            </Row>
        </Container>
    {:else}
        <h1>
            <a
                href="https://discord.com/api/oauth2/authorize?client_id=997526709148598282&redirect_uri=http%3A%2F%2Flocalhost%3A8000%2F&response_type=code&scope=identify%20guilds"
            >
                You are not logged in. Log in
            </a>
        </h1>
    {/if}
{/await}

<style>
h1 {
    text-align: center;
}
</style>
