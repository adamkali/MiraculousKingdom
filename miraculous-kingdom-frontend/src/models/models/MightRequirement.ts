/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */

import type { MightEnum } from "./MightEnum";
import type { RollTier } from "./RollTier";

export type MightRequirement = {
    might: MightEnum;
    roll_tier: RollTier;
    unlock: number;
};
