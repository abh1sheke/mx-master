<script lang="ts">
	import { save, message } from '@tauri-apps/api/dialog';
	import { invoke } from '@tauri-apps/api/tauri';
	import type { MXRecord, ErrorRecord } from '../types/records';

	export let records: string;
	let recordsJson: { mx: MXRecord[]; err: ErrorRecord[] };

	function setRecords() {
		if (records.length > 0) recordsJson = JSON.parse(records);
    console.log(recordsJson);
	}

	async function saveFile() {
		let path: string | null;
		try {
			path = await save({
				filters: [{ name: 'Excel', extensions: ['xlsx'] }]
			});
		} catch (e) {
			await message(`Could not save file.\nReason: ${e}`, { title: 'MX Lookup', type: 'error' });
			console.log(e);
			return;
		}
		try {
			let res: string = await invoke('save_to', { records, path });
			let parsed = JSON.parse(res);
			if (parsed.success === true) {
				await message(`Saved file successfully at '${path as string}'`, { title: 'Success' });
			}
		} catch (e) {
			await message(`Could not save file.\nReason: ${e}`, { title: 'Error', type: 'error' });
		}
	}

	$: records && setRecords();
</script>

<section class="my-12 md:w-2/3 w-10/12">
	<h1 class="text-xl font-bold text-center mb-4">Results</h1>
	<div class="flex justify-center my-4">
		<button class="px-2 py-1 bg-green-800 font-semibold rounded-md" on:click={saveFile}
			>Download as Excel</button
		>
	</div>
	<div class="bg-gray-800 rounded-md h-52 p-4 text-gray-300 max-h-52 overflow-auto">
		<div class="res-row mb-2" id="res-head">
			<h2>Domain</h2>
			<h2>TTL</h2>
			<h2>Priority</h2>
			<h2>MX Target</h2>
		</div>
		{#if recordsJson}
			{#each recordsJson.mx as record}
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

<section class="my-12 md:w-2/3 w-10/12">
	<h1 class="text-xl font-bold text-center mb-4">Errors</h1>
	<div class="bg-gray-800 rounded-md h-52 p-4 text-gray-300 max-h-52 overflow-auto">
		<div class="res-row-err mb-2" id="res-head">
			<h2>Domain</h2>
			<h2>Reason</h2>
		</div>
		{#if recordsJson}
			{#each recordsJson.err as record}
				<div class="res-row-err">
					<h3>{record.domain}</h3>
					<h3>{record.reason}</h3>
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
	.res-row-err {
		@apply grid grid-cols-2 font-light md:text-sm text-xs;
	}
	.res-row h3, .res-row-err h3 {
		@apply overflow-x-scroll h-10 overflow-y-hidden;
	}
</style>
