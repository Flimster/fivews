# Database server

The FiveWs project also implements a very simple HTTP server that is easy to use and setup.

## Endpoints

### Create a new log entry

`POST /write`

**Parameters**

`who` (Required): Who did it?
`what` (Required): What happened?
`when` (Required): When did it happen?
`where` (Required): Where did it happen?
`why` (Required): Why did it happen?

### Get log entries

`GET /read`

**Parameters**

`query` (Required): A query string that the server should use when querying the database.   

## TODO

- [] More server configuration