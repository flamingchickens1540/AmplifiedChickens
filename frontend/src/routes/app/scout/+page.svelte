<script lang="ts">
    import Carousel from "$lib/components/Carousel.svelte";
    import { match_data } from "$lib/stores";
    import { onMount } from "svelte";
    import type { PageData } from "./$types";

    export let data: PageData;

    onMount(() => {
        $match_data.team_key = data.team_key as unknown as `${number}`
	$match_data.scout_id = data.scout_id
	$match_data.match_key = data.match_key
    })
</script>

<Carousel />
