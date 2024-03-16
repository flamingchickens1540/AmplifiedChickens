<script lang="ts">
    import Carousel from "$lib/components/Carousel.svelte";
    import { match_data, team_color } from "$lib/stores";
    import { onMount } from "svelte";
    import type { PageData } from "./$types";
    import { default_match_data, type TeamMatchData } from "$lib/types";

    export let data: PageData;

    onMount(() => {
        let cached = localStorage.getItem("match_data");
        if (data.team_key == undefined || (cached != "" && cached != null)) {
            let data: TeamMatchData = JSON.parse(cached as string);
            console.log("received cache: ", data);
            $match_data = data;
            $team_color = localStorage.getItem("team_color") as
                | ""
                | "blue"
                | "red";
        } else {
            $match_data = default_match_data;
            $match_data.team_key = data.team_key as unknown as `${number}`;
            $match_data.scout_id = data.scout_id as string;
            $match_data.match_key = data.match_key;
            $team_color = data.team_color;
            localStorage.setItem("team_color", data.team_color);
        }
    });
</script>

<Carousel />
