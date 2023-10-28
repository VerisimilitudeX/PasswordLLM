<script lang="ts">
    import { getContext } from "svelte";
    import { GPU_SELECT } from "./GpuSelect.svelte";
    import type { GpuStats } from "../backend_types";

	export let gpu: GpuStats;

	const {register, select, selected} = getContext(GPU_SELECT) as any;
	register(gpu);

	function get_title_color() {
		let name = gpu.name.toLowerCase();
		if (name.includes("nvidia")) {
			return "#a3be8c";
		} else if (name.includes("intel")) {
			return "#88c0d0";
		} else if (name.includes("amd") || name.includes("radeon")) {
			return "#bf616a";
		} else {
			return "gray";
		}
	}
</script>

<!-- svelte-ignore a11y-click-events-have-key-events -->
<!-- svelte-ignore a11y-no-static-element-interactions -->
<div class="option" on:click={select(gpu)}>
	<span class="title" style="color: {get_title_color()};">{gpu.name}</span>
	<div class="extra-info">
		<span>Clock speed:</span>
		<span>{gpu.clock} MHz</span>

		<span>CUDA cores:</span>
		<span>{gpu.cuda_cores} ({gpu.smp} SMP units)</span>

		<span>FP32 GFLOPS:</span>
		<span>{gpu.gflops_fp32} </span>

		<span>FP64 GFLOPS:</span>
		<span>{gpu.gflops_fp64}</span>
	</div>
</div>

<style>
	.option {
		padding: 0.5em 1em;
		cursor: pointer;
		transition: 0.2s ease;
	}

	.extra-info {
		display: grid;
		grid-template-columns: 50% 50%;
		text-align: start;
	}

	.extra-info > span:nth-child(odd) {
		font-size: 12px;
	}

	.extra-info > span:nth-child(even) {
		font-size: 10px;
		color: #d8dee9;
	}

	.option:hover {
		background-color: #434c5e;
	}
</style>