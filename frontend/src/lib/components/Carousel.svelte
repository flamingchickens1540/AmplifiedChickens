<script>
    import { onMount } from 'svelte';
<<<<<<< HEAD
  
=======
    import { match_data } from '$lib/stores/matchStores.ts';
>>>>>>> efa7ad755fe28f2d82c266b2b627fb6637c0756f
    let currentIndex = 0;
    let startX = 0;
    
    import Prematch from '$lib/components/Prematch.svelte';
    import Auto from '$lib/components/Auto.svelte';
    import Tele from '$lib/components/Tele.svelte';
    import Postmatch from '$lib/components/Postmatch.svelte';
    // Import other slide components
    let items = [Prematch, Auto, Tele, Postmatch];
  
    function nextSlide() {
      if (currentIndex != 3){
      currentIndex = (currentIndex + 1) % items.length;}
    }
  
    function prevSlide() {
      if (currentIndex != 0){
      currentIndex = (currentIndex - 1 + items.length) % items.length;}
    }
  
    function handleTouchStart(event) {
      startX = event.touches[0].clientX;
    }
  
    function handleTouchEnd(event) {
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
    function preventDefault(event) {
      event.preventDefault();
    }
  
    onMount(() => {
      document.addEventListener('touchmove', preventDefault, { passive: false });
      return () => {
        document.removeEventListener('touchmove', preventDefault);
      };
    });
  </script>
  
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
<<<<<<< HEAD
  
=======

>>>>>>> efa7ad755fe28f2d82c266b2b627fb6637c0756f
  <div
    class="carousel-container"
    on:touchstart={handleTouchStart}
    on:touchend={handleTouchEnd}
  >
    <div class="carousel" style="transform: translateX(-{currentIndex * 100}%)">
      {#each items as Item, i (i)}
        <div class="carousel-item"><Item /></div>
      {/each}
    </div>
  </div>