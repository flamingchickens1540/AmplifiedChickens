<script lang="ts">
    import QueueMatch from "$lib/components/AdminDB/QueueMatch.svelte";
    import LastMatch from "$lib/components/AdminDB/LastMatch.svelte";
    import QueuedScouts from "$lib/components/AdminDB/QueuedScouts.svelte";
    import NumberScouted from "$lib/components/AdminDB/NumberScouted.svelte";
    import EventManagement from "$lib/components/AdminDB/EventManagement.svelte";
    import UserManagement from "$lib/components/AdminDB/UserManagement.svelte";

    export let data: PageData;

    let access_token: string = data.access_token ?? "";

    import type { PageData } from "./$types";
    import type { Scout, TeamKey, TeamMatch } from "$lib/types";
    import { onMount } from "svelte";
    import { io } from "socket.io-client";

    let all_scouts: Scout[] = [];
    let queued_scouts: string[] = []; //data.queued_scouts
    let scouted_robots: TeamMatch[] = [];

    const socket = io("https://scout.team1540.org/api");

    socket.on("connect", () => {
        console.log("Admin Connected to server");
    });

    socket.on(
        "team_match_assigned_admin",
        ({ team, scout_name, match_key }) => {
            console.log(
                scout_name,
                +" Scouting " + team + " in match " + match_key,
            );
            let team_match: TeamMatch = {
                team_key: team,
                match_key: match_key,
                scout_name: scout_name,
                status: "pending",
            };

            scouted_robots.push(team_match);
        },
    );

    onMount(async () => {
        const BACKEND_URL = import.meta.env.VITE_BACKEND_URL_FOR_FRONTEND;
        console.log("mounted");
        const sse_source = new EventSource(
            BACKEND_URL + "/admin/sse/get/stream",
        );
        sse_source.onmessage = (event) => {
            let data = JSON.parse(event.data);
            if (data["DeQueuedScout"] != undefined) {
                let scout_name = data["DeQueuedScout"];
                let index = queued_scouts.indexOf(scout_name);
                if (index != -1) {
                    queued_scouts.splice(index, 1);
                }
                console.log(scout_name);
            } else if (data["QueuedScout"] != undefined) {
                let scout_name = data["QueuedScout"];
                queued_scouts.push(scout_name);
                console.log(scout_name);
            }
            queued_scouts = queued_scouts;
            console.log(queued_scouts);
        };
    });

    function clear_scouts() {
        queued_scouts = [];
    }

    function queue_match_auto_handler(event: any) {
        console.log("Queue Match Auto");
        console.log(event.detail);

        socket.emit("new_match_auto", event.detail);
    }

    function queue_match_manual_handler(event: any) {
        console.log("Queue Match Manual");
        console.log(event.detail);

        socket.emit("new_match_manual", event.detail);
    }
</script>

<div
    style="background-color: #1C1C1C; padding:2rem; overflow: hidden;"
    class="grid grid-cols-2 gap-5 overflow-hidden"
>
    <div class="col-span-1 row-span-1 col-start-1 row-start-1">
        <QueueMatch
            on:queue_match_auto={queue_match_auto_handler}
            on:queue_match_manual={queue_match_manual_handler}
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
            <NumberScouted {access_token}/>
        </div>
        <div class="col-span-3">
            <EventManagement
                on:clear_scouts={clear_scouts}
                {access_token}
            />
        </div>
    </div>

    <div>
        <UserManagement bind:scouts={all_scouts} {access_token} />
    </div>
</div>
