<template>
<div>
  <el-table :data="tableData" style="width: 100%">
    <el-table-column fixed label="STT" prop="index" width="50">
    </el-table-column>
    <el-table-column label="Tài khoản" prop="_id" width="150">
    </el-table-column>
    <el-table-column label="Vai trò" prop="role_desc" width="300">
    </el-table-column>
    <el-table-column fixed="right" align="right"  width="250">
      <template slot="header" slot-scope="scope">
        <el-button
          size="mini"
          icon="el-icon-plus"          
          @click="handleNew(scope.$index, scope.row)">
        </el-button>
        <el-input
          v-model="search"
          size="mini"
          placeholder="Tìm kiếm tài khoản"
          @change="handleSearch"/>
      </template>
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
    :total="total">
  </el-pagination>
  
  <DeleteDialog :state="deleteState" @confirm="updateData"/>
  <NewAdminDialog :state="newState" @confirm="updateData"/>
  <UpdateAdminDialog :state="updateState" @confirm="updateData"/>    
</div>

</template>

<script>
import { TABLE_LIMIT } from "../api/config";
import { searchAdmin } from "../api/admin";

export default {
    name: "AdminTable",
    components: {
        DeleteDialog: () => import("./DeleteDialog.vue"),
        NewAdminDialog: () => import("./NewAdminDialog.vue"),
        UpdateAdminDialog: () => import("./UpdateAdminDialog.vue"),
    },
    data() {
        return {
            tableData: [],
            search: "",
            total: 0,
            deleteState: {
                title: "Bệnh viện",
                doc: "admin",
                data: {},
                visible: false,
            },
            newState: {
                title: "Quản trị viên",
                doc: "admin",
                visible: false,
            },
            updateState: {
                title: "Quản trị viên",
                doc: "admin",
                data: {},
                visible: false,
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
            var data = await searchAdmin(this.search, start);
            this.tableData = [];            
            if (data) {
                this.total = data.total;
                data = data.data;
                for (let i = 0; i < Math.min(TABLE_LIMIT, this.total - start); ++i) {
                    data[i].index = i + 1 + start;
                    if(data[i].role == 0) data[i].role_desc = "Quản trị viên"
                    else if(data[i].role == 1) data[i].role_desc = "Nhân viên phòng khám"
                }
                this.tableData = data;
            }
        },
        handleEdit(index, row) {
            this.updateState.id = row._id;
            this.updateState.visible = true;
            this.updateState = {...this.updateState};
        },
        handleDelete(index, row) {
            this.deleteState.data = row;
            this.deleteState.visible = true;
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
