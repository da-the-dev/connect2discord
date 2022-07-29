<script lang="ts">
import { Col, Container, Row, Styles } from 'sveltestrap'
import GuildInfo from './components/GuildInfo.svelte'
import GuildList from './components/GuildList.svelte'
import Profile from './components/Profile.svelte'
import type UserGuild from './interfaces/userGuild'
import Bot from './modules/bot'
import User from './modules/user'

const user = new User()
const bot = new Bot()
let activeGuild: UserGuild
</script>

<Styles />

{#await Promise.all([user.login(), bot.getGuilds()]) then _}
    <Container fluid>
        <Row>
            <h1>Hi, {user.discordUser.username}</h1>
        </Row>
        <Profile {user} />
        <Row>
            <Col xs="2">
                <GuildList guilds={user.getOwnerGuilds()} bind:activeGuild />
            </Col>
            <Col>
                <GuildInfo bind:activeGuild botGuilds={bot.guilds} />
            </Col>
        </Row>
    </Container>
{/await}

<style>
h1 {
    text-align: center;
}
</style>
