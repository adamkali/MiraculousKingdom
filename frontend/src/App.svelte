<script lang="ts">
    import { AppBar } from "@skeletonlabs/skeleton";
    import * as api from "$api/apis";
    //import * as models from '$api/models';

    const getAll = async () => {
        const classesApi = new api.ClassesApi();
        const response = await classesApi.getAll();
        if (response.data && response.successful) {
            console.log({ ...response });
            return response.data;
        } else return [];
    };
    let classes = $state(getAll());
</script>

<main data-theme="mk-skeleton-config" class="bg-surface-900 h-screen w-full">
    <AppBar
        gridColumns="grid-cols-3"
        slotDefault="place-self-center"
        slotTrail="place-content-end"
        class="w-full px-16 text-primary-800-100-token"
        background="bg-surface-500"
    >
        <svelte:fragment slot="lead">(icon)</svelte:fragment>
        Hello world
        <svelte:fragment slot="trail">(actions)</svelte:fragment>
    </AppBar>

    <div class="variant-filled-primary mx-16 mt-4 p-14 rounded-xl">
        <h1>Rsbuild with Svelte</h1>
        <p>Start building amazing things with Rsbuild.</p>
    </div>

    {#await classes}
        <span class="text-primary-50">... waiting</span>
    {:then values}
        <div class="flex flex-col justify-center w-full">
            {#each values as classValue}
                <div class="flex flex-row justify-center my-16" id={`${classValue.id}`}>
                    <div
                        class="variant-filled-primary w-64 rounded-xl justify-self-center text-primary-900-50-token"
                    >
                        <b class="text-lg">{classValue.displayName}</b>
                        <br/>
                        {classValue.description}
                    </div>
                </div>
            {/each}
        </div>
    {/await}
</main>
