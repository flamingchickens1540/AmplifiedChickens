<script lang="ts">
    import { type PageData } from "./$types";
    import Carousel from "$lib/components/Carousel.svelte";
    import MatchScoutHomepage from "$lib/components/MatchScoutHomepage.svelte";
    import { match_data } from "$lib/stores";
    import { io } from "socket.io-client";

    export let data: PageData

    let match = data.current_match ?? ""
    
    let scout_name = data.name ?? "Generic Name"

    let red: string[] = []

    let blue: string[] = []

    let scouting: boolean = false

    const socket = io();

    socket.on("connect", () => {
        console.log("Connected to server");
    });

    socket.on("team_to_scout", (team: string) => {
        $match_data.team_key = team as `${number}`;
        scouting = true;
    });

    function joinQueue() {
        socket.emit("join_queue", scout_name);
    }

    function leaveQueue() {
        socket.emit("leave_queue", scout_name);
    }

    function submit_match(event: any) {
        console.log("Submitting match")
        socket.emit("submit_team_match", event.detail)
        scouting = false
        console.log("Submitted match")
    }
</script>

{#if !scouting}
    <MatchScoutHomepage {blue} {red} {match} on:join={joinQueue} on:leave={leaveQueue} />
{:else}
    <Carousel on:submit_match={submit_match}/>
{/if}
