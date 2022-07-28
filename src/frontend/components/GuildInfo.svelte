<script lang="ts">
import type Guild from '../interfaces/guild'
import type UserGuild from '../interfaces/userGuild'

export let activeGuild: UserGuild
export let botGuilds: [Guild]
const isBotInGuild = () => botGuilds.find(g => g.id == activeGuild.id)
</script>

<section>
    {#if activeGuild}
        {#if isBotInGuild}
            <h1>{activeGuild.id}</h1>
        {:else}
            <h1>
                The bot is not a member of this Server.
                <a href="https://discord.com/api/oauth2/authorize?client_id=997526709148598282&permissions=8&scope=bot">
                    Please add it via this link.
                </a>
            </h1>
        {/if}
    {:else}
        <h1>Plese select a guild on the left first</h1>
    {/if}
</section>
