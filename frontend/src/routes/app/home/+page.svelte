<script lang="ts">
  import type { PageData } from "./$types";

  export let data: PageData;
  let name = "Test";

  import {spring, tweened} from 'svelte/motion';
  import Pie from './Pie.svelte';

  let percents = [["Name", 0], ["Name", 0], ["Name", 0], ["Name", 0], ["Name", 0], ["Name", 0], ["Name", 0], ["Name", 0], ["Name", 0], ["Name", 0], ["Name", 0], ["Name", 0], ];
</script>

<main class="bg-bg_gray h-screen flex flex-col justify-between">
  <div>
    <h1 class="px-3 text-text_white pt-6"
    >Good Afternoon</h1>
    <h1 class="px-3 text-cresc_green"
    >{name}</h1>
    <p class="px-3 text-outline_gray"
    >You are at 2024orore</p>
  </div>

  <div class="flex flex-col content-center">
    <div class="bg-btn_grey h-[185px] mx-3" style="border-radius: 3px;">
      {#each percents as percent, i} 
        <Pie size={200} percent={percent[1]}/>
        <p>{percent[0]}</p>
      {/each}
    </div>
    <div class="flex flex-row w-full content-center justify-around">
      <button class="w-full"
        onclick="location.href='https://www.thebluealliance.com/event/2024orsal';"
      > TheBlueAlliance </button>
      <button class="w-full" 
        onclick="location.href='https://www.statbotics.io/event/2024orsal';"
      > Statbotics </button>
    </div>
    <button
      onclick="location.href='https://www.statbotics.io/event/2024orsal';"
    >
      Twitch Stream
    </button>
    <button id="Pit-Scounts"> Pit Scount </button>
    <button id="Match-Scounts"> Match Scount </button>
  </div>

  <nav class="bg-navbar_black text-text_white h-[81px]">
    navbar goes here
  </nav>
</main>

<style lang="postcss">

  h1 {
    font-family: "Poppins-Bold";
    font-weight: 700;
    font-size: 35px;
    line-height: 35px;
  }
  p {
    font-family: "Poppins-Medium";
    font-weight: 500;
    font-size: 20px;
    line-height: 35px;

  }
  button {
    border-radius: 5px;
    font-family: "Poppins-Bold";
    @apply text-text_white bg-btn_grey m-3 
    py-2
  }
  #Match-Scounts {
    font-weight: 600;
    font-size: 40px;
    line-height: 35px;
    @apply text-navbar_black bg-cresc_green py-5;
  }
  #Pit-Scounts {
    font-weight: 600;
    font-size: 30px;
    line-height: 35px;
  }

</style>