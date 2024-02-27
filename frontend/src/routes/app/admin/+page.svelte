<script lang="ts">
    import QueueMatch from "$lib/components/AdminDB/QueueMatch.svelte";
    import LastMatch from "$lib/components/AdminDB/LastMatch.svelte";
    import QueuedScouts from "$lib/components/AdminDB/QueuedScouts.svelte";
    import NumberScouted from "$lib/components/AdminDB/NumberScouted.svelte";
    import EventManagement from "$lib/components/AdminDB/EventManagement.svelte";
    import UserManagement from "$lib/components/AdminDB/UserManagement.svelte";

    import type { PageData } from "./$types";
    import type { Scout, TeamKey, TeamMatch } from "$lib/types";
    import { onMount } from "svelte";

    export let data: PageData;

    console.log("DATA ", data)

    let access_token = data.access_token as string;

    let red_teams: (TeamKey | "")[] = ["", "", ""];
    let blue_teams: (TeamKey | "")[] = ["", "", ""];

    let all_scouts: Scout[] = [];
    let queued_scouts: string[] = []; //data.queued_scouts

    let auto_assign: boolean = false;

    let scouted_robots: TeamMatch[] = [];

    onMount(async () => {
        const BACKEND_URL = import.meta.env.VITE_BACKEND_URL_FOR_FRONTEND;
        console.log("mounted");
    });

    function clear_teams() {
        red_teams = [];
        blue_teams = [];
    }

    function clear_scouts() {
        queued_scouts = [];
    }
</script>

<div
    style="background-color: #1C1C1C; padding:2rem; overflow: hidden;"
    class="grid grid-cols-2 gap-5 overflow-hidden"
>
    <div class="col-span-1 row-span-1 col-start-1 row-start-1">
        <QueueMatch
            bind:red_teams
            bind:blue_teams
            bind:auto_assign
            {access_token}
        />
    </div>

    <div class="grid grid-cols-2 grid-rows-1 gap-5">
        <div>
            <LastMatch bind:scouted_robots />
        </div>
        <div>
            <QueuedScouts bind:queued={queued_scouts} />
        </div>
    </div>


    <div class="grid grid-cols-5 gap-5">
        <div class="col-span-2">
            <NumberScouted {access_token} />
        </div>
        <div class="col-span-3">
            <EventManagement
                on:clear_scouts={clear_scouts}
                on:clear_teams={clear_teams}
                {access_token}
            />
        </div>
    </div>

    <div>
        <UserManagement bind:scouts={all_scouts} {access_token} />
    </div>
</div>
