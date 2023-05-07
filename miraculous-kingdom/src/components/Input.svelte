<script lang="ts">
    export let label: string
    export let placeholder: string
    export let value: string
    export let onChange: (value: string) => void
    export let inputType: string | null | undefined

    inputType = inputType ? inputType : 'text'
</script>

<div class="relative w-full">
    <div
        class="absolute h-24 w-full rounded bg-gradient-to-r from-fuchsia-600 to-blue-600 opacity-75 blur"
    />
    <div
        class="relative my-8 flex w-full flex-row items-center divide-x divide-red-600 rounded-lg bg-black px-8 py-4 text-lg leading-none"
    >
        <div class="mr-4 text-red-600">
            {label}
        </div>
        <input
            type={inputType}
            on:change={(e) => onChange(e.currentTarget.value)}
            {placeholder}
            value={value ? value : ''}
            class="ml-4 h-full w-full bg-black p-4 text-slate-300 placeholder-slate-600 focus:outline-none"
        />
    </div>
</div>
