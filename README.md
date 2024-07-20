# Based Ten

A data-driven application for CS 348. Created in Spring 2024 by Nandan Patel, Emily Wang, Justin Lin, Jessie Zhang, and Zain Salman.

## client

> The client web-app for the based-ten application

### Getting Started: Install dependencies

Install the dependencies by running `npm i` in the client directory.

### Running the client app

Run the client app using `npm run dev` in the client directory. Ensure the server is running.

## server

> The API service for the based-ten application

### Getting Started: Setting up the Database

1. Make sure you have [Rust](https://www.rust-lang.org/tools/install) installed and can use the `cargo` package manager.
2. Install the [sqlx-cli](https://crates.io/crates/sqlx-cli) tool
3. Run the following command in the terminal at the root of the project directory, with the `Cargo.toml` file.
   ```commandline
   sqlx migrate run
   ```
4. From the root of the project directory, run the following command to run our script for seeding the database with our production data

```commandline
cargo run --package seed
```

5. To delete all tables and thus the data stored in the database, run the following command in the terminal at the root of the project directory with the `Cargo.toml` file.

```commandline
sqlx migrate revert
```

### Running the Backend Server

To run the backend server is quite simply; simply run the following command in the terminal anywhere near the project directory:

```commandline
cargo run
```

Doing so, you should have the following message (at least similar to the following message) appear in the terminal:

```commandline
DEBUG based_ten: Now listening on port 5000
```

Now, you can make normal HTTP requests to our backend server, with the following endpoints being currently available. Sample objects for POST or UPDATE requests are not shown, as this API is an internal tool.

#### API Endpoints Currently Functioning

- `GET /api/anime`
- `POST /api/anime`
- `GET /api/anime/:id`
- `PATCH /api/anime/:id`
- `DELETE /api/anime/:id`

- `GET /api/movies`
- `POST /api/movies`
- `GET /api/movies/:id`
- `PATCH /api/movies/:id`
- `DELETE /api/movies/:id`

- `GET /api/songs`
- `POST /api/songs`
- `GET /api/songs/:id`
- `PATCH /api/songs/:id`
- `DELETE /api/songs/:id`

- `GET /api/videogames`
- `POST /api/videogames`
- `GET /api/videogames/:id`
- `PATCH /api/videogames/:id`
- `DELETE /api/videogames/:id`

- `GET /api/media`
- `GET /api/:type`

- `GET /api/lists`
- `POST /api/lists`
- `GET /api/lists/:email`
- `GET /api/lists/:email/:list_name`
- `GET /api/lists/:email/:list_name/items`
- `PATCH /api/lists/:email/:list_name`
- `DELETE /api/lists/:email/:list_name`

- `GET /api/listitems/:email/:listname/:item_id`
- `POST /api/listitems/:email/:listname/:item_id`
- `PATCH /api/listitems/:email/:listname/:item_id`
- `DELETE /api/listitems/:email/:listname/:item_id`

- `GET /api/users`
- `POST /api/users`
- `GET /api/users/:email`
- `PATCH /api/users/:email`
- `DELETE /api/users/:email`

- `GET /api/likes`
- `POST /api/likes`
- `GET /api/likes/:email`
- `DELETE /api/likes/:likeremail/:likingemail/:listname`

- `GET /api/follow`
- `POST /api/follow`
- `GET /api/follow/:email`
- `DELETE /api/follow/:followeremail/:followingemail`
- `GET /api/follow/mutual/:useremail`

- `GET /api/groups`
- `POST /api/groups/:gid/:groupName/:ownedBy`
- `GET /api/groups/:gid`
- `DELETE /api/groups/:gid`


#### Website Pages
- Search (Main Page)
- Explore (Recommended Lists) Page
- Create/View a list
- Account page for a user (view all user's lists, groups, followers, etc.)
- Manage Groups page (accept/reject invites?)
- A "Search Users & Groups" page: almost identical to Main Search page, but shows results for users and groups

#### Open APIs being used to retrieve and add new data

[Spotify API](https://developer.spotify.com/documentation/web-api)
[OMDb API](http://www.omdbapi.com/)
[Jikan API](https://docs.api.jikan.moe/)
[RAWG API](https://rawg.io/apidocs)