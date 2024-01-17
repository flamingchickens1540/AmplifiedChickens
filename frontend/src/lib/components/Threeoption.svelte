<script lang=ts>
    let location = "far"
    export let text1 = ""
    export let text2 = ""
    export let text3 = ""
    export let value = text1
    function closebutton(){
    location = "far"
    location = location
    value = text1
    value = value
    } 
    function midbutton(){
    location = "middle"
    location = location
    value = text2
    value = value
    } 
    function farbutton(){
    location = "close"
    location = location
    value = text3
    value = value
    } 
    </script>
    {#if location == "far"}
    <div class="border border-outline_gray rounded mt-[-9px] visible">
        <h3 class="border-b border-outline_gray mt-[-9px] visible" style="background-color: #00D586; color: #353839; border-top-left-radius: 0.2rem; border-top-right-radius: 0.2rem">✓{text1}</h3>
        <h3 class="border-b border-outline_gray mt-[-9px] visible"  style="color: #FFFFFF;" on:click={midbutton}>{text2}</h3>
        <h3 style="color: #FFFFFF;" on:click={farbutton}>{text3}</h3>
        </div>
    {:else if location == "middle"}
    <div class="border border-outline_gray rounded mt-[-9px] visible">
        <h3 class="border-b border-outline_gray mt-[-9px] visible" style="color: #FFFFFF;" on:click={closebutton}>{text1}</h3>
        <h3 class="border-b border-outline_gray mt-[-9px] visible" style="background-color: #00D586; color: #353839">✓{text2}</h3>
        <h3 style="color: #FFFFFF;" on:click={farbutton}>{text3}</h3>
        </div>
    {:else}
    <div class="border border-outline_gray rounded mt-[-9px] visible">
        <h3 class="border-b border-outline_gray mt-[-9px] visible" style="color: #FFFFFF;" on:click={closebutton}>{text1}</h3>
        <h3 class="border-b border-outline_gray mt-[-9px] visible" style="color: #FFFFFF;" on:click={midbutton}>{text2}</h3>
        <h3   style="background-color: #00D586; color: #353839; border-bottom-left-radius: 0.2rem; border-bottom-right-radius: 0.2rem">✓{text3}</h3>
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
    padding: 0.1rem;
    padding-left:0.8rem
    }
    div{
            margin: 17px;
            color: lightgrey;
            background-color: #353839;
        }
    </style>