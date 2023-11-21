package api

import (

	"github.com/adamkali/MiraculousKingdom/src/structs"
	"github.com/gin-gonic/gin"
)

// Used to pass data to the Index Page
// in the Index function 
//
// GameLinkData: LinkData for the Game Button
// RulesLinkData: LinkData for the Rules Button
type indexData struct {
    GameLinkData structs.LinkData
    RulesLinkData structs.LinkData
}

// Displays the Root Index Page
// of the website at /
func Index(c *gin.Context) {
    index := indexData {
        GameLinkData: structs.LinkData{
            Href: "/game",
            Text: "Play Game",
            SvgName: "/static/images/dice-20.svg",
        },
        RulesLinkData: structs.LinkData{
            Href: "/rules",
            Text: "Read the Rules",
            SvgName: "/static/images/secret-book.svg",
        },
    }
    c.HTML(200, "root/index.html", index)
}


// Used to pas to the Game Page 
// in the Game function
//
//  CreateGameButtonData: ButtonData for the Create Game Button
//  GoBackRootLinkData: LinkData for the Go Back to Root Button 
type gameData struct {
    CreateGameButtonData structs.ButtonData
    GoBackRootLinkData structs.LinkData
}

// Displays the Game Page 
// of the website at /game
func Game(c *gin.Context) {
    index := gameData {
        CreateGameButtonData: structs.ButtonData{
            Endpoint: "/games",
            Method: "post", // this will make the hx-{{.Method}} turn into hx-post so it is not Capitalized
            Target: "#game-wrapper",
            Action: "click", 
            Text: "Create Game",
            SvgName: "/static/images/dice-20.svg",
        },
        GoBackRootLinkData: structs.LinkData{
            Href: "/",
            Text: "Go Back",
            SvgName: "/static/images/arrow-left.svg",
        },
    }
    
    c.HTML(200, "game/index.html", index)
}
