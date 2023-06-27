<script lang="ts">
    import { currentGame, gameCharacter } from "../../store";
    import {
        type GameInfo,
        type CharacterResponse,
        type IsReady,
        type IsReadyItem,
        ApiQueueApiService
    } from "../../models";
    import { onMount } from "svelte";

    let game: GameInfo = currentGame.get()
    let character: CharacterResponse = gameCharacter.get()
    $: isReady = {} as IsReady

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



                // just copy out whateve we fet from the WebSocket
                socket.onmessage = (event) => {
                    const data = JSON.parse(event.data);
                    console.log(data);

                };
            };
        }
    });

</script>

<div>
    <h1>Who Is Ready!</h1>
</div>
