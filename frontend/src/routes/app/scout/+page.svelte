<script lang="ts">
    import Carousel from "$lib/components/Carousel.svelte";
    import { match_data } from "$lib/stores";
    import { onMount } from "svelte";
    import type { PageData } from "./$types";

    export let data: PageData;

    onMount(() => {
        $match_data.team_key = data as unknown as `frc${number}`; // FIXME: test data type
    })
</script>

<Carousel />
