use async_graphql::{SimpleObject, Object, Context, Error, EmptySubscription, Schema};
use aws_sdk_dynamodb::Client;
use crate::db::{add_todo, get_all_todos_for_user, mark_complete, get_all_todos, TodoItem};
use itertools::Itertools;


#[derive(SimpleObject)]
struct User {
    username: String,
    todos: Vec<Todo>,
}

#[derive(SimpleObject)]
struct Todo {
    task: String,
    completed: bool,
}

impl From<TodoItem> for Todo {
    fn from(todo: TodoItem) -> Self {
        Self {
            task: todo.task,
            completed: todo.completed,
        }
    }
}

pub struct Query;

#[Object]
impl Query {
    async fn user<'ctx>(&self, ctx: &Context<'ctx>, username: String) -> Result<User, Error> {
        let dynamo_client = ctx.data::<Client>()?;
        let users_todos = get_all_todos_for_user(dynamo_client, username.clone()).await?;
        Ok(
            User {
                username,
                todos: users_todos.into_iter().map(|todo| todo.into()).collect(),
            }
        )
    }
    async fn users<'ctx>(&self, ctx: &Context<'ctx>) -> Result<Vec<User>, Error> {
        let dynamo_client = ctx.data::<Client>()?;
        let all_todos = get_all_todos(dynamo_client).await?;

        let users: Vec<User> = all_todos.iter()
            .group_by(|todo| todo.username.clone())
            .into_iter()
            .map(|(username, todos)| {
                User {
                    username,
                    todos: todos.map(|todo| todo.clone().into()).collect(),
                }
            }).collect();
        Ok(users)
    }
}

pub struct Mutation;

#[derive(SimpleObject)]
struct CompleteResult {
    username: String,
    task: String,
}

#[Object]
impl Mutation {
    async fn add_todo<'ctx>(&self, ctx: &Context<'ctx>, username: String, task: String) -> Result<Todo, Error> {
        let dynamo_client = ctx.data::<Client>()?;
        let created_todo = add_todo(dynamo_client, username, task).await?;
        Ok(created_todo.into())
    }

    async fn mark_todo_complete<'ctx>(&self, ctx: &Context<'ctx>, username: String, task: String) -> Result<CompleteResult, Error> {
        let dynamo_client = ctx.data::<Client>()?;
        mark_complete(dynamo_client, username.clone(), task.clone()).await?;
        Ok(CompleteResult { username, task })
    }
}

/**
 * This function is used to build the schema for our GraphQL API, loading the DynamoDB client 
 * from the environment and passing it to the schema as context data.
 */
pub async fn build_schema() -> Schema<Query, Mutation, EmptySubscription> {
    let config = aws_config::load_from_env().await;
    let dynamodb_client = Client::new(&config);
    Schema::build(Query, Mutation, EmptySubscription).data(dynamodb_client).finish()
}

