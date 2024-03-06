<script lang="ts">
    import Game from "$lib/components/Lottery.svelte";
    import Pie from "$lib/components/Pie.svelte";
    import { Modal, Content, Trigger } from "sv-popup";
    import Navbar from "$lib/components/Navbar.svelte";
    import ScoutPercents from "$lib/components/ScoutPercents.svelte";
    import { createEventDispatcher, onMount } from "svelte";
    import { match_data } from "$lib/stores";
    import { goto } from "$app/navigation";
    import { count } from "$lib/stores";
    const BACKEND_URL = import.meta.env.VITE_BACKEND_URL;

    export let blue: string[];
    export let red: string[];
    export let match: string;
    export let access_token: string;
    export let scout_data: (string | number)[][] = [];

    let in_queue = false;

    let clicked = false;
    function joinQueue() {
        in_queue = true;
    }

    function leaveQueue() {
        in_queue = false;
    }

    async function timeToScout() {
        in_queue = false;
        $match_data.match_key = match;
        goto("/app/scout");
    }

    onMount(() => {
        const server_source = new EventSource(
            `${BACKEND_URL}/scout/sse/get/stream`,
        );

        server_source.addEventListener("team_match", (event) => {
            console.log(event);
            var message = JSON.parse(event.data);
            console.log(message);
        });
    });
</script>

<div class="grid content-end pt-10">
    <h1 class="px-3 text-text_white pt-10">The next match will be</h1>
    <h1 style="width:auto" class="px-3 text-cresc_green">
        {match}
    </h1>
</div>

{#if blue.length != 0 || red.length != 0}
    <div
        class="grid grid-cols-3 grid-rows-2 gap-3 rounded mains"
        style="background-color: #5C5C5C; margin: 15px; padding:15px"
    >
        <center>
            <h2 class="rounded" style="background-color: #ED1C24;">
                {red[0] ?? ""}
            </h2>
        </center>
        <center>
            <h2 class="rounded" style="background-color: #ED1C24;">
                {red[1] ?? ""}
            </h2>
        </center>
        <center>
            <h2 class="rounded" style="background-color: #ED1C24;">
                {red[2] ?? ""}
            </h2>
        </center>
        <center>
            <h2 class="rounded" style="background-color: #0083E6;">
                {blue[0] ?? ""}
            </h2>
        </center>
        <center>
            <h2 class="rounded" style="background-color: #0083E6;">
                {blue[1] ?? ""}
            </h2>
        </center>
        <center>
            <h2 class="rounded" style="background-color: #0083E6;">
                {blue[2] ?? ""}
            </h2>
        </center>
    </div>
{/if}

<ScoutPercents {scout_data} />
<div class="mains grid place-items-center">
    <button style="padding: 2.5rem" id="Match-Scounts" on:click={joinQueue}>
        Get Robot
    </button>
</div>
<div class="bottom-div">
    <Navbar page="match" />
</div>

<style lang="postcss">
    .bottom-div {
        position: fixed;
        bottom: 0;
        left: 0;
        width: 100%;
        background-color: #f0f0f0; /* Just for visualization */
        text-align: center;
    }
    h2 {
        font-family: poppins-bold;
        color: #ffffff;
        font-size: 24px;
        padding: 0.75rem;
    }

    h1 {
        @apply font-bold text-3xl;
    }
    p {
        font-size: 16px;
    }
    button {
        font-family: "Poppins-Bold";
        margin: 15px;
        margin-top: 10px;
        margin-bottom: 10px;
        @apply text-text_white bg-btn_grey py-2 rounded-md;
    }
    #Match-Scounts {
        @apply text-navbar_black bg-cresc_green py-5 font-semibold;
        font-size: 52px;
    }
    #Match-Scouts {
        @apply text-navbar_black  py-5 font-semibold;
        font-size: 44px;
        background-color: #ed1c24;
    }
    #Match-Scountss {
        @apply py-5 font-semibold;
        font-size: 44px;
        background-color: #6c6c6c;
        color: #1c1c1c;
    }
    .mains {
        margin-top: 2rem;
        margin-bottom: 2rem;
    }
    h3 {
        margin: 17px;
        color: #ffffff;
        background-color: #5c5c5c;
        font-size: 3rem;
        padding: 1rem;
        text-align: center;
        /* Teams Remaining */
        font-family: "Poppins-medium";
        font-style: bold;
    }
</style>
