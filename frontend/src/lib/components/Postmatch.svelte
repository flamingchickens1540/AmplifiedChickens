<script lang="ts">
    import Header from "$lib/components/Header.svelte";
    import Stage from "$lib/components/Stage.svelte";
    import Toggle from "$lib/components/Toggle.svelte";
    import TextArea from "$lib/components/TextArea.svelte";
    import Rating from "$lib/components/Rating.svelte";
    import { match_data } from "$lib/stores";
    import SubmitButton from "./SubmitButton.svelte";
    import { createEventDispatcher } from "svelte";
    const BACKEND_URL = import.meta.env.VITE_BACKEND_URL;

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

    let dispatch = createEventDispatcher()

    async function submit_match() {
        let req: any = { id: scout_id };
        req.push($match_data);

        dispatch("submit_match", req)
    }
</script>

<Header phase="Post Match" />
<Stage bind:value={$match_data.stage} />
<Rating name="Driver Skill" bind:value={$match_data.skill} />
<Toggle text1="Undamaged" text2="Broken" bind:value={brokenstatus} />
<Toggle text1="Functional" text2="Died on Field" bind:value={deadstatus} />
<TextArea bind:value={$match_data.notes} />
<SubmitButton text="Submit" onClick={submit_match} />
