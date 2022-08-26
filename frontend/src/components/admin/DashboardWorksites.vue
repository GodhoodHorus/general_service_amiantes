<template>
  <div class="flex flex-col h-screen">
    <dashboard-header />

    <div class="flex flex-row h-full">
      <dashboard-aside focusFor="Dossier" />
      <dashboard-main>
        Creation de dossier
        <dashboard-button type="submit" class="bg-lightblue" value="Home" />
      </dashboard-main>
    </div>

  </div>
</template>


<script lang="ts">
import DashboardButton from "./templates/DashboardButton.vue";
import DashboardHeader from "./templates/DashboardHeader.vue";
import DashboardAside from "./templates/DashboardAside.vue";
import DashboardMain from "./templates/DashboardMain.vue";

export default {
  name: "DashboardWorksites",
  components: { DashboardButton, DashboardHeader, DashboardAside, DashboardMain }
}
</script>

<style scoped>

</style>