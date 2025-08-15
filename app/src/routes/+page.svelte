<script lang="ts">
    import { open } from "@tauri-apps/plugin-dialog";
    import { goto } from "$app/navigation";

    const openWorkspace = async () => {
        const path = await open({
            directory: true,
            multiple: false,
        });

        if (!path) {
            return;
        }

        const encoded = encodeURIComponent(path);
        goto(`/workspace/${encoded}`);
    };
</script>

<div class="flex h-screen items-center justify-center">
    <button class="btn btn-primary" onclick={openWorkspace}>
        Open Workspace
    </button>
</div>
