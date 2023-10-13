
<h1>Welcome to LimesKey</h1>
<p>Visit <a href="https://kit.svelte.dev">kit.svelte.dev</a> to read the documentation</p>

<script lang="js">
    let password = "";
    // When using the Tauri API npm package:
    import { invoke } from '@tauri-apps/api/tauri'
    // When using the Tauri global script (if not using the npm package)
    // Be sure to set `build.withGlobalTauri` in `tauri.conf.json` to true
    const water = invoke('greet');
    const test =  invoke('test', { password });
</script>

<input bind:value={password} placeholder="enter password name" />

{#await water}
    <p>...waiting</p>
{:then water}
    {water}
{/await}

{#await test}
    <p>...2</p>
{:then test}
    {test}
{/await}