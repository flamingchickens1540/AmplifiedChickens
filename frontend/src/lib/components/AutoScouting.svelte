<script lang="ts">
    import WhereAuto from "./WhereAuto.svelte";
    import NumberInput from "./NumberInput.svelte";
    let list: { letter: string, time: number }[] = [];
    let pressed = false
    let teamNumber: Number
    let locationkinda = "onstage"
    let timer = 0

    function onclick() {
        pressed = true
        timer = 0
    }

    function updateTimer() {
        timer += 0.01
    }
    function H() {
        list.push({letter: "H", time: timer});
    }
    function P() {
        list.push({letter: "P", time: timer});
    }
    function G() {
        list.push({letter: "G", time: timer});
    }
    function F() {
        list.push({letter: "F", time: timer});  
    }
    function E() {
        list.push({letter: "E", time: timer});
    }
    function D() {
        list.push({letter: "D", time: timer});  
    }
    function C() {
        list.push({letter: "C", time: timer}); 
    }
    function B() {
        list.push({letter: "B", time: timer});
    console.log(list)
    }
    function A() {
        list.push({letter: "A", time: timer});
    }
    function submit() {
        const json = JSON.stringify(list);
        console.log(json);
        //dont forget to put team number
    }
    setInterval(updateTimer, 10)
</script>
{#if pressed == false}
<h2>Auto Scouting</h2>
<NumberInput name="Team Number" bind:value = {teamNumber}/>
<WhereAuto bind:value = {locationkinda}/>
<button class="realbutton" on:click={onclick}>Start Auto!</button>
{:else}


<h2>
    {teamNumber}
</h2>
<button class="realbutton" on:click={onclick}>Reset Timer</button>
<button style="background-color: orange;" class="realbutton" on:click={P}>Preload</button>
<div style="" class="grid grid-cols-2 grid-rows-5 gap-1 hello">
{#if locationkinda == "onstage"}
<button style="background-color: orange;" on:click={H}>H</button>
<button style="background-color: orange;" on:click={G} class="col-start-1 row-start-2">G</button>
<button style="background-color: orange;" on:click={F} class="col-start-1 row-start-3">F</button>
<button style="background-color: orange;" on:click={E} class="col-start-1 row-start-4">E</button>
<button style="background-color: orange;" on:click={D} class="col-start-1 row-start-5">D</button>
<button style="background-color: orange;" on:click={A} class="col-start-2 row-start-4">A</button>
<button style="background-color: orange;" on:click={B} class="col-start-2 row-start-3">B</button>
<button style="background-color: orange;" on:click={C} class="col-start-2 row-start-2">C</button>
{:else if locationkinda == "parked"}
<button style="background-color: orange;" on:click={H} class="col-start-2 row-start-1">H</button>
<button style="background-color: orange;" on:click={G} class="col-start-2 row-start-2">G</button>
<button style="background-color: orange;" on:click={F} class="col-start-2 row-start-3">F</button>
<button style="background-color: orange;" on:click={E} class="col-start-2 row-start-4">E</button>
<button style="background-color: orange;" on:click={D} class="col-start-2 row-start-5">D</button>
<button style="background-color: orange;" on:click={A} class="col-start-1 row-start-4">A</button>
<button style="background-color: orange;" on:click={B} class="col-start-1 row-start-3">B</button>
<button style="background-color: orange;" on:click={C} class="col-start-1 row-start-2">C</button>
{:else if locationkinda == "failed"}
<button style="background-color: orange;" on:click={H} class="col-start-2 row-start-5">H</button>
<button style="background-color: orange;" on:click={G} class="col-start-2 row-start-4">G</button>
<button style="background-color: orange;" on:click={F} class="col-start-2 row-start-3">F</button>
<button style="background-color: orange;" on:click={E} class="col-start-2 row-start-2">E</button>
<button style="background-color: orange;" on:click={D} class="col-start-2 row-start-1">D</button>
<button style="background-color: orange;" on:click={A} class="col-start-1 row-start-2">A</button>
<button style="background-color: orange;" on:click={B} class="col-start-1 row-start-3">B</button>
<button style="background-color: orange;" on:click={C} class="col-start-1 row-start-4">C</button>
{:else if locationkinda == "notattempted"}
<button style="background-color: orange;" on:click={H} class="col-start-1 row-start-5">H</button>
<button style="background-color: orange;" on:click={G} class="col-start-1 row-start-4">G</button>
<button style="background-color: orange;" on:click={F} class="col-start-1 row-start-3">F</button>
<button style="background-color: orange;" on:click={E} class="col-start-1 row-start-2">E</button>
<button style="background-color: orange;" on:click={D} class="col-start-1 row-start-1">D</button>
<button style="background-color: orange;" on:click={A} class="col-start-2 row-start-2">A</button>
<button style="background-color: orange;" on:click={B} class="col-start-2 row-start-3">B</button>
<button style="background-color: orange;" on:click={C} class="col-start-2 row-start-4">C</button>
{/if}
</div>
<button on:click={submit} style="background-color: #00D586;" class="realbutton">Submit!</button>
{/if}





<style>
         h2 {
        @apply text-4xl;
        color: white;
        margin: 15px;
        font-family: Poppins-bold;
        
    }
     h3 {
        font-size: 30px;
        color: white;
        margin: 15px;
        font-family: Poppins-bold;
        
    }
    p {
        @apply font-medium text-xl;
    }
    button {
        font-family: "Poppins-Medium";
        margin: 15px;
        margin-top: 8px;
        margin-bottom: 8px;
        font-size: 30px;
        padding: 1rem;
        @apply text-text_white bg-btn_grey py-2 rounded-md;
        
    }
    .realbutton{
        width: calc(100% - 30px);
        font-size: 20px; 
        padding: 0.3rem;
    }
    .hello{
        height: 60vh;
        margin-top: 25px;
        margin-bottom: 25px;
    }
</style>