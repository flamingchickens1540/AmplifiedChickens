<script lang="ts">
    import Carousel from "$lib/components/Carousel.svelte";
    import QueuePage from "$lib/components/MatchScoutHomepage.svelte";

    // Write the push notification system to modify each of these things
    let queued = false;
    let selected = true;
    let started = false;
</script>

{#if queued == true && selected == true && started == true}
    <Carousel />
{:else}
    <QueuePage bind:clicked={queued} />
{/if}

{#if queued == true && selected == true && started == true}
    <Carousel />
{:else}
    <QueuePage bind:clicked={queued} />
{/if}
