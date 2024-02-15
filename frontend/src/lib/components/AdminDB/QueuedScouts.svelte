<script lang="ts">
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
