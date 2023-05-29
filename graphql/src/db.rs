use std::{env, collections::HashMap};
use lambda_runtime::Error;
use aws_sdk_dynamodb::{Client, types::AttributeValue};
use serde::{Serialize, Deserialize};
use serde_dynamo::{from_items, to_item};

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
pub struct TodoItem {
    pub task: String,
    pub completed: bool,
    pub username: String
}

pub async fn get_all_todos(client: &Client) -> Result<Vec<TodoItem>, Error> {
    let table_name = env::var("TABLE_NAME").expect("TABLE_NAME must be set");

    let result = client.scan().table_name(&table_name).send().await?;

    let items = result.items().map(|slice| slice.to_vec()).unwrap();
    let todos: Vec<TodoItem> = from_items(items)?;
    Ok(todos)
}

pub async fn get_all_todos_for_user(client: &Client, username: String) -> Result<Vec<TodoItem>, Error> {
    let table_name = env::var("TABLE_NAME").expect("TABLE_NAME must be set");

    let result = client.query().table_name(&table_name)
        .key_condition_expression("#un = :username")
        .expression_attribute_names("#un", "username")
        .expression_attribute_values(":username", AttributeValue::S(username))
        .send().await?;

    let items = result.items().map(|slice| slice.to_vec()).unwrap();
    let todos: Vec<TodoItem> = from_items(items)?;
    Ok(todos)
}

pub async fn mark_complete(client: &Client, username: String, task: String, ) -> Result<(), Error> {
    let table_name = env::var("TABLE_NAME").expect("TABLE_NAME must be set");

    client.update_item().table_name(table_name)
        .key("username", AttributeValue::S(username))
        .key("task", AttributeValue::S(task))
        .update_expression("SET #c = :completed")
        .expression_attribute_names("#c", "completed")
        .expression_attribute_values(":completed", AttributeValue::Bool(true))
        .send().await?;

    Ok(())
}

pub async fn add_todo(client: &Client, username: String, task: String) -> Result<TodoItem, Error> {
    let table_name = env::var("TABLE_NAME").expect("TABLE_NAME must be set");

    let todo = TodoItem {
        task,
        completed: false,
        username
    };

    let item = to_item::<TodoItem, HashMap<String, AttributeValue>>(todo.clone())?;
    client.put_item().table_name(table_name).set_item(Some(item)).send().await?;
    Ok(todo)
}
