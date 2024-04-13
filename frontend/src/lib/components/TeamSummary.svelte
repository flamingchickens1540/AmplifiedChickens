<script>
    export let team = "1540"
    export let teamspeakmin = 0
    export let teamspeak = 0
    export let teamspeakmax = 0
    export let teamampmin = 0
    export let teamamp = 0
    export let teamampmax = 0
    export let teamshufflemin = 0
    export let teamshuffle = 0
    export let teamshufflemax = 0
    export let trap = 0
    export let climb = 0
    let endgame = "Park"
    if (trap >= 1) {
        endgame = "Trap"
    }
    else if (climb >= 1) {
        endgame = "Climb"
    }
</script>

<div class="rounded grid justify-items-center grid-cols-5 gap-2" style=" background-color: #5C5C5C; margin: 1rem; padding:0.5rem">
    <h3  style="align-self: center; font-size: 20px">{team}</h3>
    <div style="margin-right: -0.2rem">
    <div class="grid grid-cols-3 grid-rows-2 content-end items-center borderplz">
    <h2 style="align-self: end; ">{teamspeakmin}</h2>
    <h3 style="align-self: end;">{teamspeak}</h3>
    <h2 style="align-self: end;">{teamspeakmax}</h2>
    <h3 style="font-family: Poppins-medium; font-size: 12px" class="cols-span-3">Speaker</h3>
    </div>  
    
</div>
<div style="margin-right: -0.2rem">
    <div class="grid grid-cols-3 grid-rows-2 content-end items-center borderplz">
    <h2 style="align-self: end; ">{teamampmin}</h2>
    <h3 style="align-self: end;">{teamamp}</h3>
    <h2 style="align-self: end;">{teamampmax}</h2>
    <h3 style="font-family: Poppins-medium; font-size: 13px" class="cols-span-3">Amp</h3>
    </div>  
    
</div>

<div style="margin-right: -0.2rem">
    <div class="grid grid-cols-3 grid-rows-2 content-end items-center borderplz">
    <h2 style="align-self: end; ">{teamshufflemin}</h2>
    <h3 style="align-self: end;">{teamshuffle}</h3>
    <h2 style="align-self: end;">{teamshufflemax}</h2>
    <h3 style="font-family: Poppins-medium; font-size: 12px" class="cols-span-3">Shuffle</h3>
    </div>  
</div>
<center>
<div>
    <h3 style="align-self: end; margin-bottom: 4px">{endgame}</h3>
    <h3 style="font-family: Poppins-medium; font-size: 11px" class="cols-span-3">Endgame</h3>
    </div> 

</center>
</div>



<style>
h3 {
        font-family: "Poppins-bold";
        color: #ffffff;
        font-size: 16px;
    }
h2{

font-family: 'Poppins-medium';
font-size: 16px;
color: #D6D6D6;


}
.borderplz {
  border-right: 2px solid #d6d6d6;
  border-style: dashed;
}
</style>

