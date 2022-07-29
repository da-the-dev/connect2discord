<script lang="ts">
import { ListGroup, ListGroupItem } from 'sveltestrap'
import type UserGuild from '../interfaces/userGuild'

export let guilds: UserGuild[]
export let activeGuild: UserGuild

const activateGuild = (i: number) => (activeGuild = guilds[i])
</script>

<ListGroup>
    {#each guilds as guild, i}
        <ListGroupItem on:click={() => activateGuild(i)}>{guild.name}</ListGroupItem>
    {/each}
</ListGroup>
