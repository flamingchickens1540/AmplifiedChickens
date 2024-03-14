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
<Threeoption text1="Far" text2="Middle" text3="Close" bind:value = {$match_data.location}/>
<Position text1 = "Fielded" text2 = "Missed Match" bind:buttonon = {$match_data.is_fielded}/> 
