<script lang="ts">
    import type { PageData } from "./$types";
    import SubmitButton from "$lib/components/SubmitButton.svelte";
    import { Modal, Content, Trigger } from "sv-popup";
    import Navbar from "$lib/components/Navbar.svelte";
    import Pie from "$lib/components/Pie.svelte";
    import { goto } from "$app/navigation";
    import { onMount } from "svelte";
    import { current_event_key } from "$lib/stores/homeStores";

    export let data: PageData;
    export let nextMatchTime = 1706579944;
    export let twitchURL = "https://www.twitch.tv/firstinspires";

    let name = data.scout_name;

    current_event_key.set(data.current_event_key?.toString() || "2024orore");

    function determine_greeting(): string {
        const currentTime = new Date().getHours();
        if (currentTime < 12) {
            return "Morning";
        } else {
            return "Afternoon";
        }
    }

    async function get_scout_percents(): Promise<readonly [string[], number[]]> {
        let res = await fetch("https://localhost:3007/admin/users/get/all", {
            method: "GET",
            headers: {
                Accept: "application/json",
                "Content-Type": "application/json",
            }
        })

        console.log(res)

        return res.json()
    }

    let greeting = determine_greeting();

    let scout_names: string[] = []
    let scout_percents: number[] = []

    onMount(async () => {
        let result = await get_scout_percents()

        console.log(result)

        scout_names = result[0]
        scout_percents = result[1]
    })

</script>

<main class="bg-bg_gray h-screen flex flex-col justify-between">
    <div class="grid content-end">
        <h1 class="px-3 text-text_white pt-10">Good {greeting}</h1>
        <h1 class="px-3 text-cresc_green">{name}.</h1>
        <Modal>
            <Content
                style="background-color: #2C2C2C; width:92%; margin:auto; overflow-wrap: normal;
      overflow-wrap: break-word;
      overflow-wrap: anywhere;"
                class="p-4 rounded"
            >
                <h2 style="color: white;">Debug Menu:</h2>
                <SubmitButton text="Subscribe!" />
                <code
                    style="font-size: 20px; font-family: poppins-medium; color: white; overflow: wrap;"
                    >{data}</code
                >
            </Content>
            <Trigger>
                <p class="px-3 text-outline_gray">You are at {$current_event_key}</p>
            </Trigger>
        </Modal>
    </div>

    <div class="flex flex-col content-center items-stretch">
        <div
            style=""
            class="bg-btn_grey h-[185px] mx-3 grid-cols-4 grid gap-2 content-center items-center rounded-md"
        >
            {#each scout_percents as _, i}
                {#if i < 8}
                    <div class="p-1 flex flex-col items-center">
                        <Pie size={46} percent={scout_percents[i]} />
                        <p class="text-text_white">{scout_names[i]}</p>
                    </div>
                {/if}
            {/each}
        </div>
        <div class="flex flex-row w-full content-center justify-around items-end">
            <a href="https://www.thebluealliance.com/event/{current_event_key}" style="margin-left: 15px; margin-right: 15px;">
                <button
                    style="margin-left:0px; width: 100%"
                >
                    TheBlueAlliance
                </button>
            </a>
            <a href="https://www.statbotics.io/event/{current_event_key}" style="margin-left: 0px; margin-right: 15px; margin-top: 8px; width: 100%;">
                <button
                    style="width:100%; margin-right: 15px; margin-left: 0px; padding-left: 0px; padding-right: 0px;"
                >
                    Statbotics
                </button>
            </a>
        </div>
        <a href="{twitchURL}" style="margin-left: 15px; margin-right: 15px;">
            <button
                style="margin-left:0px; width: 100%"
            >
                Twitch Stream
            </button>
        </a>
        <button
            style="padding: 1.5rem"
            id="Pit-Scounts"
            on:click={() => goto("/app/pit")}>Pit Scout</button
        >
        <button
            style="padding: 2.5rem"
            id="Match-Scounts"
            on:click={() => goto("/app/match")}>Match Scout</button
        >
    </div>
    <div>
        <Navbar page="home" />
    </div>
</main>

<style lang="postcss">
    h1 {
        @apply font-bold text-4xl;
    }
    h2 {
        @apply text-4xl;
        color: white;
        margin: 15px;
        font-family: Poppins-bold;
    }
    p {
        @apply font-medium text-xl;
    }
    button {
        font-family: "Poppins-Medium";
        margin: 15px;
        margin-top: 8px;
        margin-bottom: 8px;
        font-size: 20px;
        padding: 1rem;
        @apply text-text_white bg-btn_grey py-2 rounded-md;
    }
    #Match-Scounts {
        @apply text-navbar_black bg-cresc_green py-5 font-semibold text-4xl;
    }
    #Pit-Scounts {
        @apply font-medium text-3xl;
    }
</style>
