<script lang="ts">
    export let scout_data: (string | number)[][] = []
</script>

<div class="rounded" style="background-color: #2C2C2C; padding:1rem">
    <div class="flex justify-between">
        <h3>Name</h3>
        <h3>% Scouted</h3>
    </div>
    <div class="main">
        {#each scout_data as scout, i}
            <hr style="color: #C2C2C2" />
            <div class="flex justify-between">
                <h2>{scout[0]}</h2>
                <!-- TODO: Maybe show the number scouted instead of a percent, a different endpoint would be needed for this -->
                <h2>{scout[1]}</h2> 
            </div>
        {/each}
    </div>
</div>

<style>
    h3 {
        font-family: "Poppins-bold";
        font-style: normal;
        font-size: 17px;
        margin: 13px;
        color: #ffffff;
    }
    h2 {
        font-family: "Poppins-medium";
        font-style: normal;
        font-size: 20px;
        margin: 16px;
        margin-top: 10px;
        margin-bottom: 10px;
        color: #ffffff;
    }
    .main {
        overflow-y: scroll;
        max-height: 255px;
    }
</style>
