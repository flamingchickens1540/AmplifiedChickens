<script lang="ts">
    import Header from "$lib/components/Header.svelte";
    import Stage from "$lib/components/Stage.svelte";
    import Toggle from "$lib/components/Toggle.svelte";
    import TextArea from "$lib/components/TextArea.svelte";
    import Rating from "$lib/components/Rating.svelte";
    import { match_data } from "$lib/stores";

    export let scout_id = "";

    let brokenstatus = "";
    let deadstatus = "";
    let deadboolean = false;
    let brokeboolean = false;

    $: {
        if ((deadstatus = "Died on Field")) {
            deadboolean = true;
        } else {
            deadboolean = false;
        }
        if ((brokenstatus = "Broken")) {
            brokeboolean = true;
        } else {
            brokeboolean = false;
        }

        $match_data.is_broke = brokeboolean;
        $match_data.is_died = deadboolean;
    }

    async function submit_match() {
        let req: any = { id: scout_id };
        req.push($match_data);

        let res = await fetch("https://localhost:3007/submit/match", {
            method: "POST",
            headers: {
                Accept: "application/json",
                "Content-Type": "application/json",
            },
            body: JSON.stringify(req),
        });
        console.log(res);
</script>

<Header phase="Post Match" />
<Stage bind:value={$match_data.stage} />
<Rating name="Driver Skill" bind:value={$match_data.skill} />
<Toggle text1="Undamaged" text2="Broken" bind:value={brokenstatus} />
<Toggle text1="Functional" text2="Died on Field" bind:value={deadstatus} />
<TextArea bind:value={$match_data.notes} />
<button on:click={submit_match}> Submit! </button>

<style>
    button {
        margin: 17px;
        font-family: poppins-medium;
        color: #ffffff;
        background-color: #5c5c5c;
        font-size: 2rem;
        padding: 2rem;
        text-align: center;
    }
</style>
