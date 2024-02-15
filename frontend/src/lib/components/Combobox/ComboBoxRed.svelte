<script lang="ts">
	import Context from "./Context.svelte";
	import ComboBox from "./ComboBox.svelte";
	export let value = "";
	export let color = "";
	export let options: string[] = [];
	//   const getFlag = code => String.fromCodePoint(...[...code.toUpperCase()].map(char => 127397 + char.charCodeAt()))
</script>

<Context>
	<div class="stack">
		<ComboBox name="Assignment" bind:value {color} {options} />
	</div>
</Context>

<style>
	.stack {
		display: flex;
		flex-direction: column;
		gap: 1.5rem;
	}
</style>
