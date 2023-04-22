package main

import (
	"context"
	"database/sql"
	"fmt"
	"github.com/labstack/echo/v4"
	"github.com/labstack/echo/v4/middleware"
	_ "github.com/lib/pq"
	"http-echo-db/dbdata"
	"net/http"
)

type Payload struct {
	Key string `json:"key"`
}

type Response struct {
	Result bool `json:"result"`
}

type CustomContext struct {
	echo.Context
	db  *dbdata.Queries
	ctx context.Context
}

func authorToString(author dbdata.Author) string {
	return fmt.Sprintf("dbdata.Authhor{ID: %d, Name: %s, Bio: %s}", author.ID, author.Name, author.Bio.String)
}

func main() {

	db, err := sql.Open("postgres", "host=localhost port=5432 user=myuser password=mypassword dbname=mydatabase sslmode=disable")
	if err != nil {
		println(err.Error())
		panic(1)
	}

	// Create a new SQLC client
	dbClient := dbdata.New(db)
	dbCtx := context.Background()

	// Echo instance
	e := echo.New()

	// Middleware
	e.Use(middleware.Logger())
	e.Use(middleware.Recover())

	// Routes

	//curl localhost:1323
	e.GET("/", hello)

	// curl "localhost:1323/somepath/type=123&kek=wait"
	e.GET("/:path", withPathAndQueryParams)

	// curl -X POST -H "Content-Type: application/json" -d '{"key": "value"}' http://localhost:1323/post
	e.POST("/post", postHandler)

	// curl localhost:1323/db/create
	e.GET("/db/create", handleWithDb(dbClient, dbCtx, dbInsertOne))

	// curl localhost:1323/db/all
	e.GET("db/all", handleWithDb(dbClient, dbCtx, dbSelectAll))

	// Start server
	e.Logger.Fatal(e.Start(":1323"))
}

// expand default Context with DB access params
func handleWithDb(dbClient *dbdata.Queries, dbCtx context.Context, handlingFunction func(cc *CustomContext) error) echo.HandlerFunc {
	return func(c echo.Context) error {
		cc := &CustomContext{c, dbClient, dbCtx}
		return handlingFunction(cc)
	}
}

func dbInsertOne(c *CustomContext) error {
	newAuthor, err := c.db.CreateAuthor(c.ctx, dbdata.CreateAuthorParams{
		Name: "Ivan",
		Bio: sql.NullString{
			String: "first Ivan",
		},
	})

	if err != nil {
		return err
	}

	return c.JSON(http.StatusOK, fmt.Sprintf("Created author %s", authorToString(newAuthor)))
}

func dbSelectAll(cc *CustomContext) error {
	authors, err := cc.db.ListAuthors(cc.ctx)
	if err != nil {
		return err
	}

	var authorsStr string
	for _, author := range authors {
		authorsStr += fmt.Sprintf("%s, ", authorToString(author))
	}

	return cc.JSON(http.StatusOK, fmt.Sprintf("all authors: %d, values: %s", len(authors), authorsStr))
}

func postHandler(c echo.Context) error {

	// read as payload
	var requestPayload Payload
	if err := c.Bind(&requestPayload); err != nil {
		return err
	}

	fmt.Println("received request", requestPayload)

	// Do something with the payload
	response := &Response{
		Result: true,
	}
	fmt.Println("returning response", response)
	return c.JSON(http.StatusOK, response)

}

func withPathAndQueryParams(c echo.Context) error {
	queryParam := c.QueryParams()
	path := c.Param("path")

	return c.String(http.StatusOK, "Hello "+path+" params: "+queryParam.Encode())
}

// Handler
func hello(c echo.Context) error {
	path := c.Request().URL.Path
	return c.String(http.StatusOK, "Hello, World. Called on "+path)
}
