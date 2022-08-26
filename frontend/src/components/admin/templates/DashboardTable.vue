<template>
  <table class="table-auto border-collapse border border-blue w-full text-center">
    <thead>
      <tr>
        <th v-for="header in head_list" class="border border-blue py-1rem">{{ header }}</th>
      </tr>
    </thead>
    <tbody v-if="body_list" v-for="(items, index) in body_list" :key="index">
      <tr v-if="index % 2 !== 0" class="bg-slate-200">
        <td v-for="item in items" class="border border-blue py-1rem" >{{ item }}</td>
      </tr>
      <tr v-if="index % 2 === 0" >
        <td v-for="item in items" class="border border-blue py-1rem" >{{ item }}</td>
      </tr>
    </tbody>
  </table>
</template>

<script>
export default {
  name: "DashboardTable",
  props: ['head_list', 'body_list']
}
</script>

<style scoped>

</style>