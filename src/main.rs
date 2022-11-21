// It is recommended that you read the README file, it is very important to this example.
// This example will help us to use a sqlite database with our bot.

use serde::{Deserialize, Serialize};
use std::fmt::{Display, Write as _};

use serenity::async_trait;
use serenity::model::prelude::*;
use serenity::prelude::*;

const HELP: &'static str = r#"```
/gz                      -- SKREEONK!
/gz help                 -- Get help
/gz countdown            -- Run a countdown from 10 to 0 
/gz monster {NAME or ID} -- Get monster info

Film Info:
https://github.com/alexeagleson/godzilla-bot/blob/main/films.json

Monster Info:
https://github.com/alexeagleson/godzilla-bot/blob/main/monsters.json
```"#;

struct Bot {
    database: sqlx::SqlitePool,
}

#[derive(Deserialize, Debug)]
struct Film {
    pub id: i32,
    pub title: String,
    pub year: i32,
    pub wikipedia: String,
    pub monsters: Vec<i32>,
}

#[derive(Deserialize, Debug)]
struct Monster {
    pub id: i32,
    pub name: String,
    pub alternate_names: String,
    pub wikipedia: Option<String>,
    pub wikizilla: String,
    pub description: String,
}

#[async_trait]
impl EventHandler for Bot {
    async fn message(&self, ctx: Context, msg: Message) {
        let user_id = msg.author.id.0 as i64;

        // if let Some(task_description) = msg.content.strip_prefix("~todo add") {
        //     let task_description = task_description.trim();
        //     // That's how we are going to use a sqlite command.
        //     // We are inserting into the todo table, our task_description in task column and our user_id in user_Id column.
        //     sqlx::query!(
        //         "INSERT INTO todo (task, user_id) VALUES (?, ?)",
        //         task_description,
        //         user_id,
        //     )
        //     .execute(&self.database) // < Where the command will be executed
        //     .await
        //     .unwrap();

        //     let response = format!(
        //         "Successfully added `{}` to your todo list",
        //         task_description
        //     );
        //     msg.channel_id.say(&ctx, response).await.unwrap();
        // } else if let Some(task_index) = msg.content.strip_prefix("~todo remove") {
        //     let task_index = task_index.trim().parse::<i64>().unwrap() - 1;

        //     // "SELECT" will return to "entry" the rowid of the todo rows where the user_Id column = user_id.
        //     let entry = sqlx::query!(
        //         "SELECT rowid, task FROM todo WHERE user_id = ? ORDER BY rowid LIMIT 1 OFFSET ?",
        //         user_id,
        //         task_index,
        //     )
        //     .fetch_one(&self.database) // < Just one data will be sent to entry
        //     .await
        //     .unwrap();

        //     // Every todo row with rowid column = entry.rowid will be deleted.
        //     sqlx::query!("DELETE FROM todo WHERE rowid = ?", entry.rowid)
        //         .execute(&self.database)
        //         .await
        //         .unwrap();

        //     let response = format!("Successfully completed `{}`!", entry.task);
        //     msg.channel_id.say(&ctx, response).await.unwrap();
        // } else if msg.content.trim() == "~todo list" {
        //     // "SELECT" will return just the task of all rows where user_Id column = user_id in todo.
        //     let todos = sqlx::query!(
        //         "SELECT task FROM todo WHERE user_id = ? ORDER BY rowid",
        //         user_id
        //     )
        //     .fetch_all(&self.database) // < All matched data will be sent to todos
        //     .await
        //     .unwrap();

        //     let mut response = format!("You have {} pending tasks:\n", todos.len());
        //     for (i, todo) in todos.iter().enumerate() {
        //         writeln!(response, "{}. {}", i + 1, todo.task).unwrap();
        //     }

        //     msg.channel_id.say(&ctx, response).await.unwrap();
        if msg.content.trim() == "/gz" {
            msg.channel_id
                .say(&ctx, format!("SKREEONK!"))
                .await
                .unwrap();
        } else if msg.content.trim() == "/gz films" {
            let mut buffer_string = String::new();

            let films = sqlx::query!("SELECT * FROM films")
                .fetch_all(&self.database)
                .await
                .unwrap();

            for film in films {
                buffer_string.push_str(&format!(
                    "{}. {} ({})\n<{}>\n\n",
                    film.id, film.title, film.year, film.wikipedia
                ));
            }

            msg.channel_id.say(&ctx, buffer_string).await.unwrap();
        } else if msg.content.trim() == "/gz monsters" {
            // let mut buffer_string = String::new();

            // let monsters = sqlx::query!("SELECT * FROM monsters")
            //     .fetch_all(&self.database)
            //     .await
            //     .unwrap();

            // for monster in monsters {
            //     buffer_string.push_str(&format!("{}. {}\n", monster.id, monster.name));

            //     if let Some(wikipedia) = monster.wikipedia {
            //         buffer_string.push_str(&format!("<{}>\n", wikipedia));
            //     }
            // }

            // msg.channel_id.say(&ctx, buffer_string).await.unwrap();
        } else if msg.content.trim() == "/gz countdown" {
            let mut buffer_string = String::new();

            for i in (1..=10).rev() {
                msg.channel_id.say(&ctx, format!("{}", i)).await.unwrap();
                std::thread::sleep(std::time::Duration::from_millis(1500));
            }

            msg.channel_id.say(&ctx, "GOZILLA!").await.unwrap();
        } else if msg.content.trim() == "/gz help" {
            msg.channel_id.say(&ctx, HELP).await.unwrap();
        } else if let Some(year) = msg.content.strip_prefix("/gz year") {
            // println!("{}", year);

            // let year: i32 = year.trim().parse().unwrap();

            // let films = sqlx::query!("SELECT * FROM films WHERE year = ?", year)
            //     .fetch_all(&self.database) // < Where the command will be executed
            //     .await
            //     .unwrap();

            // for film in films {
            //     msg.channel_id.say(&ctx, film.title).await.unwrap();
            // }
        } else if let Some(film_id) = msg.content.strip_prefix("/gz next") {
            //     let film_id: i32 = film_id.trim().parse().unwrap();

            //     sqlx::query!("INSERT INTO watched (film_id) VALUES (?)", film_id)
            //         .execute(&self.database) // < Where the command will be executed
            //         .await
            //         .unwrap();

            //     let film = sqlx::query!(
            //         "SELECT films.title FROM watched INNER JOIN films ON watched.film_id = films.id ORDER BY watched.id DESC"
            //     )
            //     .fetch_one(&self.database) // < Where the command will be executed
            //     .await
            //     .unwrap();

            //     // for film in films {
            //     //     msg.channel_id.say(&ctx, film.title).await.unwrap();
            //     // }

            //     msg.channel_id.say(&ctx, &film.title).await.unwrap();
            // } else if let Some(monster_id) = msg.content.strip_prefix("/gz film_by_monster") {
            //     let monster_id: i32 = monster_id.trim().parse().unwrap();

            //     let films = sqlx::query!(
            //         "SELECT films.* FROM monsters_by_film INNER JOIN films ON monsters_by_film.film_id = films.id WHERE monsters_by_film.monster_id = ?", monster_id
            //     )
            //     .fetch_all(&self.database) // < Where the command will be executed
            //     .await
            //     .unwrap();

            //     for film in films {
            //         msg.channel_id.say(&ctx, film.title).await.unwrap();
            //     }

            // msg.channel_id.say(&ctx, &film.title).await.unwrap();
        } else if let Some(search_term) = msg.content.strip_prefix("/gz monster") {
            let search_term = search_term.trim();

            let monster_id: i64 = match search_term.parse::<i32>() {
                Ok(monster_id) => {
                    let monster = sqlx::query!("SELECT * FROM monsters WHERE id = ?", monster_id)
                        .fetch_one(&self.database) // < Where the command will be executed
                        .await
                        .unwrap();

                    let message = format!(
                        "**{}. {}**\n_AKA {}_\n{}\n<{}>\n\n",
                        monster.id,
                        monster.name,
                        monster.alternate_names,
                        monster.description,
                        monster.wikizilla
                    );

                    msg.channel_id.say(&ctx, &message).await.unwrap();

                    monster.id
                }
                Err(_) => {
                    let wildcard_term = format!("%{}%", search_term);

                    let monster = sqlx::query!(
                        "SELECT * FROM monsters WHERE name LIKE ? OR alternate_names LIKE ?",
                        wildcard_term,
                        wildcard_term
                    )
                    .fetch_one(&self.database) // < Where the command will be executed
                    .await
                    .unwrap();

                    let message = format!(
                        "**{}. {}**\n_AKA {}_\n{}\n<{}>\n\n",
                        monster.id,
                        monster.name,
                        monster.alternate_names,
                        monster.description,
                        monster.wikizilla
                    );

                    msg.channel_id.say(&ctx, &message).await.unwrap();

                    monster.id
                }
            };

            let films = sqlx::query!(
                "SELECT films.* FROM monsters_by_film INNER JOIN films ON monsters_by_film.film_id = films.id WHERE monsters_by_film.monster_id = ?", monster_id
            )
            .fetch_all(&self.database) // < Where the command will be executed
            .await
            .unwrap();

            let mut appears_in = "**Appears in:**\n".to_string();

            for film in films {
                appears_in.push_str(&format!(
                    "{}. {} ({})\n<{}>\n\n",
                    film.id, film.title, film.year, film.wikipedia
                ));
            }

            msg.channel_id.say(&ctx, &appears_in).await.unwrap();
        }
    }
}

impl Display for Film {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}. {} ({})", self.id, self.title, self.year)
    }
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    // Configure the client with your Discord bot token in the environment.
    let token = std::env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    // Initiate a connection to the database file, creating the file if required.
    let database = sqlx::sqlite::SqlitePoolOptions::new()
        .max_connections(5)
        .connect_with(
            sqlx::sqlite::SqliteConnectOptions::new()
                .filename("database.sqlite")
                .create_if_missing(true),
        )
        .await
        .expect("Couldn't connect to database");

    // Run migrations, which updates the database's schema to the latest version.
    sqlx::migrate!("./migrations")
        .run(&database)
        .await
        .expect("Couldn't run database migrations");

    let path = "./monsters.json";
    let data = std::fs::read_to_string(path).expect("Unable to read file");
    let monsters: Vec<Monster> = serde_json::from_str(&data).expect("Unable to parse");
    for monster in &monsters {
        sqlx::query!(
            "INSERT OR REPLACE INTO monsters (id, name, alternate_names, wikipedia, wikizilla, description) VALUES (?, ?, ?, ?, ?, ?)",
            monster.id,
            monster.name,
            monster.alternate_names,
            monster.wikipedia,
            monster.wikizilla,
            monster.description
        )
        .execute(&database) // < Where the command will be executed
        .await
        .unwrap();
    }

    let path = "./films.json";
    let data = std::fs::read_to_string(path).expect("Unable to read file");
    let films: Vec<Film> = serde_json::from_str(&data).expect("Unable to parse");
    for film in &films {
        sqlx::query!(
            "INSERT OR REPLACE INTO films (id, title, year, wikipedia) VALUES (?, ?, ?, ?)",
            film.id,
            film.title,
            film.year,
            film.wikipedia,
        )
        .execute(&database) // < Where the command will be executed
        .await
        .unwrap();

        for monster_id in &film.monsters {
            sqlx::query!(
                "INSERT OR REPLACE INTO monsters_by_film (monster_id, film_id) VALUES (?, ?)",
                monster_id,
                film.id
            )
            .execute(&database) // < Where the command will be executed
            .await
            .unwrap();
        }
    }

    let bot = Bot { database };

    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;
    let mut client = Client::builder(&token, intents)
        .event_handler(bot)
        .await
        .expect("Err creating client");
    client.start().await.unwrap();
}
