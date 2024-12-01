<script lang="ts">
    import { Link } from 'svelte-navigator'
    export let href: string

    const onClick = (to: string) => {
        window.location.href = to
    }
</script>

<span
    class="font-bold text-blue-600 hover:text-cyan-600"
    on:click={() => onClick(href)}
>
    <slot />
</span>
