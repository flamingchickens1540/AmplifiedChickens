<script>
    import TextInput from "$lib/components/TextInput.svelte"
</script>
<div class="rounded" style="background-color: #2C2C2C; padding:1rem">
<center><h3 style="margin:1rem">Event Management</h3></center>
<div class="rounded" style="margin: 1rem">
<TextInput name="Event Key"/>
</div>
<center><h2 class="rounded">Set Event</h2></center>
<center><h2 class="rounded">Refresh TBA</h2></center>
<center><h2 class="rounded">Load Teams</h2></center>
</div>
<style>
    h3{
       font-family: 'Poppins-bold';
       font-size: 35px;
       line-height: 91.5%;
       color: #FFFFFF;
       margin:0.5rem
       }
       h2{
       font-family: 'Poppins-bold';
       font-size: 24px;
       line-height: 91.5%;
       color: #2C2C2C;
       background-color: #00D586;
       padding-top:0.7rem;
       padding-bottom:0.7rem;
       margin:2rem;
       margin-top:1rem;
       margin-bottom:1rem
       }
</style>