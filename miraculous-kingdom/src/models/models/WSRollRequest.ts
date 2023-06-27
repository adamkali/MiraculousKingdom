/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */

import type { Ability } from './Ability';
import type { CharacterResponse } from './CharacterResponse';

export type WSRollRequest = {
    ability?: Ability | null;
    character?: CharacterResponse | null;
    owner: string;
    /**
     * the person who sent the request filled
 * by the client
     */
    secret: string;
};
