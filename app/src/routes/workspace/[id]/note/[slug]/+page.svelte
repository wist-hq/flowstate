<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import { listen } from "@tauri-apps/api/event";
    import { onDestroy, onMount } from "svelte";

    let { data } = $props();

    let html = $state("");

    onMount(async () => {
        await invoke("watch", { path: `${data.id}/${data.slug}` });
    });

    onDestroy(async () => {
        await invoke("stop_watch");
    });

    listen<string>("typst-update", (event) => {
        html = event.payload;
    });
</script>

<h1>{data.slug}</h1>

<!-- eslint-disable svelte/no-at-html-tags -->
{@html html}
