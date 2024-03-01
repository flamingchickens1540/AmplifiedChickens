<script lang="ts">
    import Carousel from "$lib/components/Carousel.svelte";
    import MatchScoutHomepage from "$lib/components/MatchScoutHomepage.svelte";

    export let data: PageData

    let red = []

    let blue = []

</script>
<MatchScoutHomepage {blue} {red} access_token={data.accessToken}/>
