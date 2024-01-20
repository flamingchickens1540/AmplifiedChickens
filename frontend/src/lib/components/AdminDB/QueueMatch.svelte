<script>
    export let match = "2024orsal_qm67"
    import AssignStudentRed from "./AssignStudentRed.svelte";
    import AssignStudentBlue from "./AssignStudentBlue.svelte"
    export let redteam1 = "1540"
    export let redteam2 = "1540"
    export let redteam3 = "1540"
    export let blueteam1 = "1540"
    export let blueteam2 = "1540"
    export let blueteam3 = "1540"
    export let bluescout1 = "Auto-Assign"
    export let bluescout2 = "Auto-Assign"
    export let bluescout3 = "Auto-Assign"
    export let redscout1 = "Auto-Assign"
    export let redscout2 = "Auto-Assign"
    export let redscout3 = "Auto-Assign"
</script>
<div class="rounded" style="background-color: #2C2C2C; padding:1rem">
<div class="flex justify-between items-center rounded" style="background-color: #5C5C5C; padding:0.2rem; margin:17px">
<h3>{match}</h3>
<h2 class="rounded">Load</h2>
</div>
<div class="grid grid-cols-2 grid-rows-1">
<div>
    <AssignStudentRed team1={redteam1} team2={redteam2} team3={redteam3} bind:selected1={redscout1} bind:selected2={redscout2} bind:selected3={redscout3}/>
</div>
<div>
    <AssignStudentBlue team1={blueteam1} team2={blueteam2} team3={blueteam3} bind:selected1={bluescout1} bind:selected2={bluescout2} bind:selected3={bluescout3}/>
</div>
</div>
<center><h2 class="rounded" style="padding-top:1rem; padding-bottom: 1rem">Queue Match</h2></center>
</div>
<style>
    h3{
       font-family: 'Poppins-bold';
       font-size: 20px;
       line-height: 91.5%;
       color: #FFFFFF;
       margin:1rem
       }
       h2{
       font-family: 'Poppins-bold';
       font-size: 20px;
       line-height: 91.5%;
       color: #2C2C2C;
       background-color: #00D586;
       padding: 1.5rem;
       padding-top:0.75rem;
       padding-bottom:0.75rem;
       margin:1rem
       }
</style>