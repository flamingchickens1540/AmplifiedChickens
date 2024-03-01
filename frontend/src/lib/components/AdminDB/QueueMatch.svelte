<script lang="ts">
    import AssignStudent from "./AssignStudent.svelte";
    import type { Scout } from "$lib/types";
    import { match_data } from "$lib/stores"
    const BACKEND_URL = import.meta.env.VITE_BACKEND_URL;
    const TBA_API_KEY = import.meta.env.VITE_TBA_API_KEY

    export let red_teams: string[] = [];
    export let blue_teams: string[] = [];

    export let blue_scouts: Scout[] = [];
    export let red_scouts: Scout[] = [];

    export let queued_scouts: Scout[] = [];

    export let auto_assign: boolean;

    export let access_token = "";

    async function auto_populate() {
    console.log(TBA_API_KEY)
        let res = await fetch(
            `https://www.thebluealliance.com/api/v3/match/${$match_data.match_key}`,
            {
                // FIXME: DO NOT COMMIT API KEY
                headers: {
                    "X-TBA-Auth-Key": TBA_API_KEY,
                },
            },
        );
        let match = await res.json();

        console.log(match);
        red_teams = match.alliances.red.team_keys.map((key) => key.split('c')[1]);
        blue_teams = match.alliances.blue.team_keys.map((key) => key.split('c')[1]);
	console.log(blue_teams)
	console.log(red_teams)
	red_teams = red_teams
	blue_teams = blue_teams
    }

    async function queue_match() {
        console.log("Auto Assign: ", auto_assign)
        console.log("url ",  BACKEND_URL)
        if (auto_assign) {
            let res = await fetch(`${BACKEND_URL}/admin/new/match/auto`, {
                method: "POST",
                headers: {
                    "Accept": "application/json",
                    "Content-Type": "application/json",
                    "x-access-token": access_token,
                },
                body: JSON.stringify({teams: red_teams.concat(blue_teams), match_key: $match_data.match_key}),
            });

            console.log(res);

            red_teams = ["", "", ""];
            blue_teams = ["", "", ""];
            blue_scouts = [];
            red_scouts = [];

            red_teams = red_teams
            blue_teams = blue_teams
            blue_scouts = blue_scouts
            red_scouts = red_scouts
        } else {
            let res = await fetch(`${BACKEND_URL}/admin/new/match/manual`, {
                method: "POST",
                headers: {
                    Accept: "application/json",
                    "Content-Type": "application/json",
                    "x-access-token": access_token,
                },
                body: JSON.stringify({
                    teams: red_teams.concat(blue_teams),
                    scouts: queued_scouts,
		    match_key: $match_data.match_key 
                }),
            });

            console.log(res);

            red_teams = ["", "", ""];
            blue_teams = ["", "", ""];
            blue_scouts = [] 
            red_scouts = [];

            red_teams = red_teams
            blue_teams = blue_teams
            blue_scouts = blue_scouts
            red_scouts = red_scouts
        }
    }
</script>

<div class="rounded" style="background-color: #2C2C2C; padding:0.75rem">
    <div
        class="flex justify-between items-center rounded"
        style="background-color: #5C5C5C; padding:0.2rem; margin:17px"
    >
        <input class="" bind:value={$match_data.match_key} alt="Match Key" />
        <button on:click={auto_populate} class="rounded">Load Match</button>
    </div>
    <div class="grid grid-cols-2 grid-rows-1">
        <div>
            <AssignStudent
                bind:teams={red_teams}
                bind:selected={red_scouts}
                bind:queued={queued_scouts}
                bind:auto_assign
                color="#ED1C24"
            />
        </div>

        <div>
            <AssignStudent
                bind:teams={blue_teams}
                bind:selected={blue_scouts}
                bind:queued={queued_scouts}
                bind:auto_assign
                color="#0083E6"
            />
        </div>
    </div>
    <div class="grid grid-cols-2 grid-rows-1 place-items-center">
        <button
            on:click={() => (auto_assign = !auto_assign)}
            class="rounded {auto_assign ? 'green' : 'red'}">Auto Assign</button
        >
        <button
            on:click={queue_match}
            class="rounded"
            style="padding-top:0.9rem; padding-bottom: 0.9rem"
        >
            Queue Match
        </button>
    </div>
</div>

<style>
    h3 {
        font-family: "Poppins-bold";
        font-size: 20px;
        line-height: 91.5%;
        color: #ffffff;
        margin: 1rem;
    }
    h2 {
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

    input {
        font-family: "Poppins-bold";
        font-size: 20px;
        line-height: 91.5%;
        color: #2c2c2c;
        background-color: #5c5c5c;
        padding: 1.5rem;
        padding-top: 0.75rem;
        padding-bottom: 0.75rem;
        margin: 1rem;
    }

    *:focus {
        outline: none;
    }
</style>
