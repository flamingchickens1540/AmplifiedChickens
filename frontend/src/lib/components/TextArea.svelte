<script>
    export let value = ""
</script>

<div class="border border-outline_gray rounded mt-[-9px] visible ">
<textarea bind:value={value} rows="4" placeholder="Notes"></textarea>
</div>

<style>
    textarea{
        background-color: #353839;
        width: 100%; 
        color: lightgrey;
        font-family: Poppins-medium;
    }
    div {
        margin: 17px;
        padding:0.2rem;

       
    }
</style>