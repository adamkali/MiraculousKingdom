/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */

import type { Progress } from './Progress';
import type { SeasonResponse } from './SeasonResponse';

export type SeasonsDetailedResponse = {
    data: Array<SeasonResponse>;
    success: Progress;
};
