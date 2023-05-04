<script lang="ts">
    import {
        ApiGameApiService,
        ClassEnum,
        MightEnum,
        type NewCharacter,
    } from '../../models'
    import { Input } from '../../components'

    export let pass: string
    let character: NewCharacter

    let record: Record<string, number>
    record[MightEnum.MILITARY] = 0
    record[MightEnum.CULTURE] = 0
    record[MightEnum.SCIENCE] = 0
    record[MightEnum.RELIGION] = 0
    record[MightEnum.DIPLOMACY] = 0
    record[MightEnum.ESPIONAGE] = 0

    const handleSubmit = () => {
        ApiGameApiService.addCharacter(pass, character)
            .then((res) => {
                console.log({ response: res })
            })
            .catch((err) => {
                console.log({ error: err })
            })
    }

    const handleInput = () => {}
</script>

<div
    class="flex min-h-screen w-10/12 flex-col rounded-xl bg-slate-600/70 px-12 pt-6 text-center align-top text-xl text-slate-300 backdrop-blur-lg"
>
    <div
        class="top-0 mb-8 rounded-xl bg-slate-800 p-8 text-7xl font-bold text-blue-600"
    >
        Create a Character
    </div>
    <form class="grid grid-rows-6" on:submit|preventDefault={handleSubmit}>
        <Input
            label="Character Name"
            placeholder="Tywin Lanister"
            onChange={handleSubmit}
            value={character.char_name}
        />
        <select bind:value={character.char_class} on:change={(e) => character.char_class =   }>
            {#each ClassEnum as c }
                <option value={c.toString()}>
                    {c.toString()} 
                </option>
            {/each}
        </select>
        <div class="grid-cols-3">
            <div class="grid grid-rows-3 border-slate-400">
                <Input
                    label={MightEnum.MILITARY}
                    placeholder="0"
                    value={record[MightEnum.MILITARY].valueOf().toString()}
                    onChange={(value) => {
                        record[MightEnum.MILITARY] = +value
                    }}
                />
            </div>
            <div class="grid grid-rows-3 border-slate-400">
                <Input
                    label={MightEnum.CULTURE}
                    placeholder="0"
                    value={record[MightEnum.CULTURE].valueOf().toString()}
                    onChange={(value) => {
                        record[MightEnum.CULTURE] = +value
                    }}
                />
            </div>
            <div class="grid grid-rows-3 border-slate-400">
                <Input
                    label={MightEnum.SCIENCE}
                    placeholder="0"
                    value={record[MightEnum.SCIENCE].valueOf().toString()}
                    onChange={(value) => {
                        record[MightEnum.SCIENCE] = +value
                    }}
                />
            </div>
        </div>
        <div class="grid-cols-3">
            <div class="grid grid-rows-3 border-slate-400">
                <Input
                    label={MightEnum.RELIGION}
                    placeholder="0"
                    value={record[MightEnum.RELIGION].valueOf().toString()}
                    onChange={(value) => {
                        record[MightEnum.RELIGION] = +value
                    }}
                />
            </div>
            <div class="grid grid-rows-3 border-slate-400">
                <Input
                    label={MightEnum.DIPLOMACY}
                    placeholder="0"
                    value={record[MightEnum.DIPLOMACY].valueOf().toString()}
                    onChange={(value) => {
                        record[MightEnum.DIPLOMACY] = +value
                    }}
                />
            </div>
            <div class="grid grid-rows-3 border-slate-400">
                <Input
                    label={MightEnum.ESPIONAGE}
                    placeholder="0"
                    value={record[MightEnum.ESPIONAGE].valueOf().toString()}
                    onChange={(value) => {
                        record[MightEnum.ESPIONAGE] = +value
                    }}
                />
            </div>
            <Input
                label={"Character Password"}
                placeholder="GOTMaster!"
                value={character.secret}
                onChange={(value) => {
                    character.secret = value
                }}
            />
        </div>
    </form>
</div>
