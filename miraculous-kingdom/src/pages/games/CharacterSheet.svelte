<script lang="ts">
    import * as Components from '../../components'
    import {
        type CharacterResponse,
        type GameInfo,
        type Ability,
        ApiCharacterApiService,
        ClassEnum,
        type SeasonResponse,
        ApiQueueApiService,
    } from '../../models'
    import { currentGame, gameCharacter, queue } from '../../store'
    import SeasonRoll from './components/SeasonRoll.svelte'
    import AbilityChoice from './components/AbilityChoice.svelte'
    import ChooseTarget from './components/ChooseTarget.svelte';
    import { OpenAPI } from '../../models'

    let game: GameInfo = currentGame.get()
    let character: CharacterResponse = gameCharacter.get()
    let tookturn = false
    $: season = {} as SeasonResponse
    $: hand = character.char_hand
    $: might = character.char_might
    $: clocks = character.char_clocks
    $: characterTarget = {} as CharacterResponse
    $: abilityChoose = {} as Ability
    $: target = false
    $: waiting = false
    $: ws = {} as WebSocket

    const asyncDiscard = async (ability: Ability) => {
        const res = await ApiCharacterApiService.discardCard(
            character.secret,
            game.game_pass,
            ability,
        )
        // TODO: Eventually make this into trashing the card
        //       a websocket request
        if (res.success === 'Succeeding') {
            gameCharacter.set(res.data)
            character = gameCharacter.get()
        } else {
            throw new Error(res.success.Failing.message)
        }
    }

    const asyncInit = async () => {
        let setter = await ApiQueueApiService.setQueue(game.game_pass);
        if (setter.success === 'Succeeding') {
            // connect to the websocket
            const base = OpenAPI.BASE;
            // remove the http:// or https://
            base.replace(/(^\w+:|^)\/\//, '');
            ws = new WebSocket(
                `ws://${base}/queue/${game.game_pass}`,
            )
        } else {
        }
        if (!character.char_hand.length) {
            let res = await ApiCharacterApiService.getCharacterForGame(
                character.secret,
                game.game_pass,
            )
            if (
                res.success === 'Succeeding' &&
                res.data.char_hand.length === 0
            ) {
                res = await ApiCharacterApiService.initHand(
                    character.secret,
                    game.game_pass,
                )
                if (res.success === 'Succeeding') {
                    gameCharacter.set(res.data)
                    character = gameCharacter.get()
                } else {
                    throw new Error(res.success.Failing.message)
                }
            } else if (res.success === 'Succeeding') {
                gameCharacter.set(res.data)
                character = gameCharacter.get()
            } else {
                throw new Error(res.success.Failing.message)
            }
        }
        hand = gameCharacter.get().char_hand
    }

    const asyncDraw = async () => {
        const drawAmount = character.char_class === ClassEnum.SCIENTIST ? 2 : 1
        const res = await ApiCharacterApiService.drawCard(
            drawAmount,
            character.secret,
            game.game_pass,
        )
        if (res.success === 'Succeeding') {
            gameCharacter.set(res.data)
            character = gameCharacter.get()
        } else {
            throw new Error(res.success.Failing.message)
        }
    }

    const take_turn = async (ability: Ability) => {
    }
</script>

<div
    class="flex min-h-screen w-10/12 flex-col items-center rounded-xl bg-slate-600/70 px-12 pt-6 text-center align-top text-xl text-slate-300 backdrop-blur-lg"
>
    <div
        class="top-0 mb-8 w-full rounded-xl bg-slate-800 p-8 text-5xl font-bold text-blue-600"
    >
        Miraculous Kingdom
    </div>
    {#await asyncInit()}
        <div>...waiting</div>
    {:then}
        <div class="flex w-full flex-col">
            <div class="mb-8 flex h-24 flex-row items-center justify-center">
                <div class="mx-4">
                    <Components.Input
                        label="Name"
                        placeholder=""
                        value={character.char_name}
                        onChange={(_value) => {}}
                        inputType="text"
                        disabled={true}
                    />
                </div>
                <div class="mx-4">
                    <Components.Input
                        label="Class"
                        placeholder=""
                        value={character.char_class}
                        onChange={(_value) => {}}
                        inputType="text"
                        disabled={true}
                    />
                </div>
                <div class="mx-4">
                    <Components.Input
                        label="Country Name"
                        placeholder=""
                        value={game.game_name}
                        onChange={(_value) => {}}
                        inputType="text"
                        disabled={true}
                    />
                </div>
                <div class="mx-4">
                    <Components.Input
                        label="Ruler"
                        placeholder=""
                        value={game.game_ruler}
                        onChange={(_value) => {}}
                        inputType="text"
                        disabled={true}
                    />
                </div>
                <div class="mx-4 h-full">
                    <Components.Button
                        onClick={() => {
                            currentGame.set(null)
                            gameCharacter.set(null)
                            window.location.href = '/'
                        }}
                    >
                        <span>Exit</span>
                    </Components.Button>
                </div>
            </div>
            {#if waiting}
                <div class="flex flex-col items-center justify-center h-full">
                    <div class="text-2xl font-bold">Waiting for other players...</div>
                    <div class="text-2xl font-bold">Players: {game.game_chars.length}</div>
                    <Components.Button
                        onClick={() => {
                        }}
                    >
                        <span>Start</span>
                    </Components.Button>
                </div>
            {:else if !target }
                <AbilityChoice
                    {hand}
                    {season}
                    {might}
                    {clocks}
                    secret={character.secret}
                    {asyncDiscard}
                    asyncPlay={take_turn}
                    {asyncDraw}
                />
            {:else }
                <ChooseTarget 
                    {season}
                    ability={abilityChoose}
                    {might}
                    pass={game.game_pass}
                />
            {/if}
        </div>
    {:catch err}
        {err}
    {/await}
</div>
