<script lang="ts">
    export let head = false;
    export let columnSpan = 1;
    export let resize = true;
    export let align: 'right' | 'left' | 'center' = 'left';

    let tableHeadColumn;
    let resizing = false;
    let startOffset;

    function resizeHandleOnMouseDown(event: MouseEvent) {
        startOffset = tableHeadColumn.offsetWidth - event.pageX;
        resizing = true;
    }

    function wndOnMouseMove(event: MouseEvent) {
        if (resizing) {
            tableHeadColumn.style.width = startOffset + event.pageX + 'px';
        }
    }

    function wndOnMouseUp(event: MouseEvent) {
        resizing = false;
    }
</script>

<style>
    th {
        position: relative;
        border: 1px solid black;
        padding: 4px 16px;
        text-align: var(--align);
    }
    td {
        position: relative;
        padding: 4px 16px;
        border: 1px solid black;
        text-align: var(--align);
        justify-items: var(--align);
    }
    .resize-handle {
        position: absolute;
        top: 0;
        right: 0;
        bottom: 0;
        width: 5px;
        cursor: col-resize;
    }
    /*.resize-handle:hover {*/
    /*    background-color: grey;*/
    /*}*/
</style>

<svelte:window on:mouseup={wndOnMouseUp} on:mousemove={wndOnMouseMove} />

{#if head}
<th bind:this={tableHeadColumn} style='--align: {align}'>
    <slot></slot>
    {#if resize}
    <div class="resize-handle" on:mousedown={resizeHandleOnMouseDown}></div>
    {/if}
</th>
{:else}
<td bind:this={tableHeadColumn} colspan={columnSpan} style='--align: {align}'>
    <slot></slot>
    {#if resize}
        <div class="resize-handle" on:mousedown={resizeHandleOnMouseDown}></div>
    {/if}
</td>
{/if}