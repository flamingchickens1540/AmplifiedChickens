<script>
    export let name = "";
    export let value = "";
</script>

<!-- CREDIT: https://play.tailwindcss.com/asmAkefxLr -->
<div class="relative m-4 group">
<<<<<<< HEAD
    <input
        type="text"
        bind:value
        class="outline-none px-3 py-3 peer text-text_white"
        style="background-color: #353839;"
        placeholder=" "
    />
=======
  <input
    type="text"
    bind:value
    class="outline-none px-3 py-3 peer text-text_white"
    style="background-color: #353839;"
    placeholder=" "

  />
>>>>>>> efa7ad755fe28f2d82c266b2b627fb6637c0756f

    <!-- svelte-ignore a11y-label-has-associated-control -->
    <label
        class="absolute left-[9px] top-px text-sm text-outline_gray transition-all duration-200 px-1 transform -translate-y-1/2 pointer-events-none
  peer-placeholder-shown:top-1/2 peer-placeholder-shown:text-xl group-focus-within:!top-px group-focus-within:!text-sm group-focus-within:!text-cresc_green"
        >{name}</label
    >

    <fieldset
        class="inset-0 absolute border border-outline_gray rounded pointer-events-none mt-[-9px] invisible peer-placeholder-shown:visible
  group-focus-within:!border-cresc_green group-focus-within:border-2"
    >
        <legend
            class="ml-2 px-0 text-sm transition-all duration-200 invisible max-w-[0.01px] group-focus-within:max-w-full group-focus-within:px-1 whitespace-nowrap"
            >{name}</legend
        >
    </fieldset>

    <fieldset
        class="inset-0 absolute border border-outline_gray rounded pointer-events-none mt-[-9px] visible peer-placeholder-shown:invisible
  group-focus-within:border-2 group-focus-within:!border-cresc_green"
    >
        <legend class="ml-2 text-sm invisible px-1 max-w-full whitespace-nowrap"
            >{name}</legend
        >
    </fieldset>
</div>

<style>
<<<<<<< HEAD
    input {
        width: 100%;
    }
</style>
=======
  input{
    width: 100%;
  }
</style>
>>>>>>> efa7ad755fe28f2d82c266b2b627fb6637c0756f
