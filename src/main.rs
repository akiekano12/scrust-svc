extern crate mysql;

#[derive(Debug, PartialEq, Eq)]
struct Person {
    person_id: i32,
    person_name: String
}

fn main() {
    let pool = mysql::Pool::new("mysql://root:@localhost:3306/rust").unwrap();
    
    let all_persons: Vec<Person> = 
    pool.prep_exec("SELECT person_id, person_name from person", ())
    .map(|result| {
        result.map(|x| x.unwrap()).map(|row| {
            let (person_id, person_name) = mysql::from_row(row);
            
            Person {
                person_id,
                person_name
            }
        }).collect()
    }).unwrap(); // Unwrap `Vec<Person>`
    
    println!("ID    |    Name");
    for person in all_persons.iter() {
        println!("{}     |    {}", person.person_id, person.person_name);
    }
    
}