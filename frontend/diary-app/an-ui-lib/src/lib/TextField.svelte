<script lang="ts">
  import { createEventDispatcher } from 'svelte';

  export let label: string | undefined = undefined;
  export let value: string | number = '';
  export let type: 'text' | 'password' | 'number' = 'text';
  export let width: string = '400px';
  export let appendIcon: string | undefined = undefined;

  let id = 'textField' + Math.random().toString(16);
  let glow = false;

  let dispatch = createEventDispatcher();

  function onChange(event: CustomEvent) {
    event["value"] = (event.target as HTMLInputElement).value;
    dispatch('change', event);
  }

  function onInput(event: InputEvent) {
    event["value"] = (event.target as HTMLInputElement).value;
    dispatch('input', event);
  }
  function onFocus(event: FocusEvent) {
    dispatch('focus', event.detail);
    glow = true;
  }
  function onBlur(event: FocusEvent) {
    dispatch('blur', event.detail);
    glow = false;
  }
</script>

<div class='control-wrapper' style='--width: {width}'>
  {#if label !== undefined}
    <label for={id}>{label}</label>
  {/if}
  <div class='input-wrapper' class:glow>
    {#if appendIcon !== undefined}
      <div style='display: flex; justify-content: center; align-content: center; align-items: center'>
        <span class='material-icons' style='font-size: 0.95rem; padding: 0 4px'>{appendIcon}</span>
      </div>
    {/if}
    <input id={id} type={type} value='{value}' on:input={onInput} on:focus={onFocus} on:blur={onBlur} on:change={onChange}>
  </div>
</div>

<style lang='scss'>
  @use 'base';

  .control-wrapper {
    display: flex;
    flex-direction: column;
    padding: 0;
    margin: 0;
    border: 0;
    text-decoration: inherit;
    width: var(--width);
  }

  label {
    font-size: 14px;
    font-weight: 600;
    margin: 4px 0;
  }

  .glow {
    box-shadow: 0 0 5px rgb(81, 203, 238);
  }

  .input-wrapper {
    transition: box-shadow 0.30s ease-in-out;
    border: 1px solid rgb(44, 44, 44);
    border-radius: 2px;
    position: relative;
    display: flex;
    flex-direction: row;
    align-content: center;
    justify-content: center;
  }

  input {
    -webkit-appearance: none;
    -moz-appearance: none;

    border: medium none;
    outline: none;
    font-size: 1rem;
    color: black;
    padding: 0 4px;
    background: transparent;
    width: 100%;
  }

  input:focus {
    box-shadow: none;
    // border: 1px solid rgb(81, 203, 238);
  }
</style>