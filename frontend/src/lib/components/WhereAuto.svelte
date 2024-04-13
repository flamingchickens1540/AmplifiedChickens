<script lang="ts">
    export let value = "onstage";
    import checkmark from "$lib/svgs/checkmark.svg";
</script>

{#if value == "onstage"}
    <div class="border border-outline_gray mt-[-9px] visible rounded">
        <button
            class="border-b border-outline_gray mt-[-9px] visible"
            style="background-color: #00D586; color: #353839; border-top-left-radius: 0.2rem; border-top-right-radius: 0.2rem"
            ><img style="width:18px" src={checkmark} alt="check" /> Blue Amp</button
        >
        <button
            class="border-b border-outline_gray mt-[-9px] visible"
            style="color: #FFFFFF;"
            on:click={() => {
                value = "parked";
            }}>Red Amp</button
        >
        <button
            class="border-b border-outline_gray mt-[-9px] visible"
            style="color: #FFFFFF;"
            on:click={() => {
                value = "failed";
            }}>Blue Source</button
        >
        <button
            style="color: #FFFFFF;"
            on:click={() => {
                value = "notattempted";
            }}>Red Source</button
        >
    </div>
{:else if value == "parked"}
    <div class="border border-outline_gray rounded mt-[-9px] visible">
        <button
            class="border-b border-outline_gray mt-[-9px] visible"
            style="color: #FFFFFF;"
            on:click={() => {
                value = "onstage";
            }}>Blue Amp</button
        >
        <button
            class="border-b border-outline_gray mt-[-9px] visible"
            style="background-color: #00D586; color: #353839;"
            ><img style="width:18px" src={checkmark} alt="check" /> Red Amp</button
        >
        <button
            class="border-b border-outline_gray mt-[-9px] visible"
            style="color: #FFFFFF;"
            on:click={() => {
                value = "failed";
            }}>Blue Source</button
        >
        <button
            style="color: #FFFFFF;"
            on:click={() => {
                value = "notattempted";
            }}>Red Source</button
        >
    </div>
{:else if value == "failed"}
    <div class="border border-outline_gray rounded mt-[-9px] visible">
        <button
            class="border-b border-outline_gray mt-[-9px] visible"
            style="color: #FFFFFF;"
            on:click={() => {
                value = "onstage";
            }}>Blue Amp</button
        >
        <button
            class="border-b border-outline_gray mt-[-9px] visible"
            style="color: #FFFFFF;"
            on:click={() => {
                value = "parked";
            }}>Red Amp</button
        >
        <button
            class="border-b border-outline_gray mt-[-9px] visible"
            style="background-color: #00D586; color: #353839;"
            ><img style="width:18px" src={checkmark} alt="check" /> Blue Source</button
        >
        <button
            style="color: #FFFFFF;"
            on:click={() => {
                value = "notattempted";
            }}>Red Source</button
        >
    </div>
{:else}
    <div class="border border-outline_gray rounded mt-[-9px] visible">
        <button
            class="border-b border-outline_gray mt-[-9px] visible"
            style="color: #FFFFFF;"
            on:click={() => {
                value = "onstage";
            }}>Blue Amp</button
        >
        <button
            class="border-b border-outline_gray mt-[-9px] visible"
            style="color: #FFFFFF;"
            on:click={() => {
                value = "parked";
            }}>Red Amp</button
        >
        <button
            class="border-b border-outline_gray mt-[-9px] visible"
            style="color: #FFFFFF;"
            on:click={() => {
                value = "failed";
            }}>Blue Source</button
        >
        <button
            style="background-color: #00D586; color: #353839; border-bottom-left-radius: 0.2rem; border-bottom-right-radius: 0.2rem"
            ><img style="width:18px" src={checkmark} alt="check" /> Red Source</button
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
