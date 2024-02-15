<script lang="ts">
    import AssignStudent from "./AssignStudent.svelte";

    export let red_teams: string[] = [];
    export let blue_teams: string[] = [];

    export let blue_scouts: string[] = [];
    export let red_scouts: string[] = [];

    export let match_key = ""

    async function auto_populate() {
        let res = await fetch(
            `https://www.thebluealliance.com/api/v3/match/${match_key}`,
            {
                // FIXME: DO NOT COMMIT API KEY
                headers: { "X-TBA-Auth-Key": "N4U95xQAy80xNrlc6ZEEAM8bKCCimCgKTckEG8zVQViQomM3GpZwVQ8qhtwWsBqc" },
            },
        );
        let match = await res.json();

        console.log(match)
        red_teams = match.alliances.red.team_keys;
        blue_teams = match.alliances.blue.team_keys;
    }

    async function queue_match() {
        let res = await fetch("https://localhost:3007/")
    }
</script>

<div class="rounded" style="background-color: #2C2C2C; padding:0.75rem">
    <div
        class="flex justify-between items-center rounded"
        style="background-color: #5C5C5C; padding:0.2rem; margin:17px"
    >
        <input class="" bind:value={match_key} alt="Match Key" />
        <button on:click={auto_populate} class="rounded">Load Match</button>
    </div>
    <div class="grid grid-cols-2 grid-rows-1">
        <div>
            <AssignStudent
                bind:teams={red_teams}
                bind:selected={red_scouts}
                color="#ED1C24"
            />
        </div>

        <div>
            <AssignStudent
                bind:teams={blue_teams}
                bind:selected={blue_scouts}
                color="#0083E6"
            />
        </div>
    </div>
    <center
        ><button on:click={queue_match}
            class="rounded"
            style="padding-top:0.9rem; padding-bottom: 0.9rem"
        >
            Queue Match
        </button></center
    >
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
