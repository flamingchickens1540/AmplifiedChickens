<script>
    import Header from "$lib/components/Header.svelte"
    import Position from "$lib/components/Toggle.svelte"
    import Threeoption from "$lib/components/Threeoption.svelte"
    import { match_data } from '$lib/stores';

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
    

</script>
<Header phase = "Pre Match"/>
<Threeoption text1="Far" text2="Middle" text3="Close" bind:value = {location}/>
<Position text1 = "Fielded" text2 = "Missed Match" bind:value = {fieldedstring}/> 
