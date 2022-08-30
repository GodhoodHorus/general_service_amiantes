<template>
  <table class="table-auto border-collapse border border-blue w-full text-center">
    <thead>
      <tr>
        <th v-for="header in head_list" class="border border-blue py-1rem">{{ header }}</th>
      </tr>
    </thead>
    <tbody >
      <tr v-if="body_list" v-for="(items, index) in body_list" :key="index">
        <td v-if="index % 2 !== 0" v-for="item in items" class="border border-blue py-1rem bg-slate-200" >{{ item }}</td>
        <td v-if="index % 2 === 0"  v-for="item in items" class="border border-blue py-1rem" >{{ item }}</td>
      </tr>
    </tbody>
  </table>
</template>

<script>
export default {
  name: "DashboardTable",
  props: ['head_list', 'body_list'],
}
</script>
