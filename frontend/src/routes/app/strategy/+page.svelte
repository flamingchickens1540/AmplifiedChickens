<script>
    import Navbar from "$lib/components/Navbar.svelte";
</script>

<div class="yellow-screen">
    <h1 class="coming-soon">COMING SOON!</h1>
</div>

<div class="bottom-div">
    <Navbar page="strategy" />
</div>

<style>
    .yellow-screen {
        background-color: #f6d93f;
        display: flex;
        flex-direction: column;
        justify-content: center;
        align-items: center;
        height: calc(100vh - 60px); /* Adjusted height to accommodate navbar */
    }

    .coming-soon {
        font-size: 36px;
        font-family: poppins-medium;
        color: black;
        margin-bottom: 20px;
    }

    .navbar {
        background-color: #333;
        color: white;
        display: flex;
        justify-content: center;
        align-items: center;
        height: 60px;
        width: 100%;
        position: fixed;
        bottom: 0;
    }
    .bottom-div {
        position: fixed;
        bottom: 0;
        left: 0;
        width: 100%;
        background-color: #f0f0f0; /* Just for visualization */
        text-align: center;
    }
</style>
