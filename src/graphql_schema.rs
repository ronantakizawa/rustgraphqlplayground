// graphql_schema.rs
#[macro_use]
use juniper::{EmptyMutation, RootNode, FieldResult};
use serde_json::{Result, Value};
use std::fs::File;
use std::io::prelude::*;
  
struct Member {
  first_name: String,
  last_name: String,
  email:String,
  password:String
}



#[juniper::object(description = "A member of a team")]
    impl Member {
      pub fn first_name(&self) -> &str {
        self.first_name.as_str()
      }
      pub fn last_name(&self) -> &str {
        self.last_name.as_str()
      }
      pub fn email(&self) -> &str {
        self.email.as_str()
      }
      pub fn password(&self) -> &str {
        self.password.as_str()
      }
    }



pub struct QueryRoot;


#[juniper::object]
impl QueryRoot {
  fn members() -> Vec<Member> {
    let mut file = File::open("/Users/ronantakizawa/Documents/Personal CS projects/rust/rustpractice/src/data.json").expect("Could not read file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Could not read file");
    let data:Value = serde_json::from_str(&contents).expect("Could not read file");
    let mut num_members: usize = data.as_array().unwrap().len();
    let mut vec:Vec<Member> = Vec::new();
    for n in 0..num_members{
      let new_member: Member = Member {
        first_name: data[n]["first_name"].to_string().replace("\"", "").to_owned(),
        last_name:data[n]["last_name"].to_string().replace("\"", "").to_owned(),
        email: data[n]["email"].to_string().replace("\"", "").to_owned(),
        password:data[n]["password"].to_string().replace("\"", "").to_owned(),
      };
      vec.push(new_member);
    }
    return vec;
    
}

}



pub type Schema = RootNode<'static, QueryRoot, EmptyMutation<()>>;
    
pub fn create_schema() -> Schema {
  Schema::new(QueryRoot {}, EmptyMutation::new())
}