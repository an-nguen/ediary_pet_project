<style lang='scss'>
    @import "ripple";

    .nav-item {
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
        display: flex;
        min-height: 48px;
        width: 100%;
        padding: 0 10px;
        z-index: inherit;
        cursor: pointer;
        align-items: center;
        border-radius: 5px;
        color: black;
        background-color: white;
        font: inherit;
        @include add-ripple(#bbbbbb, #656565);
    }
</style>

<button class="nav-item">
    <slot></slot>
</button>
