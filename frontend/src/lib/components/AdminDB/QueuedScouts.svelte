<script>
    export let queued = ["Rex", "Azalea", "Jack", "Blaze", "Charlie", "Domnic", "Jackson"]
    queued = queued.slice(0, 6)
</script>
<h3>Queued Scouts</h3>
{#each queued as person}
<h3 class="rounded" style="padding: 1rem; background-color: #5C5C5C">{person}</h3>
{/each}
<style>
h3{
font-family: 'Poppins';
font-style: normal;
font-weight: 700;
font-size: 40px;
line-height: 91.5%;
color: #FFFFFF;
margin:17px
}

</style>