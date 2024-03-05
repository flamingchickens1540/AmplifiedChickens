<script>
    export let text = "Submit!";
    
    export let onClick = () => {} 
</script>

<center><button on:click={onClick} class="rounded">
    {text}
</button></center>

<style>
    button {
        margin: 17px;
        color: #ffffff;
        background-color: #5c5c5c;
        font-size: 2rem;
        padding: 1rem;
        text-align: center;
        font-family: "Poppins-medium";
        font-style: bold;
        border-radius: 5px;
    width: calc(100vw - 34px);
    }
</style>
