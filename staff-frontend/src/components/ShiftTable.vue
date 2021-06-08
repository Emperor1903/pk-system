<template>
<div>
  <el-pagination
    @current-change="handlePageChange"
    background
    layout="prev, pager, next"
    :total="total">
  </el-pagination>  
  <el-table :data="tableData" style="width: 100%" border>
    <el-table-column fixed label="STT" prop="index" width="50">
    </el-table-column>
    <el-table-column label="Bác sĩ" prop="doctor_name" width="300">
    </el-table-column>
    <el-table-column label="Số người đăng ký" prop="client_number" width="150">
    </el-table-column>
    <el-table-column label="Thời gian bắt đầu" prop="start_datetime" width="200">
    </el-table-column>
    <el-table-column label="Thời gian kết thúc" prop="end_datetime" width="200">
    </el-table-column>
    <el-table-column align="right" fixed="right" width="120">
      <template slot-scope="scope">
        <el-button
          size="mini"
          type="danger"
          icon="el-icon-delete"
          @click="handleDelete(scope.$index, scope.row)">
        </el-button>
      </template>
    </el-table-column>
  </el-table>
  
  <DeleteDialog :state="deleteState" @confirm="updateData"/>
</div>

</template>

<script>
const TABLE_LIMIT = 1000;
import { searchShift, getDocument } from "../api/index";

export default {
    name: "ShiftTable",
    props: ["id", "reload", "history"],
    components: {
        DeleteDialog: () => import("./DeleteDialog.vue"),
        // NewShiftDialog: () => import("./NewShiftDialog.vue"),
        // UpdateShiftDialog: () => import("./UpdateShiftDialog.vue"),
    },
    data() {
        return {
            tableData: [
            ],
            search: "",
            total: 0,
            deleteState: {
                title: "Ca trực",
                doc: "shift",
                data: {},
                visible: false,
                confirmed: false,
            },
            newState: {},
            updateState: {},
            page: 1,
        };
    },
    async mounted() {
        await this.getData(this.page);
    },
    watch: {
        reload: async function(val) {
            if (val) {
                await this.getData(this.page);
                this.$emit("reloaded");
            }
        }
    },
    methods: {
        async getData(page) {
            var start = (page - 1) * TABLE_LIMIT;
            let start_time = (history) ? null : Math.floor(Date.now() / 1000)
            var data = await searchShift(this.search, this.id, start,
                                         TABLE_LIMIT, start_time);
            this.tableData = [];
            if (data) {
                this.total = data.total;
                data = data.data;
                for (let i = 0; i < Math.min(TABLE_LIMIT, this.total - start); ++i) {
                    data[i].index = i + 1 + start;
                    var doctor = await getDocument("doctor", data[i].doctor);
                    data[i].doctor_name = doctor.name;
                    data[i].start_datetime = this.timestampToDate(data[i].start_time);
                    data[i].end_datetime = this.timestampToDate(data[i].end_time);
                }

                this.tableData = data;
                
            }
        },
        timestampToDate(timestamp) {
            var date = new Date(timestamp * 1000);
            return date.toLocaleDateString("vi-VN") + "-" + date.toLocaleTimeString("vi-VN");
        },
        handleDetail(index, row) {
            var id = row._id["$oid"];
            this.$router.push(`/doctor/${id}`)
        },
        handleDelete(index, row) {
            this.deleteState.visible = true;
            this.deleteState.data = row;
        },
        async handlePageChange(page) {
            this.page = page;
            await this.getData(page);
        },
        async handleSearch() {
            console.log();
            await this.getData(1);
        },
        async updateData() {
            await this.getData(this.page);
        }
    },
};
</script>
