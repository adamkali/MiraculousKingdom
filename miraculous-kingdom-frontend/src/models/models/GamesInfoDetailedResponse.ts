/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */

import type { GameInfo } from "./GameInfo";
import type { Progress } from "./Progress";

export type GamesInfoDetailedResponse = {
    data: Array<GameInfo>;
    success: Progress;
};
