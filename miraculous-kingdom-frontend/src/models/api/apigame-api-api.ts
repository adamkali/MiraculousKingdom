// tslint:disable
import { AxiosPromise } from "axios";
import { APIHandler, APIURLTypes, APIParams } from "../base";
import { CharAddedDetailedResponse, GameCreation, GameInfoDetailedResponse, GamesInfoDetailedResponse, NewCharacter, PassDetailedResponse } from "../model";

export class ApigameApiApi extends APIHandler {
    static urls: APIURLTypes.ApigameApiApi = {
        addCharacter: "/api/game/{pass}",
        getGame: "/api/game/{pass}",
        getGames: "/api/game",
        startGame: "/api/game"
    };
    
    constructor() {
        super("ApigameApiApi");
    }
    
    /**
     * @param pass Password for entering the game.
     * @param newCharacter 
     */
    public addCharacter(params: APIParams & {
        pass: string;
        newCharacter: NewCharacter;
    }): AxiosPromise<CharAddedDetailedResponse> {
        const apiURL: string = this.initAPIURL(ApigameApiApi.urls.addCharacter, { pass: params.pass } );
        params.options = this.initOptions(params.options, {  }, {  }, params.canceler);
        const body: any = params.newCharacter;
        return this.makeCall<CharAddedDetailedResponse>("POST".toLowerCase(), apiURL, "addCharacter", params.options, body, "addCharacter");
    }

    /**
     * @param pass Password for entering the game.
     */
    public getGame(params: APIParams & {
        pass: string;
    }): AxiosPromise<GameInfoDetailedResponse> {
        const apiURL: string = this.initAPIURL(ApigameApiApi.urls.getGame, { pass: params.pass } );
        params.options = this.initOptions(params.options, {  }, {  }, params.canceler);
        const body: any = null;
        return this.makeCall<GameInfoDetailedResponse>("GET".toLowerCase(), apiURL, "getGame", params.options, body, "getGame");
    }

    /**
     */
    public getGames(params: APIParams & {
        
    } = {}): AxiosPromise<GamesInfoDetailedResponse> {
        const apiURL: string = this.initAPIURL(ApigameApiApi.urls.getGames, null);
        params.options = this.initOptions(params.options, {  }, {  }, params.canceler);
        const body: any = null;
        return this.makeCall<GamesInfoDetailedResponse>("GET".toLowerCase(), apiURL, "getGames", params.options, body, "getGames");
    }

    /**
     * @param gameCreation 
     */
    public startGame(params: APIParams & {
        gameCreation: GameCreation;
    }): AxiosPromise<PassDetailedResponse> {
        const apiURL: string = this.initAPIURL(ApigameApiApi.urls.startGame, null);
        params.options = this.initOptions(params.options, {  }, {  }, params.canceler);
        const body: any = params.gameCreation;
        return this.makeCall<PassDetailedResponse>("POST".toLowerCase(), apiURL, "startGame", params.options, body, "startGame");
    }

}
