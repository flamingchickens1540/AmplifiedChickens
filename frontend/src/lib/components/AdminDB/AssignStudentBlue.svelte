<script>
    export let team1 = "1540"
    export let team2 = "1540"
    export let team3 = "1540"
    export let selected1 = "Auto-Assign"
    export let selected2 = "Auto-Assign"
    export let selected3 = "Auto-Assign"
    import Combobox from "$lib/components/Combobox/ComboBoxComponent.svelte"
    export let color = "0083E6"
</script>
<div class="grid grid-cols-1 grod-rows-3 rounded" style="background-color: #5C5C5C; padding:0.2rem; margin:17px">
<div class="flex justify-between items-center rounded" style="background-color: #0083E6; padding:0.2rem; margin:10px;">
    <h3>{team1}</h3>
    <div>
    <Combobox bind:value = {selected1}/>
    </div>
</div>

<div class="flex justify-between items-center rounded" style="background-color: #0083E6; padding:0.2rem; margin:10px;">
    <h3>{team2}</h3>
    <div>
    <Combobox bind:value = {selected2}/>
    </div>
</div>
<div class="flex justify-between items-center rounded" style="background-color: #0083E6; padding:0.2rem; margin:10px;">
    <h3>{team3}</h3>
    <div>
    <Combobox bind:value = {selected3}/>
    </div>
</div>
</div>

<style>
    h3{
       font-family: 'Poppins-bold';
       font-size: 25px;
       line-height: 91.5%;
       color: #FFFFFF;
       margin:10px
       }
</style>