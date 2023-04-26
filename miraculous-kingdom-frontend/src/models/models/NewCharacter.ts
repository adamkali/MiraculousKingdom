/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */

import type { ClassEnum } from "./ClassEnum";

/**
 * A struct representing a new character request from a client.
 */
export type NewCharacter = {
    char_class: ClassEnum;
    /**
     * The character's might.
     */
    char_might: Record<string, number>;
    /**
     * The name of the character.
     */
    char_name: string;
    /**
     * A secret lock to the character for getting the character.
     */
    secret: string;
};
