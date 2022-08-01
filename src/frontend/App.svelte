<script lang="ts">
import { Col, Container, FormGroup, Input, ListGroup, ListGroupItem, Row, Styles } from 'sveltestrap'
import Profile from './components/Profile.svelte'
import Bot from './modules/bot'
import DB from './modules/database'
import User from './modules/user'
import { activeGuild, activeGuildSettings } from './stores/activeGuild'
import _ from "lodash"

let db: DB 

async function selectGuild(i: number) {
    $activeGuild = User.guilds[i]
    db = new DB($activeGuild.id)

    if (Bot.guilds.find(g => g.id == $activeGuild.id)) $activeGuildSettings = await db.getSettings()
    console.log($activeGuildSettings)
}


async function saveChanges() {
    console.log($activeGuildSettings)
    $activeGuildSettings = await db.saveSettings($activeGuildSettings)
    console.log("saving...")
    console.log($activeGuildSettings)
}

const saveChangesDebounced = _.debounce(saveChanges, 700)
</script>

<Styles />

{#await Promise.all([User.login(), Bot.getGuilds()]) then _}
    {#if User.loggedIn}
        <Container fluid>
            <Row><h1>Hi, {User.discordUser.username}</h1></Row>
            <Profile  />
            <Row>
                <Col xs="3">
                    <ListGroup>
                        {#each User.guilds as guild, i}
                            <ListGroupItem on:click={() => selectGuild(i)}>{guild.name}</ListGroupItem>
                        {/each}
                    </ListGroup>
                </Col>
                <Col>
                    <h2>Settings</h2>
                    <FormGroup floating label="Embed color">
                        <Input
                            bind:value={$activeGuildSettings.embedColor}
                            on:keydown={async () => await saveChangesDebounced()}
                        />
                    </FormGroup>
                </Col>
            </Row>
        </Container>
    {:else}
        <h1>
            <a
                href="https://discord.com/api/oauth2/authorize?client_id=997526709148598282&redirect_uri=http%3A%2F%2Flocalhost%3A8000%2F&response_type=code&scope=identify%20guilds"
            >
                You are not logged in. Log in.
            </a>
        </h1>
    {/if}
{/await}

<style>
h1 {
    text-align: center;
}
</style>
