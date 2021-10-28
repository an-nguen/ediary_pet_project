<script lang="ts">
    import Button from './Button.svelte';
    import TableColumn from './TableColumn.svelte';
    import TextField from './TextField.svelte';

    export let headers: Array<{caption: string}> = [];
    export let data: Array<any> = [];
    export let pageSize = 10;

    let currentPageIndex = 1;
    let currentPage: Array<any> = [];
    let pageCount = Math.ceil(Math.ceil(data.length) / pageSize);
    updatePage();

    function onPageChange(event: CustomEvent) {
        const page = event.detail.value as number;
        if (page < 1 || page > pageCount) {
            currentPageIndex = 1;
        } else {
            currentPageIndex = event.detail.value as number;
        }
        updatePage();
    }

    function updatePage() {
        currentPage.length = 0;
        const startIndex = (currentPageIndex - 1) * pageSize;
        let endIndex = startIndex + pageSize;
        if (endIndex > data.length) {
            endIndex = data.length;
        }
        for (let i = startIndex; i < endIndex; i++)
            currentPage.push(data[i]);
    }
</script>

<style lang='scss'>
    table {
        border: 1px solid black;
        border-collapse: collapse;
        width: 100%;
    }
    table tbody {
        height: 40px;
    }
    tr:nth-child(even){background-color: #f2f2f2;}

    #table-settings-row {
        display: flex;
    }
    tfoot {
        tr {
            td {
                display: flex;
                align-items: flex-end;
                align-content: flex-end;
            }
        }
    }
</style>

<div>
    <div id="table-settings-row">
        <Button primary>Settings</Button>
    </div>
    <table>
        <thead>
            <tr>
                {#each headers as h, i}
                    <TableColumn head resize={i !== headers.length - 1}>{h.caption}</TableColumn>
                {/each}
            </tr>
        </thead>
        <tbody>
        {#each currentPage as d, i}
            <tr>
                {#each headers as h}
                    <TableColumn resize={i !== headers.length - 1}>{d[h.value]}</TableColumn>
                {/each}
            </tr>
        {/each}
        </tbody>
        <tfoot>
            <tr>
                <TableColumn columnSpan='{headers.length}' resize={false} align='right'>
                    <div style='display: flex; justify-content: right; align-items: center'>
                        <TextField type='number' value={currentPageIndex} on:change={onPageChange} /> of {pageCount}
                    </div>
                </TableColumn>
            </tr>
        </tfoot>
    </table>
</div>