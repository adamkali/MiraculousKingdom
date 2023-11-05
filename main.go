package main

import (
	"fmt"
	"os"

	"github.com/gin-gonic/gin"
	"github.com/joho/godotenv"
	"github.com/surrealdb/surrealdb.go"
)

// web pages ========
// ==================

// surrealdb ========
type RootDBUser struct {
    Username  string `json:"username"`
    Password  string `json:"password"`
    NameSpace string `json:"namespace"`
    Database  string `json:"database"`
}

func InitDB() (*surrealdb.DB, error) {
    // load in from the env  
    godotenv.Load()

    root := RootDBUser{
        Username:  os.Getenv("DB_USERNAME"),
        Password:  os.Getenv("DB_PASSWORD"),
        NameSpace: os.Getenv("DB_NAMESPACE"),
        Database:  os.Getenv("DB_DATABASE"),
    }

    db, err := surrealdb.New("ws://localhost:8080/rpc")
    if err != nil {
        fmt.Printf("%s !", err.Error())
        return nil, err
    }

    if _, err := db.Signin(map[string]interface{}{
        "user": root.Username,
        "pass": root.Password,
    }); err != nil {
        fmt.Printf("%s !", err.Error())
        return nil, err
    }

    if _, err := db.Use(root.NameSpace, root.Database); err != nil {
        fmt.Printf("%s !", err.Error())
        return nil, err
    }

    return db, nil
}
// ==================

func main() {
    r := gin.Default()

    r.LoadHTMLGlob("routes/**/*")
    r.Static("/static", "./static")

    _, err := InitDB()
    if err != nil {
        panic(err)
    }

    r.GET("/", func(c *gin.Context) {
        c.HTML(200, "root/index.html", gin.H{})
    })

    r.GET("/rules", func(c *gin.Context) {
        c.HTML(200, "rules/index.html", gin.H{})
    })

    r.GET("/game", func(c *gin.Context) {
        c.HTML(200, "game/index.html", gin.H{})
    })

    r.Run(":8000")
}