<script lang="ts">
    import { Link } from 'svelte-navigator'
    export let href: string
</script>

<div class="font-bold text-blue-600 hover:text-cyan-600">
    <Link to={href}>
        <slot />
    </Link>
</div>
