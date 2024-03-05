<!-- <script lang="ts">
    import type { PageData } from "./$types";
    import { goto } from "$app/navigation";
    import { redirect } from "@sveltejs/kit";
	const REDIRECT_URL = import.meta.env.VITE_SLACK_REDIRECT_URL;

    // export let data: PageData;

</script>

<center>
    <img src="/logo.png" />
    <h2>Welcome</h2>
    <h3>to Amplified Chickens</h3>
    
    <a href="https://slack.com/openid/connect/authorize?scope=openid%20email%20profile&amp;response_type=code&amp;redirect_uri=https%3A%2F%2Fscout.team1540.org%2Fapi%2Fauth%2Fslack&amp;client_id=10700242916.6475903895858" style="align-items:center;color:#fff;background-color:#4A154B;border:0;border-radius:4px;display:inline-flex;font-family:Lato, sans-serif;font-size:16px;font-weight:600;height:48px;justify-content:center;text-decoration:none;width:256px"><svg xmlns="http://www.w3.org/2000/svg" style="height:20px;width:20px;margin-right:12px" viewBox="0 0 122.8 122.8"><path d="M25.8 77.6c0 7.1-5.8 12.9-12.9 12.9S0 84.7 0 77.6s5.8-12.9 12.9-12.9h12.9v12.9zm6.5 0c0-7.1 5.8-12.9 12.9-12.9s12.9 5.8 12.9 12.9v32.3c0 7.1-5.8 12.9-12.9 12.9s-12.9-5.8-12.9-12.9V77.6z" fill="#e01e5a"></path><path d="M45.2 25.8c-7.1 0-12.9-5.8-12.9-12.9S38.1 0 45.2 0s12.9 5.8 12.9 12.9v12.9H45.2zm0 6.5c7.1 0 12.9 5.8 12.9 12.9s-5.8 12.9-12.9 12.9H12.9C5.8 58.1 0 52.3 0 45.2s5.8-12.9 12.9-12.9h32.3z" fill="#36c5f0"></path><path d="M97 45.2c0-7.1 5.8-12.9 12.9-12.9s12.9 5.8 12.9 12.9-5.8 12.9-12.9 12.9H97V45.2zm-6.5 0c0 7.1-5.8 12.9-12.9 12.9s-12.9-5.8-12.9-12.9V12.9C64.7 5.8 70.5 0 77.6 0s12.9 5.8 12.9 12.9v32.3z" fill="#2eb67d"></path><path d="M77.6 97c7.1 0 12.9 5.8 12.9 12.9s-5.8 12.9-12.9 12.9-12.9-5.8-12.9-12.9V97h12.9zm0-6.5c-7.1 0-12.9-5.8-12.9-12.9s5.8-12.9 12.9-12.9h32.3c7.1 0 12.9 5.8 12.9 12.9s-5.8 12.9-12.9 12.9H77.6z" fill="#ecb22e"></path></svg>Sign in with Slack</a>
</center>

<style>
    h2 {
        margin-top: 20px;
        font-family: "Poppins-bold";
        font-weight: 700;
        font-size: 40px;
        line-height: 40px;
        color: #ffffff;
    }
    h3 {
        font-family: "Poppins-medium";
        font-size: 20px;
        line-height: 40px;
        color: #ffffff;
    }

    a {
        position: absolute;
        bottom: 50px;
        left: 0px;
        right: 0px;
        margin: auto;
        transform: scale(1.2);
    }
</style> -->

<script lang="ts">
    import Game from "$lib/components/Lottery.svelte";
    import Pie from "$lib/components/Pie.svelte";
    import { Modal, Content, Trigger } from "sv-popup";
    import Navbar from "$lib/components/Navbar.svelte";
    import ScoutPercents from "$lib/components/ScoutPercents.svelte";
    import { createEventDispatcher } from "svelte";
    import { match_data } from "$lib/stores";
    import { goto } from "$app/navigation";
    const BACKEND_URL = import.meta.env.VITE_BACKEND_URL_FOR_FRONTEND;

    export let blue: string[]
    export let red: string[]
    export let match: string
    export let access_token: string
    export let scout_percents: number[]
    export let scout_names: string[]

    let clicked = false;

    async function joinQueue() {
    	$match_data.match_key = match 
        goto("/app/scout");
    }
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
        <h2 class="rounded" style="background-color: #ED1C24;">{red[0] ?? ""}</h2>
    </center>
    <center>
        <h2 class="rounded" style="background-color: #ED1C24;">{red[1] ?? ""}</h2>
    </center>
    <center>
        <h2 class="rounded" style="background-color: #ED1C24;">{red[2] ?? ""}</h2>
    </center>
    <center>
        <h2 class="rounded" style="background-color: #0083E6;">{blue[0] ?? ""}</h2>
    </center>
    <center>
        <h2 class="rounded" style="background-color: #0083E6;">{blue[1] ?? ""}</h2>
    </center>
    <center>
        <h2 class="rounded" style="background-color: #0083E6;">{blue[2] ?? ""}</h2>
    </center>
</div>
{/if}

<ScoutPercents {scout_percents} {scout_names} />
    <div class="mains grid place-items-center">
            <button
                style="padding: 2.5rem"
                id="Match-Scounts"
                on:click={joinQueue}
            >
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
