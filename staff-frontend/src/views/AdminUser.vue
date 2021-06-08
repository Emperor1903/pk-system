<template>
  <el-container>
    <el-container>
      <el-header style="text-align: left; font-size: 24px">
        <span>Danh sách quản trị viên hệ thống</span>
      </el-header>
      <el-card shadow="never">
        <el-main>
          <AdminTable />
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
  name: "admin",
  components: {
    AdminTable: () => import("../components/AdminTable.vue"),
  },
};
</script>
