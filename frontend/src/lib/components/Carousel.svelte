<script>
    import { onMount } from 'svelte';
  
    let currentIndex = 0;
    let startX = 0;
  
    import Slide1 from '$lib/components/Prematch.svelte';
    import Slide2 from '$lib/components/Auto.svelte';
    import Slide3 from '$lib/components/Tele.svelte';
    import Slide4 from '$lib/components/Postmatch.svelte';
    // Import other slide components
    let items = [Slide1, Slide2, Slide3, Slide4];
  
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