<script lang="ts">
    import Navbar from "$lib/components/Navbar.svelte";
    import ScoutPercents from "$lib/components/ScoutPercents.svelte";
    import { onMount } from "svelte";
    import { match_data } from "$lib/stores";
    import { goto } from "$app/navigation";
    import type { PageData } from "./$types";
    import { access } from "fs";
    const BACKEND_URL = import.meta.env.VITE_BACKEND_URL;

    export let data: PageData;

    let blue: string[] = [];
    let red: string[] = [];
    let match: string = data.match_key;
    let scout_data: (string | number)[][] = data.scout_data as (
        | string
        | number
    )[][];
    let access_token = data.access_token as string;

    let in_queue = false
    let server_source: EventSource

    let event_listener = (event: any) => {
            console.log(event);
            var message = JSON.parse(event.data);
            console.log(message);
            // TODO: call timeToScout if the event is for for a new_match
        }

    async function joinQueue() {
        in_queue = true;

        let res = await fetch(`${BACKEND_URL}/scout/request_team`, {
            method: "GET",
            headers: {
                "Content-Type": "application/json",
                "x-access-code": access_token
            },
        })

        console.log(res)

        server_source.addEventListener("team_match", event_listener);
    }

    function leaveQueue() {
        server_source.removeEventListener("team_match", event_listener)

        in_queue = false
    }

    async function timeToScout() {
        in_queue = false
        $match_data.match_key = match
        goto("/app/scout")
    }

    onMount(() => {
        server_source = new EventSource(
            `${BACKEND_URL}/scout/sse/get/stream`,
        )
    })
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
    {#if in_queue == false}
        <button style="padding: 2.5rem" id="Match-Scouts" on:click={joinQueue}>
            Join Queue
        </button>
    {:else}
        <button
            style="padding: 2.5rem"
            id="Match-Scouts-Clicked"
            on:click={leaveQueue}
        >
            In Queue</button
        >
    {/if}
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
    #Match-Scouts {
        @apply text-navbar_black bg-cresc_green py-5 font-semibold;
        font-size: 52px;
    }

    #Match-Scouts-Clicked {
        @apply text-navbar_black bg-robot_red py-5 font-semibold;
        font-size: 52px;
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
