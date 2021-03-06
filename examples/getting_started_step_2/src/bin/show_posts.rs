extern crate diesel_demo_step_2;
extern crate diesel;

use diesel_demo_step_2::*;
use diesel_demo_step_2::models::*;
use diesel::prelude::*;

fn main() {
    use diesel_demo_step_2::schema::posts::dsl::*;

    let connection = establish_connection();
    let results = posts.filter(published.eq(true))
        .limit(5)
        .load::<Post>(&connection)
        .expect("Error loading posts");

    println!("Displaying {} posts", results.len());
    for post in results {
        println!("{}", post.title);
        println!("-----------\n");
        println!("{}", post.body);
    }
}
