<script lang="ts">
    import { createEventDispatcher } from 'svelte';
    import Icon from '@iconify/svelte';

    export let primary = false;
    export let icon: string;

    let button: HTMLButtonElement;

    let dispatch = createEventDispatcher();

    function onClick(event: MouseEvent) {
        dispatch('click', event.detail);
    }
</script>

<button bind:this={button}
        class='btn'
        class:primary
        on:click={onClick}
>
    {#if !!icon}
        <Icon inline icon={icon} />
    {/if}
    <slot></slot>
</button>


<style lang='scss'>
    @use "base";
    @import 'ripple';

    .btn {
        -webkit-appearance: none;
        -moz-appearance: none;
        -webkit-touch-callout: none;
        -webkit-user-select: none;
        -khtml-user-select: none;
        -moz-user-select: none;
        -ms-user-select: none;
        -webkit-tap-highlight-color: transparent;

        appearance: none;
        border: none;
        display: block;
        border-radius: 5px;
        transition: 200ms all ease-out;
        color: black;
        background: none;
        padding: 8px 24px;
        cursor: pointer;
        margin: 8px;

        font-family: 'Roboto', sans-serif;
        font-weight: 500;
        font-size: .875rem;
        letter-spacing: .09em;
        text-transform: uppercase;
    }

    .primary {
        background: base.$primary-color;
        color: base.$primary-text-color;
        @include add-ripple(adjust-color(base.$primary-color, $lightness: 35%), adjust-color(base.$primary-color, $lightness: 20%));
    }
</style>
