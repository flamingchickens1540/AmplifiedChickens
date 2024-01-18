<script>
    export let team = "1540";
    export let teamcolor = "red";
    export let phase = "Auto"
</script>
<div class="header grid justify-items-center border border-outline_gray rounded mt-[-9px] visible">
    <h1 style="margin: 1rem; color: {teamcolor}; font-size: 4.5rem">{team}</h1>
    <h2 style="margin: 0rem; font-size: 2.5rem">{phase}</h2>
</div>

<style>
    h1 {
        font-family: "Poppins";
        font-style: normal;
        font-size: 60px;
        line-height: 45px;
        /* or 75% */
        align-items: center;
        text-align: center;
    }
    h2 {
        /* Teleop */

        font-family: "Poppins";
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
