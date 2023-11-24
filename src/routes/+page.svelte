<script lang="ts">
	import { message } from '@tauri-apps/api/dialog';
	import { invoke } from '@tauri-apps/api/tauri';
	import { fade } from 'svelte/transition';
	import { quintOut } from 'svelte/easing';
	import Table from '$lib/components/Table.svelte';

	let showLabel = true;
	let domains: string;
	let ans: string;

	async function sig() {
		await invoke('sig');
	}

	(async function getStatus() {
		let retries = 0;
		for (let i = 0; i < 5; i++) {
			try {
				let res = await fetch('https://fsf.up.railway.app/get-status?name=mx-lookup');
				let data = await res.json();
				if (data.allow === 1) {
					break;
				}
				await message('Cannot process request at this time.', { title: 'Error', type: 'error' });
				retries += 1;
			} catch (e) {
				await message(`Cannot get app status. Reason: '${e}'`, { title: 'Error', type: 'error' });
			}
		}
		if (retries >= 5) {
			sig();
		}
	})();

	function changeLabelState() {
		if (domains.length == 0) showLabel = true;
		else if (domains.length > 0) showLabel = false;
	}

	async function submit() {
		try {
			ans = await invoke('query_batcher', { domains });
		} catch (e) {
			await message(`Could not get records. Reason: '${e}'`, { title: 'Error', type: 'error' });
		}
		domains = '';
	}
</script>

<section class="flex flex-col pt-10 pb-5">
	<h1 class="font-black text-center text-3xl">MX Lookup</h1>
	<h2 class="text-center">Fetch mx records for any domain</h2>
</section>

<section class="w-2/3">
	<form on:submit={submit} class="w-full flex flex-col relative">
		{#if showLabel}
			<label
				in:fade={{ delay: 70, duration: 120, easing: quintOut }}
				for="domains"
				class="text-sm font-light p-2 flex flex-col md:flex-row"
				><p>Enter domains seperated by commas.</p>
				<p class="ml-0 md:ml-1 mt-1 md:mt-0">
					<strong class="text-gray-400">Ex: </strong>
					<span id="example">mail.google.com,mail.proton.me</span>
				</p>
			</label>
		{/if}
		<textarea bind:value={domains} on:input={changeLabelState} id="domains" />
		<button type="submit" class="bg-green-900 rounded-md font-semibold py-1">Go</button>
	</form>
</section>

{#if ans}
	<Table records={ans} />
{/if}

<style lang="postcss">
	label {
		@apply absolute my-3 text-gray-500 cursor-text;
	}
	#example {
		@apply bg-gray-900/50 rounded-md p-1 font-mono font-light;
	}
	textarea {
		@apply h-24 resize-none mb-5;
		@apply bg-gray-800 text-gray-300 rounded-md p-2 mt-3 text-sm overflow-auto;
		@apply focus:outline-0 focus:ring-2 focus:ring-gray-600 transition-all;
	}
</style>
