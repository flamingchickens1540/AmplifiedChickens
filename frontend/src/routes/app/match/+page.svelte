<script>
  import Carousel from "$lib/components/Carousel.svelte"
  import Homepage from "$lib/components/MatchScoutHomepage.svelte"
  let started = false
</script>
{#if !started}
<Homepage bind:started={started}/>
{/if}
{#if started == true}
<Carousel/>
{/if}