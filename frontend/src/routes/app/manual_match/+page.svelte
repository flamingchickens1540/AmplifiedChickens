<script>
    import { goto } from "$app/navigation";
    import { team_color, match_data } from "$lib/stores";
    import { default_match_data } from "$lib/types";
    import { onMount } from "svelte";
    
    onMount(() => {
        $team_color = ""
        $match_data = JSON.parse(JSON.stringify(default_match_data))
    })
</script>

<div class="grid place-items-center pt-10">
    <!-- <input
        bind:value={$match_data.match_key}
        type="text"
        name="Match Key"
        id=""
        placeholder="MatchKey"
    /> -->

    <input
        bind:value={$match_data.team_key}
        type="text"
        name="Team Key"
        placeholder="TeamKey"
    />

    <!-- <div class="grid place-items-center" id="team">
        <label for="team_color">Team Color:</label>
        <select bind:value={$team_color} name="team_color" class="outline" id="">
            <option value="blue">Blue</option>
            <option value="red">Red</option>
        </select>
    </div> -->

    <button on:click={() => goto("/app/scout/manual")}>Scout</button>
</div>

<style lang="postcss">
    button,
    input,
    select,
    option,
    label,
    #team { 
        font-family: "Poppins-Bold";
        padding: 15px;
        margin: 15px;
        margin-top: 10px;
        margin-bottom: 10px;
        @apply text-text_white bg-btn_grey py-2 rounded-md;
        font-size: 28px;
    }
</style>
