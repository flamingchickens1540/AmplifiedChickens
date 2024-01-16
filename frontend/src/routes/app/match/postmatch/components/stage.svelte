<script lang=ts>
    export let status = "Onstage"
    </script>
    {#if status == "Onstage"}
    <div>
        <h3 style="background-color: #00D586; color: #353839; border-top-left-radius: 0.4rem; border-top-right-radius: 0.4rem">✓Onstage</h3>
        <h3 class="middle" style="color: #FFFFFF;" on:click={() => {status = "Parked"}}>Parked</h3>
        <h3 class="middlealt" style="color: #FFFFFF;" on:click={() => {status = "Failed"}}>Failed</h3>
        <h3 style="color: #FFFFFF;" on:click={() => {status = "Not"}}>Not Attempted</h3>

        </div>
    {:else if status == "Parked"}
    <div>
        <h3 style="color: #FFFFFF;" on:click={() => {status = "Onstage"}}>Onstage</h3>     
        <h3 class="middle" style="background-color: #00D586; color: #353839;">✓Parked</h3>
        <h3 class="middlealt" style="color: #FFFFFF;" on:click={() => {status = "Failed"}}>Failed</h3>
        <h3 style="color: #FFFFFF;" on:click={() => {status = "Not"}}>Not Attempted</h3>
    </div>
    {:else if status == "Failed"}
    <div>
        <h3 style="color: #FFFFFF;" on:click={() => {status = "Onstage"}}>Onstage</h3>     
        <h3 class="middle" style="color: #FFFFFF;" on:click={() => {status = "Parked"}}>Parked</h3>
        <h3 class="middlealt" style="background-color: #00D586; color: #353839;">✓Failed</h3>
        <h3 style="color: #FFFFFF;" on:click={() => {status = "Not"}}>Not Attempted</h3>
    </div>
    {:else}
    <div>
        <h3 style="color: #FFFFFF;" on:click={() => {status = "Onstage"}}>Onstage</h3>     
        <h3 class="middle" style="color: #FFFFFF;" on:click={() => {status = "Failed"}}>Parked</h3>
        <h3 class="middlealt" style="color: #FFFFFF;" on:click={() => {status = "Failed"}}>Failed</h3>
        <h3 style="background-color: #00D586; color: #353839; border-bottom-left-radius: 0.4rem; border-bottom-right-radius: 0.4rem">✓Not Attempted</h3>
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
.middlealt{
    border-style: solid;
            border-width: 0.2rem;
            border-color: grey;
            border-left: 0;
            border-right: 0;
            border-top: 0;
}
    </style>