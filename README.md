# BlueSky Jetstream RPG

## Overview

BlueSky Jetstream RPG is a Rust application designed to track and manage user experiences and events within the BlueSky
platform. It leverages ScyllaDB for database operations and implements a leveling system based on user activities such
as posts, likes, and reposts.

## Features

The main goal of the project is build a high-performance, scalable, and reliable application that can:

- Fetch and process public events from the BlueSky platform.
- Track user events and experiences.
- Implement a leveling system with experience points (XP).
- Display user levels and progress based on XP via REST API.

## Project Stack

The project uses the following technologies and packages:

- **Language:** [Rust](https://www.rust-lang.org/)
- **Database:** [ScyllaDB](https://www.scylladb.com/)
- **Packages:**
    - **HTTP Server:** [actix-web](https://github.com/actix/actix-web)
    - **ORM:** [charybdis](https://github.com/nodecosmos/charybdis)
    - **Jetstream Client:** [jetstream-oxide](https://github.com/videah/jetstream-oxide)
    - **Bluesky Client:** [atrium-api](https://github.com/sugyan/atrium)

## Installation

1. Clone the repository:
   ```sh
   git clone https://github.com/danielhe4rt/bluesky-jetstream-rpg.git
   ```
2. Navigate to the project directory:
   ```sh
   cd bluesky-jetstream-rpg
   ```
3. Build the project using Cargo:
   ```sh
   cargo build --release
   ```

## Usage

1. Run the application:
   ```sh
   cargo run --release
   ```

## Configuration

The project uses the following environment and configuration files:

- `src/main.rs`: Sets up the application environment and starts HTTP and Jetstream services.
- `src/jetstream.rs`: Configures and starts the Jetstream listener for specific events.
- `src/leveling.rs`: Defines the leveling system and calculates user levels based on experience points.

## Supported Events

The project tracks and processes the following event types:

- **Post:** app.bsky.feed.post
- **Like:** app.bsky.feed.like
- **Retweet:** app.bsky.feed.retweet

## Database Schema

The project uses ScyllaDB for database operations with the following table schemas:

| Type              | Name                           | Description                                   |
|-------------------|--------------------------------|-----------------------------------------------|
| Table             | bsky_rpg.characters            | Stores user characters and leveling states.   |
| Table             | bsky_rpg.characters_experience | Stores user experience points using Counters. |
| Table             | bsky_rpg.events                | Stores user events.                           |
| Materialized View | bsky_rpg.events_by_type        | Materialized view of user events by type.     |
| UDT               | bsky_rpg.leveling              | User leveling schema type.                    |

```cassandraql
-- Create the Leveling UDT
CREATE TYPE bsky_rpg.leveling
    (
        level                    int,
        experience               int,
        experience_to_next_level int,
        levels_gained            int,
        progress_percentage      float
    );

-- Create Character K-V Table
CREATE TABLE bsky_rpg.characters
(
    user_did       text,
    leveling_state leveling,
    name           text,
    PRIMARY KEY (user_did)
);

-- Create Experience Counter Table
CREATE TABLE bsky_rpg.characters_experience
(
    user_did           text,
    current_experience counter,
    PRIMARY KEY (user_did)
);


-- Create Events Table
CREATE TABLE bsky_rpg.events
(
    user_did       text,
    event_at       timestamp,
    event_data     frozen<map<text, text>>,
    event_id       text,
    event_type     text,
    leveling_state leveling,
    PRIMARY KEY (user_did, event_at)
) WITH CLUSTERING ORDER BY (event_at DESC);

-- Create Materialized View for Events by Type
CREATE MATERIALIZED VIEW bsky_rpg.events_by_type AS
SELECT user_did, event_type, event_at, event_data, event_id, leveling_state
FROM bsky_rpg.events
WHERE user_did IS NOT null
  AND event_type IS NOT null
  AND event_at IS NOT null
PRIMARY KEY ((user_did, event_type), event_at)
WITH CLUSTERING ORDER BY (event_at ASC);
```

## License

This project is licensed under the MIT License.

Feel free to customize this README further to better fit your project's needs.

## Useful Links

- [BlueSky Jetstream Repository](https://github.com/bluesky-social/jetstream)
- [ScyllaDB Documentation](https://docs.scylladb.com/)