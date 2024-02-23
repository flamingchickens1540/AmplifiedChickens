<script lang="ts">
    import type { Scout } from "$lib/types"
    import { onMount } from "svelte";

    export let access_token: string

    let scouts: string[] = []
    let num_scouted: number[] = []

    onMount(async () => {
        console.log("mounted")

        let res = await fetch("https://scout.team1540.org/api/admin/users/get/all", {
            method: "GET",
            headers: {
                "Content-Type": "application/json",
                "x-access-token": access_token,
            }
        })

        if (!res.ok) {
            console.error("Failed to fetch scout percents")
        }

        let data = await res.json()

        console.log("Number scouted: ", data)

        scouts = data[0]
        num_scouted = data[1]

    })
</script>

<div class="rounded" style="background-color: #2C2C2C; padding:1rem">
    <div class="flex justify-between">
        <h3>Name</h3>
        <h3># Scouted</h3>
    </div>
    <div class="main">
        {#each scouts as scout, i}
            <hr style="color: #C2C2C2" />
            <div class="flex justify-between">
                <h2>{scout}</h2>
                <h2>{num_scouted[i]}</h2>
            </div>
        {/each}
    </div>
</div>

<style>
    h3 {
        font-family: "Poppins-bold";
        font-style: normal;
        font-size: 17px;
        margin: 13px;
        color: #ffffff;
    }
    h2 {
        font-family: "Poppins-medium";
        font-style: normal;
        font-size: 20px;
        margin: 16px;
        margin-top: 10px;
        margin-bottom: 10px;
        color: #ffffff;
    }
    .main {
        overflow-y: scroll;
        max-height: 255px;
    }
</style>
