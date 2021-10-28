<script lang="ts">
    export let right: boolean = false;
    export let title = false;
    export let link: string | undefined = undefined;

    let side = right ? 'right' : 'left';
</script>

<style>
    .app-bar-item {
        display: flex;
        margin: 10px;
        font: inherit;
        cursor: pointer;
    }
    .title {
        font-size: 19px;
        font-weight: bolder;
    }
    .link {
        height: 100%;
        color: black;
        font-size: 16px;
        text-decoration: none;
        cursor: pointer;
    }
</style>

<div class='app-bar-item'
     class:title
     style="float: {side}; margin-left: {side === 'right' ? 'auto' : '10px'}">
    {#if link !== undefined}
        <a class="link" href="{link}" disabled="{link || undefined}">
            <slot></slot>
        </a>
    {:else}
        <slot></slot>
    {/if}
</div>