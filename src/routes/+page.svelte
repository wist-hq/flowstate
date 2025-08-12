<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";

    let name = $state("");
    let greetMsg = $state("");

    const greet = async (e: Event) => {
        e.preventDefault();
        greetMsg = await invoke("greet", { name });
    };
</script>

<main>
    <h1 class="text-purple-500">Welcome to Tauri + Svelte</h1>

    <form onsubmit={greet}>
        <input placeholder="Enter a name..." bind:value={name} />
        <button type="submit">Greet</button>
    </form>
    <p>{greetMsg}</p>
</main>
