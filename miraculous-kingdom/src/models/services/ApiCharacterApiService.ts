/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */
import type { Ability } from '../models/Ability'
import type { CharDetialedResponse } from '../models/CharDetialedResponse'
import type { VecCharDetailedResponse } from '../models/VecCharDetailedResponse'

import type { CancelablePromise } from '../core/CancelablePromise'
import { OpenAPI } from '../core/OpenAPI'
import { request as __request } from '../core/request'

export class ApiCharacterApiService {
    /**
     * @param secret String set by the user to get their data
     * @param pass String set by the user to get their data
     * @param requestBody
     * @returns CharDetialedResponse Discard an Ability for the player put in
     * @throws ApiError
     */
    public static discardCard(
        secret: string,
        pass: string,
        requestBody: Ability,
    ): CancelablePromise<CharDetialedResponse> {
        return __request(OpenAPI, {
            method: 'PUT',
            url: '/api/character/discard/{secret}/{pass}',
            path: {
                secret: secret,
                pass: pass,
            },
            body: requestBody,
            mediaType: 'application/json',
            errors: {
                500: `Internal error occured`,
            },
        })
    }

    /**
     * @param number String set by the user to get their data from the game
     * @param secret String set by the user to get their data from the game
     * @param pass String set by the user to get the game
     * @returns CharDetialedResponse Draw an Ability for the player put in
     * @throws ApiError
     */
    public static drawCard(
        number: number,
        secret: string,
        pass: string,
    ): CancelablePromise<CharDetialedResponse> {
        return __request(OpenAPI, {
            method: 'PUT',
            url: '/api/character/draw/{number}/{secret}/{pass}',
            path: {
                number: number,
                secret: secret,
                pass: pass,
            },
            errors: {
                500: `Internal error occured`,
            },
        })
    }

    /**
     * @param secret String set by the user to get their data from the game
     * @param pass String set by the user to get the game
     * @returns CharDetialedResponse Initialized the hand of a player for the game
     * @throws ApiError
     */
    public static initHand(
        secret: string,
        pass: string,
    ): CancelablePromise<CharDetialedResponse> {
        return __request(OpenAPI, {
            method: 'PUT',
            url: '/api/character/init_hand/{secret}/{pass}',
            path: {
                secret: secret,
                pass: pass,
            },
            errors: {
                500: `Internal error occured`,
            },
        })
    }

    /**
     * Endpoint to find all characters that the players is participating in for their specific secret.
     * Endpoint to find all characters that the players is participating in for their specific secret.
     *
     * # Example
     *
     * ```
     * GET /api/character/{secret}
     * ```
     *
     * # Parameters
     *
     * - `secret`: String set by the user. Should keep the same. For now too lazy to fix 👿
     *
     * # Responses
     *
     * - `200 OK`: Found characters with secret_code: {secret}
     *
     * ```json
     * {
     * "code": 200,
     * "message": "Found characters with secret_code: {secret}",
     * "success": true,
     * "data": [
     * {
     * "char_name": "character_name",
     * ...
     * },
     * ...
     * ]
     * }
     * ```
     *
     * - `500 Internal Server Error`: An internal error occurred.
     *
     * ```json
     * {
     * "code": 500,
     * "success": false
     * "message": "Internal error occurred",
     * "data": []
     * }
     * ```
     *
     * @param secret String set by the user to get their data
     * @returns VecCharDetailedResponse Found characters with secret_code: {secret}
     * @throws ApiError
     */
    public static getCharacters(
        secret: string,
    ): CancelablePromise<VecCharDetailedResponse> {
        return __request(OpenAPI, {
            method: 'GET',
            url: '/api/character/{secret}',
            path: {
                secret: secret,
            },
            errors: {
                500: `Internal error occured`,
            },
        })
    }

    /**
     * @param secret String set by the user to get their data
     * @param pass String generated by the api for the specific game.
     * @returns CharDetialedResponse Found characters with : {secret} and {pass}
     * @throws ApiError
     */
    public static getCharacterForGame(
        secret: string,
        pass: string,
    ): CancelablePromise<CharDetialedResponse> {
        return __request(OpenAPI, {
            method: 'GET',
            url: '/api/character/{secret}/{pass}',
            path: {
                secret: secret,
                pass: pass,
            },
            errors: {
                500: `Internal error occured`,
            },
        })
    }
}
