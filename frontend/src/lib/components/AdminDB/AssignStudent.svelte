<script lang="ts">
    import Combobox from "$lib/components/Combobox/ComboBoxRed.svelte";
    import type { Scout } from "$lib/types";

    export let color: string
    export let teams: string[] = []
    export const selected: Scout[] = []
    export let queued: Scout[] = []
    export let auto_assign: boolean

    let scout_opts: string[] = []
    let scout_names: string[] = []

    /// NOTE: THIS WAS LOOKING FOREVER BECAUSE IT WAS REACTIVE WITH A BINDING, SO IT UPDATING EVERY TIME THE BINDING REFRESHED / THE STATE CHANGED. 
    /// NEVER DO THIS AGAIN!!!
    /// WE LITERALLY LOST 3 DAYS OF DEVELOPMENT ON THE ADMIN DASHBOARD BECAUSE OF THIS
    // $: {
    //     scout_opts = queued.map((scout) => scout.name ?? "")
    //     scout_names = queued.map((scout) => scout.name ?? "")
    // }
</script>

<div
    class="grid grid-cols-1 grod-rows-3 rounded"
    style="background-color: #5C5C5C; padding:0.2rem; margin:17px"
>
    {#each teams as team, i}
        <div
            class="grid grid-cols-2 grid-rows-1 place-items-center rounded"
            style="background-color: { color }; padding:0.2rem; margin:10px;"
        >
            <!-- <input bind:value={team}/> -->
            <Combobox bind:value={team} {color} options={[]} placeholder="Team Number"/>
            {#if !auto_assign}
                <div>
                    <Combobox
                        bind:value={scout_names[i]}
                        {color}
                        options={scout_opts}
                        placeholder="Scout Name"
                    />
                </div>
            {/if}
        </div>
    {/each}
</div>

<style>
    h3 {
        font-family: "Poppins-bold";
        font-size: 25px;
        line-height: 91.5%;
        color: #ffffff;
        margin: 10px;
    }
</style>
