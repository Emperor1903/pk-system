<template>
<div>
  <el-table :data="tableData" style="width: 100%" border>
    <el-table-column fixed label="STT" prop="index" width="50">
    </el-table-column>
    <el-table-column label="Bác sĩ" prop="name" width="300">
    </el-table-column>
    <el-table-column label="Số điện thoại" prop="phone_num" width="150">
    </el-table-column>
    <el-table-column label="Chức vụ" prop="position" width="200">
    </el-table-column>
    <el-table-column label="Phòng khám" prop="clinicName" width="350">
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
          @click="handleDetail(scope.$index, scope.row)"
          >Chi tiết</el-button
                     >
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
  <NewDoctorDialog :state="newState" @confirm="updateData"/>
  <UpdateDoctorDialog :state="updateState" @confirm="updateData"/>    
</div>

</template>

<script>
import { TABLE_LIMIT } from "../api/config";
import { searchDoctor, getDocument } from "../api/index";

export default {
    name: "DoctorTable",
    props: ["id"],
    components: {
        DeleteDialog: () => import("./DeleteDialog.vue"),
        NewDoctorDialog: () => import("./NewDoctorDialog.vue"),
        UpdateDoctorDialog: () => import("./UpdateDoctorDialog.vue"),
    },
    data() {
        return {
            tableData: [],
            search: "",
            total: 0,
            deleteState: {
                title: "",
                doc: "doctor",
                data: {},
                visible: false,
                confirmed: false,
            },
            newState: {
                title: "Tạo bác sĩ",
                doc: "doctor",
                visible: false,
                clinic: {"$oid": this.id},
            },
            updateState: {
                title: "",
                doc: "doctor",
                visible: false,
                id: null,
            },
            page: 1,
        };
    },
    async mounted() {
        await this.getData(this.page);
    },
    methods: {
        async getData(page) {
            var start = (page - 1) * TABLE_LIMIT;
            var data = await searchDoctor(this.search, this.id, start);
            this.tableData = [];
            if (data) {
                this.total = data.total;
                for (var i = 0;
                     i < Math.min(TABLE_LIMIT, this.total - start);
                     ++i) {
                    data.data[i].index = i + 1 + start;
                    var r = await getDocument("clinic", data.data[i].clinic);
                    if (r)
                        data.data[i].clinicName = r.name;
                }
                this.tableData = data.data;
            }
        },
        handleDetail(index, row) {
            var id = row._id["$oid"];
            this.$router.push(`/schedule/${id}`)
        },
        handleEdit(index, row) {
            this.updateState.id = row._id;
            this.updateState.visible = true;
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
            console.log();
            await this.getData(1);
        },
        async updateData() {
            await this.getData(this.page);
        }
    },
};
</script>
