<script lang="ts">
    import Carousel from "$lib/components/Carousel.svelte";
    import MatchScoutHomepage from "$lib/components/MatchScoutHomepage.svelte";

    export let data: PageData

</script>
<MatchScoutHomepage access_token={data.0}/>
