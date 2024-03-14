<script lang="ts">
    import Header from "$lib/components/Header.svelte";
    import Stage from "$lib/components/Stage.svelte";
    import Toggle from "$lib/components/Toggle.svelte";
    import TextArea from "$lib/components/TextArea.svelte";
    import Rating from "$lib/components/Rating.svelte";
    import { match_data } from "$lib/stores";
    import { default_match_data } from "$lib/types";
    import { goto } from "$app/navigation";
    import SubmitButton from "./SubmitButton.svelte";
    import { onMount } from "svelte";
    const BACKEND_URL = import.meta.env.VITE_BACKEND_URL;

    async function submit_match() {
        console.log("SUBMIT MATCH");
        console.log($match_data);
        let req: any = { id: 0, ...$match_data };

        let res = await fetch(`${BACKEND_URL}/submit/match`, {
            method: "POST",
            headers: {
                "Content-Type": "application/json",
            },
            body: JSON.stringify(req),
        });

        let temp = $match_data.match_key;
        $match_data = default_match_data;
        $match_data.match_key = temp;

        goto("/app/match");

        onMount(() => {
            localStorage.setItem("match_data", JSON.stringify($match_data));
        });
    }
</script>

<Header phase="Post Match" />
<Stage bind:value={$match_data.stage_enum} />
<Rating name="Driver Skill" bind:value={$match_data.skill} />
<Toggle text1="Broken" text2="Undamaged" bind:buttonon={$match_data.is_broke} />
<Toggle text1="Died on Field" text2="Functional" bind:buttonon={$match_data.is_died} />
<TextArea bind:value={$match_data.notes} />
<SubmitButton text="Submit" onClick={submit_match} />
