<script lang=ts>
    export let location = "far"
    </script>
    {#if location == "far"}
    <div>
    <h3 style="background-color: #00D586; color: #353839; border-top-left-radius: 0.4rem; border-bottom-left-radius: 0.4rem">✓Far</h3>
    <h3 style="color: #FFFFFF;" on:click={() => {location = "middle"}}>Middle</h3>
    <h3 style="color: #FFFFFF;" on:click={() => {location = "close"}}>Close</h3>
    </div>
    <!-- {:else if location == "middle"}
    <div class="grid grid-cols-2 grid-rows-1 place-items-center">
        <h3 class="grid col-start-1 col-span-1 row-start-1 text-center" style="color: #FFFFFF;" on:click={toggleon}>Stayed Home</h3>
        <h3 class="grid col-start-2 col-span-1 row-start-1 text-center" style="background-color: #00D586; color: #353839; border-top-right-radius: 0.4rem; ; border-bottom-right-radius: 0.4rem">✓Left Home</h3>
        </div>
    {/if} -->
    
    <style>
    h3{
    font-family: 'Poppins';
    font-style: normal;
    font-weight: 400;
    font-size: 1.3rem;
    letter-spacing: 0.5px;
    margin: 0;
    width: 100%;
    padding: 0.1rem;
    }
    div{
            margin: 25px;
            border-style: solid;
            border-width: 0.2rem;
            border-color: grey;
            border-radius: 10px;
            color: lightgrey;
            background-color: #353839;
        }
    </style>