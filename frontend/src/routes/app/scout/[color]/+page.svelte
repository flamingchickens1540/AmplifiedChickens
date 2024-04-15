<script lang="ts">
    import Carousel from "$lib/components/Carousel.svelte";
    import { match_data, team_color, manual } from "$lib/stores";
    import { onMount } from "svelte"
    import type { PageData } from "./$types";
    import { default_match_data, type TeamMatchData } from "$lib/types";
    import { browser } from "$app/environment";

    export let data: PageData;

    // The purpose of removing onMount is to make this code run before the onMOunts of mounted components run
    // onMount(() => {
    $manual = data.manual as unknown as boolean
    console.log("color: " + $team_color)

    if (browser) {
        console.log(JSON.stringify($match_data))
        console.log(JSON.stringify(default_match_data))

        if ($manual) {
            console.log("scout manual")
            console.log("color2 " + $team_color)
            
            $match_data.match_key = "2024gapsc_pm" + Math.floor(Math.random() * 1000)
            $match_data.scout_id = data.scout_id as string;
        } else if (data.manual == false) {
            console.log("New Match Triggered");
            $match_data.team_key = data.team_key as unknown as `${number}`;
            $match_data.scout_id = data.scout_id as string;
            $match_data.match_key = data.match_key as unknown as string;
            $team_color = data.team_color as unknown as "" | "red" | "blue";
            localStorage.setItem("team_color", data.team_color as unknown as string);
        } else {
            alert("Azalea made a mistake, please contact an admin")
        }
    }
    // });
</script>

<Carousel />
