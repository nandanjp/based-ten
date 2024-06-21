# server

> The API service for the based-ten application

## About

This project uses [Feathers](http://feathersjs.com). An open source framework for building APIs and real-time applications.

## Getting Started

1. Make sure you have [NodeJS](https://nodejs.org/) and [npm](https://www.npmjs.com/) installed.
2. Install your dependencies

    ```
    cd path/to/server
    npm install
    ```

3. Start your app

    ```
    npm run compile # Compile TypeScript source
    npm run migrate # Run migrations to set up the database
    npm start
    ```

## Testing

Run `npm test` and all your tests in the `test/` directory will be run.

## Scaffolding

This app comes with a powerful command line interface for Feathers. Here are a few things it can do:

```
$ npx feathers help                           # Show all commands
$ npx feathers generate service               # Generate a new Service
```

## Help

For more information on all the things you can do with Feathers visit [docs.feathersjs.com](http://docs.feathersjs.com).

## Running SQL

To run SQL files, first ensure that the SQL file to run is in the \src\db\sql folder and the file path relative to the sql folder is included in \src\db\run-raw-sql.ts in the ProvidedCorrectFile enum. Then we can run the SQL file with the following command:

```
$ npx ts-node .\run-raw-sql.ts <sql file path relative to sql folder>
```

For example, to create tables for our database, run the following:

```
$ npx ts-node .\run-raw-sql.ts create-tables
```

To insert data from the sample data from \src\db\sample-data into the tables (available tables are shown in the /src/db/seed/insert.ts file in the ProvidedCorrectInsert enum, option all will insert data for all tables), run the following command from \src\db\seed:

```
$ npx ts-node .\insert.ts <table to insert into>
```

For example, to insert data for all tables, run the following:

```
$ npx ts-node .\insert.ts all
```

### API Endpoints Currently Functioning
- `/api/video-game`
- `/api/video-game/:id`
- `/api/anime`
- `/api/anime/:id`
- `/api/song`
- `/api/song/:id`
- `/api/movie`
- `/api/movie/:id`
- `/api/user`
- `/api/user/:id`


### TODOS - Required Queries:
1. Select all media from all media tables
2. Select all media from a media table of a specific type
3. Create a list with a given name
4. Add all selected items to the list
5. User can create an account
6. Users can select ten items and create a list out of them
7. In a given group, users can view all lists created by group members, sorted by who created the list

### Create a List Feature
Users can create a top ten list. Initially, users can search in a search box for any type of media available. Once the user selects the first item, they can then continue selecting up to a total of ten items of the same type of media as their original item. Once a user has selected all items, they can give their list a name, and then the list is created and associated with their account.