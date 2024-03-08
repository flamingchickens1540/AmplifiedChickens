<script>
let number = 10
let wonyet = false
export let wantednumber = 1000
export let prize = "candy"
function randomize(){
    number = Math.floor(Math.random() * wantednumber+1);
}
function finish(){
    wonyet = true
    wonyet = wonyet
}
</script>
<h4 style="margin-bottom:0" class="rounded">LOTTERY BUTTON</h4>
{#if wonyet == false}
<center>
    <h3  style="font-size:1.5rem; margin-bottom:0" class="col-span-5 rounded">Number: {number}</h3>
    <button on:click={randomize}>
    <h3 class="col-span-5 rounded">Click to generate number!</h3>
</button>

    
</center>

{#if number == wantednumber}
<center>
    <button on:click={finish}>
    <h3 style="background-color: #FFC145; margin-top:0" class="col-span-5 rounded"><strong>YOU WIN! CLICK TO CLAIM!</strong></h3>
</button>
</center>
{/if}
    <div>
    <h2 style="font-size: 30px;">Notes:</h2>
    <h2>Once you click the button, it will generate a random number between 1 and {wantednumber}</h2>
    <h2>If the number generated is {wantednumber}, a button will appear, and once you click that, you get CHUCK</h2>
    </div>
{:else}
<h3 style="background-color: #FFC145; margin-top:0" class="col-span-5 rounded"><strong>SHOW THIS SCREEN TO AZALEA COLBURN OR JACK STURMAN TO WIN CHUCK FOR 60 MINUTES!!!</strong></h3>
{/if}
    <style>
        h3{
            margin: 10px;
            color: #FFFFFF;
            background-color: #6c6c6c;
            font-size: 1.5rem;
            padding: 0.5rem;
            text-align: center;
            /* Teams Remaining */
font-family: 'Poppins-medium';
font-style: bold;

        }
        h4{
            margin: 10px;
            color: #FFFFFF;
            background-color: #4c4c4c;
            font-size: 1.5rem;
            padding: 1rem;
            text-align: center;
            /* Teams Remaining */
font-family: 'Poppins-medium';
font-style: bold;

        }
        div{
            margin: 10px;
            color: #FFFFFF;
            background-color: #4c4c4c;
            font-size: 1.5rem;
            padding: 1rem;
            text-align: center;
            /* Teams Remaining */
font-family: 'Poppins-medium';
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