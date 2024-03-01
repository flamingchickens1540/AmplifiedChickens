<script lang="ts">
    import { onMount } from "svelte";
    import Pie from "./Pie.svelte";
    const BACKEND_URL = import.meta.env.VITE_BACKEND_URL_FOR_FRONTEND;

    export let scout_names: string[]
    export let scout_percents: number[]

</script>

<div
    style=""
    class="bg-btn_grey h-[185px] mx-3 grid-cols-4 grid gap-2 content-center items-center rounded-md"
>
    {#each scout_percents as _, i}
        {#if i < 8}
            <div class="p-1 flex flex-col items-center">
                <Pie size={46} percent={scout_percents[i]} />
                <p class="text-text_white">{scout_names[i]}</p>
            </div>
        {/if}
    {/each}
</div>
