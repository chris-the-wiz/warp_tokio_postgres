use std::sync::Arc;


//use test_log::test;

use reqwest::{  Client, Error};  // cookie::Jar,

use serde::{Deserialize, Serialize};

use crate::data::TodoResponse;

use anyhow::{Error  as AnyhowError };

use thiserror::Error as ThisError;


use crate::db_requests::store::Store;


#[derive(ThisError, Debug)]
enum TestError {
   // #[error("error creating request: {0}")]
   // DatabaseError()
    #[error("error creating request: {0}")]
    RequestError(#[from] reqwest::Error),
    #[error("error reading json : {0}")]
    JsonError(#[from] serde_json::Error),

    
}

fn setup_client() ->reqwest::Client{      
  // let jar = Arc::new(Jar::default());
    let client = reqwest::Client::builder()
    //.cookie_provider(Arc::clone(&jar))
    .build()
    .unwrap();
   
    
    return client;
}

struct StringStruct{
    s: String,
}

impl std::process::Termination for StringStruct{
    fn report(self) -> std::process::ExitCode{std::process::ExitCode::SUCCESS}
}



#[tokio::main]
#[test]
async fn todo_get_test()-> Result<(), Error>
{
   let client = setup_client();
   let todo = todo_get_inner(&client).await?;
 

   let deserialized:Vec<crate::data::TodoResponse> =  serde_json::from_str(&todo.s).unwrap_or_else(
        |err|    panic!("Error: {}", err)
   );

   //assert!(deserialized.len() >= 0);
    Ok(())
}
  
async fn todo_get_inner(client:&Client)-> Result<StringStruct, Error>
{
    let response = client.get("http://127.0.0.1:8000/todo").send().await?;
    let todo_list = response.text().await?;  
  
    let out: StringStruct = StringStruct{s: todo_list};

    Ok(out) 
}  

async fn todo_get_inner_id(client:&Client, id:i32)-> Result<StringStruct, Error>
{
    let response = client.get(format!("http://127.0.0.1:8000/todo/?id={}",id))
    .send()
    .await?;
    let todo_list = response.text().await?;  
  
    let out: StringStruct = StringStruct{s: todo_list};
    Ok(out) 
}  



#[tokio::main]
#[test] //add a record: check how many, add, check if more
async fn todo_post_test()-> Result<(), TestError>
{
   let client = setup_client();
    //get the number of records
    let mut  todo = todo_get_inner(&client).await?;
    
    let mut deserialized:Vec<crate::data::TodoResponse> =  serde_json::from_str(&todo.s).unwrap_or_else(
        |err|    panic!("Error: {}", err)
   );

   let pre_count = deserialized.len();



    todo_post_inner(&client).await?;
    
    todo = todo_get_inner(&client).await?;

    deserialized =  serde_json::from_str(&todo.s).unwrap_or_else(
        |err|    panic!("Error: {}", err)
   );

   let post_count = deserialized.len();

   assert!(pre_count<post_count);



    Ok(())
}

async fn todo_post_inner(client:&Client)-> Result<TodoResponse, TestError>
{

    let response = client.post("http://127.0.0.1:8000/todo")
    .body("{\"name\": \"Done Todo\", \"checked\": true}")
    .send().await?;
    let todo_resp = response.text().await?;  
    let out:TodoResponse =  serde_json::from_str(&todo_resp)?;//.unwrap_or_else(
      //  |err| panic!("{}",err)
    // );
     
   // let out: StringStruct = StringStruct{s: todo_list};
    Ok(out)
   
}  



#[tokio::main]
#[test] // modify record: add a record. modify check if differs 
async fn todo_put_test()-> Result<(), TestError>
{
   let client = setup_client();
    //get the number of records
   

    let mut resp_reversed   = todo_post_inner(&client).await?;
    let resp  = resp_reversed.clone();
    let a = todo_get_inner_id(&client, resp.id).await?;
    let deserialized_a:Vec<crate::data::TodoResponse> =  serde_json::from_str(&a.s).unwrap_or_else(
        |err|    panic!("Error: {}", err)
   );

    resp_reversed.checked = !resp_reversed.checked;

    todo_put_inner(&client, resp.clone()).await?;
    let b = todo_get_inner_id(&client, resp.id).await?;
    let deserialized_b:Vec<crate::data::TodoResponse> =  serde_json::from_str(&b.s).unwrap_or_else(
        |err|    panic!("Error: {}", err)
   );
   
    assert!(deserialized_a[0].id==deserialized_b[0].id);
    assert!(deserialized_a[0].name!=deserialized_b[0].name);
    assert!(deserialized_a[0].checked!=deserialized_b[0].checked);




    Ok(())
}

async fn todo_put_inner(client:&Client, req:TodoResponse)-> Result<TodoResponse, TestError>
{
    let new_bool = !req.checked;
    let response = client.put(format!("http://127.0.0.1:8000/todo/{}", req.id) )
    .body(format!("{{\"name\": \"different string\", \"checked\":{} }}",  new_bool))
    .send().await?;
    let todo_resp = response.text().await?;  
    let out:TodoResponse =  serde_json::from_str(&todo_resp)?;//.unwrap_or_else(
      //  |err| panic!("{}",err)
    // );
     
   // let out: StringStruct = StringStruct{s: todo_list};
    Ok(out)
   
}  



// delete record but: check number, if 0 add one. check number. delete check if less

#[tokio::main]
#[test] // modify record: add a record. modify check if differs 
async fn todo_delete_test()-> Result<(), TestError>
{
   let client = setup_client();
    //get the number of records
   

    let resp   = todo_post_inner(&client).await?;
    let id = resp.id;
    let a = todo_get_inner(&client).await?;
    let deserialized_a:Vec<crate::data::TodoResponse> =  serde_json::from_str(&a.s).unwrap_or_else(
        |err|    panic!("Error: {}", err)
    );
    let len_a = deserialized_a.len();


    let del_resp = client.delete(format!("http://127.0.0.1:8000/todo/{}", id) )
    .send().await?; 

    let b = todo_get_inner(&client).await?;
    let deserialized_b:Vec<crate::data::TodoResponse> =  serde_json::from_str(&b.s).unwrap_or_else(
        |err|    panic!("Error: {}", err)
    );

    let len_b = deserialized_b.len();

    assert!(len_a>len_b);
 

    Ok(())
}




/////////////////
/// 
/// 
#[tokio::main]
#[test] // modify record: add a record. modify check if differs 
async fn stores_graphql_query_test()-> Result<(), TestError>
{
   let client = setup_client();
    //get the number of records
   

    let mut resp   =stores_graphql_query_inner(&client).await?;
  
   

    
   
   // assert!(deserialized_a[0].id==deserialized_b[0].id);
    //assert!(deserialized_a[0].name!=deserialized_b[0].name);
   // assert!(deserialized_a[0].checked!=deserialized_b[0].checked);




    Ok(())
}

async fn stores_graphql_query_inner(client:&Client)
//-> Result<StringStruct, TestError>
-> Result<(), TestError>
{
   
    let response = client.put("http://127.0.0.1:8000/stores/" )
    .body(
        "query {
            getAllStores{ id, name, clients}    
        }")
    .send().await?;
    let resp = response.text().await?;  
   // let out:Store = async_graphql_warp::GraphQLResponse::from(resp);
     
    //let out: StringStruct = StringStruct{s: out};
   // Ok(out)
   Ok(())
   
}  


