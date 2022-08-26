<script lang="ts">
  import { ref } from 'vue'

  export default {
    name: 'DashboardButton',
    props: ['value']
  }
</script>

<template>
  <button class="text-white py-1rem self-center">{{ value }}</button>
</template>