package structs

import (
	"github.com/adamkali/MiraculousKingdom/src"
	"github.com/surrealdb/surrealdb.go"
)

type Game struct {
    surrealdb.Basemodel     `table:"game"`
	ID          string      `json:"id,omitempty"`
	Name        string      `json:"name,omitempty"`
	Description string      `json:"description,omitempty"`
	Characters  []Character `json:"characters,omitempty"`
	Nation      string      `json:"nation,omitempty"`
	Passkey     string      `json:"passkey,omitempty"`
}

type GameRequest struct {
	Name        string `form:"name" json:"name" binding:"required"`
	Description string `form:"description" json:"description" binding:"required"`
	Nation      string `form:"nation" json:"nation" binding:"required"`
	Passkey     string `form:"passkey" json:"passkey" binding:"required"`
}

func (g *Game) CreateGame(db *surrealdb.DB) error {
	g.ID = "game:" + src.CreateUniqueID()
	gReturn, err := db.Create(g.ID, map[string]interface{}{
        "name": "",
        "description": "",
        "nation": "",
		"passkey": g.Passkey,
        "characters": []Character{},
	})
	if err != nil {
		return err
	}

	err = surrealdb.Unmarshal(gReturn, &g)
	if err != nil {
		return err
	}
	return nil
}

func (g *Game) FinalizeGame(db *surrealdb.DB) error {
	if _, err := db.Change(g.ID, map[string]interface{}{
		"name":        g.Name,
		"description": g.Description,
		"nation":      g.Nation,
	}); err != nil {
		return err
	}

	return nil
}

func (game *Game) GetGames(db *surrealdb.DB) ([]Game, error) {
	var games []Game

	toGames, err := db.Select("game")
	if err != nil {
		return games, err
	}

	err = surrealdb.Unmarshal(toGames, &games)
	if err != nil {
		return games, err
	}

	return games, nil
}

func (game *Game) GetGame(db *surrealdb.DB) error {
	toGame, err := db.Select(game.ID)
	if err != nil {
		return err
	}

	err = surrealdb.Unmarshal(toGame, &game)
	if err != nil {
		return err
	}

	return nil
}
