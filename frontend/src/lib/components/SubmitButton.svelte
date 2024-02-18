<script>
    export let text = "Submit!";

    export let onClick = () => {};
</script>

<button on:click={onClick} class="rounded">
    {text}
</button>

<style>
    button {
        margin: 17px;
        color: #ffffff;
        background-color: #5c5c5c;
        font-size: 2rem;
        padding: 1rem;
        text-align: center;
        /* Teams Remaining */
        font-family: "Poppins-medium";
        font-style: bold;
    }
</style>
