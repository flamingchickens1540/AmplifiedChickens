<script lang="ts">
    import Carousel from "$lib/components/Carousel.svelte";
    import { match_data, team_color, manual } from "$lib/stores";
    import { onMount } from "svelte";
    import type { PageData } from "./$types";
    import { default_match_data, type TeamMatchData } from "$lib/types";
    import { browser } from "$app/environment";

    export let data: PageData;

    // The purpose of removing onMount is to make this code run before the onMOunts of mounted components run
    // onMount(() => {
    $manual = data.manual || false

    if (browser) {
        console.log(JSON.stringify($match_data))
        console.log(JSON.stringify(default_match_data))
        if (data.reload || JSON.stringify($match_data) != JSON.stringify(default_match_data)) {
            $team_color = localStorage.getItem("team_color") as
                | ""
                | "blue"
                | "red";
        } else if (data.manual == undefined) {
            console.log("New Match Triggered");
            $match_data.team_key = data.team_key as unknown as `${number}`;
            $match_data.scout_id = data.scout_id as string;
            $match_data.match_key = data.match_key;
            $team_color = data.team_color;
            localStorage.setItem("team_color", data.team_color);
        }
    }
    // });
</script>
<footer style="touch-action: manipulation;">
    <Carousel />
</footer>