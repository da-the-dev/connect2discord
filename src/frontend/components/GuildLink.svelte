<script lang="ts">
  import { ListGroupItem } from 'sveltestrap'
  import type UserGuild from '../interfaces/userGuild'

  export let guild: UserGuild
  export let active = false
  let hovered = false
</script>

<!-- svelte-ignore a11y-mouse-events-have-key-events -->
<div on:mouseover={() => (hovered = true)} on:mouseout={() => (hovered = false)}>
  <ListGroupItem {active} color={hovered ? 'secondary' : null}>
    {guild.name}
  </ListGroupItem>
</div>

<style>
  div {
    text-align: left;
  }
</style>
