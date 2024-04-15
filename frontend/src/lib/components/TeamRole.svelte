<script lang="ts">
    export let value = "combo";
    import checkmark from "$lib/svgs/checkmark.svg";
</script>

{#if value == "shuffler"}
    <div class="border border-outline_gray mt-[-9px] visible rounded">
        <button
            class="border-b border-outline_gray mt-[-9px] visible"
            style="background-color: #00D586; color: #353839; border-top-left-radius: 0.2rem; border-top-right-radius: 0.2rem"
            ><img style="width:18px" src={checkmark} alt="shuffler" /> shuffler</button
        >
        <button
            class="border-b border-outline_gray mt-[-9px] visible"
            style="color: #FFFFFF;"
            on:click={() => {
                value = "receiver";
            }}>receiver</button
        >
        <button
            class="border-b border-outline_gray mt-[-9px] visible"
            style="color: #FFFFFF;"
            on:click={() => {
                value = "defender";
            }}>defender</button
        >
        <button
		class="border-b border-outline_gray mt-[-9px] visible"
            style="color:#FFFFFF;"
            on:click={() => {
                value = "cycler";
            }}>cycler</button
        >
        <button
        class="border-b border-outline_gray mt-[-9px] visible"
style="color: #FFFFFF;"
        on:click={() => {
            value = "combo";
        }}>combo</button
    >
    </div>
{:else if value == "receiver"}
    <div class="border border-outline_gray rounded mt-[-9px] visible">
        <button
            class="border-b border-outline_gray mt-[-9px] visible"
            style="color: #FFFFFF;"
            on:click={() => {
                value = "shuffler";
            }}>shuffler</button
        >
        <button
            class="border-b border-outline_gray mt-[-9px] visible"
            style="background-color: #00D586; color: #353839;"
            ><img style="width:18px" src={checkmark} alt="receiver" /> receiver</button
        >
        <button
            class="border-b border-outline_gray mt-[-9px] visible"
            style="color: #FFFFFF;"
            on:click={() => {
                value = "defender";
            }}>defender</button
        >
        <button
class="border-b border-outline_gray mt-[-9px] visible"
            style="color: #FFFFFF;"
            on:click={() => {
                value = "cycler";
            }}>cycler</button
        >
        <button
        class="border-b border-outline_gray mt-[-9px] visible"
	style="color: #FFFFFF;"
        on:click={() => {
            value = "combo";
        }}>combo</button
    >
    </div>
{:else if value == "defender"}
    <div class="border border-outline_gray rounded mt-[-9px] visible">
        <button
            class="border-b border-outline_gray mt-[-9px] visible"
            style="color: #FFFFFF;"
            on:click={() => {
                value = "shuffler";
            }}>shuffler</button
        >
        <button
            class="border-b border-outline_gray mt-[-9px] visible"
            style="color: #FFFFFF;"
            on:click={() => {
                value = "receiver";
            }}>receiver</button
        >
        <button
            class="border-b border-outline_gray mt-[-9px] visible"
            style="background-color: #00D586; color: #353839;"
            ><img style="width:18px" src={checkmark} alt="defender" /> defender</button
        >
        <button
class="border-b border-outline_gray mt-[-9px] visible"
            style="color: #FFFFFF;"
            on:click={() => {
                value = "cycler";
            }}>cycler</button
        >
        <button
        class="border-b border-outline_gray mt-[-9px] visible"
style="color: #FFFFFF;"
        on:click={() => {
            value = "combo";
        }}>combo</button
    >
    </div>
    {:else if value == "combo"}
    <div class="border border-outline_gray rounded mt-[-9px] visible">
        <button
            class="border-b border-outline_gray mt-[-9px] visible"
            style="color: #FFFFFF;"
            on:click={() => {
                value = "shuffler";
            }}>shuffler</button
        >
        <button
            class="border-b border-outline_gray mt-[-9px] visible"
            style="color: #FFFFFF;"
            on:click={() => {
                value = "receiver";
            }}>receiver</button
        >
        <button
            class="border-b border-outline_gray mt-[-9px] visible"
            style="color: #FFFFFF;"
            on:click={() => {
                value = "defender";
            }}>defender</button
        >
        <button
class="border-b border-outline_gray mt-[-9px] visible"
        style="color: #FFFFFF;"
        on:click={() => {
            value = "cycler";
        }}>cycler</button
    >
        <button
style="background-color: #00D586; color: #353839; border-bottom-left-radius: 0.2rem; border-bottom-right-radius: 0.2rem"
            ><img style="width:18px" src={checkmark} alt="combo" /> combo</button
        >
    </div>
{:else}
    <div class="border border-outline_gray rounded mt-[-9px] visible">
        <button
            class="border-b border-outline_gray mt-[-9px] visible"
            style="color: #FFFFFF;"
            on:click={() => {
                value = "shuffler";
            }}>shuffler</button
        >
        <button
            class="border-b border-outline_gray mt-[-9px] visible"
            style="color: #FFFFFF;"
            on:click={() => {
                value = "defender";
            }}>receiver</button
        >
        <button
            class="border-b border-outline_gray mt-[-9px] visible"
            style="color: #FFFFFF;"
            on:click={() => {
                value = "defender";
            }}>defender</button
        >
        <button
            style="background-color: #00D586; color: #353839; border-bottom-left-radius: 0.2rem; border-bottom-right-radius: 0.2rem"
            ><img style="width:18px" src={checkmark} alt="cycler" /> cycler</button
        >
        <button
        class="border-b border-outline_gray mt-[-9px] visible"
style="color: #FFFFFF;"
        on:click={() => {
            value = "combo";
        }}>combo</button
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
