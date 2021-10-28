<script lang="ts">
  import { createEventDispatcher } from 'svelte';

  export let primary: boolean = false;

  let button: HTMLButtonElement;

  let dispatch = createEventDispatcher();

  function onClick(event: MouseEvent) {
    dispatch('click', event.detail);
  }
</script>

<button bind:this={button} class='btn' class:primary on:click={onClick}>
  <i class='material-icons'><slot></slot></i>
</button>

<style lang='scss'>
  @use 'base';

  @import "ripple";

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
    color: black;
    background: none;
    border-radius: 50%;
    cursor: pointer;
    height: 2.6rem;
    width: 2.6rem;
    @include add-ripple(#b7b7b7, #ffffff);
  }

  .primary {
    background-color: base.$primary-color;
    color: base.$primary-text-color;
    @include add-ripple(adjust-color(base.$primary-color, $lightness: 35%), adjust-color(base.$primary-color, $lightness: 20%));
  }
</style>