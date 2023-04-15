// tslint:disable
import { AxiosPromise } from "axios";
import { APIHandler, APIURLTypes, APIParams } from "../base";
import { SeasonDetailedResponse, SeasonsDetailedResponse } from "../model";

export class ApiseasonApiApi extends APIHandler {
    static urls: APIURLTypes.ApiseasonApiApi = {
        getSeason: "/api/season/{id}",
        getSeasons: "/api/season",
        roll: "/api/season/roll"
    };
    
    constructor() {
        super("ApiseasonApiApi");
    }
    
    /**
     * @param id ObjectId for mongodb
     */
    public getSeason(params: APIParams & {
        id: string;
    }): AxiosPromise<SeasonDetailedResponse> {
        const apiURL: string = this.initAPIURL(ApiseasonApiApi.urls.getSeason, { id: params.id } );
        params.options = this.initOptions(params.options, {  }, {  }, params.canceler);
        const body: any = null;
        return this.makeCall<SeasonDetailedResponse>("GET".toLowerCase(), apiURL, "getSeason", params.options, body, "getSeason");
    }

    /**
     */
    public getSeasons(params: APIParams & {
        
    } = {}): AxiosPromise<SeasonsDetailedResponse> {
        const apiURL: string = this.initAPIURL(ApiseasonApiApi.urls.getSeasons, null);
        params.options = this.initOptions(params.options, {  }, {  }, params.canceler);
        const body: any = null;
        return this.makeCall<SeasonsDetailedResponse>("GET".toLowerCase(), apiURL, "getSeasons", params.options, body, "getSeasons");
    }

    /**
     */
    public roll(params: APIParams & {
        
    } = {}): AxiosPromise<SeasonDetailedResponse> {
        const apiURL: string = this.initAPIURL(ApiseasonApiApi.urls.roll, null);
        params.options = this.initOptions(params.options, {  }, {  }, params.canceler);
        const body: any = null;
        return this.makeCall<SeasonDetailedResponse>("GET".toLowerCase(), apiURL, "roll", params.options, body, "roll");
    }

}
