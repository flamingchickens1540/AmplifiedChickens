<script lang="ts">
    import type { PageData } from "./$types";
    import Navbar from "$lib/components/AutoScouting.svelte";

    export let data: PageData;
    console.log(data);
    let dataa = JSON.stringify(data);
</script>
<Navbar teamkey = {dataa}/>