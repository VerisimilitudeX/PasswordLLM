<script lang="ts" context="module">
	export const GPU_SELECT = {};
</script>

<script lang="ts">
    import { writable } from "svelte/store";
    import type { GpuStats } from "../backend_types";
    import { onMount, setContext } from "svelte";
	import { clickoutside } from '@svelte-put/clickoutside';

	export const selected = writable<GpuStats | null>(null);
	export let onSelect: (gpu: GpuStats) => void = (_) => {};

	let dropdown_visible = true;

	setContext(GPU_SELECT, {
		register: (gpu: GpuStats) => {
			selected.update(prev => prev || gpu);
		},
		select: (gpu: GpuStats) => {
			selected.set(gpu);
			onSelect(gpu);
		},
		selected: selected,
	});

	// Force the children to load so that we have the correct info from them
	onMount(() => dropdown_visible = false);
</script>


<!-- svelte-ignore a11y-click-events-have-key-events -->
<!-- svelte-ignore a11y-no-static-element-interactions -->
<div use:clickoutside on:clickoutside={() => dropdown_visible = false} on:click={() => dropdown_visible = !dropdown_visible} class="select">
	{$selected ? $selected.name : "No GPU detected"}
	<span class="pointer" style={dropdown_visible ? "transform: rotate(180deg)" : ""}/>
	{#if dropdown_visible}
		<div class="dropdown">
			<slot />
		</div>
	{/if}
</div>


<style>
	.select {
		position: relative;
		background-color: #2e3440;
		border: 2px solid #3b4252;
		border-radius: 7px;
		padding: 0.5em 1em;
		cursor: pointer;
		box-shadow: 0px 3px 6px rgba(0, 0, 0, 0.5);
		display: flex;
		align-items: center;
		gap: 1em;
	}

	.pointer {
		width: 0; 
		height: 0; 
		border-left: 5px solid transparent;
		border-right: 5px solid transparent;
		border-top: 5px solid #eceff4;
		transition: 0.4s ease;
	}

	.dropdown {
		position: absolute;
		max-height: 300px;
		background-color: #2e3440;
		border: 2px solid #3b4252;
		border-radius: 7px;
		z-index: 1;
		display: flex;
		flex-direction: column;
		margin-top: 1em;
		box-shadow: 0px 3px 6px rgba(0, 0, 0, 0.5);
		top: 100%;
		left: 0;
		right: 0;
		overflow-y: auto;
	}
</style>
