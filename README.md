# BlueSky RPG 

## Fetching all needed data
* create a script which: 
  * get all posts, likes and retweets
  * clear the current event list for the user
  * add all events and setup the user experience

## Events

* Post

```cassandraql
CREATE TABLE bsky.events_tracker
(
    user_id      UUID,
    event_type   TEXT,
    event_action TEXT, // 
    event_id     UUID,
    payload      MAP<TEXT, TEXT>,
    xp           INT,  // comments + likes
    created_at   TIMESTAMP,
    updated_at   TIMESTAMP,
    PRIMARY KEY ( user_id, event_action, event_type, event_id)
);

CREATE TABLE bsky.user_experience
(
    user_id    UUID,
    xp         INT,
    level      INT,
    created_at TIMESTAMP,
    updated_at TIMESTAMP,
    PRIMARY KEY ( user_id)
);

```