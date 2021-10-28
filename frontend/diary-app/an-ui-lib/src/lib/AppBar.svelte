<script lang="ts">
    export let height: string = '48';
</script>

<style>
    .top-nav-bar {
        overflow: hidden;
        display: inline-flex;
        position: sticky;
        background: white;
        width: 100%;
        box-shadow: 0 2px 1px gray;
        align-items: center;
        z-index: 99;
    }
    .content {
        align-items: center;
        display: flex;
        width: 70%;
        margin: 0 auto;
        padding: 10px;
        font: 1.1em 'Roboto', sans-serif;
    }
</style>

<div class="top-nav-bar" style="height: {height}">
    <div class="content">
        <slot></slot>
    </div>
</div>