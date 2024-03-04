<script lang="ts">
    import { onMount } from "svelte";
    import Pie from "./Pie.svelte";
    const BACKEND_URL = import.meta.env.VITE_BACKEND_URL_FOR_FRONTEND;

    async function get_scout_percents(): Promise<readonly [string[], number[]]> {
        let res = await fetch(`${BACKEND_URL}/admin/users/get/all`, {
            method: "GET",
            headers: {
                Accept: "application/json",
                "Content-Type": "application/json",
            }
        })
        console.log(res)

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
  
    let scout_firstnames: string[] = [];

    for (const name of scout_names) {
        scout_firstnames.push(name.split(' ')[0]);
    }
</script>

<div
    style="grid-template-columns: repeat(auto-fit, minmax(80px, 1fr));"
    class="bg-btn_grey mx-3 p-1 grid gap-2 content-center rounded-md items-start"
>
    {#each scout_percents as _, i}
        {#if i < 8}
            <div class="flex flex-col items-center">
                <Pie size={40} percent={scout_percents[i]} />
                <p class="text-text_white text-center break-all max-w-[80px]">{scout_firstnames[i]}</p>
            </div>
        {/if}
    {/each}
</div>