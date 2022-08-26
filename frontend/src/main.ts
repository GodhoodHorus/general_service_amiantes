import { createApp, provide, h } from 'vue'
import './style.css'
import App from './App.vue'
import { ApolloClient, InMemoryCache, HttpLink } from '@apollo/client/core'
import { DefaultApolloClient, provideApolloClient } from '@vue/apollo-composable'
import { router } from './VueRouting'
import 'virtual:windi.css'

const cache = new InMemoryCache()

const httpLink = new HttpLink({
  uri: 'http://localhost:5050/graphql'
})

const apolloClient = new ApolloClient({
  link: httpLink,
  cache,
})

const app = createApp({
  setup () {
    provideApolloClient(apolloClient)
  },
  render: () => h(App),
})

app
  .use(router)
  .mount('#app')
  
