<script lang="ts">
    import { type Clock as ClockModel } from '../models'

    export let clock: ClockModel
    export let secret: string
</script>

<div class="group relative h-full w-full">
    <div
        class="lg absolute h-full w-full rounded bg-gradient-to-r from-fuchsia-600 to-blue-600 opacity-75 blur transition duration-150 ease-in-out group-hover:from-fuchsia-500 group-hover:to-blue-500 group-hover:opacity-100 group-hover:blur-xl"
    />
    <div
        class="mx-2 flex h-full w-full flex-col justify-evenly rounded-lg bg-slate-300 px-4 py-2 text-justify text-sm backdrop-blur dark:bg-black"
    >
        {#if clock.clock_conf && clock.clock_char_secret !== secret}
            <div class="blur">
                <div
                    class="left-0 top-0 mb-2 text-2xl text-lg text-fuchsia-600"
                >
                    {clock.clock_name}
                </div>
                <div class="flex flex-col text-slate-800 dark:text-slate-400">
                    <div>{clock.clock_desc}</div>
                    <div>{clock.clock_remaining}/{clock.clock_duration}</div>
                </div>
            </div>
        {:else}
            <div class="left-0 top-0 mb-2 text-2xl text-lg text-fuchsia-600">
                {clock.clock_name}
            </div>
            <div class="flex flex-col text-slate-800 dark:text-slate-400">
                <div>{clock.clock_desc}</div>
                <div>{clock.clock_remaining}/{clock.clock_duration}</div>
            </div>
        {/if}
    </div>
</div>
