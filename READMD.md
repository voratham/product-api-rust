## How to run

- i follow tutorial from 
https://medium.com/@rayato159/%E0%B8%A5%E0%B8%AD%E0%B8%87%E0%B9%80%E0%B8%82%E0%B8%B5%E0%B8%A2%E0%B8%99-rest-api-%E0%B8%94%E0%B9%89%E0%B8%A7%E0%B8%A2-rust-axum-a4b8ff42ecab
- describe video about mod.rs lib.rs
https://www.youtube.com/watch?v=lI2JIkrEYnM



```sh
cargo run

cargo install cargo-watch

cargo watch -x run # for hot reload
```

Example curl GET method
```sh
curl http://localhost:3000/1
{"id":1,"title":"Brown","description":"Brown is line friends family !"}%       
```