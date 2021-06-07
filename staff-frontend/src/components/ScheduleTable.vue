<template>
<div>
  <el-table :data="tableData" style="width: 100%" border>
    <el-table-column fixed label="STT" prop="index" width="50">
    </el-table-column>
    <el-table-column label="Bác sĩ" prop="doctorName" width="300">
    </el-table-column>
    <el-table-column label="Ca số" prop="shiftDesc" width="150">
    </el-table-column>
    <el-table-column label="Thứ" prop="shiftDayDesc" width="150">
    </el-table-column>
    <el-table-column align="right" fixed="right" width="250">
      <template slot="header" slot-scope="scope">
        <el-button
          size="mini"
          icon="el-icon-plus"          
          @click="handleNew(scope.$index, scope.row)">
        </el-button>
        <el-input
          v-model="search"
          size="mini"
          placeholder="Tìm kiếm phòng khám"
          @change="handleSearch"/>
      </template>
      <template slot-scope="scope">
        <el-button
          size="mini"
          type="text"
          @click="handleDetail(scope.$index, scope.row)">
          Chi tiết
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
  <NewScheduleDialog :state="newState" @confirm="updateData"/>
</div>

</template>

<script>
import { TABLE_LIMIT } from "../api/config";
import { searchSchedule, getDocument } from "../api/admin";

export default {
    name: "ScheduleTable",
    props: ["id"],
    components: {
        DeleteDialog: () => import("./DeleteDialog.vue"),
        NewScheduleDialog: () => import("./NewScheduleDialog.vue"),
    },
    data() {
        return {
            tableData: [],
            search: "",
            total: 0,
            deleteState: {
                title: "",
                doc: "schedule",
                data: {},
                visible: false,
                confirmed: false,
            },
            newState: {
                title: "Tạo bác sĩ",
                doc: "schedule",
                visible: false,
                doctor: {"$oid": this.id},
            },
            shiftDesc: ["ca 1: 8h - 12h", "ca 2: 13h - 17h"],
            shiftDayDesc: ["Thứ 2", "Thứ 3", "Thứ 4", "Thứ 5", "Thứ 6", "Thứ 7", "Chủ nhật"],
            page: 1,
        };
    },
    async mounted() {
        await this.getData(this.page);
    },
    methods: {
        async getData(page) {
            var start = (page - 1) * TABLE_LIMIT;
            var data = await searchSchedule(this.id, start);
            this.tableData = [];
            if (data) {
                this.total = data.total;
                for (var i = 0;
                     i < Math.min(TABLE_LIMIT, this.total - start);
                     ++i) {
                    data.data[i].index = i + 1 + start;
                    var r = await getDocument("doctor", data.data[i].doctor);
                    if (r) data.data[i].doctorName = r.name;
                    data.data[i].shiftDesc = this.shiftDesc[data.data[i].shift_num];
                    data.data[i].shiftDayDesc = this.shiftDayDesc[data.data[i].shift_day];
                    console.log(data.data[i]);
                }
                this.tableData = data.data;
            }
        },
        handleDetail(index, row) {
            // var id = row._id["$oid"];
            // this.$router.push(`/schedule/${id}`)
        },
        handleDelete(index, row) {
            this.deleteState.visible = true;
            this.deleteState.data = row;
        },
        handleNew() {
            this.newState.visible = true;
        },
        async handlePageChange(page) {
            this.page = page;
            await this.getData(page);
        },
        async handleSearch() {
            await this.getData(1);
        },
        async updateData() {
            await this.getData(this.page);
        }
    },
};
</script>
