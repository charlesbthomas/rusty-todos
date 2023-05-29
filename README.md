# Rusty Todos 🦀

A fun little experiment for to help me brush up on my Rust.

This is a simple little GraphQL ToDo Api built with:

- Rust (obviously)
- [Async Graphql](https://github.com/async-graphql/async-graphql)
- AWS Lambda (via SAM and cargo-lambda)
- DynamoDB
- [serde_dynamo](https://github.com/zenlist/serde_dynamo) 🙂

![diagram](https://github.com/charlesbthomas/rusty-todos/assets/20322135/add5847c-71dd-4fe2-9227-4f11eebef9cd)


## To Deploy to a test AWS Account 📦
1) Make sure you have an aws profile configured via: `aws configure` (You will need access creds configured on your profile).
2) In the root of this repo: `sam build`
3) `sam deploy` (Follow the prompts)
4) Query Away

## If I have Time:
- [ ] Refactor the graphql error handling to not be a hack... 🙈
- [ ] Learn the best way to unit test this kind of code 🧪
- [ ] Learn about better dependency injection practices with Rust 🏗️

