<script lang="ts">
    import Pie from "./Pie.svelte";
    export let scout_data: (string | number)[][]

    let percents = scout_data.map((scout) => scout[1] as number)
    let names = scout_data.map((scout) => (scout[0] as string).split(" ")[0])

</script>

<div
    style="grid-template-columns: repeat(auto-fit, minmax(80px, 1fr));"
    class="bg-btn_grey mx-3 p-3 grid gap-2 content-center rounded-md items-start"
>
    {#each scout_data as _, i}
        {#if i < 8}
            <div class="flex flex-col items-center">
                <Pie size={40} percent={percents[i]} />
                <p class="text-text_white text-center break-all max-w-[80px]">{names[i]}</p>
            </div>
        {/if}
    {/each}
</div>