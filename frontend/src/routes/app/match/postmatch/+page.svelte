<script>
    import Header from "$lib/components/Header.svelte"
    import Stage from "$lib/components/Stage.svelte"
    import Toggle from "$lib/components/Toggle.svelte"
    import TextArea from "$lib/components/TextArea.svelte"
    import Rating from "$lib/components/Rating.svelte"
</script>
<Header let phase = "Post Match"/>
<Stage/>
<Rating name = "Driver Skill"/>
<Toggle text1="Undamaged" text2="Broken"/>
<Toggle text1="Functional" text2 = "Died on Field"/>
<TextArea/>
<div class="grid grid-cols-5">
<button class="col-span-5">
    <h3 class="col-span-5 border border-outline_gray rounded pointer-events-none mt-[-9px] visible">Submit!</h3>
</button>
</div>
<style>
    h3{
        margin: 25px;
        color: lightgrey;
        background-color: #5c5c5c;
        font-size: 2rem;
        padding: 2rem;
        text-align: center;
    }
</style>