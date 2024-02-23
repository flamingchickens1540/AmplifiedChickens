<script lang="ts">
    import Game from "$lib/components/Lottery.svelte";
    import Pie from "$lib/components/Pie.svelte";
    import { Modal, Content, Trigger } from "sv-popup";
    import Navbar from "$lib/components/Navbar.svelte";
    import ScoutPercents from "./ScoutPercents.svelte";

    export let blue: string[]
    export let red: string[]
    export let match: string
    export let selected: boolean
    export let timeuntilmatch: number
    export let timegiven: number
    export let access_token: string

    let clicked = false;

    // messy time code (NO TOUCHIE)
    let minutes = 0;
    let time = 0;
    let date = new Date(timegiven * 1000);
    let hours = date.getHours();
    let min = date.getMinutes();
    let formattedTime = hours + ":" + addLeadingZero(min);

    function addLeadingZero(number: number) {
        return number < 10 ? "0" + number : number;
    }
    function getTimestamp() {
        return Math.floor(new Date().getTime() / 1000);
    }

    setInterval(() => (time = getTimestamp()), 1000);
    $: {
        timeuntilmatch = timegiven - time;
        if (timeuntilmatch > 60) {
            minutes = timeuntilmatch / 60;
            minutes = Math.round(minutes);
            minutes = minutes;
        } else {
            minutes = timeuntilmatch;
        }
        minutes = Math.max(minutes, 0);
    }
    // messy time code (NO TOUCHIE)
    
    async function joinQueue() {
    	clicked = true
        let res = await fetch("https://scout.team1540.org/api/scout/queue", {
            method: "POST",
            headers: {
                "Content-Type": "application/json",
                "x-access-token": access_token
            },
        })

        if ((await res.json()).ok) {
            console.log("Queued user successfully")
        }
    }

    async function leaveQueue() {
    	clicked = false
        let res = await fetch("https://scout.team1540.org/api/scout/dequeue", {
            method: "POST",
            headers: {
                "Content-Type": "application/json",
                "x-access-token": access_token
            },
        })

        if ((await res.json()).ok) {
            console.log("Dequeued user successfully")
        }

    }
</script>

<div class="grid content-end pt-10">
    <h1 class="px-3 text-text_white pt-10">The next match starts in:</h1>
    <h1 style="width:auto" class="px-3 text-cresc_green">
        {minutes}
        {timeuntilmatch <= 60
            ? timeuntilmatch == 1
                ? "second"
                : "seconds"
            : minutes == 1
              ? "minute"
              : "minutes"}.
    </h1>
    <p class="px-3 text-outline_gray">
        Qualification match {match} starts at {formattedTime}
    </p>
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

<ScoutPercents />
{#if clicked == false}<!-- if queue is not full -->
    <div class="mains">
        <center>
            <button
                style="padding: 2.5rem"
                id="Match-Scounts"
                on:click={joinQueue}
            >
                Join Queue
            </button>
        </center>
    </div>
{:else if clicked == true}
    <div class="mains">
        <center>
            <button
                style="padding: 2.8rem"
                id="Match-Scouts"
                on:click={leaveQueue}
            >
                Leave Queue</button
            >
        </center>
    </div>
{:else}
    <!-- if queue is full -->
    <div class="mains">
        <center>
            <button
                style="padding: 2.5rem; padding-left: 4.5rem; padding-right:4.5rem"
                id="Match-Scountss"
            >
                Queue Full
            </button>
        </center>
    </div>
{/if}
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
