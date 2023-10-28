package structs

import (
	"math/rand"

	"github.com/google/uuid"
	"github.com/surrealdb/surrealdb.go"
)

type Game struct { 
    ID string `json:"id"`
    Name string `json:"name"`
    Description string `json:"description"`
    Characters []Character `json:"characters"`
    Nation string `json:"nation"`
    Passkey string `json:"passkey"`
}

type GameRequest struct {
    Name string `json:"name"`
    Description string `json:"description"`
    Nation string `json:"nation"`
    Passkey string `json:"passkey"`
}

func InsertGame(db *surrealdb.DB, game GameRequest) (Game, error) {
    var g Game
    g.Characters = []Character{}
    g.Name = game.Name
    g.Description = game.Description 
    g.Nation = game.Nation 
    g.Passkey = func() string { 
        ret := ""
        // make a randem 6 char passkey 
        for i := 0; i < 6; i++ {
            ret += string(rune(97 + rand.Intn(26)))
        }
        return ret
    }()
    g.ID = "game:" + uuid.New().String()

    if _, err := db.Create(g.ID, map[string]interface{}{
        "name": g.Name,
        "description": g.Description,
        "nation": g.Nation,
        "passkey": g.Passkey,
    }); err != nil {
        return g, err
    }

    return g, nil
}

func GetGames(db *surrealdb.DB) ([]Game, error) {
    var games []Game
    var toGames interface{}
    var err error

    if toGames, err = db.Select("game"); err != nil {
        return games, err
    }

    err = surrealdb.Unmarshal(toGames, &games)
    if err != nil {
        return games, err
    }

    return games, nil
}

func GetGame(db *surrealdb.DB, id string) (Game, error) {
    var game Game 
    var toGame interface{}
    var err error 

    if toGame, err = db.Select(id); err != nil {
        return game, err
    }
    
    err = surrealdb.Unmarshal(toGame, &game) 
    if err != nil {
        return game, err
    }

    return games, nil
}
