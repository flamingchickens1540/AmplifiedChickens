<script lang="ts">
    export let lastmatch = "qm4"
    import type { Scout } from "$src/lib/types.ts"
    let robots:Scout[] = [] 
    import { Modal, Content, Trigger } from "sv-popup"
    robots.push({
        name: "1540",
        status: "Complete"
    })
    robots.push({
        name: "254",
        status: "Complete"
    })
    robots.push({
        name: "1678",
        status: "Complete"
    })
    robots.push({
        name: "2056",
        status: "Complete"
    })
    robots.push({
        name: "118",
        status: "Pending"
    })
    robots.push({
        name: "5920",
        status: "Not Assigned"
    })
    robots = robots.slice(0, 6)
</script>
<div class="rounded" style="background-color: #2C2C2C; padding:1rem">
<h3>Last Match ({lastmatch})</h3>
{#each robots as robot}
{#if robot.status == "Complete"}
<div class="grid rounded grid-cols-20 justify-between content-center" style="background-color: #5C5C5C; margin: 12px">
<h3 class="self-center"> {robot.name}</h3>
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
{:else if robot.status == "Pending"}
<div class="grid rounded grid-cols-20 justify-between content-center" style="background-color: #5C5C5C; margin: 12px">
    <h3 class="self-center">{robot.name}</h3>
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
    <h3 class="self-center">{robot.name}</h3>
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