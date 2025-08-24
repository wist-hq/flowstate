<script lang="ts">
    import { readDir } from "@tauri-apps/plugin-fs";
    import Fuse from "fuse.js";
    import { onMount } from "svelte";

    let { data } = $props();

    const workspacePath = $derived(decodeURIComponent(data.id));

    let files: string[] = $state([]);
    let fuse: Fuse<string> | null = $state(null);
    let query = $state("");

    onMount(async () => {
        const entries = await readDir(workspacePath);
        files = entries.map((e) => e.name ?? "");
        fuse = new Fuse(files);
    });

    const matches = $derived.by(
        () => fuse?.search(query).map((x) => x.item) ?? [],
    );
</script>

<h1>Workspace</h1>
<p>{workspacePath}</p>

<h2>Search</h2>
<input class="input" type="text" bind:value={query} />

<h2>Search results</h2>
<ul>
    {#each matches as x, i (i)}
        <div>
            <a
                class="link"
                href="/workspace/{encodeURIComponent(
                    data.id,
                )}/note/{encodeURIComponent(x)}"
            >
                {x}
            </a>
        </div>
    {/each}
</ul>
