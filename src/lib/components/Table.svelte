<script lang="ts">
	import type { MXRecord } from '../types/records';

	export let records: string;
	let r: MXRecord[];

	function setRecords() {
		if (records.length > 0) r = JSON.parse(records);
	}

	$: records && setRecords();
</script>

<section class="my-12 md:w-2/3 w-10/12">
	<h1 class="text-xl font-bold text-center mb-4">Results</h1>
	<div class="bg-gray-800 rounded-md h-52 p-4 text-gray-300 max-h-52 overflow-auto">
		<div class="res-row mb-2" id="res-head">
			<h2>Domain</h2>
			<h2>TTL</h2>
			<h2>Priority</h2>
			<h2>MX Target</h2>
		</div>
		{#if r}
			{#each r as record}
				<div class="res-row">
					<h3>{record.domain}</h3>
					<h3>{record.ttl}</h3>
					<h3>{record.priority}</h3>
					<h3>{record.target}</h3>
				</div>
			{/each}
		{/if}
	</div>
</section>

<style lang="postcss">
	#res-head h2 {
		@apply font-bold underline;
	}
	.res-row {
		@apply grid grid-cols-4 font-light md:text-sm text-xs;
	}

	.res-row h3 {
		@apply overflow-x-scroll h-10 overflow-y-hidden;
	}
</style>
