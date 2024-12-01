<script lang="ts">
    import Button from './Button.svelte'

    export let label: string
    export let placeholder: string
    export let value: string
    export let onChange: (value: string) => void
    export let onClick: () => void
</script>

<div class="group relative h-full w-full">
    <div
        class="lg absolute -inset-0.5 h-full w-full rounded bg-gradient-to-r from-fuchsia-600 to-blue-600 opacity-75 blur transition duration-150 ease-in-out group-hover:from-fuchsia-500 group-hover:to-blue-500 group-hover:opacity-90 group-hover:blur-lg"
    />
    <div
        class="relative my-8 flex h-full w-full flex-row items-center divide-x divide-red-600 rounded-lg bg-black px-8 py-4 text-lg leading-none"
    >
        <Button {onClick}>
            {label}
        </Button>
        <input
            on:change={(e) => onChange(e.currentTarget.value)}
            {placeholder}
            value={value ? value : ''}
            class="ml-4 h-full w-full bg-black p-4 text-slate-300 placeholder-slate-600 focus:outline-none"
        />
    </div>
</div>
