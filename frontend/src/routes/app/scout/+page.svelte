<script lang="ts">
    import Carousel from "$lib/components/Carousel.svelte";
    import { match_data } from "$lib/stores";
    import { onMount } from "svelte";
    import type { PageData } from "./$types";

    export let data: PageData;

    onMount(() => {
        // FIXME: test data type
        $match_data.team_key = data.data as unknown as `frc${number}`
    })
</script>

<Carousel />
