<script lang="ts">
    export let label: string
    export let placeholder: string
    export let value: string
    export let onChange: (value: string) => void
</script>

<div class="relative">
    <div
        class="lg absolute -inset-0.5 rounded bg-gradient-to-r from-fuchsia-600 to-blue-600 opacity-75 blur"
    />
    <div
        class="relative flex flex-row items-center divide-x divide-red-600 rounded-lg bg-black/80 px-8 py-4 text-lg leading-none"
    >
        <div class="text-red-600">
            {label}
        </div>
        <input
            on:change={(e) => onChange(e.currentTarget.value)}
            {placeholder}
            {value}
            class="text-slate-300 placeholder-slate-600"
        />
    </div>
</div>
