<script lang="ts">
  import DashboardButton from './templates/DashboardButton.vue'
  import DashboardHeader from "./templates/DashboardHeader.vue";
  import DashboardAside from "./templates/DashboardAside.vue";
  import DashboardMain from "./templates/DashboardMain.vue";

  export default {
    name: "DashboardHome",
    components: { DashboardButton, DashboardHeader, DashboardAside, DashboardMain }
}
</script>

<template>
  <div class="flex flex-col h-screen">
  <dashboard-header />

    <div class="flex flex-row h-full">
      <dashboard-aside focusFor="Acceuil" />
      <dashboard-main>
        Dashboard Acceuil
      </dashboard-main>
    </div>

  </div>
</template>
