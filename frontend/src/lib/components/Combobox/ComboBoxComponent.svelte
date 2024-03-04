<script>
	import Context from "./Context.svelte"
	import ComboBox from "./ComboBox.svelte"
	export let value = ""
  const getFlag = code => String.fromCodePoint(...[...code.toUpperCase()].map(char => 127397 + char.charCodeAt()))
</script>

<Context>
	<div class="stack">
	<ComboBox
		name="Assignment"
		bind:value = {value}
		color="#0083E6"
		options={[
    	{ text: "Auto-Assign", value: "auto-assign" },
    	{ text: "Addi Dauler", value: "Addi Dauler" },
		{ text: "Adley Jackson", value: "Adley Jackson" },
		{ text: "Aldous Heaf", value: "Aldous Heaf", },
		{ text: "Alvin Zhang", value: "Alvin Zhang" },
		{ text: "Amanda Chen", value: "Amanda Chen" },
		{ text: "Andrew Uan-Zo-Li", value: "Andrew Uan-Zo-Li" },
		{ text: "Ariana Bajaj", value: "Ariana Bajaj" },
		{ text: "Ariv Basu", value: "Ariv Basu" },
		{ text: "Atharva Deepak", value: "Atharva Deepak" },
		{ text: "Audrey Tsai", value: "Audrey Tsai" },
		{ text: "Azalea Colburn", value: "Azalea Colburn" },
		{ text: "Blaze Annison", value: "Blaze Annison" },
		{ text: "Brandon Harad", value: "Brandon Harad" },
		{ text: "Brendan Lee", value: "Brendan Lee" },
		{ text: "Cayllie Zheng", value: "Cayllie Zheng" },
		{ text: "Charlie Phillips", value: "Charlie Phillips" },
		{ text: "Charlie Rose", value: "Charlie Rose" },
		{ text: "Charlotte Hartley", value: "Charlotte Hartley" },
		{ text: "Cora Hagerman", value: "Cora Hagerman" },
		{ text: "Crow Jahncke", value: "Crow Jahncke" },
		{ text: "Damien Ireland", value: "Damien Ireland" },
		{ text: "Darby Palmer", value: "Darby Palmer" },
		{ text: "David Kong", value: "David Kong" },
		{ text: "Deliah Weeden", value: "Deliah Weeden" },
		{ text: "Domnic Nearn", value: "Domnic Nearn" },
		{ text: "Ethan Chen", value: "Ethan Chen" },
		{ text: "Finnegan Dion-Kuhn", value: "Finnegan Dion-Kuhn" },
		{ text: "Heidi Mack", value: "Heidi Mack" },
		{ text: "Henry Willis", value: "Henry Willis" },
		{ text: "Jack Milewski", value: "Jack Milewski" },
		{ text: "Jack Sturman", value: "Jack Sturman" },
		{ text: "Jackson Tang", value: "Jackson Tang" },
		{ text: "Joseph Cole", value: "Joseph Cole" },
		{ text: "Lindsey Wohlgemuth", value: "Lindsey Wohlgemuth" },
		{ text: "Lucas Kugel", value: "Lucas Kugel" },
		{ text: "Luke Steyskal", value: "Luke Steyskal" },
		{ text: "Luna González González", value: "Luna González González" },
		{ text: "Maddox Gumboc", value: "Maddox Gumboc" },
		{ text: "Marcel Cova Brancho", value: "Marcel Cova Brancho" },
		{ text: "Mateo Luria", value: "Mateo Luria" },
		{ text: "Max Bram", value: "Max Bram" },
		{ text: "Maya Igarashi", value: "Maya Igarashi" },
		{ text: "Meera Mohit", value: "Meera Mohit" },
		{ text: "Mia Yasukawa", value: "Mia Yasukawa" },
		{ text: "Nate Lydem", value: "Nate Lydem" },
		{ text: "Neva Scott", value: "Neva Scott" },
		{ text: "Noah Conzetti", value: "Noah Conzetti" },
		{ text: "Nuri Sahavi", value: "Nuri Sahavi" },
		{ text: "Quinn Bartlo", value: "Quinn Bartlo" },
		{ text: "Quinn Hatcher-Ross", value: "Quinn Hatcher-Ross" },
		{ text: "Rachel Cohrs", value: "Rachel Cohrs" },
		{ text: "Rex Huang", value: "Rex Huang" },
		{ text: "Ryan Turner", value: "Ryan Turner" },
		{ text: "Sal Lettau", value: "Sal Lettau" },
		{ text: "Sana Shah", value: "Sana Shah" },
		{ text: "Seth Kugel", value: "Seth Kugel" },
		{ text: "Sienna Cooper", value: "Sienna Cooper" },
		{ text: "Simon Weil", value: "Simon Weil" },
		{ text: "Sinead Hough", value: "Sinead Hough" },
		{ text: "Sophia Hu", value: "Sophia Hu" },
		{ text: "Spencer Tsai", value: "Spencer Tsai" },
		{ text: "Theo Fisher", value: "Theo Fisher" },
		{ text: "William Spry", value: "William Spry" },
		{ text: "Xinyo Luo", value: "Xinyu Luo" },
		{ text: "Zach Rutman", value: "Zach Rutman" },
  	]}
	/>
	
	
	</div>
		
</Context>

<style>	
	.stack {
		display: flex;
		flex-direction: column;
		gap: 1.5rem;
	}
</style>