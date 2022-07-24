<script lang="ts">
  import type UserGuild from '../interfaces/userGuild'

  export let activeGuild: UserGuild
</script>

<section>
  {#if activeGuild}
    <ul>
      <li>{activeGuild.id}</li>
      <!-- <li></li>
      <li></li>
      <li></li> -->
    </ul>
  {/if}
</section>
