const express = require('express');
const { graphqlHTTP } = require('express-graphql');
const { buildSchema } = require('graphql');
const { hello, add } = require('./resolvers');

const schema = buildSchema(`
  type Query {
    hello: String,
    add: String,
  }
`);

const root = {
  hello: () => {
    return hello();
  },
  add: () => {
    return add();
  },
};

const app = express();

app.use('/graphql', graphqlHTTP({
  schema: schema,
  rootValue: root,
  graphiql: true,
}));

app.listen(4000);

console.log('Running a GraphQL API server at http://localhost:4000/graphql');
