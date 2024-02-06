<script lang="ts">
  import QueueMatch from "$lib/components/AdminDB/QueueMatch.svelte";
  import LastMatch from "$lib/components/AdminDB/LastMatch.svelte";
  import QueuedScouts from "$lib/components/AdminDB/QueuedScouts.svelte";
  import NumberScouted from "$lib/components/AdminDB/NumberScouted.svelte";
  import EventManagement from "$lib/components/AdminDB/EventManagement.svelte";
  import UserManagement from "$lib/components/AdminDB/UserManagement.svelte";
</script>

<div
  style="background-color: #1C1C1C; padding:2rem; overflow: hidden;"
  class="grid grid-cols-2 gap-5 overflow-hidden"
>
  <div class="col-span-1 row-span-1 col-start-1 row-start-1">
    <QueueMatch />
  </div>

  <div class="grid grid-cols-2 grid-rows-1 gap-5">
    <div>
      <LastMatch />
    </div>
    <div>
      <QueuedScouts />
    </div>
  </div>

  <div class="grid grid-cols-5 gap-5">
    <div class="col-span-2">
      <NumberScouted />
    </div>
    <div class="col-span-3">
      <EventManagement />
    </div>
  </div>

  <div>
    <UserManagement />
  </div>
</div>
