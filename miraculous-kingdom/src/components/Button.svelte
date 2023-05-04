<script lang="ts">
    export let onClick: () => void
</script>

<div class="relative">
    <div
        class="lg absolute -inset-0.5 rounded bg-gradient-to-r from-fuchsia-600 to-blue-600 opacity-75 blur"
    />
    <button
        on:click={onClick}
        class="relative flex items-center rounded-lg bg-black/80 px-8 py-4 leading-none"
    >
        <slot />
    </button>
</div>
