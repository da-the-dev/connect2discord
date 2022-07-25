<script lang="ts">
import { onMount } from 'svelte'

import type UserGuild from '../interfaces/userGuild'
import Bot from '../modules/discordBot'

export let activeGuild: UserGuild

let bot = new Bot()
let botIsInGuild: boolean
onMount(async () => {
    await bot.getGuilds()
    botIsInGuild = bot.guilds.find(bg => bg.id == activeGuild.id) != undefined
})
</script>

<section>
    {#await bot.getGuilds() then _}
        {#if activeGuild}
            {#if botIsInGuild}
                <ul>
                    <li>{activeGuild.id}</li>
                </ul>
            {:else}
                <h1>Bot is not in this guild</h1>
                <h1>
                    <a
                        href="https://discord.com/api/oauth2/authorize?client_id=997526709148598282&permissions=8&scope=bot%20applications.commands"
                    >
                        Add it
                    </a>
                </h1>
            {/if}
        {:else}
            <h1>Select a guild of the left first</h1>
        {/if}
    {/await}
</section>
