<script lang="ts">

export default {
  name: 'DashboardAside',
  props: ['focusFor'],
  data() {
    return {
      links: [
        ['Acceuil', '/admin/dashboard'],
        ['Clients', '/admin/clients'],
        ['Dossier', '/admin/worksites'],
        ['Administration', '/admin/security'],
      ]
    }
  }
}
</script>

<template>
  <div class="w-2/12 bg-blue">
    <ul v-for="link in links" class="list-none my-0 p-0">
      <li v-if="link[0] === focusFor" class="bg-white text-blue text-center py-2rem"><a :href="link[1]">{{ link[0] }}</a></li>
      <li v-if="link[0] !== focusFor" class="text-white text-center py-2rem"><a :href="link[1]">{{ link[0] }}</a></li>
    </ul>
  </div>
</template>