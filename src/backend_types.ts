export type PasswordStrength = "ProneToDictionaryAttack" | "ProneToBruteforceAttack" | "Weak" | "Medium" | "Strong" | "VeryStrong"

export type PasswordEvaluationResult = {
	strength: PasswordStrength,
	entropy: number,
	times_pwned: number,
	possible_combinations: number | null,
	approximate_time_to_crack_secs: number,
}

export type GpuStats = {
	name: string,
	clock: number,
	cuda_cores: number,
	smp: number,
	gflops_fp32: number,
	gflops_fp64: number,
};