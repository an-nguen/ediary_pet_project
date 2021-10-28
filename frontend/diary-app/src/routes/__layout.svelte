<script lang='ts'>
    import {AppBar, AppBarItem, NavMenu, NavItem} from 'an-ui-lib';
    import IconButton from '../../an-ui-lib/src/lib/IconButton.svelte';

    let topBarHeight: string = '48px';

    let navMenu = false;
    function toggleNavMenu() {
        navMenu = !navMenu;
    }
</script>

<style>

    .content {
        margin: auto;
        width: 60%;
        padding: 10px;
        z-index: 1;
        font: 16px 'Roboto', sans-serif;
    }
</style>

<AppBar height="{topBarHeight}">
    <AppBarItem>
        <IconButton on:click={toggleNavMenu}>
            menu
        </IconButton>
    </AppBarItem>
    <AppBarItem title>
       App
    </AppBarItem>
    <AppBarItem right link="page">
        Page
    </AppBarItem>
</AppBar>

<NavMenu topBarHeight show={navMenu}>
    <NavItem>
        test
    </NavItem>
</NavMenu>

<div class="content">
    <slot></slot>
</div>

