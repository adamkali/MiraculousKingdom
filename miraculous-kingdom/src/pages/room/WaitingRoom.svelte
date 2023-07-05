<script lang="ts">
    import { currentGame, gameCharacter } from "../../store";
    import {
        type GameInfo,
        type CharacterResponse,
        type IsReady,
        type IsReadyItem,
        type WSRequest,
        type WSReadyToStart,
        type WSAbilityRequest,
        type Ability,
        type MightRequirement, RollTier,
        MightEnum,
        ApiQueueApiService
    } from "../../models";
    import { onMount } from "svelte";
    import Abilities from "../rules/Abilities.svelte";

    let game: GameInfo = currentGame.get()
    let character: CharacterResponse = gameCharacter.get()
    $: isReady = {} as IsReady
    let socketGlobal: WebSocket

    onMount(async () => {
        const response = await ApiQueueApiService.setQueue(game.game_pass);

        if ( response.success === "Succeeding" ) {
            // connect to the socket 
            const socket = new WebSocket("ws://localhost:8050/api/queue");
            // console log the WebSocket connection
            socket.onopen = () => {
                console.log("WebSocket connection established");
                

                // send to the wedsocket the character secret as text message
                socket.send(character.secret);

                // set the socket to the socket variable 
                socketGlobal = socket;

                // print out what ever we get back from the server
                // and wen it is finished send out the ready readyObject
                socket.onmessage = (event) => {
                    const data = JSON.parse(event.data);
                    console.log(data);
                };
            };
        }
    });

    const ready = async () => {
        const readyObject: WSRequest = {
            READYTOSTART: {
                owner: character.secret,
            } as WSReadyToStart
        };
        socketGlobal.send(JSON.stringify(readyObject));
    }

    const ability = async () => {
        const abilityObject: WSRequest = {
            ABILITYREQUEST: {
                owner: character.secret,
                ability: {
                    ability_name: "Fireball",
                    ability_desc: "A ball of fire",
                    ability_unlock: {
                        roll_tier: RollTier.FANTASTIC,
                        might: MightEnum.SCIENCE,
                        unlock: 1 
                    } as MightRequirement,
                    ability_rewards: [],
                } as Ability
            }
        };
        socketGlobal.send(JSON.stringify(abilityObject));
    }
</script>

<div>
    <button on:click={async () => await ready()}>Ready</button>
    <button on:click={async () => await ability()}>Ability</button>

</div>
