/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */

import type { Ability } from './Ability'
import type { CharacterResponse } from './CharacterResponse'

export type TurnRequest = {
    ability: Ability
    character: CharacterResponse
    initiatve: number
}
