/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */

import type { Ability } from './Ability';
import type { ClassEnum } from './ClassEnum';

export type ClassResponse = {
    class_abilities: Array<Ability>;
    class_desc: string;
    class_enum: ClassEnum;
    class_name: string;
    class_perks: string;
};
