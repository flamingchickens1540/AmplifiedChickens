<script lang="ts">
    import type { Team } from "$lib/types.ts"

    export let remaining_teams: string[] = []
    export let value: string = ""
</script>

<div class="rounded" style="background-color: #2C2C2C; padding:1rem">
    <h3 style="margin:0.5rem">Teams Remaining</h3>
    <div class="main">
        {#if remaining_teams.length > 0}
            {#each remaining_teams as team}
                <button
                    on:click={() => {
                        value = team;
                    }}
                    class="rounded"
                    style="padding: 1rem; background-color: #5C5C5C"
                >
                    {team}
                </button>
            {/each}
        {:else}
            <h3
                class="rounded"
                style="padding: 1rem; background-color: #5C5C5C"
            >
                No Teams Remaining!
            </h3>
        {/if}
    </div>
</div>

<style>
    h3 {
        font-family: "Poppins-bold";
        font-size: 30px;
        line-height: 91.5%;
        color: #ffffff;
        margin: 0.75rem;
    }
    .main {
        overflow-y: scroll;
        max-height: 500px;
    }
</style>
