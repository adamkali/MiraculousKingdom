/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */

import type { Progress } from './Progress';
import type { RollResponse } from './RollResponse';

export type RollDetailedResponse = {
    data: RollResponse;
    success: Progress;
};
