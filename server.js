const express = require('express');
const { graphqlHTTP } = require('express-graphql');
const { buildSchema } = require('graphql');
const { hello, add } = require('./resolvers');
const { get } = require('lodash/fp');

const schema = buildSchema(`
  type Query {
    hello: String,
    add(x: Float, y: Float): String,
  }
`);

const root = {
  hello: () => {
    return hello();
  },
  add: (obj, args, context, info) => {
    const x = get('x', obj);
    const y = get('y', obj);
    return add(x, y);
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
