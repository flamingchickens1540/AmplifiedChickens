<script lang="ts">
    import TextInput from "$lib/components/TextInput.svelte";
    import NumberInput from "$lib/components/NumberInput.svelte";
    import Toggle from "$lib/components/Toggle.svelte";
    import Threeoption from "$lib/components/Threeoption.svelte";
    import Rating from "$lib/components/Rating.svelte";
    import Textarea from "$lib/components/TextArea.svelte";
    import SubmitButton from "$lib/components/SubmitButton.svelte";
    import TeamsRemainingPopup from "$lib/components/TeamsRemainingPopup.svelte";
    import Navbar from "$lib/components/Navbar.svelte";
    import ImageUpload from "$lib/components/ImageUpload.svelte";
    import { Modal, Content, Trigger } from "sv-popup";
    import { pit } from "$lib/stores";
    import type { Team } from "$lib/types";
    import type { PageData } from "./$types";
    import { onMount } from "svelte";

    export let data: PageData
    export let team_key = ""

    let intake = ""
    let remaining_teams: string[] = []
    onMount(async () => {
        remaining_teams = await get_remaining_teams()
    })
    
    $: {
        if (intake == "Both") {
            $pit.is_ground_intake = true;
            $pit.is_chute_intake = true;
        } else if (intake == "Ground") {
            $pit.is_ground_intake = true;
            $pit.is_chute_intake = false;
        } else {
            $pit.is_ground_intake = false;
            $pit.is_chute_intake = true;
        }
    }

    async function handle_submit() {
        let req: any = { id: data.scout_id }
        req.push($pit)

        let res = fetch("https://scout.team1540.org/api/submit/pit", {
            method: "POST",
            headers: {
                "Content-Type": "application/json",
            },
            body: JSON.stringify(req).concat(),
        });
    }

    async function get_remaining_teams() {
        let res = await fetch("https://scout.team1540.org/api/scout/get/unpitted");
        let json = await res.json();
        console.log("remaining teams: ", json);
        return json;
    }

    
</script>

<Modal>
    {#if team_key == ""}
        <Content
            style="background-color: #2C2C2C; width:92%; margin:auto"
            class="p-4 rounded"
        >
            <TeamsRemainingPopup bind:value={team_key} bind:remaining_teams={remaining_teams} />
        </Content>
    {/if}
    <Trigger>
        <SubmitButton text="Teams Remaining" />
    </Trigger>
</Modal>
<TextInput  name="Team Number" bind:value={$pit.team_key} />
<NumberInput name="Width (ft)" bind:value={$pit.width} />
<NumberInput name="Length (ft)" bind:value={$pit.length} />
<NumberInput name="Weight (lbs)" bind:value={$pit.weight} />
<Toggle
    text1="Under Stage"
    text2="Around Stage"
    bind:buttonon={$pit.is_short}
/>
<Threeoption
    text1="Swerve"
    text2="Tank"
    text3="Other"
    bind:value={$pit.drivetrain}
/>
<Threeoption text1="Chute" text2="Ground" text3="Both" bind:value={intake} />
<Rating name="Robot Polish" bind:value={$pit.polish} />
<Textarea bind:value={$pit.notes} />
<ImageUpload />
<div id="navbar">
    <Navbar page="pit" />
</div>

<style>
    #navbar {
        position: fixed;
        bottom: 0;
        left: 0;
        width: 100%;
        background-color: #f0f0f0; /* Just for visualization */
        text-align: center;
    }
</style>
