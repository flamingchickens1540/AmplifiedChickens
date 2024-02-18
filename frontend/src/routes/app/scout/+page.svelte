<script lang="ts">
    import Carousel from "$lib/components/Carousel.svelte";
    import { match_data } from "$lib/stores";
    import type { PageData } from "./$types";

    export let data: PageData;

    $match_data.team_key = data as unknown as `frc${number}`; // FIXME: test data type
</script>

<Carousel />
