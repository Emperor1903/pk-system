<template>
<el-row type="flex" justify="center">
  <el-col>
    <span v-if="id">
      <HospitalDetail :id="id"/>
    </span>
    <el-header style="text-align: left; font-size: 24px">
      <span>Danh sách phòng khám</span>
    </el-header>
    <el-card shadow="never">
      <el-main>
        <ClinicTable :id="id"/>
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
    name: "hospital",
    components: {
        HospitalDetail: () => import("../components/HospitalDetail.vue"),
        ClinicTable: () => import("../components/ClinicTable.vue"),
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
