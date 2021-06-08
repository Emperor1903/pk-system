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
      <el-button round type="success" @click="exportToCsv" >
        Xuất file csv
      </el-button>     

      <el-main>
        <ShiftTable :id="$route.params.id"
                    :reload="reload"
                    @reloaded="handleReloaded"
                    ref="shiftTable"/>
      </el-main>
    </el-card>
  </el-col>

  <AutoCreateShiftDialog :state="autoCreateShiftState" @confirm="handleCreateShiftConfirm"/>
  
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
    },
    data() {
        return {
            reload: false,
            autoCreateShiftState: {
                visible: false,
            },
            history: false,
        }
    },
    methods: {
        handleHistory() {
            this.history = !this.history;
        },
        handleCreateShiftConfirm() {
            this.reload = true;
        },
        handleReloaded() {
            this.reload = false;
        },
        exportToCsv() {
            var data = this.$refs.shiftTable.tableData;
            var rows = data.map(i => 
                [i.doctor_name, i.client_number, i.start_datetime, i.end_datetime]
            );

            console.log(rows);
            var csvContent = "data:text/csv;charset=utf-8," 
                + rows.map(e => e.join(",")).join("\n");
            
            var encodedUri = encodeURI(csvContent);
            var link = document.createElement("a");
            link.setAttribute("href", encodedUri);
            link.setAttribute("download", "ca_truc.csv");
            document.body.appendChild(link); // Required for FF

            link.click(); 
        }
    }
};
</script>
