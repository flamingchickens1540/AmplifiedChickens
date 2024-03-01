<script>
    import { match_data, team_color } from "$lib/stores";
    export let phase = "Auto"
</script>
<div class="header grid justify-items-center border border-outline_gray rounded mt-[-9px] visible">
    <h1 style="margin: 1rem; color: {team_color}; font-size: 4.5rem">{$match_data.team_key}</h1>
    <h2 style="margin: 0rem; font-size: 2.3rem">{phase}</h2>
</div>

<style>
    h1 {
        font-family: "Poppins-medium";
        font-style: normal;
        font-size: 60px;
        line-height: 45px;
        /* or 75% */
        align-items: center;
        text-align: center;
    }
    h2 {
        /* Teleop */

        font-family: "Poppins-medium";
        font-style: normal;
        font-size: 36px;
        line-height: 45px;
        align-items: center;
        text-align: center;

        color: #c2c2c2;
    }
    .header {
        margin: 17px;
        padding: 1rem;

        color: lightgrey;
        background-color: #353839;
    }
</style>
