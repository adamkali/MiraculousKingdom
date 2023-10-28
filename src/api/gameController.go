package api

import (
	"net/http"

	"github.com/gin-gonic/gin"
	"github.com/surrealdb/surrealdb.go"
    "github.com/adamkali/MiraculousKingdom/src/structs"
)

type GamesController struct {
    DB *surrealdb.DB
}

func (gc *GamesController) GetGames(ctx *gin.Context) {
    games, err := structs.GetGames(gc.DB)
    if err != nil {
        ctx.HTML(
            http.StatusInternalServerError,
            "error.html",
            gin.H{
                "ErrorMessage": err.Error(),
            },
        )
        return
    }
    ctx.HTML(
        http.StatusOK,
        "games/games.html",
        games,
    )
}

func (gc *GamesController) GetGame(ctx *gin.Context) {
    id, found := ctx.Params.Get("id")
    if !found {
        ctx.HTML(
            http.StatusBadRequest,
            "error.html",
            gin.H{
                "ErrorMessage": "Id was not passed to the server.",
            },
        )
        return
    }
    game, err := structs.GetGame(gc.DB, id)
    if err != nil {
        ctx.HTML(
            http.StatusBadRequest,
            "error.html",
            gin.H{
                "ErrorMessage": err.Error(),
            },
        )
        return
    }

    ctx.HTML(
        http.StatusOK,
        "games/game.html",
        game,
    )
}
