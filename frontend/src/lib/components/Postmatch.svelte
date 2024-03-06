<script lang="ts">
    import Header from "$lib/components/Header.svelte";
    import Stage from "$lib/components/Stage.svelte";
    import Toggle from "$lib/components/Toggle.svelte";
    import TextArea from "$lib/components/TextArea.svelte";
    import Rating from "$lib/components/Rating.svelte";
    import { match_data } from "$lib/stores";
    import { default_match_data } from "$lib/types"
    import { goto } from "$app/navigation"
    import SubmitButton from "./SubmitButton.svelte";
    const BACKEND_URL = import.meta.env.VITE_BACKEND_URL;

    let brokenstatus = "";
    let deadstatus = "";
    let deadboolean = false;
    let brokeboolean = false;

    $: {
        if (deadstatus == "Died on Field") {
            deadboolean = true;
        } else {
            deadboolean = false;
        }
        if (brokenstatus == "Broken") {
            brokeboolean = true;
        } else {
            brokeboolean = false;x
        }

        $match_data.is_broke = brokeboolean;
        $match_data.is_died = deadboolean;
    }

    async function submit_match() {
    console.log("SUBMIT MATCH")
	console.log($match_data)
	let req: any = {id: 0, ...$match_data}

        let res = await fetch(`${BACKEND_URL}/submit/match`, {
            method: "POST",
            headers: {
                "Content-Type": "application/json",
            },
            body: JSON.stringify(req),
        });
        console.log("Submitted match:", res);

let temp = $match_data.match_key
        $match_data = default_match_data
$match_data.match_key = temp

goto("/app/match")

    }
</script>

<Header phase="Post Match" />
<Stage bind:value={$match_data.stage} />
<Rating name="Driver Skill" bind:value={$match_data.skill} />
<Toggle text1="Undamaged" text2="Broken" bind:value={brokenstatus} />
<Toggle text1="Functional" text2="Died on Field" bind:value={deadstatus} />
<TextArea bind:value={$match_data.notes} />
<SubmitButton text="Submit" onClick={submit_match} />
