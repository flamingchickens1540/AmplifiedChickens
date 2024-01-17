<script lang=ts>
    export let status = "Onstage"
    </script>
    {#if status == "Onstage"}
    <div class="border border-outline_gray mt-[-9px] visible rounded">
        <h3 class="border-b border-outline_gray mt-[-9px] visible" style="background-color: #00D586; color: #353839; border-top-left-radius: 0.2rem; border-top-right-radius: 0.2rem">✓Onstage</h3>
        <h3 class="border-b border-outline_gray mt-[-9px] visible" style="color: #FFFFFF;" on:click={() => {status = "Parked"}}>Parked</h3>
        <h3 class="border-b border-outline_gray mt-[-9px] visible" style="color: #FFFFFF;" on:click={() => {status = "Failed"}}>Failed</h3>
        <h3 style="color: #FFFFFF;" on:click={() => {status = "Not"}}>Not Attempted</h3>

        </div>
    {:else if status == "Parked"}
    <div class="border border-outline_gray rounded mt-[-9px] visible">
        <h3 class="border-b border-outline_gray mt-[-9px] visible" style="color: #FFFFFF;" on:click={() => {status = "Onstage"}}>Onstage</h3>     
        <h3 class="border-b border-outline_gray mt-[-9px] visible" style="background-color: #00D586; color: #353839;">✓Parked</h3>
        <h3 class="border-b border-outline_gray mt-[-9px] visible" style="color: #FFFFFF;" on:click={() => {status = "Failed"}}>Failed</h3>
        <h3 style="color: #FFFFFF;" on:click={() => {status = "Not"}}>Not Attempted</h3>
    </div>
    {:else if status == "Failed"}
    <div class="border border-outline_gray rounded mt-[-9px] visible">
        <h3 class="border-b border-outline_gray mt-[-9px] visible" style="color: #FFFFFF;" on:click={() => {status = "Onstage"}}>Onstage</h3>     
        <h3 class="border-b border-outline_gray mt-[-9px] visible" style="color: #FFFFFF;" on:click={() => {status = "Parked"}}>Parked</h3>
        <h3 class="border-b border-outline_gray mt-[-9px] visible" style="background-color: #00D586; color: #353839;">✓Failed</h3>
        <h3 style="color: #FFFFFF;" on:click={() => {status = "Not"}}>Not Attempted</h3>
    </div>
    {:else}
    <div class="border border-outline_gray rounded mt-[-9px] visible">
        <h3 class="border-b border-outline_gray mt-[-9px] visible" style="color: #FFFFFF;" on:click={() => {status = "Onstage"}}>Onstage</h3>     
        <h3 class="border-b border-outline_gray mt-[-9px] visible" style="color: #FFFFFF;" on:click={() => {status = "Failed"}}>Parked</h3>
        <h3 class="border-b border-outline_gray mt-[-9px] visible" style="color: #FFFFFF;" on:click={() => {status = "Failed"}}>Failed</h3>
        <h3 style="background-color: #00D586; color: #353839; border-bottom-left-radius: 0.2rem; border-bottom-right-radius: 0.2rem">✓Not Attempted</h3>
    </div>
    {/if} 
    
    <style>
    h3{
    font-family: 'Poppins';
    font-style: normal;
    font-size: 1.3rem;
    letter-spacing: 0.5px;
    margin: 0;
    width: 100%;
    padding: 0.3rem;
    padding-left: 0.8rem;
    }
    div{
            margin: 17px;
            background-color: #353839;
        }
    </style>