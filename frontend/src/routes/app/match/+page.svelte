<script lang="ts">
  import Carousel from "$lib/components/Carousel.svelte"
  import Homepage from "$lib/components/MatchScoutHomepage.svelte"
  let queued = false
  export let selected = true
  export let started = true
</script>
{queued}
{#if queued == true && selected == true && started == true}
<Carousel/>
{:else}
<Homepage bind:clicked = {queued}/>
{/if}