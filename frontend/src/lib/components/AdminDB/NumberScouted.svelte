<script lang="ts">
    import type { Scout } from "$lib/types.ts"
    let scouts:Scout[] = []
    scouts.push({
    name: "Rex",
    admin: true,
    hasnotifs: true,
    numscouted: 100
    })
    scouts.push({
    name: "Azalea",
    admin: false,
    hasnotifs: true,
    numscouted: 100
    })
    scouts.push({
    name: "Blaze",
    admin: true,
    hasnotifs: false,
    numscouted: 100
    })
    scouts.push({
    name: "Jackson",
    admin: false,
    hasnotifs: false,
    numscouted: 0
    })
    scouts.push({
    name: "Rex",
    admin: true,
    hasnotifs: true,
    numscouted: 100
    })
    scouts.push({
    name: "Azalea",
    admin: false,
    hasnotifs: true,
    numscouted: 100
    })
    scouts.push({
    name: "Blaze",
    admin: true,
    hasnotifs: false,
    numscouted: 100
    })
    scouts.push({
    name: "Jackson",
    admin: false,
    hasnotifs: false,
    numscouted: 0
    })
    scouts.push({
    name: "Rex",
    admin: true,
    hasnotifs: true,
    numscouted: 100
    })
   
</script>
<div class="rounded" style="background-color: #2C2C2C; padding:1rem">
<div class="flex justify-between">
<h3>Name</h3>
<h3># Scouted</h3>
</div>
<div class="main">
{#each scouts as scout}
<hr style="color: #C2C2C2">
<div class="flex justify-between">
<h2>{scout.name}</h2>
<h2>{scout.numscouted}</h2>
</div>
{/each}
</div>
</div>
<style>
h3{
font-family: 'Poppins-bold';
font-style: normal;
font-size: 24px;
margin:13px;
color: #FFFFFF;
}
h2{
font-family: 'Poppins-medium';
font-style: normal;
font-size: 20px;
margin:16px;
margin-top:10px;
margin-bottom: 10px;
color: #FFFFFF;
}
.main{
    overflow-y: scroll;
    max-height: 280px;
}
</style>