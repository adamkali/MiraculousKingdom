<script lang="ts">
    import Special from '../components/Special.svelte'
    const src: string = '/src/assets/mk.svg'
</script>

<div
    class="flex min-h-screen w-10/12 flex-col rounded-xl bg-slate-600/70 px-12 pt-6 text-justify align-top text-xl text-slate-300 backdrop-blur-lg"
>
    <div
        class="top-0 mb-8 flex flex-col rounded-xl bg-slate-800 p-8 text-7xl font-bold text-blue-600"
    >
        <img class="h-96" {src} alt="Miraculous Kingdom Logo" />
    </div>
    <div class="rounded-xl bg-black p-16">
        <p class="text-3xl text-slate-100">
            Have you ever wanted to backstab your friends and still retain your
            friendship? Well then this is the game! Welcome to the Miraculous
            Kingdom. Miraculous Kingdom is a deck building game about
            backstabbing, skullduggery, and betrayal. Each <Special
                href="/rules/characters">Character</Special
            > has its own <Special href="/rules/classes">Class</Special> with its
            own cards called
            <Special href="/rules/abilities">Abilities</Special>. Take turns
            leveaging abilities through out the game to get more resources,
            abilities, and strategies to win the round called a <Special
                href="/rules/seasons">Season</Special
            >. Think of this game as a season or an arc in one of your favorite
            dramas like Game of Thrones, Last of Us, or The Wire.
            <br />
            <br />
            To begin a game: Go to <Special href="/start/create">Create</Special
            > and make a setting for your game. Have the rest of your friends using
            the password you get from the server and have them to go <Special
                href="/start/join">here</Special
            >
            Roll <Special href="/rules/might">Might</Special> for your character.
            Each one will have an effect on how you play. Choose your class, and
            at the beginning of the game get 10 abilities plus the 8 that you get
            from your class. Then after rolling a season from the server you will
            get a special case for how the next couple of rounds (one rotation of
            everyone playing). Each round the roll a 20 sided dice, or use the application,
            to figure out the order of the round. Then take your turn. This will
            keep going until a game ending ability is played and its conditions are
            met. A game ending ability is denoted on the ability's card face.
            <br />
            <br />
            Remember if you have any questions check the <Special href="/rules"
                >Rules</Special
            >. If you dont find the answer you are looking for, or if you find a
            bug, then please submit it as a github issue, follow the link at the
            top of the page. If you want to contribute, we thank you immensly.
            Submit a pull request on the github as well! You can also self host
            the server, by using the docker link above. Follow the instructions
            from there, to begin.
            <br />
            <br />
            Now get out there, betray your friends, take part in backend deals, and
            have a fun time taking part of the Miraculous Kingdom.
        </p>
    </div>
</div>
