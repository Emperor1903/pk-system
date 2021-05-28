<template>
    <el-container>
        <el-container>
            <el-header style="text-align: left; font-size: 16px">
                <span>Danh sách bệnh viện</span>
            </el-header>
            <el-main>
                <HospitalTable />
            </el-main>
        </el-container>
    </el-container>
</template>

<style>
  .el-header {
    background-color: #B3C0D1;
    color: #333;
    line-height: 60px;
  }
  
  .el-aside {
    color: #333;
  }
</style>

<script>
export default ({
    name: "hospital",
    components: {
        HospitalTable: () => import("../components/HospitalTable.vue"),
    }
})
</script>