<script lang="ts">
    import type { TeamMatch } from "$lib/types.ts";
    import { Modal, Content, Trigger } from "sv-popup";
    import { onMount } from "svelte";

    // The dashboard page needs to pass these in through the SSE Stream or smth
    export let scouted_robots: TeamMatch[] = [];
    export let last_match = "";
    const BACKEND_URL = import.meta.env.VITE_BACKEND_URL;

    // const server_source = new EventSource(
        // `${BACKEND_URL}/admin/lastMatchStream`,
    // );

    // server_source.addEventListener("new_scouted_match", (event) => {
    //     var message = JSON.parse(event.data);
    //     console.log(message);
    //     let i = scouted_robots.indexOf(message.scouted_robot);
    //     if (i == -1) {
    //         scouted_robots.push(message.scouted_robot);
    //     }
    //     scouted_robots[i].status = message.scouted_robot.status;
    //     last_match = message.scouted_robot.match_key;
    // });
</script>

<div class="rounded" style="background-color: #2C2C2C; padding:1rem">
    <h3>Last Match ({last_match})</h3>
    {#each scouted_robots as scouted_robot}
        <div
            class="grid rounded grid-cols-20 justify-between content-center"
            style="background-color: #5C5C5C; margin: 12px"
        >
            <h3 class="self-center">{scouted_robot.scout_name}</h3>
            <div class="col-start-5">
                <Modal basic>
                    <!-- <Content>    
                    </Content> -->
                    <Trigger>
                        {#if scouted_robot.status == "complete"}
                            <h2
                                class="rounded"
                                style="background-color: #00D586; padding: 1.5rem; padding-top: 0.8rem; padding-bottom: 0.8rem"
                            >
                                Complete
                            </h2>
                        {:else if scouted_robot.status == "pending"}
                            <h2
                                class="rounded col-start-5"
                                style="background-color: #F6D93F; padding: 2rem; padding-top: 0.8rem; padding-bottom: 0.8rem"
                            >
                                Pending
                            </h2>
                        {:else}
                            <h2
                                class="rounded col-start-5"
                                style="background-color: ##EE3C42; padding: 2rem; padding-top: 0.8rem; padding-bottom: 0.8rem"
                            >
                                Unassigned
                            </h2>
                        {/if}
                    </Trigger>
                </Modal>
            </div>
        </div>
    {/each}
</div>

<style>
    h3 {
        font-family: "Poppins-bold";
        font-size: 25px;
        line-height: 91.5%;
        color: #ffffff;
        margin: 0.5rem;
    }
    h2 {
        font-family: "Poppins-bold";
        font-size: 20px;
        color: #1c1c1c;
        line-height: 91.5%;
        margin: 0.5rem;
    }
</style>
