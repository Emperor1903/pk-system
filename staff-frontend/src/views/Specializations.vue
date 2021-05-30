<template>
  <el-container>
    <el-container>
      <el-header style="text-align: left; font-size: 24px">
        <span>Danh sách chuyên môn</span>
      </el-header>
      <el-card shadow="never">
        <el-main><SpecializationTable /> </el-main>
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

.el-aside {
  color: #333;
}
</style>

<script>
export default {
  name: "Specializations",
  components: {
    SpecializationTable: () => import("../components/SpecializationTable.vue"),
  },
};
</script>
