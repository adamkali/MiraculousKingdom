// tslint:disable
/**
 * miraculous-kingdom-self-server
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 0.1.0
 * 
 *
 * NOTE: This class is auto generated by OpenAPI Generator (https://openapi-generator.tech).
 * https://openapi-generator.tech
 * Do not edit the class manually.
 */


import { MightEnum } from './might-enum';

// tslint:disable
import { APIModel, APIModelOptions, AttributeMapItem } from "../base";

export class MightStat extends APIModel {
    stat_enum: MightEnum;
    stat_exp: number;
    stat_name: string;
    stat_token: number;
    stat_value: number;

    constructor(options?: APIModelOptions) {
        super(MightStat.attributeTypeMap, options);
    }

    static get attributeTypeMap(): Array<AttributeMapItem> {
        return [
            { name: "stat_enum", type: "MightEnum" },
            { name: "stat_exp", type: "number" },
            { name: "stat_name", type: "string" },
            { name: "stat_token", type: "number" },
            { name: "stat_value", type: "number" }
        ];
    }
}




