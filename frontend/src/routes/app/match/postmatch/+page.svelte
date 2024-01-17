<script>
    import Header from "$lib/components/header.svelte"
    import Stage from "$lib/components/Stage.svelte"
    import Toggle from "$lib/components/Toggle.svelte"
    import TextArea from "$lib/components/TextArea.svelte"
    import Rating from "$lib/components/Rating.svelte"
</script>

<Header let phase = "Post Match"/>
<Stage/>
<Rating name = "Driver Skill"/>
<Toggle text1="Undamaged" text2="Broken"/>
<Toggle text1="functional" text2 = "Died on Field"/>
<TextArea/>
<button>
    <h3>Submit</h3>
</button>

<style>
    h3{
        margin: 25px;
        border-color: grey;
        border-radius: 10px;
        color: lightgrey;
        background-color: #353839;
        font-size: 4rem;
        padding:2rem
    }
</style>