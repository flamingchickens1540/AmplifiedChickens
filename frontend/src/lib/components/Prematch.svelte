<script>
    import Header from "$lib/components/Header.svelte"
    import Position from "$lib/components/Toggle.svelte"
    import Threeoption from "$lib/components/Threeoption.svelte"
    import { match_data } from '$lib/stores';
    import { onMount } from "svelte";

    onMount(() => {
        localStorage.setItem("match_data", JSON.stringify($match_data))
    })

</script>
<Header phase = "Pre Match"/>
<Position text1 = "Fielded" text2 = "Missed Match" bind:buttonon = {$match_data.is_fielded}/> 
