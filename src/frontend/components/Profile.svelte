<script lang="ts">
import { Button, Dropdown, DropdownItem, DropdownMenu, DropdownToggle } from 'sveltestrap'
// import User from '../modules/user'
</script>

<div class="profile">
    <Dropdown>
        <DropdownToggle caret>Profile</DropdownToggle>
        <DropdownMenu end>
            <!-- <DropdownItem header>{User.discordUser.username}</DropdownItem> -->
            <DropdownItem><Button color="danger">Logout</Button></DropdownItem>
        </DropdownMenu>
    </Dropdown>
</div>

<style>
.profile {
    position: absolute;
    top: 1em;
    right: 1em;
}
</style>
