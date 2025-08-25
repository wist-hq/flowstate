<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import { onDestroy, onMount } from "svelte";

    let { data } = $props();

    let container: HTMLIFrameElement | null = $state(null);

    onMount(async () => {
        await invoke("start_preview", { path: `${data.id}/${data.slug}` });

        container?.contentWindow?.location.reload();
        console.log("here");
    });

    onDestroy(async () => {
        await invoke("stop_preview");
    });
</script>

<h1>{data.slug}</h1>

<iframe
    bind:this={container}
    title="Tinymist Preview"
    src="https://google.com"
    class="h-screen w-full"
></iframe>
