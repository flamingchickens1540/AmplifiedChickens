<script lang="ts">
  import TextInput from "$lib/components/TextInput.svelte";
  import Toggle from "$lib/components/Toggle.svelte";
  import Threeoption from "$lib/components/Threeoption.svelte";
  import Rating from "$lib/components/Rating.svelte";
  import Textarea from "$lib/components/TextArea.svelte";
  import SubmitButton from "$lib/components/SubmitButton.svelte";
  import TeamsRemainingPopup from "$lib/components/TeamsRemainingPopup.svelte";
  import Navbar from "$lib/components/Navbar.svelte";
  import ImageUpload from "$lib/components/ImageUpload.svelte";
  import { Modal, Content, Trigger } from "sv-popup";
  let teamnumber = "";
  let width = "";
  let height = "";
  let weight = "";
  let wheredrive = "";
  let drivetrain = "";
  let intake = "";
  let chute = false;
  let ground = false;
  let polish = 3;
  let notes = "";
  if (intake == "both") {
    chute = true;
    ground = true;
    chute = chute;
    ground = ground;
  } else if (intake == "Ground") {
    ground = true;
    ground = ground;
  } else {
    chute = true;
    chute = chute;
  }
</script>

<Modal>
  {#if teamnumber == ""}
    <Content
      style="background-color: #2C2C2C; width:92%; margin:auto"
      class="p-4 rounded"
    >
      <TeamsRemainingPopup bind:value={teamnumber} />
    </Content>
  {/if}
  <Trigger>
    <SubmitButton text="Teams Remaining" />
  </Trigger>
</Modal>
<TextInput name="Team Number" bind:value={teamnumber} />
<TextInput name="Width" bind:value={width} />
<TextInput name="Length" bind:value={height} />
<TextInput name="Weight" bind:value={weight} />
<Toggle text1="Under Stage" text2="Around Stage" bind:value={wheredrive} />
<Threeoption
  text1="Swerve"
  text2="Tank"
  text3="Other"
  bind:value={drivetrain}
/>
<Threeoption text1="Chute" text2="Ground" text3="Both" bind:value={intake} />
<Rating name="Robot Polish" bind:value={polish} />
<Textarea bind:value={notes} />
<ImageUpload />

<div>
  <Navbar page="pit" />
</div>

<style>
  #navbar {
    position: fixed;
    bottom: 0;
    left: 0;
    width: 100%;
    background-color: #f0f0f0; /* Just for visualization */
    text-align: center;
  }
</style>
