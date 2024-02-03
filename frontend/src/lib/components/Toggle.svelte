<script lang="ts">
    let buttonon = true;
    export let text1 = "Text1";
    export let text2 = "Text2";
    export let value = text1;
    import checkmark from "$lib/svgs/checkmark.svg";
    function toggleon() {
        value = text1;
        value = value;
        buttonon = true;
        buttonon = buttonon;
    }
    function toggleoff() {
        value = text2;
        buttonon = false;
        buttonon = buttonon;
        value = value;
    }
</script>

{#if buttonon == true}
    <div
        class="border border-outline_gray mt-[-9px] visible grid grid-cols-2 grid-rows-1 place-items-center rounded"
    >
            <h3 class="border-r border-outline_gray mt-[-9px] visible grid col-start-1 col-span-1 row-start-1 text-center"
            style="background-color: #00D586; color: #353839; border-top-left-radius: 0.2rem; border-bottom-left-radius: 0.2rem"><img style="width:18px" src={checkmark}/> {text1}</h3>
        <h3 class="text-center" style="color: #FFFFFF;" on:click={toggleoff}>
            {text2}
        </h3>
    </div>
{:else}
<div
        class="border border-outline_gray mt-[-9px] visible grid grid-cols-2 grid-rows-1 place-items-center rounded"
    >
            <h3 class="border-r border-outline_gray mt-[-9px] visible grid col-start-2 col-span-1 row-start-1 text-center"
            style="background-color: #00D586; color: #353839; border-top-right-radius: 0.2rem; border-bottom-right-radius: 0.2rem"><img style="width:18px" src={checkmark}/> {text2}</h3>
        <h3 class="text-center" style="color: #FFFFFF;" on:click={toggleon}>
            {text1}
        </h3>
    </div>
{/if}

<style>
    h3 {
        font-family: "Poppins-medium";
        font-style: normal;
        font-size:18px;
        letter-spacing: 0.5px;
        margin: 0;;
        padding: 0.3rem;
        width: 100%
    }
    img{
    }
    div {
        margin: 17px;
        color: lightgrey;
        background-color: #353839;
    }
  h3, img {
    display: inline-block;
    vertical-align: middle;
  }
</style>
