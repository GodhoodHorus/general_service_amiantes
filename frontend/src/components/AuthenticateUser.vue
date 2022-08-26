<script lang="ts" src>
  import { useMutation } from '@vue/apollo-composable'
  import { ref, h } from 'vue'
  import { AUTHENTICATE_USER } from '../graphql/AuthenticateUser.mutation'
  import { router } from '../VueRouting'

  

  export default {
    name: "AuthenticateUser",
    setup() {
      if (sessionStorage.hasOwnProperty('user_id')) {
        router.push('/admin/dashboard')
      }
    },
    data() {
      return {
        title: 'Authenticate User',
        loading: false,
        submitted: false,
        name: ref(''),
        password: ref(''),
        graphql_error: ref(),
        network_error: ref()
      }
    },
    methods: 
    {
      async handleAuthenticateUser () {

        const { mutate, error, onError, onDone }  = useMutation(
          AUTHENTICATE_USER, 
          { 
            errorPolicy: "all",
            variables: {
              name: this.$data.name,
              password: this.$data.password,
            },
          }
        )

        await onDone(result => {
          sessionStorage.setItem('user_id', result.data.authenticateUser.id);
          sessionStorage.setItem('user_authorization', result.data.authenticateUser.authorization[0].level);

          router.push('/admin/dashboard')
        })

        await onError(error => {
          this.$data.network_error = error.networkError?.message
        })

        let authenticateUser = await mutate()
          .then((result) => {

            if (result?.errors !== undefined) {
            
              this.$data.graphql_error = result!.errors?.map(({ message }, i) => (
                { message }
              ))
            }
          })
      }
    }
  }
</script>`



<template>
  <div class="flex flex-col h-screen bg-dark">
    <header class="max-h-60px">
        <div class="logo">
            <img src="/images/logo_client.png" alt="App Logo" class="w-60px h-60px ml-3" />
        </div>
    </header>

    <div @submit.prevent="handleAuthenticateUser" class='flex place-content-center place-items-center h-full'>
        <form class="flex flex-col w-300px md:w-500px" method="post">
            <h1 class='text-20pt text-white font-mclaren text-center mb-4rem'>Access Dashboard</h1>
            <div v-if="graphql_error" v-for="error in graphql_error" class='bg-red-500 mb-1rem text-white py-1rem'><span class="block text-center">{{ error.message }}</span></div>
            <div v-if="network_error" class='bg-red-500 mb-1rem text-white py-1rem'><span class="block text-center">{{ network_error }}</span></div>
            <input
              v-model="name"
              id="name"
              name="name"
              type="text"
              placeholder="username"
              class='bg-black mb-1rem text-white px-5px'
            />
            <input
              v-model="password"
                id="password"
                name="password"
                type="password"
                placeholder="***********"
                class='bg-black mb-1rem text-white px-5px'
            />
            <button class='bg-green text-white py-1rem w-3/4 self-center mt-1rem'>Authenticate</button>
        </form>
    </div>
  </div>
</template>