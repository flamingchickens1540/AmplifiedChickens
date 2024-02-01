<script lang="ts">
    export let location = "far"
    import checkmark from "$lib/svgs/checkmark.svg";
    </script>
    {#if location == "far"}
    <div class="border border-outline_gray rounded mt-[-9px] visible">
        <h3 class="border-b border-outline_gray mt-[-9px] visible" style="background-color: #00D586; color: #353839; border-top-left-radius: 0.2rem; border-top-right-radius: 0.2rem"><img style="width:18px" src={checkmark}/> Far</h3>
        <h3 class="border-b border-outline_gray mt-[-9px] visible"  style="color: #FFFFFF;" on:click={() => {location = "middle"}}>Middle</h3>
        <h3 style="color: #FFFFFF;" on:click={() => {location = "close"}}>Close</h3>
        </div>
    {:else if location == "middle"}
    <div class="border border-outline_gray rounded mt-[-9px] visible">
        <h3 class="border-b border-outline_gray mt-[-9px] visible" style="color: #FFFFFF;" on:click={() => {location = "far"}}>Far</h3>
        <h3 class="border-b border-outline_gray mt-[-9px] visible" style="background-color: #00D586; color: #353839"><img style="width:18px" src={checkmark}/> Middle</h3>
        <h3 style="color: #FFFFFF;" on:click={() => {location = "close"}}>Close</h3>
        </div>
    {:else}
    <div class="border border-outline_gray rounded mt-[-9px] visible">
        <h3 class="border-b border-outline_gray mt-[-9px] visible" style="color: #FFFFFF;" on:click={() => {location = "far"}}>Far</h3>
        <h3 class="border-b border-outline_gray mt-[-9px] visible" style="color: #FFFFFF;" on:click={() => {location = "middle"}}>Middle</h3>
        <h3   style="background-color: #00D586; color: #353839; border-bottom-left-radius: 0.2rem; border-bottom-right-radius: 0.2rem"><img style="width:18px" src={checkmark}/> Close</h3>
        </div>
    {/if} 
    
    <style>
    h3{
    font-family: 'Poppins-medium';
    font-style: normal;
    font-size: 1.3rem;
    letter-spacing: 0.5px;
    margin: 0;
    width: 100%;
    padding: 0.1rem;
    padding-left:0.8rem
    }
    div{
            margin: 17px;
            color: lightgrey;
            background-color: #353839;
        }
        h3, img {
    display: inline-block;
    vertical-align: middle;
  }
    </style>