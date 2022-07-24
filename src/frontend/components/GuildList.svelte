<script lang="ts">
  import { ListGroup } from 'sveltestrap'
  import type UserGuild from '../interfaces/userGuild'
  import GuildLink from './GuildLink.svelte'
  export let guilds: [UserGuild]
  export let activeGuild: UserGuild
</script>

<div>
  <ListGroup numbered>
    {#each guilds as guild, i}
      {#if guild.owner}
        <div on:click={() => (activeGuild = guild)}>
          <GuildLink {guild} />
        </div>
      {/if}
    {/each}
  </ListGroup>
</div>

<style>
  div {
    user-select: none;
  }
</style>
