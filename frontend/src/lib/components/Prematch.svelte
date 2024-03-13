<script>
    import Header from "$lib/components/Header.svelte"
    import Position from "$lib/components/Toggle.svelte"
    import Threeoption from "$lib/components/Threeoption.svelte"
    import { match_data } from '$lib/stores';
    import { onMount } from "svelte";

    let location = ""
    let fieldedstring = ""
    let fielded = true 
    $: if (fieldedstring == "Missed Match") {
        fielded = false
        fielded = fielded
    }
    $: if (fieldedstring == "Fielded"){
        fielded = true
        fielded = fielded
    }
    $: {
        $match_data.is_fielded = fielded
        $match_data.location = location
    }

    onMount(() => {
        localStorage.setItem("match_data", JSON.stringify($match_data))
    })
    
</script>
<Header phase = "Pre Match"/>
<Position text1 = "Fielded" text2 = "Missed Match" bind:buttonon = {$match_data.is_fielded}/> 
