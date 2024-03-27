<script>
    import { goto } from "$app/navigation";
    import { team_color, match_data } from "$lib/stores";
</script>

<div class="grid place-items-center pt-4">
    <input
        bind:value={$match_data.match_key}
        type="text"
        name="Match Key"
        id=""
        placeholder="MatchKey"
    />

    <input bind:value={$match_data.team_key} type="text" name="Team Key" placeholder="TeamKey" />

    <input bind:value={$team_color} type="text" name="Team Color" id="" placeholder="TeamColor"/>

    <button on:click={() => goto("/app/scout/manual")}>Scout</button>
</div>

<style lang="postcss">
    button, input {
        font-family: "Poppins-Bold";
        padding: 15px;
        margin: 15px;
        margin-top: 10px;
        margin-bottom: 10px;
        @apply text-text_white bg-btn_grey py-2 rounded-md;
        font-size: 34px;
    }
</style>
