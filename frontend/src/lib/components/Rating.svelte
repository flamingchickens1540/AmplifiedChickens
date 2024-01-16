<script>
  export let name = "";
  export let value = 3;
</script>

<div class="relative m-4 h-16">
  <div class="grid grid-flow-col justify-center items-center py-2">
    {#each Array(value) as _, i}
      <button on:click={() => (value = i + 1)}>
        <svg
          width="45"
          height="45"
          viewBox="0 0 45 45"
          class="fill-cresc_green shrink-0 mx-3"
          xmlns="http://www.w3.org/2000/svg"
        >
          <path
            d="M22.3743 0.5L27.5381 16.3926H44.2486L30.7295 26.2148L35.8933 42.1074L22.3743 32.2852L8.85521 42.1074L14.019 26.2148L0.499968 16.3926H17.2104L22.3743 0.5Z"
          />
        </svg>
      </button>
    {/each}
    {#each Array(5 - value) as _, i}
      <button on:click={() => (value += i + 1)}>
        <svg
          width="45"
          height="45"
          viewBox="0 0 45 45"
          class="fill-outline_gray shrink-0 mx-3"
          xmlns="http://www.w3.org/2000/svg"
        >
          <path
            d="M22.3743 0.5L27.5381 16.3926H44.2486L30.7295 26.2148L35.8933 42.1074L22.3743 32.2852L8.85521 42.1074L14.019 26.2148L0.499968 16.3926H17.2104L22.3743 0.5Z"
          />
        </svg>
      </button>
    {/each}
  </div>

  <!-- svelte-ignore a11y-label-has-associated-control -->
  <label
    class="absolute left-[9px] top-px text-sm text-outline_gray transition-all duration-200 px-1 transform -translate-y-1/2 pointer-events-none"
    >{name}</label
  >
  <fieldset
    class="inset-0 absolute border border-outline_gray rounded pointer-events-none mt-[-9px] visible"
  >
    <legend class="ml-2 text-sm invisible px-1 max-w-full whitespace-nowrap"
      >{name}</legend
    >
  </fieldset>
</div>
