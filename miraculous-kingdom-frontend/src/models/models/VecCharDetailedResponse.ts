/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */

import type { CharacterResponse } from "./CharacterResponse";
import type { Progress } from "./Progress";

export type VecCharDetailedResponse = {
    data: Array<CharacterResponse>;
    success: Progress;
};
