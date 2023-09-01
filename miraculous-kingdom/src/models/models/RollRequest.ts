/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */

import type { CharacterResponse } from './CharacterResponse'

export type RollRequest = {
    owner: string
    rolls: Array<CharacterResponse>
}
