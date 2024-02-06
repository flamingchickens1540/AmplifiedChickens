<script>
    import TextInput from "$lib/components/TextInput.svelte";
</script>

<div class="rounded" style="background-color: #2C2C2C; padding:1rem">
    <center><h3 style="margin:0.8rem">Event Management</h3></center>
    <div class="rounded" style="margin: 1rem">
        <TextInput name="Event Key" />
    </div>
    <center><h2 class="rounded">Set Event</h2></center>
    <center><h2 class="rounded">Refresh TBA</h2></center>
    <center><h2 class="rounded">Load Teams</h2></center>
    <center><h2 class="rounded">Clear Scouts</h2></center>
    <center><h2 class="rounded">Clear Teams</h2></center>
</div>

<style>
    h3 {
        font-family: "Poppins-bold";
        font-size: 23px;
        line-height: 91.5%;
        color: #ffffff;
        margin: 0.5rem;
        margin-top: 0rem;
    }
    h2 {
        font-family: "Poppins-bold";
        font-size: 20px;
        line-height: 91.5%;
        color: #2c2c2c;
        background-color: #00d586;
        padding-top: 0.4rem;
        padding-bottom: 0.4rem;
        margin: 2rem;
        margin-top: 0.5rem;
        margin-bottom: 0.5rem;
    }
</style>
