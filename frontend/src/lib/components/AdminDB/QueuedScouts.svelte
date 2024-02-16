<script lang="ts">
<<<<<<< HEAD
    import { onMount } from "svelte";
    import type { Scout } from "$lib/types.ts";

    // This needs to be passed in through sse stream
    export let queued: Scout[] = [];
</script>

<div class="rounded" style="background-color: #2C2C2C; padding:1rem">
    <h3 style="margin:0.5rem">Queued Scouts</h3>
    {#each queued as scout}
        <h3 class="rounded" style="padding: 1rem; background-color: #5C5C5C">
            {scout.name}
        </h3>
    {/each}
</div>

<style>
    h3 {
        font-family: "Poppins-bold";
        font-size: 30px;
        line-height: 91.5%;
        color: #ffffff;
        margin: 0.75rem;
    }
</style>
=======
    // import type { Scout } from "$lib/types.ts"
    export let queued/*:Scout[] */= ["Rex", "Azalea", "Jack", "Blaze", "Charlie", "Domnic", "Jackson"]
    queued = queued.slice(0, 6)
</script>
<div class="rounded" style="background-color: #2C2C2C; padding:1rem">
<h3 style="margin:0.5rem">Queued Scouts</h3>
{#each queued as person}
<h3 class="rounded" style="padding: 1rem; background-color: #5C5C5C">{person}</h3>
{/each}
</div>
<style>
 h3{
    font-family: 'Poppins-bold';
    font-size: 30px;
    line-height: 91.5%;
    color: #FFFFFF;
    margin:0.75rem
    }

</style>
>>>>>>> efa7ad755fe28f2d82c266b2b627fb6637c0756f
