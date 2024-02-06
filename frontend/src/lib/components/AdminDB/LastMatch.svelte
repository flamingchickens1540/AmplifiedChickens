<script lang="ts">
    import type { Scout } from "$src/lib/types.ts"
    import { Modal, Content, Trigger } from "sv-popup"

    let team_matches: Scout[] = []
    let last_match = ""
    
    const server_source = new EventSource("https://sse.dev/test")

    server_source.addEventListener("new_team_match", (event) => {
        var message = JSON.parse(event.data)
        team_matches.push(message.team_match)
        last_match = message.team_match.match_key
    })

</script>
<div class="rounded" style="background-color: #2C2C2C; padding:1rem">
<h3>Last Match ({last_match})</h3>
{#each team_matches as team_match}
{#if team_match.status == "Complete"}
<div class="grid rounded grid-cols-20 justify-between content-center" style="background-color: #5C5C5C; margin: 12px">
<h3 class="self-center"> {team_match.name}</h3>
<div class="col-start-5">
    <Modal basic>
        <Content>
            <h2>media data display design goes here</h2>
            <h2>these are filler lines to show that this can hold multiple lines</h2>
            <h2>these are filler lines to show that this can hold multiple lines</h2>
            <h2>these are filler lines to show that this can hold multiple lines</h2>
            <h2>these are filler lines to show that this can hold multiple lines</h2>
            <h2>these are filler lines to show that this can hold multiple lines</h2>
            <h2>these are filler lines to show that this can hold multiple lines</h2>
            <h2>(I can change the background color)</h2>
        </Content>
        <Trigger>
            <h2 class="rounded" style="background-color: #00D586; padding: 1.5rem; padding-top: 0.8rem; padding-bottom: 0.8rem">Complete</h2>
        </Trigger>
      </Modal>
</div>
</div>
{:else if team_match.status == "Pending"}
<div class="grid rounded grid-cols-20 justify-between content-center" style="background-color: #5C5C5C; margin: 12px">
    <h3 class="self-center">{team_match.name}</h3>
    <div class="col-start-5">
        <Modal basic>
            <Content>
                <h2>media data display design goes here</h2>
                <h2>these are filler lines to show that this can hold multiple lines</h2>
                <h2>these are filler lines to show that this can hold multiple lines</h2>
                <h2>these are filler lines to show that this can hold multiple lines</h2>
                <h2>these are filler lines to show that this can hold multiple lines</h2>
                <h2>these are filler lines to show that this can hold multiple lines</h2>
                <h2>these are filler lines to show that this can hold multiple lines</h2>
                <h2>(I can change the background color)</h2>
            </Content>
            <Trigger>
                <h2 class="rounded col-start-5" style="background-color: #F6D93F; padding: 2rem; padding-top: 0.8rem; padding-bottom: 0.8rem">Pending</h2>
            </Trigger>
          </Modal>
    </div>
    </div>
{:else}
<div class="grid rounded grid-cols-20 justify-between content-center" style="background-color: #5C5C5C; margin: 12px">
    <h3 class="self-center">{team_match.name}</h3>
    <div class="col-start-5">
        <Modal basic>
            <Content>
              <h2>media data display design goes here</h2>
              <h2>these are filler lines to show that this can hold multiple lines</h2>
              <h2>these are filler lines to show that this can hold multiple lines</h2>
              <h2>these are filler lines to show that this can hold multiple lines</h2>
              <h2>these are filler lines to show that this can hold multiple lines</h2>
              <h2>these are filler lines to show that this can hold multiple lines</h2>
              <h2>these are filler lines to show that this can hold multiple lines</h2>
              <h2>(I can change the background color)</h2>
            </Content>
            <Trigger>
                <h2 class="rounded col-start-5" style="background-color: #EE3C42; padding: 0.8rem; padding-top: 0.8rem; padding-bottom: 0.8rem;">Unassigned</h2>
            </Trigger>
          </Modal>
    </div>
    </div>
{/if}
{/each}
</div>
<style>
    h3{
    font-family: 'Poppins-bold';
    font-size: 25px;
    line-height: 91.5%;
    color: #FFFFFF;
    margin:0.5rem
    }
    h2{
font-family: 'Poppins-bold';
font-size: 20px;
color: #1C1C1C;
line-height: 91.5%;
margin:0.5rem
    }
    </style>
