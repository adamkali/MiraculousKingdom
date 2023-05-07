import { writable } from 'svelte/store'
import type { CharacterResponse, GameInfo } from './models'

export const gameCharacter = writable({} as CharacterResponse)
export const currentGame = writable({} as GameInfo)
