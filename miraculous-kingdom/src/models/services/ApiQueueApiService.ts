/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */
import type { QueueDetailedResponse } from '../models/QueueDetailedResponse';
import type { RollDetailedResponse } from '../models/RollDetailedResponse';
import type { RollRequest } from '../models/RollRequest';
import type { SeasonResponse } from '../models/SeasonResponse';
import type { TurnRequest } from '../models/TurnRequest';

import type { CancelablePromise } from '../core/CancelablePromise';
import { OpenAPI } from '../core/OpenAPI';
import { request as __request } from '../core/request';

export class ApiQueueApiService {

    /**
     * @returns QueueDetailedResponse Found Queue from database
     * @throws ApiError
     */
    public static getQueue(): CancelablePromise<QueueDetailedResponse> {
        return __request(OpenAPI, {
            method: 'GET',
            url: '/api/queue',
        });
    }

    /**
     * @param requestBody 
     * @returns RollDetailedResponse Roll Added the Queue
     * @throws ApiError
     */
    public static roll(
requestBody: RollRequest,
): CancelablePromise<RollDetailedResponse> {
        return __request(OpenAPI, {
            method: 'POST',
            url: '/api/queue/roll',
            body: requestBody,
            mediaType: 'application/json',
            errors: {
                400: `Bad Request: Invalid Roll`,
                500: `Internal error occured`,
            },
        });
    }

    /**
     * @param requestBody 
     * @returns QueueDetailedResponse Listed classes from database
     * @throws ApiError
     */
    public static setSeason(
requestBody: SeasonResponse,
): CancelablePromise<QueueDetailedResponse> {
        return __request(OpenAPI, {
            method: 'POST',
            url: '/api/queue/season',
            body: requestBody,
            mediaType: 'application/json',
            errors: {
                400: `Bad Request: id`,
                500: `Internal error occured`,
            },
        });
    }

    /**
     * @param requestBody 
     * @returns QueueDetailedResponse Found Queue from database
     * @throws ApiError
     */
    public static takeTurn(
requestBody: TurnRequest,
): CancelablePromise<QueueDetailedResponse> {
        return __request(OpenAPI, {
            method: 'POST',
            url: '/api/queue/turn',
            body: requestBody,
            mediaType: 'application/json',
            errors: {
                404: `Could not find queue from database`,
                500: ` Internal error occured`,
            },
        });
    }

    /**
     * @param pass Password of the game to be set
     * @returns QueueDetailedResponse Found Queue from database
     * @throws ApiError
     */
    public static setQueue(
pass: string,
): CancelablePromise<QueueDetailedResponse> {
        return __request(OpenAPI, {
            method: 'PUT',
            url: '/api/queue/{pass}',
            path: {
                'pass': pass,
            },
            errors: {
                404: `Could not find queue from database`,
                500: ` Internal error occured`,
            },
        });
    }

}
