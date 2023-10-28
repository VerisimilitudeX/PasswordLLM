<script lang="ts">
    import { invoke } from '@tauri-apps/api/tauri';
    import type { GpuStats, PasswordEvaluationResult, PasswordStrength } from './backend_types';
    import GpuSelect from './components/GpuSelect.svelte';
    import Gpu from './components/GpuOption.svelte';

	let selected_gpu = 0;
	let gpu_list: GpuStats[];
	invoke("list_gpus")
		.then(list => gpu_list = list as GpuStats[])
		.catch(console.error);

	let password: string;
	let password_evaluation: PasswordEvaluationResult;
	let gpu_index_used = 0;
	let password_evaluating = false;

	// Wait for a few milliseconds after the user presses a key
	// so as not to spam the backend
	let bounce: number;
	function onPasswordChange() {
		clearTimeout(bounce);
		bounce = setTimeout(() => {
			password_evaluating = true;
			gpu_index_used = selected_gpu;
			invoke("evaluate_password_strength", { password: password, gpuIndex: selected_gpu })
				.then(res => password_evaluation = res as PasswordEvaluationResult)
				.catch(console.error)
				.finally(() => password_evaluating = false)
		}, 700);
	}

	function get_color_for_pass_strength(strength: PasswordStrength) {
		if (strength == 'ProneToBruteforceAttack' || strength == 'ProneToDictionaryAttack' || strength == 'Weak') {
			return "#bf616a";
		} else if (strength == 'Medium') {
			return "#ebcb8b";
		} else {
			return "#a3be8c";
		}
	}

	function seconds_to_wdhms_str(total_seconds: number) {

		//create a function that displays the total seconds in a human readable format
		

		let seconds = Math.floor(total_seconds % 60);
		let minutes = Math.floor((total_seconds / 60) % 60);
		let hours = Math.floor((total_seconds / (60 * 60)) % 24);
		let days = Math.floor((total_seconds / (60 * 60 * 24)) % 7);
		let weeks = Math.floor((total_seconds / (60 * 60 * 24 * 7)) % 52.1429);
		const months = Math.floor((total_seconds / (60 * 60 * 24 * 30)) % 12);
		const years = Math.floor(total_seconds / (60 * 60 * 24 * 365));
		
		//const block = `${weeks > 0 ?? weeks + "Weeks, "}`
		
		return `${years > 0 ? years.toLocaleString() + " Years, " : ""}${months > 0 ? months.toLocaleString() + " Months, " : ""}${weeks > 0 ? weeks.toLocaleString() + " Weeks, " : ""}${days > 0 ? days.toLocaleString() + " Days, " : ""}${hours > 0 ? hours.toLocaleString() + " Hours, " : ""}${minutes > 0 ? minutes.toLocaleString() + " Minutes, " : ""}${seconds > 0 && years < 0 ? seconds + " Seconds" : ""}`;
		//return `${weeks.toLocaleString()} weeks, ${days} days, ${hours} hours, ${minutes} minutes and ${seconds} seconds`;
	}
</script>

<main class="container">
	<nav>
		<span>GPU</span>
		{#if gpu_list && gpu_list.length > 0}
			<GpuSelect onSelect={(gpu) => {
				selected_gpu = gpu_list.indexOf(gpu);
				onPasswordChange();
			}}>
				{#each gpu_list as gpu}
					<Gpu gpu={gpu}/>
				{/each}
			</GpuSelect>
		{:else}
			<GpuSelect />
		{/if}
	</nav>
	<input bind:value={password} on:input={onPasswordChange} placeholder="Type your password" spellcheck="false">
	<h2>Password Strength:
		{#if password_evaluation && !password_evaluating}
			<span style="color: {get_color_for_pass_strength(password_evaluation.strength)}">{password_evaluation.strength}</span>
		{:else if password_evaluating}
			<span>Evaluating...</span>
		{/if}
	</h2>
	<div class="extra-info" style={password_evaluating ? "opacity: 0" : "opacity: 1"}>
		{#if password_evaluation}
			<span>Entropy:</span>
			<span>{password_evaluation.entropy} bits</span>
			<span>Pwned info:</span>
			<span>{password_evaluation.times_pwned > 0 ? "Your password has been detected" + password_evaluation.times_pwned + "times in database leaks!" : "Your password is not present in any database leaks."}</span>
			{#if password_evaluation.possible_combinations && password_evaluation.approximate_time_to_crack_secs}
				<span>Possible combinations:</span>
				<span>{password_evaluation.possible_combinations.toLocaleString()}</span>
				<span>Time to crack on your {gpu_list[gpu_index_used].name}:</span>
				<span>Approximately {seconds_to_wdhms_str(password_evaluation.approximate_time_to_crack_secs)}</span>
			{/if}
		{/if}
	</div>
</main>

<style>
	nav {
		top: 0;
		position: fixed;
		width: 100%;
		display: flex;
		align-items: center;
		padding: 1em;
		margin: 0;
		gap: 1em;
		box-shadow: 0px 3px 6px rgba(0, 0, 0, 0.5);
	}

	main {
		text-align: center;
		margin: 30vh 0 0;
	}

	input {
		width: 50%;
		height: 2.5em;
		padding: 0 0.5em;
		transition: ease 0.3s;
		border: 4px solid transparent;
		background-color: #3b4252;
		border-radius: 5px;
		font-size: 1.5em;
		color: #eceff4;
		box-shadow: 0px 3px 6px rgba(0, 0, 0, 0.5);
	}
	
	input:focus {
		border-color: #88c0d0;
		outline: none;
	}

	.extra-info {
		display: grid;
		grid-template-columns: 50% 50%;
		margin: 4em auto 0;
		transition: 0.5s ease;
		text-align: start;
		width: min(64em, 70%);
		justify-self: center;
		column-gap: 2em;
	}

	.extra-info > span:nth-child(even) {
		font-size: small;
		color: #d8dee9;
	}

</style>