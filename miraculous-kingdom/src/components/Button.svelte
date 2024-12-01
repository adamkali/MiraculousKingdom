<script lang="ts">
    export let onClick: () => void = () => {}
    export let buttonType: any = 'button'
</script>

<div class="group relative h-full w-full">
    <div
        class="absolute h-full w-full rounded bg-gradient-to-r from-fuchsia-600 to-blue-600 opacity-75 blur transition duration-150 ease-in-out group-hover:from-fuchsia-500 group-hover:to-blue-500 group-hover:opacity-100 group-hover:blur-xl"
    />
    <button
        type={buttonType}
        on:click={onClick}
        class="mx-2 flex h-full w-full flex-col justify-evenly rounded-lg bg-slate-300 px-4 py-2 text-justify text-sm backdrop-blur dark:bg-black"
    >
        <slot />
    </button>
</div>
