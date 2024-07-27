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

- `GET /api/lists`
- `GET /api/lists/top`
- `GET /api/lists/likes/:list_type`

- `GET /api/users`
- `POST /api/users`
- `GET /api/users/:user_name`
- `GET /api/users/:user_name/info`
- `GET /api/users/:user_name/:list_name`
- `GET /api/users/:user_name/:list_name/type`
- `GET /api/users/:user_name/likes`

- `GET /api/likes`
- `GET /api/likes/:user_name`

- `GET /api/follow`
- `GET /api/follow/:user_name/following`
- `GET /api/follow/:user_name/mutual`

- `GET /api/groups`
- `GET /api/groups/all`
- `GET /api/groups/:user_name/group_members`
- `GET /api/groups/:gid/group`
- `GET /api/groups/:gid/members`
- `GET /api/groups/:gid/lists`
- `GET /api/groups/:gid/circles`

- `POST /api/auth/register`
- `POST /api/auth/login`

#### Protected Routes

#### To Do with Authentication

- `GET /api/auth/logout`
- `GET /api/auth/user`

#### To Do with Lists

- `GET /api/explore/:user_name`
- `POST /api/lists/:user_name`
- `PATCH /api/lists/:user_name/:list_name`
- `DELETE /api/lists/:user_name/:list_name`
- `GET /api/listitems/:list_name/:item_id`
- `POST /api/listitems/:list_name/:item_id`
- `PATCH /api/listitems/:list_name/:item_id`
- `DELETE /api/listitems/:list_name/:item_id`

#### To Do with Users

- `GET /api/users/:user_name/me`
- `PATCH /api/users/:user_name/me`
- `DELETE /api/users/:user_name/me`

#### To Do with Likes

- `POST /api/likes/`
- `DELETE /api/likes/:liking/:list_name`

#### To Do With Follow

- `POST /api/follow`
- `DELETE /api/follow/:following`

#### To Do With Groups

- `POST /api/groups/:gid/join`
- `DELETE /api/groups/:gid/unjoin`
- `POST /api/groups/:user_name/me`
- `DELETE /api/groups/:user_name/me/:group_name`

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
