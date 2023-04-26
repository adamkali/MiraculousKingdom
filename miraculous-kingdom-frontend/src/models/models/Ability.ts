/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */

import type { Clock } from "./Clock";
import type { MightRequirement } from "./MightRequirement";

export type Ability = {
    ability_clock?: Clock | null;
    ability_desc: string;
    ability_name: string;
    ability_unlock: MightRequirement;
};
