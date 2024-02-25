<script lang="ts">
    import { onMount } from "svelte";
    import Pie from "./Pie.svelte";
    const BACKEND_URL = import.meta.env.VITE_BACKEND_URL;

    async function get_scout_percents(): Promise<readonly [string[], number[]]> {
        let res = await fetch(`${BACKEND_URL}/admin/users/get/all`, {
            method: "GET",
            headers: {
                Accept: "application/json",
                "Content-Type": "application/json",
            }
        })

        if (!res.ok) {
            console.error("Failed to fetch scout percents")
        }

        return res.json()
    }

    let scout_names: string[] = [];
    let scout_percents: number[] = [];

    onMount(async () => {
        let result = await get_scout_percents()

        scout_names = result[0]
        scout_percents = result[1]
    })
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
