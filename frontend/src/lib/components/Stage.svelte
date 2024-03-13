<script lang="ts">
    export let value = "Onstage";
    import checkmark from "$lib/svgs/checkmark.svg";
</script>

{#if value == "Onstage"}
    <div class="border border-outline_gray mt-[-9px] visible rounded">
        <button
            class="border-b border-outline_gray mt-[-9px] visible"
            style="background-color: #00D586; color: #353839; border-top-left-radius: 0.2rem; border-top-right-radius: 0.2rem"
            ><img style="width:18px" src={checkmark} alt="Onstage" /> Onstage</button
        >
        <button
            class="border-b border-outline_gray mt-[-9px] visible"
            style="color: #FFFFFF;"
            on:click={() => {
                value = "parked";
            }}>Parked</button
        >
        <button
            class="border-b border-outline_gray mt-[-9px] visible"
            style="color: #FFFFFF;"
            on:click={() => {
                value = "failed";
            }}>Failed</button
        >
        <button
            style="color: #FFFFFF;"
            on:click={() => {
                value = "notattempted";
            }}>Not Attempted</button
        >
    </div>
{:else if value == "parked"}
    <div class="border border-outline_gray rounded mt-[-9px] visible">
        <button
            class="border-b border-outline_gray mt-[-9px] visible"
            style="color: #FFFFFF;"
            on:click={() => {
                value = "onstage";
            }}>On Stage</button
        >
        <button
            class="border-b border-outline_gray mt-[-9px] visible"
            style="background-color: #00D586; color: #353839;"
            ><img style="width:18px" src={checkmark} alt="Parked" /> Parked</button
        >
        <button
            class="border-b border-outline_gray mt-[-9px] visible"
            style="color: #FFFFFF;"
            on:click={() => {
                value = "failed";
            }}>Failed</button
        >
        <button
            style="color: #FFFFFF;"
            on:click={() => {
                value = "notattempted";
            }}>Not Attempted</button
        >
    </div>
{:else if value == "Failed"}
    <div class="border border-outline_gray rounded mt-[-9px] visible">
        <button
            class="border-b border-outline_gray mt-[-9px] visible"
            style="color: #FFFFFF;"
            on:click={() => {
                value = "onstage";
            }}>On Stage</button
        >
        <button
            class="border-b border-outline_gray mt-[-9px] visible"
            style="color: #FFFFFF;"
            on:click={() => {
                value = "parked";
            }}>Parked</button
        >
        <button
            class="border-b border-outline_gray mt-[-9px] visible"
            style="background-color: #00D586; color: #353839;"
            ><img style="width:18px" src={checkmark} alt="Failed" /> Failed</button
        >
        <button
            style="color: #FFFFFF;"
            on:click={() => {
                value = "notattempted";
            }}>Not Attempted</button
        >
    </div>
{:else}
    <div class="border border-outline_gray rounded mt-[-9px] visible">
        <button
            class="border-b border-outline_gray mt-[-9px] visible"
            style="color: #FFFFFF;"
            on:click={() => {
                value = "Onstage";
            }}>Onstage</button
        >
        <button
            class="border-b border-outline_gray mt-[-9px] visible"
            style="color: #FFFFFF;"
            on:click={() => {
                value = "Failed";
            }}>Parked</button
        >
        <button
            class="border-b border-outline_gray mt-[-9px] visible"
            style="color: #FFFFFF;"
            on:click={() => {
                value = "Failed";
            }}>Failed</button
        >
        <button
            style="background-color: #00D586; color: #353839; border-bottom-left-radius: 0.2rem; border-bottom-right-radius: 0.2rem"
            ><img style="width:18px" src={checkmark} alt="Not Attempted" /> Not Attempted</button
        >
    </div>
{/if}

<style>
    button {
        font-family: "Poppins-medium";
        font-style: normal;
        font-size: 1.3rem;
        letter-spacing: 0.5px;
        margin: 0;
        width: 100%;
        padding: 0.3rem;
        padding-left: 0.8rem;
    }
    div {
        margin: 17px;
        background-color: #353839;
    }
    img {
        display: inline-block;
        vertical-align: middle;
    }
</style>
