<script>
    import Header from "$lib/components/Header.svelte"
    import Position from "$lib/components/Toggle.svelte"
    import Threeoption from "$lib/components/Where.svelte"
    import Game from "$lib/components/HeheButton.svelte"
    let location = ""
    let fieldedstring = ""
    let fielded = true 
    $: if (fieldedstring == "Missed Match"){
        fielded = false
        fielded = fielded
    }
    $: if (fieldedstring == "Fielded"){
        fielded = true
        fielded = fielded
    }
    import { Modal, Content, Trigger } from "sv-popup"

</script>
<Header phase = "Pre Match"/>
<Threeoption text1="Far" text2="Middle" text3="Close" bind:value = {location}/>
<Position text1 = "Fielded" text2 = "Missed Match" bind:value = {fieldedstring}/> 


<Modal>
    <Content style="background-color: #2C2C2C; width:92%; margin:auto" class="p-4 rounded">
    <Game/>
    </Content> 
    <Trigger>
    <div class="grid grid-cols-5">
        <button class="col-span-5">
            <h3 style="margin-top: 17rem;" class="col-span-5 rounded ">Bored?</h3>
        </button>
        </div>
    </Trigger>
  </Modal>


    <style>
        h3{
            margin: 17px;
            color: #FFFFFF;
            background-color: #5c5c5c;
            font-size: 3rem;
            padding: 1rem;
            text-align: center;
            /* Teams Remaining */
font-family: 'Poppins-medium';
font-style: bold;}
</style>