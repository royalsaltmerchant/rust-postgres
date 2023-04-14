use dotenv;
use sqlx::{postgres, Connection, PgConnection};
mod objects;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Args {
    #[structopt(subcommand)]
    cmd: Option<Command>,
}

struct AddParams {
    name: String,
    age: i32,
    class: String,
    race: String,
}

#[derive(StructOpt)]
enum Command {
    Add {
        name: String,
        age: i32,
        class: String,
        race: String,
    },
}

#[tokio::main]
async fn main() {
    let my_path = "./.env";
    dotenv::from_path(my_path).unwrap();

    // DB
    let database_url = dotenv::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let mut conn = postgres::PgConnection::connect(&database_url)
        .await
        .unwrap();

    // read args
    let args = Args::from_args_safe().unwrap();

    match args.cmd {
        Some(Command::Add {
            name,
            age,
            class,
            race,
        }) => {
            let params = AddParams {
                name,
                age,
                class,
                race,
            };

            println!("Creating new character");
            add_character(&mut conn, params).await
        }

        None => list_characters(&mut conn).await,
    }

    // write

    // let write_monster_query =
    //     sqlx::query("insert into monster (name, kind, hp) values ($1,$2,$3);")
    //         .bind(first_monster.name)
    //         .bind(first_monster.kind)
    //         .bind(first_monster.hp)
    //         .execute(&mut conn)
    //         .await
    //         .unwrap();
    // println!("query result: {:?}", write_monster_query);
}

async fn list_characters(pool: &mut PgConnection) {
    // read
    let characters = sqlx::query!("select * from character;")
        .fetch_all(pool)
        .await
        .unwrap();
    for character in characters {
        println!("{:?}", character);
    }
}

async fn add_character(pool: &mut PgConnection, params: AddParams) {
    let new_character = objects::Character::new(params.name, params.age, params.class, params.race);
    let write_character_query = sqlx::query(
        "insert into character (name, age, class, race, hp, exp) values ($1,$2,$3,$4,$5,$6);",
    )
    .bind(new_character.name)
    .bind(new_character.age)
    .bind(new_character.class)
    .bind(new_character.race)
    .bind(new_character.hp)
    .bind(new_character.exp)
    .execute(pool)
    .await
    .unwrap();
    println!("query result: {:?}", write_character_query);
}
