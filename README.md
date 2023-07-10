# Running

1. Create a `.env` file to specify the database path
```bash
DATABASE_URL="sqlite:shared-bookmarks.db"
```
2. Install sqlx-cli
```bash
cargo install sqlx-cli
```
3. Create the database
```bash
sqlx db create
```
4. Start the application
```bash
cargo run
```
5. Hit the api with `curl` to add new posts
```bash
curl -H 'Content-Type: application/json' http://localhost:8000/api/addPost -d '{"url":"http://my-interesting-url.com/cool-beans","title":"Growing beans in a cold climate"}'
```
6. Navigate to http://localhost:8000 to view the web interface
