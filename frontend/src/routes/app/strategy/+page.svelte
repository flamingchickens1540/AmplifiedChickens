<script lang="ts">
    import type { PageData } from "./$types";
    import Navba from "$lib/components/AutoScouting.svelte";
    import Navbar from "$lib/components/Navbar.svelte";
    export let data: PageData;
    console.log(data);
    let dataa = JSON.stringify(data);
</script>
<Navba teamkey = {dataa}/>
<footer class="bottom-div">
    <Navbar page="strategy" />
</footer>

<style lang="postcss">
    .bottom-div {
        position: fixed;
        bottom: 0;
        left: 0;
        width: 100%;
        background-color: #f0f0f0; /* Just for visualization */
        text-align: center;
    }
</style>