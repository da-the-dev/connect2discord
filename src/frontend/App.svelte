<script lang="ts">
import { Col, Container, Row, Styles } from 'sveltestrap'
import GuildInfo from './components/GuildInfo.svelte'
import GuildList from './components/GuildList.svelte'
import Profile from './components/Profile.svelte'
import type Guild from './interfaces/guild'
import type UserGuild from './interfaces/userGuild'
import Bot from './modules/discordBot'
import User from './modules/discordUser'

const user = new User()
const bot = new Bot()
let activeGuild: UserGuild
</script>

<Styles />

{#await Promise.all([user.login(), bot.getGuilds()]) then _}
    <Container fluid>
        <Row>
            <h1>Hi, noname#6969</h1>
        </Row>
        <Profile />
        <Row>
            <Col xs="2">
                <GuildList guilds={user.getOwnerGuilds()} />
            </Col>
            <Col>
                <GuildInfo activeGuild={user.getOwnerGuilds()[0]} botGuilds={bot.guilds} />
            </Col>
        </Row>
    </Container>
{/await}

<style>
h1 {
    text-align: center;
}
</style>
