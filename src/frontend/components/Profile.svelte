<script lang="ts">
  import type User from '../modules/discordUser'
  export let user: User
  import { Button, Dropdown, DropdownItem, DropdownMenu, DropdownToggle } from 'sveltestrap'
</script>

<div>
  <Dropdown size="md">
    <DropdownToggle caret>Profile</DropdownToggle>
    <DropdownMenu end>
      <DropdownItem header>{user.discordUser.username}</DropdownItem>
      <DropdownItem divider />
      <DropdownItem style={'text-align: right;'}>
        <Button color={'danger'} style={'width: 100%;'}>Logout</Button>
      </DropdownItem>
    </DropdownMenu>
  </Dropdown>
</div>

<style>
  div {
    position: absolute;
    top: 1em;
    right: 1em;
  }
</style>
