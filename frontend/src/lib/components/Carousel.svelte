<script lang="ts">
    // Hopefully my changes work, if they don't, uncomment the commented out each loop and submit match with a new socket connection from this component
    import { createEventDispatcher, onMount } from "svelte";
    import { match_data } from "$lib/stores";
    let currentIndex = 0;
    let startX = 0;

    import Prematch from "$lib/components/Prematch.svelte";
    import Auto from "$lib/components/Auto.svelte";
    import Tele from "$lib/components/Tele.svelte";
    import Postmatch from "$lib/components/Postmatch.svelte";
    // Import other slide components
    let items = [Prematch, Auto, Tele, Postmatch];

    function nextSlide() {
        if (currentIndex != 3) {
            currentIndex = (currentIndex + 1) % items.length;
        }
    }

    function prevSlide() {
        if (currentIndex != 0) {
            currentIndex = (currentIndex - 1 + items.length) % items.length;
        }
    }

    function handleTouchStart(event: any) {
        startX = event.touches[0].clientX;
    }

    function handleTouchEnd(event: any) {
        const endX = event.changedTouches[0].clientX;
        const deltaX = endX - startX;

        if (Math.abs(deltaX) > 50) {
            if (deltaX > 0) {
                prevSlide();
            } else {
                nextSlide();
            }
        }
    }

    // Optional: Add this if you want to prevent scrolling during swipe
    function preventDefault(event: any) {
        event.preventDefault();
    }

    onMount(() => {
        document.addEventListener("touchmove", preventDefault, {
            passive: false,
        });
        return () => {
            document.removeEventListener("touchmove", preventDefault);
        };
    });

    let dispatch = createEventDispatcher()
</script>

<div
    class="carousel-container"
    on:touchstart={handleTouchStart}
    on:touchend={handleTouchEnd}
>
    <div class="carousel" style="transform: translateX(-{currentIndex * 100}%)">

        <div class="carousel-item"><Prematch /> </div>
        <div class="carousel-item"><Auto /></div>
        <div class="carousel-item"><Tele /></div>
        <div class="carousel-item"><Postmatch on:submit_match={(event) => dispatch("submit_match", event.detail)}/></div>
        <!-- {#each items as Item, i (i)}
            <div class="carousel-item"><Item /></div>
        {/each} -->
    </div>
</div>

<style>
    .carousel-container {
        width: 100%;
        overflow: hidden;
        position: relative;
    }

    .carousel {
        display: flex;
        transition: transform 0.2s ease-in-out;
    }

    .carousel-item {
        min-width: 100%;
        box-sizing: border-box;
    }
</style>
