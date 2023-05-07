<script lang="ts">
    export let onClick: () => void = () => {}
    export let buttonType: any = 'button'
</script>

<div class="group relative h-full w-full">
    <div
        class="lg absolute -inset-0.5 h-full w-full rounded bg-gradient-to-r from-fuchsia-600 to-blue-600 opacity-75 blur transition duration-150 ease-in-out group-hover:from-fuchsia-500 group-hover:to-blue-500 group-hover:opacity-90 group-hover:blur-lg"
    />
    <button
        type={buttonType}
        on:click={onClick}
        class="relative flex h-full w-full items-center rounded-lg bg-black px-8 py-4 leading-none"
    >
        <slot />
    </button>
</div>
