import { writable } from 'svelte/store'
import {
    type QueueResonse,
    type CharacterResponse,
    type GameInfo,
} from './models'

const createLocalStorage = <T>(key: string, value: T) => {
    const { subscribe, set } = writable(value)

    return {
        subscribe,
        set: (value: T) => {
            localStorage.setItem(key, JSON.stringify(value))
        },
        get: (): T => {
            const json = localStorage.getItem(key)
            if (json) {
                return JSON.parse(json)
            }

            return value as T
        },
    }
}

export const gameCharacter = createLocalStorage<CharacterResponse>(
    'currentCharacter',
    {} as CharacterResponse,
)
export const currentGame = createLocalStorage<GameInfo>(
    'currentGame',
    {} as GameInfo,
)

export const queue = createLocalStorage<QueueResonse>(
    'queue',
    {} as QueueResonse,
)
