<template>
<div>
  <el-table :data="tableData" style="width: 100%">
    <el-table-column fixed label="STT" prop="index" width="50">
    </el-table-column>
    <el-table-column label="Tài khoản" prop="username" width="300">
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
          type="text"
          @click="handleDetail(scope.$index, scope.row)">
          Chi tiết
        </el-button>
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
  <NewHospitalDialog :state="newState" @confirm="updateData"/>
  <UpdateHospitalDialog :state="updateState" @confirm="updateData"/>    
</div>

</template>

<script>
import { TABLE_LIMIT } from "../api/config";
import { searchHospital, searchProvince } from "../api/admin";

export default {
    name: "HospitalTable",
    components: {
        DeleteDialog: () => import("./DeleteDialog.vue"),
        NewHospitalDialog: () => import("./NewHospitalDialog.vue"),
        UpdateHospitalDialog: () => import("./UpdateHospitalDialog.vue"),
    },
    data() {
        return {
            tableData: [],
            search: "",
            total: 0,
            deleteState: {
                title: "Bệnh viện",
                doc: "hospital",
                data: {},
                visible: false,
            },
            newState: {
                title: "Bệnh viện",
                doc: "hospital",
                visible: false,
            },
            updateState: {
                title: "Bệnh viện",
                doc: "hospital",
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
            var data = await searchHospital(this.search, start);
            this.tableData = [];            
            if (data) {
                this.total = data.total;
                for (let i = 0; i < Math.min(TABLE_LIMIT, this.total - start); ++i) {
                    data.data[i].index = i + 1 + start;
                }
                this.tableData = data.data;
            }
        },
        handleDetail(index, row) {
            // var id = row._id["$oid"];
            // this.$router.push(`clinic/${id}`)
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
