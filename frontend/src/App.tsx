import React, { Component, ReactNode } from 'react'
import User, { AuthenticateUser }  from './models/User'
import Header from './templates/Header'
import { ApolloClient, ApolloProvider, InMemoryCache } from '@apollo/client';

const client = new ApolloClient({
  cache: new InMemoryCache(),
  uri: "http://localhost:5050/graphql",
});



export default function App() {
  return (
    <ApolloProvider client={client}>
      <Header />
      <AuthenticateUser />
    </ApolloProvider>
  );
}