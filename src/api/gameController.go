package api

import (
	"math/rand"
	"net/http"

	"github.com/adamkali/MiraculousKingdom/src/structs"
	"github.com/gin-gonic/gin"
	"github.com/surrealdb/surrealdb.go"
)

type GamesController struct {
    DB *surrealdb.DB
}

func (gc *GamesController) GetGames(ctx *gin.Context) {
    nilGame := structs.Game{}
    games, err := nilGame.GetGames(gc.DB)
    if err != nil {
        ctx.HTML(
            http.StatusInternalServerError,
            "error.html",
            gin.H{
                "ErrorCode": http.StatusInternalServerError,
                "ErrorMessage": err.Error(),
            },
        )
        return
    }
    ctx.HTML(
        http.StatusOK,
        "game/games.html",
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
                "ErrorCode": http.StatusBadRequest,
                "ErrorMessage": "Id was not passed to the server.",
            },
        )
        return
    }

    game := structs.Game{ ID: id }
    err := game.GetGame(gc.DB)
    if err != nil {
        ctx.HTML(
            http.StatusBadRequest,
            "error.html",
            gin.H{
                "ErrorCode": http.StatusBadRequest,
                "ErrorMessage": err.Error(),
            },
        )
        return
    }

    ctx.HTML(
        http.StatusOK,
        "game/game.html",
        game,
    )
}

func (gc *GamesController) FinalizeGame(ctx *gin.Context) {
    var game structs.GameRequest
    err := ctx.Bind(&game)
    if err != nil {
        ctx.JSON(
            http.StatusBadRequest,
            gin.H{
                "ErrorMessage": err.Error(),
            },
        )
        return
    } 

    gameCreated := structs.Game{
        Name: game.Name,
        Description: game.Description,
        Nation: game.Nation,
    }

    err = gameCreated.FinalizeGame(gc.DB)
    if err != nil {
        ctx.JSON(
            http.StatusInternalServerError,
            gin.H{
                "ErrorCode": http.StatusInternalServerError,
                "ErrorMessage": err.Error(),
            },
        )
        return
    }
    ctx.HTML(
        http.StatusOK,
        "game/created-response.html",
        gameCreated,
    )
}


func (gc *GamesController) CreateGame(ctx *gin.Context) {
    var game structs.Game
    var letters = []rune("abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ1234567890")
    b := make([]rune, 8)
    for i := range b {
        b[i] = letters[rand.Intn(len(letters))]
    }
    game.Passkey = string(b)
    
    err := game.CreateGame(gc.DB)
    if err != nil {
        ctx.JSON(
            http.StatusInternalServerError,
            gin.H{
                "ErrorCode": http.StatusInternalServerError,
                "ErrorMessage": err.Error(),
            },
        )
        return
    }

    ctx.HTML(http.StatusOK, "game/form.html", game)
}


func CreateGameController(r *gin.Engine, db *surrealdb.DB) {
    g := GamesController{ DB: db, } 
    games := r.Group("/games")
    {
        games.GET("", g.GetGames)
        games.GET("/{id}", g.GetGame)
        games.POST("/", g.CreateGame)
        games.PUT("/", g.FinalizeGame)
    }
}
