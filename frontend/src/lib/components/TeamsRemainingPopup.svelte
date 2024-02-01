<script lang="ts">
    // import type { Scout } from "$lib/types.ts"
    export let queued/*:Scout[] */= ["1540", "2910", "1540", "2046", "1540", "3663", "1540", "1540", "2910", "1540", "2046", "1540", "3663", "1540"]
    export let value = ""
</script>
<div class="rounded" style="background-color: #2C2C2C; padding:1rem">
<h3 style="margin:0.5rem">Teams Remaining</h3>
<div class="main">
{#each queued as person}

<h3 on:click={() => {value = person}} class="rounded" style="padding: 1rem; background-color: #5C5C5C">{person}</h3>
{/each}
</div>
</div>
<style>
 h3{
    font-family: 'Poppins-bold';
    font-size: 30px;
    line-height: 91.5%;
    color: #FFFFFF;
    margin:0.75rem
    }
    .main{
    overflow-y: scroll;
    max-height: 500px;
}

</style>