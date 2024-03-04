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
    import { goto } from "$app/navigation"
    import type { PageData } from "./$types";
    import { onMount } from "svelte";
    const BACKEND_URL = import.meta.env.VITE_BACKEND_URL;

    export let data: PageData;
    let team_key = "";

    let intake = "";
    let remaining_teams: string[] = data.unpittscouted_teams 

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
    	console.log("Pit scout submit called")
        let req: any = { id: 0, ...$pit, scout_id: data.scout_id, event_key: "2024orsal"};

	delete req["weight"]

	console.log(req)

        let res = await fetch(`${BACKEND_URL}/submit/pit`, {
            method: "POST",
            headers: {
                "Content-Type": "application/json",
            },
            body: JSON.stringify(req)
        });

	console.log("PITSCOUTING ", res)

	// goto("/app/home")
    }

</script>

<Modal>
    {#if team_key == ""}
        <Content
            style="background-color: #2C2C2C; width:92%; margin:auto"
            class="p-4 rounded"
        >
            <TeamsRemainingPopup bind:value={team_key} bind:remaining_teams />
        </Content>
    {/if}
    <Trigger>
        <SubmitButton text="Teams Remaining" />
    </Trigger>
</Modal>
<TextInput name="Team Number" bind:value={$pit.team_key} />
<NumberInput name="Width (in)" bind:value={$pit.width} />
<NumberInput name="Length (in)" bind:value={$pit.length} />
<NumberInput name="Weight (lbs)" bind:value={$pit.weight} />
<!-- We don't care about binding value and buttonon to both of these, since we want a boolean from one and a string for the other -->
<Toggle
    text1="Under Stage"
    text2="Around Stage"
    bind:buttonon={$pit.is_short}
/>
<Toggle text1="Has Camera" text2="No Camera" bind:buttonon={$pit.is_camera} />
<Threeoption
    text1="Swerve"
    text2="Tank"
    text3="Other"
    bind:value={$pit.drivetrain}
/>
<Threeoption text1="Chute" text2="Ground" text3="Both" bind:value={intake} />
<Rating name="Robot Polish" bind:value={$pit.polish} />
<Textarea bind:value={$pit.notes} />
<SubmitButton text="Submit!" onClick={handle_submit} />
<Navbar page="pit" />
