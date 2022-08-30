<template>
  <div class="flex flex-col h-screen">
    <dashboard-header />

    <div class="flex flex-row h-full">
      <dashboard-aside focusFor="Clients" />
      <dashboard-main class="relative">
        <div class="flex flex-col">
          <div class="actions flex flex-row justify-end">
            <dashboard-button type="click" id="search_client" class="bg-blue w-300px mr-1rem"  value="Search" />
            <dashboard-button @click.prevent="openClientPopUp" type="click" id="creat_new_client" class="bg-dark w-300px"  value="Create" />
          </div>
          <div  class="table-data mt-4rem">
            <table class="table-auto border-collapse border border-blue w-full text-center">
              <thead>
              <tr>
                <th v-for="header in header_client_table" class="border border-blue py-1rem">{{ header }}</th>
              </tr>
              </thead>
              <tbody >
              <tr v-if="clients" v-for="(items, index) in clients" :key="index">
                <td v-if="index % 2 !== 0" v-for="item in items" class="border border-blue py-1rem bg-slate-200" >{{ item }}</td>
                <td v-if="index % 2 === 0"  v-for="item in items" class="border border-blue py-1rem">{{ item }}</td>
              </tr>
              </tbody>
            </table>
          </div>
        </div>

        <div id="new-client-pop-up" class="absolute top-0 left-0 h-full w-full bg-slate-900/80 flex place-content-center place-items-center invisible">
          <div class="bg-white flex flex-col w-min-500px h-3/4">
            <span @click.prevent="closeClientPopUp" class="self-end m-1rem">close</span>
            <h2 class="text-center my-1rem text-16pt">Create Client</h2>
            <div v-if="create_client_network_error" class='bg-red-500 mb-1rem text-white py-1rem'><span class="block text-center">{{ create_client_network_error }}</span></div>
            <form @submit.prevent="handleNewClient" class="flex flex-col">

              <div class="mx-0.5rem">
                <input
                    v-model="create_client_name"
                    type="text"
                    placeholder="name"
                    class="w-12/12 mb-0.5rem"
                >
              </div>


              <div class="flex flex-row mb-0.5rem">
                <input
                    v-model="create_client_address.street"
                    type="text"
                    placeholder="street"
                    class="w-1/2 mx-0.5rem"
                >
                <input
                    v-model="create_client_address.streetNumber"
                    type="number"
                    placeholder="street number"
                    class="w-1/2 mx-0.5rem"
                >
              </div>

              <button class='bg-teal-600 text-white py-1rem w-3/4 self-center mt-1rem'>Submit</button>
            </form>
          </div>
        </div>
      </dashboard-main>
    </div>

  </div>
</template>

<script>
import DashboardButton from "./templates/DashboardButton.vue";
import DashboardHeader from "./templates/DashboardHeader.vue";
import DashboardAside from "./templates/DashboardAside.vue";
import DashboardMain from "./templates/DashboardMain.vue";
import DashboardTable from "./templates/DashboardTable.vue";
import {ref} from "vue";
import {useMutation, useQuery} from "@vue/apollo-composable";
import {CREATE_CLIENT, GET_CLIENTS} from "../../graphql/Client.ts";



export default {
  name: "DashboardClients",
  components: { DashboardButton, DashboardHeader, DashboardAside, DashboardMain, DashboardTable },
  mounted() {
    if (isNaN(this.$route.query.offset))
      this.offset = 0
    else
      this.offset = Number(this.$route.query.offset)

    const variables = ref({
      offset: this.offset
    })

    const {onResult} = useQuery(
        GET_CLIENTS,
        variables
    )

    onResult(result => {
      return this.clients = this.propagateClient(result.data.clients)
    })
  },
  data() {
    return {
      clients: [],
      offset: ref(0),
      header_client_table: [
          'id',
          'name',
          'address',
          'created_at',
          'edited_at',
      ],
      create_client_name: ref(''),
      create_client_address: {street: '', streetNumber: 0},
      create_client_interlocutors: ref([]),
      create_client_created_at: ref(''),
      create_client_edited_at: ref(''),
      create_client_network_error: ref('')
    }
  },
  methods: {
    openClientPopUp() {
      let popup = document.getElementById('new-client-pop-up')

      popup.classList.replace('invisible', 'visible')
    },
    closeClientPopUp() {
      let popup = document.getElementById('new-client-pop-up')

      popup.classList.replace('visible', 'invisible')

    },
    async handleNewClient() {
      if (this.create_client_address.street === '' && this.create_client_address.streetNumber === 0)
        return null

      const { mutate, onError }  = useMutation(
          CREATE_CLIENT,
          {
            errorPolicy: "all",
            variables: {
              name: this.create_client_name,
              street: this.create_client_address.street,
              streetNumber: this.create_client_address.streetNumber,
            },
          }
      )

      let client = await mutate().then((result) => {
        if (result.errors) {
          return this.create_client_network_error = result.errors[0]?.message
        }

        this.closeClientPopUp()

        return this.formatClientResult(result.data.createClient)
      })

      this.clients.splice(this.clients.length, 0, client[0])

      onError(error => {
        return this.create_client_network_error = error.networkError?.message
      })

    },

    propagateClient(data) {
      let temp_clients = []

      for (let i = 0; i < data.length; i++) {
        temp_clients.push([
          data[i].id,
          data[i].name,
          data[i].address.streetNumber + ', ' +  data[i].address.street,
          data[i].createdAt,
          data[i].editedAt,
        ])
      }

      console.table(data)

      return temp_clients
    },

    formatClientResult(data) {
      let temp_clients = []

      temp_clients.push([
        data.id,
        data.name,
        data.address.streetNumber + ', ' +  data.address.street,
        data.createdAt,
        data.editedAt,
      ])

      return temp_clients
    }
  }
}
</script>

<style scoped>

</style>