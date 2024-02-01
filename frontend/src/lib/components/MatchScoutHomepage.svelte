<script lang="ts">
    export let bteam1 = 5920;
    export let bteam2 = 5920;
    export let bteam3 = 5920;
    export let rteam1 = 5920;
    export let rteam2 = 5920;
    export let rteam3 = 5920;
    export let started = false
    import Pie from '$lib/components/Pie.svelte';
    let clicked = false
    export let selected = true
    import Navbar from "$lib/components/Navbar.svelte"
  let percents = [["Name", 20], ["Name", 50], ["Name", 0], ["Name", 0], ["Name", 20], ["Name", 0], ["Name", 0], ["Name", 0], ["Name", 0], ["Name", 0], ["Name", 0], ["Name", 0], ];
  
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
    $: timeuntilmatch = timegiven - time;
    $: if (timeuntilmatch > 60) {
        minutes = timeuntilmatch / 60;
        minutes = Math.round(minutes);
        minutes = minutes;
    } else {
        minutes = timeuntilmatch;
    }
    // messy time code (NO TOUCHIE)

    $: if (timeuntilmatch <= 0 && clicked == true && selected == true) {
        started = true
        started = started
    }
  </script>
  
  <div class="grid content-end pt-10">
    <h1 class="px-3 text-text_white pt-10">
        The next match starts in: 
    </h1>
    <h1
            style="width:auto"
            class="px-3 text-cresc_green"
        >
            {minutes}
            {timeuntilmatch <= 60 ?  timeuntilmatch == 1 ? "second" : "seconds" : minutes == 1 ? "minute" : "minutes"}.
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
  
  <div class="bg-btn_grey h-[185px] mx-3 grid-cols-4 grid gap-2 content-center items-center rounded-md mains">
    {#each percents as percent, i} 
      {#if i < 8} 
      <div class="p-1 flex flex-col items-center">
        <Pie size={46} percent={percent[1]}/>
        <p class="text-text_white">{percent[0]}</p>
      </div>
      {/if}  
    {/each}
  </div>
  {#if clicked == false}
  <div class="mains">
    <center>
  <button style="padding: 2.5rem" id="Match-Scounts" on:click={()=>clicked = true}> Join Queue </button>
  </center>
  </div>
  {:else}
  <div class="mains">
    <center>
    <button style="padding: 2.8rem" id="Match-Scouts" on:click={()=>clicked = false}> Leave Queue</button>
  </center>
    </div>

  {/if}
<div class="bottom-div">
<Navbar green3/>
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
        padding: 0.75rem
    }
  
    h1 {
        @apply font-bold text-3xl;
    }
    p {
       font-size: 16px
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
        font-size: 52px
    }
    #Match-Scouts {
        @apply text-navbar_black  py-5 font-semibold;
        font-size: 44px;
        background-color: #ED1C24;
    }
    .mains{
    margin-top: 2rem;
    margin-bottom: 2rem;
    }
  </style>
  