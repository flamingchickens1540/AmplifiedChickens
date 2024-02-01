<script lang="ts">
  import type { PageData } from "./$types";
  let greeting;

const currentTime = new Date().getHours();
  import Navbar from "$lib/components/Navbar.svelte"
if (currentTime < 12) {
  greeting = "Morning";
} else {
  greeting = "Afternoon";
}
  export let data: PageData;
  let name = "Test";

  import {spring, tweened} from 'svelte/motion';
  import Pie from '$lib/components/Pie.svelte';
  export let giventime = 1706579944;
  let percents = [["Name", 20], ["Name", 50], ["Name", 0], ["Name", 0], ["Name", 20], ["Name", 0], ["Name", 0], ["Name", 0], ["Name", 0], ["Name", 0], ["Name", 0], ["Name", 0], ];
</script>

<main class="bg-bg_gray h-screen flex flex-col justify-between">
  <div class="grid content-end">
    <h1 class="px-3 text-text_white pt-10"
    >Good {greeting}</h1>
    <h1 class="px-3 text-cresc_green"
    >{name}.</h1>
    <p class="px-3 text-outline_gray"
    >You are at 2024orore</p>
  </div>

  <div class="flex flex-col content-center items-stretch">
    <div style="margin-top: -2rem" class="bg-btn_grey h-[185px] mx-3 grid-cols-4 grid gap-2 content-center items-center rounded-md">
      {#each percents as percent, i} 
        {#if i < 8} 
        <div class="p-1 flex flex-col items-center">
          <Pie size={46} percent={percent[1]}/>
          <p class="text-text_white">{percent[0]}</p>
        </div>
        {/if}  
      {/each}
    </div>
    <div class="flex flex-row w-full content-center justify-around items-end">
      <button style="margin-top: 12px" class="w-full"
        onclick="location.href='https://www.thebluealliance.com/event/2024orsal';"
      > TheBlueAlliance </button>
      <button style="margin-left: 0px; margin-top: 12px" class="w-full" 
        onclick="location.href='https://www.statbotics.io/event/2024orsal';"
      > Statbotics </button>
    </div>
    <button style="margin-right: px" onclick="location.href='https://www.youtube.com/watch?v=dQw4w9WgXcQ';">
      Twitch Stream
    </button>
    <button style="padding: 1.5rem" id="Pit-Scounts"><a href="/app/pit">Pit Scout </button>
    <button style="padding: 2.5rem" id="Match-Scounts"><a href="/app/match"> Match Scout </button>
  </div>
    <div>
    <Navbar green1/>
  </div>
</main>

<style lang="postcss">

  h1 {
    @apply font-bold text-4xl
  }
  p {
    @apply font-medium text-xl
  }
  button {
    font-family: "Poppins-Medium";
    margin: 15px;
    margin-top: 8px;
    margin-bottom: 8px;
    font-size: 20px;
    padding: 1rem;
    @apply text-text_white bg-btn_grey py-2 rounded-md
    
  }
  #Match-Scounts {
    @apply text-navbar_black bg-cresc_green py-5 font-semibold text-4xl
  }
  #Pit-Scounts {
    @apply font-medium text-3xl
  }

</style>