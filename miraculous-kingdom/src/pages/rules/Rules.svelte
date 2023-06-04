<script lang="ts">
    import { onMount } from 'svelte'
    import * as Components from '../../components'
    import { RulesConst, type Rule } from './rules'

    let search: string

    $: handledRules = [] as Rule[]

    onMount(() => {
        handledRules = RulesConst
    })

    const handleSearch = () => {
        if (search !== '') {
            const newRules: Rule[] = []
            RulesConst.forEach((rule) => {
                let arr: String[] = rule.searchableName
                let flag: boolean = false
                let item: number = 0
                while (!flag) {
                    flag = arr[item].match(search) !== null
                    ++item
                    if (item >= arr.length) {
                        break
                    }
                }
                if (flag) {
                    newRules.push(rule)
                }
            })
            if (newRules.length === 0) {
                handledRules = RulesConst
                return
            }
            handledRules = newRules
        } else {
            handledRules = RulesConst
        }
    }

    const handleClick = (link: string) => {
        window.location.href = link
    }
</script>

<Components.Wrapper title="Rules">
    <Components.Input
        value={search}
        placeholder="Search for a Rule"
        onChange={(_value) => {
            handleSearch()
        }}
        label="Search"
        inputType="text"
    />
    {#each handledRules as rule}
        <div class="group relative my-8 h-24 w-full">
            <div
                class="absolute h-full w-full rounded bg-gradient-to-r from-fuchsia-600 to-blue-600 opacity-75 blur transition duration-150 ease-in-out group-hover:from-fuchsia-500 group-hover:to-blue-500 group-hover:opacity-90 group-hover:blur-lg"
            />
            <div
                on:click={() => handleClick(rule.link)}
                class="relative grid h-full w-full grid-cols-5 items-center divide-red-600 rounded-lg bg-slate-300 px-8 py-4 leading-none dark:bg-black"
            >
                <div class="text-red-600">{rule.title}</div>
                <div class="col-span-4">{rule.description}</div>
            </div>
        </div>
    {/each}
</Components.Wrapper>
