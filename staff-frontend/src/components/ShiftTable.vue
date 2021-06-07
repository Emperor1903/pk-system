<template>
<div>
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
          icon="el-icon-edit"
          @click="handleEdit(scope.$index, scope.row)">
        </el-button>
        <el-button
          size="mini"
          type="danger"
          icon="el-icon-delete"
          @click="handleDelete(scope.$index, scope.row)">
        </el-button>
      </template>
    </el-table-column>
  </el-table>
  <el-pagination
    @current-change="handlePageChange"
    background
    layout="prev, pager, next"
    :total="total"
    >
  </el-pagination>
  
  <DeleteDialog :state="deleteState" @confirm="updateData"/>
  <!-- <NewShiftDialog :state="newState" @confirm="updateData"/> -->
  <!-- <UpdateShiftDialog :state="updateState" @confirm="updateData"/> -->
</div>

</template>

<script>
import { TABLE_LIMIT } from "../api/config";
import { searchShift, getDocument } from "../api/admin";

export default {
    name: "ShiftTable",
    props: ["id", "reload"],
    components: {
        DeleteDialog: () => import("./DeleteDialog.vue"),
        NewShiftDialog: () => import("./NewShiftDialog.vue"),
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
            var data = await searchShift(this.search, this.id, start);
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
            return date.toLocaleTimeString("vi-VN") + " - " + date.toLocaleDateString("vi-VN");
        },
        handleDetail(index, row) {
            var id = row._id["$oid"];
            this.$router.push(`/doctor/${id}`)
        },
        handleEdit(index, row) {
            this.updateState = {
                title: "Chỉnh sửa phòng khám",
                doc: "shift",
                data: row,
                visible: true,
                confirmed: false,
                provinces: this.updateState.provinces,
                hospitalName: this.updateState.hospitalName,
            };
        },
        handleDelete(index, row) {
            this.deleteState.visible = true;
            this.deleteState.data = row;
        },
        handleNew() {
            this.newState = {
                title: "Tạo phòng khám",
                doc: "shift",
                visible: true,
                confirmed: false,
                hospital: this.newState.hospital,
                hospitalName: this.newState.hospitalName,
            }
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
