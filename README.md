# Stacky Sides

_The game you're about to realize you've been missing!_

## Quick Start

* Rename the `.env.example` file to `.env` and set the `BASE_URL` and `DATABASE_URL` variables in it.

For example:

```
BASE_URL=http://localhost:8000/
DATABASE_URL=postgres://<db-username>:<db-user-password>@localhost/<db-name>
```

* Run the tests to check that all is well (at least for the few utility functions I wrote tests for, LOL).

```
cargo test
```

* Run the project -- to fire up the webserver

```
cargo run
```

* Visit your BASE_URL in your browser and have fun!
