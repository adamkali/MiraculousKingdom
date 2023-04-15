// tslint:disable
import { AxiosPromise } from "axios";
import { APIHandler, APIURLTypes, APIParams } from "../base";
import { ClassDetailedResponse, VecClassDetailedResponse } from "../model";

export class ApiclassApiApi extends APIHandler {
    static urls: APIURLTypes.ApiclassApiApi = {
        getClass: "/api/class/{id}",
        getClasses: "/api/class"
    };
    
    constructor() {
        super("ApiclassApiApi");
    }
    
    /**
     * @param id ObjectId for mongodb
     */
    public getClass(params: APIParams & {
        id: string;
    }): AxiosPromise<ClassDetailedResponse> {
        const apiURL: string = this.initAPIURL(ApiclassApiApi.urls.getClass, { id: params.id } );
        params.options = this.initOptions(params.options, {  }, {  }, params.canceler);
        const body: any = null;
        return this.makeCall<ClassDetailedResponse>("GET".toLowerCase(), apiURL, "getClass", params.options, body, "getClass");
    }

    /**
     */
    public getClasses(params: APIParams & {
        
    } = {}): AxiosPromise<VecClassDetailedResponse> {
        const apiURL: string = this.initAPIURL(ApiclassApiApi.urls.getClasses, null);
        params.options = this.initOptions(params.options, {  }, {  }, params.canceler);
        const body: any = null;
        return this.makeCall<VecClassDetailedResponse>("GET".toLowerCase(), apiURL, "getClasses", params.options, body, "getClasses");
    }

}
