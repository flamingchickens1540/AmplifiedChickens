<script lang="ts">
    import type { PageData } from "./$types";
    import SubmitButton from "$lib/components/SubmitButton.svelte";
    import { Modal, Content, Trigger } from "sv-popup";
    import Navbar from "$lib/components/Navbar.svelte";
    import { goto } from "$app/navigation";
    import { onMount } from "svelte";
    import { current_event_key } from "$lib/stores";
    import ScoutPercents from "$lib/components/ScoutPercents.svelte";
    import { subscribeToPush } from "$lib/swutil.js";

    export let data: PageData;
    let twitchURL = "https://www.twitch.tv/firstinspires";

    let name = data.scout_name;
    let scout_data = data.scout_data_sorted;

    $current_event_key = data.current_event_key?.toString() || "2024pncmp";
    let subscription = "";

    onMount(() => {
        // TODO: fetch next match time
        subscription = localStorage.getItem("subscription") || "No subscription";
        localStorage.setItem("match_data", "");
        localStorage.setItem("team_color", "");
    });
</script>

<main class="bg-bg_gray h-screen flex flex-col justify-between">
    <div class="grid content-end">
        <h1 class="px-3 text-text_white pt-10">Good Afternoon</h1>
        <h1 class="px-3 text-cresc_green">{name}.</h1>
        <Modal>
            <Content
                style="background-color: #2C2C2C; width:92%; margin:auto; overflow-wrap: normal;
      overflow-wrap: break-word;
      overflow-wrap: anywhere;"
                class="p-4 rounded"
            >
                <h2 style="color: white;">Debug Menu:</h2>
                <SubmitButton
                    onClick={() => subscribeToPush(data?.access_token)}
                    text="Subscribe!"
                />
                <code style="font-size: 15px; color: white; overflow: wrap;"
                    >{subscription}</code
                >
            </Content>
            <Trigger>
                <p class="px-3 text-outline_gray">
                    You are at {$current_event_key}
                </p>
            </Trigger>
        </Modal>
    </div>

    <div class="flex flex-col content-center items-stretch">
        <ScoutPercents {scout_data} />
        <div
            class="flex flex-row w-full content-center justify-around items-end"
        >
            <a
                href="https://www.thebluealliance.com/event/{$current_event_key}"
                style="margin-left: 15px; margin-right: 15px;"
            >
                <button style="margin-left:0px; width: 100%">
                    TheBlueAlliance
                </button>
            </a>
            <a
                href="https://www.statbotics.io/event/{$current_event_key}"
                style="margin-left: 0px; margin-right: 15px; margin-top: 8px; width: 100%;"
            >
                <button
                    style="width:100%; margin-right: 15px; margin-left: 0px; padding-left: 0px; padding-right: 0px;"
                >
                    Statbotics
                </button>
            </a>
        </div>
        <a
            href="https://www.youtube.com/watch?v=uHgt8giw1LY"
            style="margin-left: 15px; margin-right: 15px;"
        >
            <button style="margin-left:0px; width: 100%">
                Twitch Stream
            </button>
        </a>
        <button
            style="padding: 1.5rem"
            id="Pit-Scounts"
            on:click={() => goto("/app/pit")}>Pit Scout</button
        >
        <button
            style="padding: 2.5rem"
            id="Match-Scounts"
            on:click={() => goto("/app/match")}>Match Scout</button
        >
    </div>
    <footer>
        <Navbar page="home" />
    </footer>
</main>

<style lang="postcss">
    h1 {
        @apply font-bold text-4xl;
    }
    h2 {
        @apply text-4xl;
        color: white;
        margin: 15px;
        font-family: Poppins-bold;
    }
    p {
        @apply font-medium text-xl;
    }
    button {
        font-family: "Poppins-Medium";
        margin: 15px;
        margin-top: 8px;
        margin-bottom: 8px;
        font-size: 20px;
        padding: 1rem;
        @apply text-text_white bg-btn_grey py-2 rounded-md;
    }
    #Match-Scounts {
        @apply text-navbar_black bg-cresc_green py-5 font-semibold text-4xl;
    }
    #Pit-Scounts {
        @apply font-medium text-3xl;
    }
</style>
