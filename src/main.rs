use sqlx::{postgres, Connection};
mod objects;

#[tokio::main]
async fn main() {
    println!("***** Wyrld *****\n\n");

    let first_character = objects::Character::new(
        String::from("Jules"),
        22,
        String::from("Warrior"),
        String::from("Human"),
    );

    let first_monster = objects::Monster::new(String::from("Blork"), String::from("Mud"));

    println!(
        "First character: {}\nFirst Monster: {}",
        first_character.name, first_monster.name
    );

    // DB
    let mut conn = postgres::PgConnection::connect("postgresql://localhost/postgres")
        .await
        .unwrap();

    let query_result = sqlx::query(
        "insert into character (name, age, class, race, hp, exp) values ($1,$2,$3,$4,$5,$6);",
    )
    .bind(first_character.name)
    .bind(first_character.age)
    .bind(first_character.class)
    .bind(first_character.race)
    .bind(first_character.hp)
    .bind(first_character.exp)
    .execute(&mut conn)
    .await
    .unwrap();
    println!("query result: {:?}", query_result);

    let query_result = sqlx::query("insert into monster (name, kind, hp) values ($1,$2,$3);")
        .bind(first_monster.name)
        .bind(first_monster.kind)
        .bind(first_monster.hp)
        .execute(&mut conn)
        .await
        .unwrap();
    println!("query result: {:?}", query_result);

    println!("**** FINISHED ****")
}
