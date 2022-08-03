<script lang="ts">
import { Col, Container, FormGroup, Input, ListGroup, ListGroupItem, Row, Styles } from 'sveltestrap'
import User from './modules/user'
import { activeGuild } from './stores/activeGuild'
import _ from 'lodash'
import DB from './modules/database'
import type { Settings } from './modules/database'

const user = new User()
const userLoginPromise = user.login()

// Settings fetching
let oldSettings = {} as Settings
const settingsFetcher = async () => {
    console.log("about to start to fetch the guild 2...")
    oldSettings = await DB.getSettings($activeGuild.id)
    return oldSettings
}
let settingsFetch = DB.getSettings('')

// Form validation and saving settings + debouncing
let formValid = true
let trueEmbedColor = ''
async function saveSettings(e: Event) {
    trueEmbedColor = (e.target as HTMLInputElement).value
    formValid = trueEmbedColor != "" && trueEmbedColor.startsWith('#') && trueEmbedColor.length == 7
    console.log(formValid)

    if (!formValid) return
    await DB.saveSettings(oldSettings)
    settingsFetch = settingsFetcher()
}
const saveSettingsDebounced = _.debounce(saveSettings, 700)

// Guild selection
async function selectGuild(i: number) {
    $activeGuild = user.guilds[i]
    settingsFetch = settingsFetcher()
}

</script>

<Styles />

{#await userLoginPromise then _}
    {#if user.loggedIn}
        <Container>
            <Row><h1>Hi, {user.discordUser.username}</h1></Row>
            <!-- <Profile /> -->
            <Row>
                <Col xs="3">
                    <ListGroup>
                        {#each user.guilds as guild, i}
                            <ListGroupItem on:click={() => selectGuild(i)}>{guild.name}</ListGroupItem>
                        {/each}
                    </ListGroup>
                </Col>

                <Col >
                    {#await settingsFetch then settings}
                        {#if settings}
                            <FormGroup floating label="Embed color">
                                <Input
                                    bind:value={oldSettings.embedColor}
                                    on:input={saveSettingsDebounced}
                                    style={`color: ${trueEmbedColor || "#000000"};`}
                                    valid={formValid}
                                    invalid={!formValid}
                                />
                            </FormGroup>
                        {/if}
                    {/await}
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
{:catch}
    <h1>An error occured during login</h1>
{/await}

<style>
h1 {
    text-align: center;
}
</style>
