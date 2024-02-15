<script lang="ts">
    import TextInput from "$lib/components/TextInput.svelte";
    import { createEventDispatcher } from 'svelte';

    const dispatch = createEventDispatcher();

    let event_name = "";

    export let access_token: string;

    async function set_event() {
        let opts = {
            method: "POST",
            headers: {
                "Content-Type": "application/json",
                "x-access-token": access_token,
                // 'Content-Type': 'application/x-www-form-urlencoded',
            },
            body: JSON.stringify(event_name), // body data type must match "Content-Type" header
        };
        let res = await fetch("https://localhost:3007/admin/newEvent", opts);
        if (res.ok) {
            console.log("Event set");
        } else {
            console.log("Event not set");
        }
    }

    async function refresh_tba() {
        let res = await fetch("https://localhost:3007/admin/tba/refresh", {
            method: "GET",
            headers: {
                "Content-Type": "application/json",
                "x-access-token": access_token,
                // 'Content-Type': 'application/x-www-form-urlencoded',
            }
        })
        console.log("Refresh tba: " + res)
    }

    async function load_teams() {
        let res = await fetch("https://localhost:3007/admin/tba/loadTeams", {
            method: "GET",
            headers: {
                "Content-Type": "application/json",
                "x-access-token": access_token,
                // 'Content-Type': 'application/x-www-form-urlencoded',
            }
        })
        console.log("Load teams: " + res)
    }
</script>

<div class="rounded" style="background-color: #2C2C2C; padding:1rem">
    <h3 style="margin:0.8rem">Event Management</h3>
    <div class="rounded" style="margin: 1rem">
        <TextInput bind:value={event_name} name="Event Key" />
    </div>
    <div class="grid grid-cols-2">
        <button on:click={set_event} class="rounded">Set Event</button>
        <button on:click={refresh_tba} class="rounded">Refresh TBA</button>
        <button on:click={load_teams} class="rounded">Load Teams</button>
        <button on:click={() => dispatch('clear_scouts', {})} class="rounded">Clear Scouts</button>
        <button on:click={() => dispatch('clear_teams', {})} class="rounded">Clear Teams</button>
    </div>
</div>

<style>
    h3 {
        font-family: "Poppins-bold";
        font-size: 23px;
        line-height: 91.5%;
        color: #ffffff;
        margin: 0.5rem;
        margin-top: 0rem;
    }
    h2 {
        font-family: "Poppins-bold";
        font-size: 20px;
        line-height: 91.5%;
        color: #2c2c2c;
        background-color: #00d586;
        padding-top: 0.4rem;
        padding-bottom: 0.4rem;
        margin: 2rem;
        margin-top: 0.5rem;
        margin-bottom: 0.5rem;
    }

    button {
        font-family: "Poppins-bold";
        font-size: 20px;
        line-height: 91.5%;
        color: #2c2c2c;
        background-color: #00d586;
        padding: 1.5rem;
        padding-top: 0.75rem;
        padding-bottom: 0.75rem;
        margin: 1rem;
    }
</style>
