<template>
<el-row type="flex" justify="center">
  <el-col>   
    <el-header style="text-align: left; font-size: 24px">
      <span>Danh sách ca trực</span>
    </el-header>
    
    <el-card shadow="never">
      <el-button round type="primary" @click="autoCreateShiftState.visible = true">
        Tạo ca cho tuần sau
      </el-button> 
      <el-button round @click="filterShiftState.visible = true">
        Lọc kết quả
      </el-button> 

      <el-main>
        <ShiftTable :id="id" :reload="reload" @reloaded="handleReloaded"/>
      </el-main>
    </el-card>
  </el-col>

  <AutoCreateShiftDialog :state="autoCreateShiftState" @confirm="handleCreateShiftConfirm"/>
  <FilterShiftDialog :state="filterShiftState" @confirm="handleFilterShiftConfirm"/>  
  
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
    name: "shift",
    components: {
        ShiftTable: () => import("../components/ShiftTable.vue"),
        AutoCreateShiftDialog: () => import("../components/AutoCreateShiftDialog.vue"),
        FilterShiftDialog: () => import("../components/FilterShiftDialog.vue"),
    },
    data() {
        return {
            id: null,
            reload: false,
            autoCreateShiftState: {
                visible: false,
            },
            filterShiftState: {
                visible: false,
            },
        }
    },
    mounted() {
        this.id = this.$route.params.id;
    },
    methods: {
        handleCreateShiftConfirm() {
            this.reload = true;
        },
        handleFilterShiftConfirm() {
            this.reload = true;
        },
        handleReloaded() {
            this.reload = false;
        }
    }
};
</script>
