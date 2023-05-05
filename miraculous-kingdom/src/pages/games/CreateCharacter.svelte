<script lang="ts">
    import {
        ApiGameApiService,
        ClassEnum,
        MightEnum,
        type NewCharacter,
    } from '../../models'
    import { Input, Button } from '../../components'

    export let pass: string
    let character: NewCharacter = {} as NewCharacter

    let classArr = [
        ClassEnum.WAR_GENERAL,
        ClassEnum.AFICIANADO,
        ClassEnum.RESEARCHER,
    ]

    let record: Record<string, number> = {} as Record<string, number>
    record[MightEnum.MILITARY] = 0
    record[MightEnum.CULTURE] = 0
    record[MightEnum.SCIENCE] = 0
    record[MightEnum.RELIGION] = 0
    record[MightEnum.DIPLOMACY] = 0
    record[MightEnum.ESPIONAGE] = 0

    console.log(record)

    const handleSubmit = () => {
        character.char_might = record
        ApiGameApiService.addCharacter(pass, character)
            .then((res) => {
                console.log(res)
            })
            .catch((err) => {
                console.log({ error: err })
            })
    }

    const isClassEnum = (val: string): ClassEnum => {
        if (val === ClassEnum.WAR_GENERAL) {
            return ClassEnum.WAR_GENERAL
        } else if (val === ClassEnum.AFICIANADO) {
            return ClassEnum.AFICIANADO
        } else if (val === ClassEnum.RESEARCHER) {
            return ClassEnum.RESEARCHER
        } else {
            return ClassEnum.WAR_GENERAL
        }
    }
</script>

<div
    class="flex min-h-screen w-10/12 flex-col items-center rounded-xl bg-slate-600/70 px-12 pt-6 text-center align-top text-xl text-slate-300 backdrop-blur-lg"
>
    <div
        class="top-0 mb-8 w-full rounded-xl bg-slate-800 p-8 text-7xl font-bold text-blue-600"
    >
        Create a Character
    </div>
    <form class="flex-col" on:submit|preventDefault={handleSubmit}>
        <Input
            label="Character Name"
            placeholder="Tywin Lanister"
            onChange={(value) => {
                character.char_name = value
            }}
            value={character.char_name}
        />
        <div>
            <Input
                label={'Character Password'}
                placeholder="GOTMaster!"
                value={character.secret}
                onChange={(value) => {
                    character.secret = value
                }}
            />
            <div>
                <div class="flex w-full flex-row">
                    <div class="mr-4">
                        <Input
                            label={MightEnum.MILITARY}
                            placeholder="0"
                            value={record[MightEnum.MILITARY].toString()}
                            onChange={(value) => {
                                record[MightEnum.MILITARY] = +value
                            }}
                        />
                    </div>
                    <div class="mr-4">
                        <Input
                            label={MightEnum.CULTURE}
                            placeholder="0"
                            value={record[MightEnum.CULTURE].toString()}
                            onChange={(value) => {
                                record[MightEnum.CULTURE] = +value
                            }}
                        />
                    </div>
                    <div>
                        <Input
                            label={MightEnum.SCIENCE}
                            placeholder="0"
                            value={record[MightEnum.SCIENCE].toString()}
                            onChange={(value) => {
                                record[MightEnum.SCIENCE] = +value
                            }}
                        />
                    </div>
                </div>
                <div class="flex flex-row">
                    <div class="mr-4">
                        <Input
                            label={MightEnum.RELIGION}
                            placeholder="0"
                            value={record[MightEnum.RELIGION].toString()}
                            onChange={(value) => {
                                record[MightEnum.RELIGION] = +value
                            }}
                        />
                    </div>
                    <div class="mr-4">
                        <Input
                            label={MightEnum.DIPLOMACY}
                            placeholder="0"
                            value={record[MightEnum.DIPLOMACY].toString()}
                            onChange={(value) => {
                                record[MightEnum.DIPLOMACY] = +value
                            }}
                        />
                    </div>
                    <div class="mr-4">
                        <Input
                            label={MightEnum.ESPIONAGE}
                            placeholder="0"
                            value={record[MightEnum.ESPIONAGE].toString()}
                            onChange={(value) => {
                                record[MightEnum.ESPIONAGE] = +value
                            }}
                        />
                    </div>
                </div>
                <div class="grid grid-cols-2 gap-x-8">
                    <div class="w-full">
                        <select
                            class=" h-24 w-full rounded-lg bg-black p-4 text-xl text-slate-300"
                            bind:value={character.char_class}
                            on:change={(e) => {
                                character.char_class = isClassEnum(
                                    e.currentTarget.value,
                                )
                            }}
                        >
                            {#each classArr as c}
                                <option value={c.toString()}>
                                    {c.toString()}
                                </option>
                            {/each}
                        </select>
                    </div>
                    <Button
                        onClick={() => {
                            handleSubmit()
                        }}
                    >
                        Create Your Character
                    </Button>
                </div>
            </div>
        </div>
    </form>
</div>
