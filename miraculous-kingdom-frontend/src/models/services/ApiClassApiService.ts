/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */
import type { ClassDetailedResponse } from "../models/ClassDetailedResponse";
import type { VecClassDetailedResponse } from "../models/VecClassDetailedResponse";

import type { CancelablePromise } from "../core/CancelablePromise";
import { OpenAPI } from "../core/OpenAPI";
import { request as __request } from "../core/request";

export class ApiClassApiService {
    /**
     * @returns VecClassDetailedResponse Listed classes from database
     * @throws ApiError
     */
    public static getClasses(): CancelablePromise<VecClassDetailedResponse> {
        return __request(OpenAPI, {
            method: "GET",
            url: "/api/class",
            errors: {
                500: `Internal error occured`,
            },
        });
    }

    /**
     * @param id ObjectId for mongodb
     * @returns ClassDetailedResponse Found class from database
     * @throws ApiError
     */
    public static getClass(
        id: string
    ): CancelablePromise<ClassDetailedResponse> {
        return __request(OpenAPI, {
            method: "GET",
            url: "/api/class/{id}",
            path: {
                id: id,
            },
            errors: {
                400: `Bad Request: id`,
                500: ` Internal error occured`,
            },
        });
    }
}
