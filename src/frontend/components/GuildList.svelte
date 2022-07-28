<script lang="ts">
import { ListGroup, ListGroupItem } from 'sveltestrap'
import type UserGuild from '../interfaces/userGuild'

export let guilds: UserGuild[]
</script>

<ListGroup>
    {#each guilds as guild}
        <ListGroupItem>{guild.name}</ListGroupItem>
    {/each}
</ListGroup>
