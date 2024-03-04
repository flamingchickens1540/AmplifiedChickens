<script lang=ts>
    export let option1selected = true
    export let option2selected = true
    export let option3selected = true
    export let text1 = ""
    export let text2 = ""
    export let text3 = ""
    export let value = text1
    function closebutton(){
        if (option1selected == true){
            option1selected = false
        }
        else {
            option1selected = true
        }
    option1selected = option1selected
    } 
    function midbutton(){
        if (option2selected == true){
            option2selected = false
        }
        else {
            option2selected = true
        }
    option2selected = option2selected
    } 
    function farbutton(){
        if (option3selected == true){
            option3selected = false
        }
        else {
            option3selected = true
        }
    option3selected = option3selected
    } 
    import checkmark from "$lib/svgs/checkmark.svg";
    </script>
    <div class="border border-outline_gray rounded mt-[-9px] visible">
        {#if option1selected == true}
        <h3 on:click={closebutton} class="border-b border-outline_gray mt-[-9px] visible" style="background-color: #00D586; color: #353839; border-top-left-radius: 0.2rem; border-top-right-radius: 0.2rem"><img style="width:18px" src={checkmark}/> {text1}</h3>
        {:else}
        <h3 class="border-b border-outline_gray mt-[-9px] visible" style="color: #FFFFFF;" on:click={closebutton}>{text1}</h3>
        {/if}
        {#if option2selected == true}
        <h3 on:click={midbutton} class="border-b border-outline_gray mt-[-9px] visible" style="background-color: #00D586; color: #353839;"><img style="width:18px" src={checkmark}/> {text2}</h3>
        {:else}
        <h3 class="border-b border-outline_gray mt-[-9px] visible" style="color: #FFFFFF;" on:click={midbutton}>{text2}</h3>
        {/if}
        {#if option3selected == true}
        <h3 on:click={farbutton} style="background-color: #00D586; color: #353839; border-bottom-left-radius: 0.2rem; border-bottom-right-radius: 0.2rem"><img style="width:18px" src={checkmark}/> {text3}</h3>
        {:else}
        <h3 style="color: #FFFFFF;" on:click={farbutton}>{text3}</h3>
        {/if}
        </div>x``
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