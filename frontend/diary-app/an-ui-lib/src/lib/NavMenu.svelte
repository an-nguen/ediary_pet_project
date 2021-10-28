<script>
    import { fly } from 'svelte/transition';

    export let width = 240;
    export let show = false;

</script>

<style>
    .nav-menu {
        background: white;
        display: block;
        float: left;
        padding: 16px;
        position: fixed;
        margin-bottom: 0;
        box-shadow: 0 2px 1px grey;
        width: var(--width);
        height: 100%;
        z-index: 10;
        font: 1.1em 'Roboto', sans-serif;
    }
</style>

{#if show}
<div class="nav-menu" style="--width: {width}px" transition:fly={{ x: -width, duration: 600 }}>
    <slot></slot>
</div>
{/if}