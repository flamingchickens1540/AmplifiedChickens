<script lang="ts">
  import Carousel from "$lib/components/Carousel.svelte";
  import Homepage from "$lib/components/MatchScoutHomepage.svelte";
  let queued = false;
  export let selected = true;
  export let started = false;
</script>

{#if queued == true && selected == true && started == true}
  <Carousel />
{:else}
  <Homepage bind:clicked={queued} />
{/if}
