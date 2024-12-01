/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */
import type { SeasonDetailedResponse } from '../models/SeasonDetailedResponse'
import type { SeasonsDetailedResponse } from '../models/SeasonsDetailedResponse'

import type { CancelablePromise } from '../core/CancelablePromise'
import { OpenAPI } from '../core/OpenAPI'
import { request as __request } from '../core/request'

export class ApiSeasonApiService {
    /**
     * @returns SeasonsDetailedResponse Listed classes from database
     * @throws ApiError
     */
    public static getSeasons(): CancelablePromise<SeasonsDetailedResponse> {
        return __request(OpenAPI, {
            method: 'GET',
            url: '/api/season',
            errors: {
                500: `Internal error occured`,
            },
        })
    }

    /**
     * @returns SeasonDetailedResponse Listed classes from database
     * @throws ApiError
     */
    public static roll(): CancelablePromise<SeasonDetailedResponse> {
        return __request(OpenAPI, {
            method: 'GET',
            url: '/api/season/roll',
            errors: {
                400: `Bad Request: id`,
                500: `Internal error occured`,
            },
        })
    }

    /**
     * @param id ObjectId for mongodb
     * @returns SeasonDetailedResponse Found class from database
     * @throws ApiError
     */
    public static getSeason(
        id: string,
    ): CancelablePromise<SeasonDetailedResponse> {
        return __request(OpenAPI, {
            method: 'GET',
            url: '/api/season/{id}',
            path: {
                id: id,
            },
            errors: {
                400: `Bad Request: id`,
                500: ` Internal error occured`,
            },
        })
    }
}
