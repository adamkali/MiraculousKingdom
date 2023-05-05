<script lang="ts">
    export let onClick: () => void
</script>

<div class="relative w-full">
    <div
        class="lg absolute w-full -inset-0.5 rounded bg-gradient-to-r from-fuchsia-600 to-blue-600 opacity-75 blur"
    />
    <button
        on:click={onClick}
        class="relative flex w-full items-center rounded-lg bg-black px-8 py-4 leading-none"
    >
        <slot />
    </button>
</div>
