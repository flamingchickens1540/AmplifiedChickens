<script>
    let number = 10;
    export let wantednumber = 10;
    export let prize = "candy";
    function randomize() {
        number = Math.floor(Math.random() * wantednumber + 1);
    }
    function finish() {
        number = 832948719238000;
    }
</script>

<h4 style="margin-bottom:0" class="rounded">LOTTERY BUTTON</h4>
<center>
    <h3 style="font-size:1.5rem; margin-bottom:0" class="col-span-5 rounded">
        Number: {number}
    </h3>
    <button on:click={randomize}>
        <h3 class="col-span-5 rounded">Click to generate number!</h3>
    </button>
</center>

{#if number == wantednumber}
    <center>
        <button on:click={finish}>
            <h3
                style="background-color: #FFC145; margin-top:0"
                class="col-span-5 rounded"
            >
                <strong>YOU WIN! CLICK TO CLAIM!</strong>
            </h3>
        </button>
    </center>
{/if}
<div>
    <h2 style="font-size: 30px;">Notes:</h2>
    <h2>
        Once you click the button, it will generate a random number between 1
        and {wantednumber}
    </h2>
    <h2>
        If the number generated is {wantednumber}, a button will appear, and
        once you click that, you get {prize}
    </h2>
</div>

<style>
    h3 {
        margin: 17px;
        color: #ffffff;
        background-color: #6c6c6c;
        font-size: 2rem;
        padding: 0.75rem;
        text-align: center;
        /* Teams Remaining */
        font-family: "Poppins-medium";
        font-style: bold;
    }
    h4 {
        margin: 17px;
        color: #ffffff;
        background-color: #4c4c4c;
        font-size: 2rem;
        padding: 1rem;
        text-align: center;
        /* Teams Remaining */
        font-family: "Poppins-medium";
        font-style: bold;
    }
    div {
        margin: 17px;
        color: #ffffff;
        background-color: #4c4c4c;
        font-size: 2rem;
        padding: 1rem;
        text-align: center;
        /* Teams Remaining */
        font-family: "Poppins-medium";
        font-style: bold;
    }
    h2 {
        font-family: "Poppins-medium";
        font-style: normal;
        font-size: 15px;
        letter-spacing: 0.5px;
        margin: 0;
        width: 100%;
        padding: 0.3rem;
    }
</style>
