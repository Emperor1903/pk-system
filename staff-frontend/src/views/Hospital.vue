<template>
  <el-container>
    <el-container>
      <el-header style="text-align: left; font-size: 24px">
        <span>Danh sách bệnh viện</span>
      </el-header>
      <el-card shadow="never">
        <el-main>
          <HospitalTable />
        </el-main>
      </el-card>
    </el-container>
  </el-container>
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
  name: "hospital",
  components: {
    HospitalTable: () => import("../components/HospitalTable.vue"),
  },
};
</script>
