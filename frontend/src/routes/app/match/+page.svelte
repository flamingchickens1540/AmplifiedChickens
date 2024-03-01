<script lang="ts">
    import Carousel from "$lib/components/Carousel.svelte";
    import MatchScoutHomepage from "$lib/components/MatchScoutHomepage.svelte";
    import type { PageData } from "./$types";

    export let data: PageData

    let red: string[] = []

    let blue: string[] = []

</script>
<MatchScoutHomepage {blue} {red} access_token={data.accessToken ?? ""} match={data.match_key}/>
