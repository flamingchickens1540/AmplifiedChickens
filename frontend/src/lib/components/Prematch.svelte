<script>
    import Header from "$lib/components/Header.svelte"
    import Position from "$lib/components/Toggle.svelte"
    import Threeoption from "$lib/components/Where.svelte"
    let location = ""
</script>

<Header phase = "Pre Match"/>
<Threeoption text1="Far" text2="Middle" text3="Close" bind:value = {location}/>
<Position text1 = "Fielded" text2 = "Missed Match"/>