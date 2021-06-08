<template>
<el-row type="flex" justify="center">
  <el-col>
    <span v-if="$route.params.id">
      <DoctorDetail :id="$route.params.id"/>
    </span>
    <el-header style="text-align: left; font-size: 24px">
      <span>Danh sách lịch trình</span>
    </el-header>
    <el-card shadow="never">
      <el-main>
        <ScheduleTable :id="$route.params.id"/>
      </el-main>
    </el-card>
  </el-col>
</el-row>
</template>

<style>
.el-header {
    background-color: #b3c0d1;
    color: #333;
    line-height: 60px;
}
</style>

<script>
export default {
    name: "schedule",
    components: {
        DoctorDetail: () => import("../components/DoctorDetail.vue"),
        ScheduleTable: () => import("../components/ScheduleTable.vue"),
    },
};
</script>
