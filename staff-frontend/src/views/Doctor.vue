<template>
<el-row type="flex" justify="center">
  <el-col>
    <span v-if="id">
      <ClinicDetail :id="id"/>
    </span>
    <el-header style="text-align: left; font-size: 24px">
      <span>Danh sách bác sĩ</span>
    </el-header>
    <el-card shadow="never">
      <el-main>
        <DoctorTable :id="id"/>
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
    name: "clinic",
    components: {
        ClinicDetail: () => import("../components/ClinicDetail.vue"),
        DoctorTable: () => import("../components/DoctorTable.vue"),
    },
    data() {
        return {
            id: null,
        }
    },
    mounted() {
        this.id = this.$route.params.id;
    },
};
</script>
