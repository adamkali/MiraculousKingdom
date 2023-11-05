<script lang="ts">
    import * as Components from '../../components'
    import {
        type CharacterResponse,
        type GameInfo,
        type Ability,
        ApiCharacterApiService,
        type SeasonResponse,
        ApiQueueApiService,
        type WSRequest,
        Episode,
        type IsReady,
        type WSAbilityRequest,
        type WSRollRequest,
        type WSReadyToStart,
        type WSResponse,
    } from '../../models'
    import * as utils from '../../utils'

    import { currentGame, gameCharacter } from '../../store'
    //import SeasonRoll from './components/SeasonRoll.svelte'
    import AbilityChoice from './components/AbilityChoice.svelte'
    import ChooseTarget from './components/ChooseTarget.svelte'
    import { OpenAPI } from '../../models'

    let game: GameInfo = currentGame.get()
    let character: CharacterResponse = gameCharacter.get()
    let ws = {} as WebSocket
    //let tookturn = false

    $: season = {} as SeasonResponse
    $: hand = character.char_hand
    $: might = character.char_might
    $: clocks = character.char_clocks
    $: characterTarget = {} as CharacterResponse
    $: abilityChoose = {} as Ability
    $: currentState = Episode.NONE
    $: waiting = false
    $: isReady = {} as IsReady

    const asyncInit = async () => {
        let setter = await ApiQueueApiService.setQueue(game.game_pass)
        if (setter.success === 'Succeeding') {
            // connect to the websocket
            let base = OpenAPI.BASE
            base = utils.convertToWS(base)
            ws = new WebSocket(
                `${base}/api/queue`,
                //`ws://mk_api:8050/api/queue`
            )
            ws.onopen = () => {
                ws.send(character.secret)
                console.log('Entered Event')
                ws.onmessage = (event) => {
                    console.log('data', event.data)
                    let data: WSResponse = JSON.parse(event.data) as WSResponse
                    if (utils.isSeasonResponse(data)) {
                        console.log('season', data)
                        season = data
                    } else if (utils.isEpisodeResponse(data)) {
                        console.log('episode', data)
                        currentState = data['EPISODERESPONSE']  
                        console.log('currentState', currentState)
                    } else if (utils.isIsReady(data)) {
                        console.log('isReady', data)
                        const dataAsIsReady: IsReady = data['ISREADY']
                        isReady = dataAsIsReady
                    } else if (utils.isWaiting(data)) {
                        console.log('waiting', data)
                        waiting = true
                    } else {
                        console.log('Received unknown data:', data)
                    }
                    console.log('closing event');
                }
            }
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

    const asyncDiscard = async (ability: Ability) => {
        console.log(ability)
    }

    const asyncDraw = async () => {
        console.log('draw')
        // to do
    }

    const asyncReady = async () => {
        ws.send(
            JSON.stringify({
                READYTOSTART: {
                    owner: character.secret,
                } as WSReadyToStart,
            } as WSRequest),
        )
    }

    const asyncPlay = async (ability: Ability) => {
        ws.send(
            JSON.stringify({
                ABILITYREQUEST: {
                    ability,
                    owner: character.secret,
                } as WSAbilityRequest,
            } as WSRequest),
        )
    }

    const asyncTargetChoose = async (character: CharacterResponse) => {
        ws.send(
            JSON.stringify({
                TARGETREQUEST: {
                    owner: character.secret,
                    character: character,
                },
            }),
        )
    }

    const asyncRollConfirm = async (ability: Ability) => {
        ws.send(
            JSON.stringify({
                ROLLREQUEST: {
                    owner: character.secret,
                    ability: null,
                    character: null,
                    secret: '',
                } as WSRollRequest,
            }),
        )
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
                <div>
                    <div class="text-2xl font-bold">
                        Waiting for other players...
                    </div>
                    {#each isReady.items as item}
                        <div class="flex flex-row">
                            <div class="text-xl">✔️</div>
                            <div class="text-xl text-blue-600">{item.name}</div>
                            <div class="text-xl">
                                {item.is_ready ? '🤯 ' : '🫥 '}
                            </div>
                        </div>
                    {/each}
                </div>
            {:else if currentState === Episode.NONE}
                <div class="flex h-full flex-col items-center justify-center">
                    <div class="text-2xl font-bold">
                        Waiting for other players...
                    </div>
                    <div class="text-2xl font-bold">
                        Players: {game.game_chars.length}
                    </div>
                    <Components.Button onClick={async () => await asyncReady()}>
                        <span>Start</span>
                    </Components.Button>
                </div>
            {:else if currentState === Episode.ABILITYCHOOSE}
                <AbilityChoice
                    {hand}
                    {season}
                    {might}
                    {clocks}
                    secret={character.secret}
                    {asyncDiscard}
                    {asyncPlay}
                    {asyncDraw}
                />
            {:else if currentState === Episode.TARGETCHOICE}
                <ChooseTarget
                    {season}
                    ability={abilityChoose}
                    {might}
                    pass={game.game_pass}
                />
            {:else if currentState === Episode.ROLLRESULT}
                <div>Roll</div>
            {:else if currentState === Episode.RESOLUTION}
                <div>RESOLUTION</div>
            {:else}
                <div>This should not be happening</div>
            {/if}
        </div>
    {:catch err}
        {err}
    {/await}
</div>
