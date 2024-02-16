<script lang="ts">
  export let bteam1 = 5920;
  export let bteam2 = 5920;
  export let bteam3 = 5920;
  export let rteam1 = 5920;
  export let rteam2 = 5920;
  export let rteam3 = 5920;
<<<<<<< HEAD
  import Game from "$lib/components/Lottery.svelte";
  import Pie from "$lib/components/Pie.svelte";
  export let clicked = false;
  import { Modal, Content, Trigger } from "sv-popup";
  export let selected = false;
=======
  import Game from "$lib/components/HeheButton.svelte";
  import Pie from "$lib/components/Pie.svelte";
  export let clicked = false;
  import { Modal, Content, Trigger } from "sv-popup";
  export let selected = true;
>>>>>>> efa7ad755fe28f2d82c266b2b627fb6637c0756f
  import Navbar from "$lib/components/Navbar.svelte";
  let percents = [
    ["Name", 20],
    ["Name", 50],
    ["Name", 0],
    ["Name", 0],
    ["Name", 20],
    ["Name", 0],
    ["Name", 0],
    ["Name", 0],
    ["Name", 0],
    ["Name", 0],
    ["Name", 0],
    ["Name", 0],
  ];

  // messy time code (NO TOUCHIE)
  let minutes = 0;
  let time = 0;
  export let timeuntilmatch = 60;
  export let timegiven = 1706658804;
  var date = new Date(timegiven * 1000);
  var hours = date.getHours();
  var min = date.getMinutes();
  function addLeadingZero(number: number) {
    return number < 10 ? "0" + number : number;
  }
  var formattedTime = hours + ":" + addLeadingZero(min);
  export let match = 13;
  function getTimestamp() {
    return Math.floor(new Date().getTime() / 1000);
  }

  setInterval(() => (time = getTimestamp()), 1000);
  $: {
    timeuntilmatch = timegiven - time;
    if (timeuntilmatch > 60) {
      minutes = timeuntilmatch / 60;
      minutes = Math.round(minutes);
      minutes = minutes;
    } else {
      minutes = timeuntilmatch;
    }
    minutes = Math.max(minutes, 0);
  }
  // messy time code (NO TOUCHIE)
</script>

  <div class="grid content-end pt-10">
<<<<<<< HEAD
    <h1 class="px-3 text-text_white pt-10">The next match starts in:</h1>
=======
    <h1 class="px-3 text-text_white">The next match starts in:</h1>
>>>>>>> efa7ad755fe28f2d82c266b2b627fb6637c0756f
    <h1 style="width:auto" class="px-3 text-cresc_green">
      {minutes}
      {timeuntilmatch <= 60
        ? timeuntilmatch == 1
          ? "second"
          : "seconds"
        : minutes == 1
          ? "minute"
          : "minutes"}.
    </h1>
    <p class="px-3 text-outline_gray">
      Qualification match {match} starts at {formattedTime}
    </p>
  </div>

  <div
    class="grid grid-cols-3 grid-rows-2 gap-3 rounded mains"
    style="background-color: #5C5C5C; margin: 15px; padding:15px"
  >
    <center>
      <h2 class="rounded" style="background-color: #ED1C24;">{rteam1}</h2>
    </center>
    <center>
      <h2 class="rounded" style="background-color: #ED1C24;">{rteam2}</h2>
    </center>
    <center>
      <h2 class="rounded" style="background-color: #ED1C24;">{rteam3}</h2>
    </center>
    <center>
      <h2 class="rounded" style="background-color: #0083E6;">{bteam1}</h2>
    </center>
    <center>
      <h2 class="rounded" style="background-color: #0083E6;">{bteam2}</h2>
    </center>
    <center>
      <h2 class="rounded" style="background-color: #0083E6;">{bteam2}</h2>
    </center>
  </div>

  <div
    class="bg-btn_grey h-[185px] mx-3 grid-cols-4 grid gap-2 content-center items-center rounded-md mains"
  >
    {#each percents as percent, i}
      {#if i < 8}
        <div class="p-1 flex flex-col items-center">
          <Pie size={46} percent={percent[1]} />
          <p class="text-text_white">{percent[0]}</p>
        </div>
      {/if}
    {/each}
  </div>
  {#if clicked == false}<!-- if queue is not full -->
    <div class="mains">
      <center>
        <button
          style="padding: 2.5rem"
          id="Match-Scounts"
          on:click={() => (clicked = true)}
        >
          Join Queue
        </button>
      </center>
    </div>
  {:else if clicked == true}
<<<<<<< HEAD
  <div class="mains">
=======
  <div style="padding-bottom: 5rem;" class="mains">
>>>>>>> efa7ad755fe28f2d82c266b2b627fb6637c0756f
    <center>
      <button
        style="padding: 2.8rem"
        id="Match-Scouts"
        on:click={() => (clicked = false)}
      >
        Leave Queue</button
      >
    </center>
  </div>
  {:else}
    <!-- if queue is full -->
<<<<<<< HEAD
    <div class="mains">
      <center>
        <button
          style="padding: 2.5rem; padding-left: 4.5rem; padding-right:4.5rem"
=======
    <div style="padding-bottom: 5rem;" class="mains">
      <center>
        <button
          style="padding: 2.5rem; padding-left: 4.5rem; padding-right:4.5rem; "
>>>>>>> efa7ad755fe28f2d82c266b2b627fb6637c0756f
          id="Match-Scountss"
        >
          Queue Full
        </button>
      </center>
    </div>
  {/if}
<div class="bottom-div">
<<<<<<< HEAD
  <Navbar page="match" />
=======
  <Navbar green3 />
>>>>>>> efa7ad755fe28f2d82c266b2b627fb6637c0756f
</div>

<style lang="postcss">
  .bottom-div {
    position: fixed;
    bottom: 0;
    left: 0;
    width: 100%;
    background-color: #f0f0f0; /* Just for visualization */
    text-align: center;
  }
  h2 {
    font-family: poppins-bold;
    color: #ffffff;
    font-size: 24px;
    padding: 0.75rem;
  }

  h1 {
    @apply font-bold text-3xl;
  }
  p {
    font-size: 16px;
  }
  button {
    font-family: "Poppins-Bold";
    margin: 15px;
    margin-top: 10px;
    margin-bottom: 10px;
    @apply text-text_white bg-btn_grey py-2 rounded-md;
  }
  #Match-Scounts {
    @apply text-navbar_black bg-cresc_green py-5 font-semibold;
<<<<<<< HEAD
    font-size: 52px;
  }
  #Match-Scouts {
    @apply text-navbar_black  py-5 font-semibold;
    font-size: 44px;
=======
    font-size: 46px;
  }
  #Match-Scouts {
    @apply text-navbar_black  py-5 font-semibold;
    font-size: 39px;
>>>>>>> efa7ad755fe28f2d82c266b2b627fb6637c0756f
    background-color: #ed1c24;
  }
  #Match-Scountss {
    @apply py-5 font-semibold;
    font-size: 44px;
    background-color: #6c6c6c;
    color: #1c1c1c;
  }
  .mains {
<<<<<<< HEAD
    margin-top: 2rem;
    margin-bottom: 2rem;
=======
    margin-top: 1.5rem;
    margin-bottom: 1.5rem;
>>>>>>> efa7ad755fe28f2d82c266b2b627fb6637c0756f
  }
  h3 {
    margin: 17px;
    color: #ffffff;
    background-color: #5c5c5c;
    font-size: 3rem;
    padding: 1rem;
    text-align: center;
    /* Teams Remaining */
    font-family: "Poppins-medium";
    font-style: bold;
  }
</style>
