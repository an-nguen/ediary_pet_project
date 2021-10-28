<script>
    import {DataTable} from 'an-ui-lib'
    import TextField from '../../an-ui-lib/src/lib/TextField.svelte';

    let num = 10;
    let headers = [
        {caption: 'First value', value: 'first_value' },
        {caption: 'Second value', value: 'second_value' },
    ];
    const values = [
        {first_value: 'Value 1', second_value: 'Value 2'},
    ];
    for (let i = 0; i < 45; i++) {
      values.push({first_value: `Value ${i}`, second_value: `Value ${i}`});
    }
</script>

<h1>Welcome to SvelteKit</h1>
<p>Visit <a href="https://kit.svelte.dev">kit.svelte.dev</a> to read the documentation</p>
<input type="number" bind:value={num}>
<TextField appendIcon='search' label='test' value='test' />
<DataTable {headers} pageSize={10} data={values}>

</DataTable>