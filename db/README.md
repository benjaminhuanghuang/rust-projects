


## PostgresSQL
```
[dependencies]
dotenv = "0.15.10"

sqlx =  {version = "1.0.134", features= [
  "postgres",
  "reuntime-tokio-result",
  "macros",
  "chrono"
  ]}

```