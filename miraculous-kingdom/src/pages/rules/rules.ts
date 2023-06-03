export type Rule = {
    link: string
    title: string
    description: string
    searchableName: string[]
}

const RulesConst: Rule[] = [
    {
        link: '/rules/abilities',
        title: 'Abilities',
        description:
            'Abilities or Cards are the main crux to Miraculous Kingdom.',
        searchableName: ['Ability', 'Class', 'Abilities'],
    },
    {
        link: '/rules/characters',
        title: 'Characters',
        description:
            'You are a character in you own drama of the week show. Backstab and make Backend deals to further your goals.',
        searchableName: ['Characters', 'Character', 'Player'],
    },
    {
        link: '/rules/classes',
        title: 'Classes',
        description:
            'Classes give each character a unique expereience when playing. A Cultural Aficianado will pay bribes, and the Royal Scientist will draw cards just as an example',
        searchableName: [
            'Class',
            'Classes',
            'General',
            'Aficianado',
            'Royal',
            'War',
            'Cultural',
            'Cardinal',
            'Diplomat',
            'Spy',
            'Master',
        ],
    },
    {
        link: '/rules/might',
        title: 'Might',
        description: 'Might decides your fate in the Miraculous Kingdom.',
        searchableName: [
            'Military',
            'Cultural',
            'Religion',
            'Science',
            'Diplomacy',
            'Espionage',
        ],
    },
    {
        link: '/rules/get-started',
        title: 'Get Started',
        description: 'Read this to get started.',
        searchableName: ['Get', 'Started', 'Beginning', 'Start', 'Set', 'Up'],
    },
    {
        link: '/rules/victory',
        title: 'Victory',
        description:
            'Read this in order to find out what victory conditions there are.',
        searchableName: ['Victory', 'Condition', 'Win', 'Loose'],
    },
    {
        link: '/rules/seasons',
        title: 'Seasons',
        description: 'A season tells you modifiers are on.',
        searchableName: ['Characters', 'Character', 'Player'],
    },
    {
        link: '/rules/clocks',
        title: 'Clock',
        description: 'A clock will act as a passive changing how you play.',
        searchableName: ['Clock', 'Clocks', 'Passive'],
    }
]

export { RulesConst }
