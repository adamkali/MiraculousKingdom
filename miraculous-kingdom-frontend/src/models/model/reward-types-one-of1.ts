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



// tslint:disable
import { APIModel, APIModelOptions, AttributeMapItem } from "../base";

export class RewardTypesOneOf1 extends APIModel {
    Experience: number;

    constructor(options?: APIModelOptions) {
        super(RewardTypesOneOf1.attributeTypeMap, options);
    }

    static get attributeTypeMap(): Array<AttributeMapItem> {
        return [
            { name: "Experience", type: "number" }
        ];
    }
}




