<script>
    import Header from "$lib/components/Header.svelte"
    import Stage from "$lib/components/Stage.svelte"
    import Toggle from "$lib/components/Toggle.svelte"
    import TextArea from "$lib/components/TextArea.svelte"
    import Rating from "$lib/components/Rating.svelte"
    import { match_data } from '$lib/stores/matchStores.ts';
    let stagestatus = ""
    let driverskill = 3
    let brokenstatus = ""
    let deadstatus = ""
    let notes = ""
    let deadboolean = false
    let brokeboolean = false
    if (deadstatus = "Died on Field"){
    deadboolean = true
    }
    else{
    deadboolean = false
    }
    if (brokenstatus = "Broken"){
    brokeboolean = true
    }
    else{
    brokeboolean = false
    }
    $: {
        $match_data.stage = stagestatus
        $match_data.skill = driverskill
        $match_data.is_broke = brokeboolean
        $match_data.is_died = deadboolean
        $match_data.notes = notes
    }
</script>
<Header phase = "Post Match"/>
<Stage bind:value = {stagestatus}/>
<Rating name = "Driver Skill" bind:value = {driverskill}/>
<Toggle text1="Undamaged" text2="Broken" bind:value = {brokenstatus}/>
<Toggle text1="Functional" text2 = "Died on Field" bind:value = {deadstatus}/>
<TextArea bind:value = {notes}/>
<div class="grid grid-cols-5">
    <button class="col-span-5">
        <h3 class="col-span-5 rounded ">Submit!</h3>
    </button>
    </div>
<style>
    h3{
        margin: 17px;
        font-family: poppins-medium;
        color: #FFFFFF;
        background-color: #5c5c5c;
        font-size: 2rem;
        padding: 2rem;
        text-align: center;
    }
</style>