<script lang=ts>
    export let location = "far"
    </script>
    {#if location == "far"}
    <div>
        <h3 style="background-color: #00D586; color: #353839; border-top-left-radius: 0.4rem; border-top-right-radius: 0.4rem">✓Far</h3>
        <h3 class="middle" style="color: #FFFFFF;" on:click={() => {location = "middle"}}>Middle</h3>
        <h3 style="color: #FFFFFF;" on:click={() => {location = "close"}}>Close</h3>
        </div>
    {:else if location == "middle"}
    <div>
        <h3  style="color: #FFFFFF;" on:click={() => {location = "far"}}>Far</h3>
        <h3  class= "middle" style="background-color: #00D586; color: #353839">✓Middle</h3>
        <h3 style="color: #FFFFFF;" on:click={() => {location = "close"}}>Close</h3>
        </div>
    {:else}
    <div>
        <h3  style="color: #FFFFFF;" on:click={() => {location = "far"}}>Far</h3>
        <h3 class="middle" style="color: #FFFFFF;" on:click={() => {location = "middle"}}>Middle</h3>
        <h3   style="background-color: #00D586; color: #353839; border-bottom-left-radius: 0.4rem; border-bottom-right-radius: 0.4rem">✓Close</h3>
        </div>
    {/if} 
    
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
.middle{
    border-style: solid;
            border-width: 0.2rem;
            border-color: grey;
            border-left: 0;
            border-right: 0;
}
    </style>