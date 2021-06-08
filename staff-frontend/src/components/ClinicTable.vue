<template>
<div>
  <el-table :data="tableData" style="width: 100%">
    <el-table-column fixed label="STT" prop="index" width="50">
    </el-table-column>
    <el-table-column label="Phòng khám" prop="name" width="300">
    </el-table-column>
    <el-table-column label="Số điện thoại" prop="phone_num" width="250">
    </el-table-column>
    <el-table-column label="Địa chỉ" prop="address" width="500">
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
  <NewClinicDialog :state="newState" @confirm="updateData"/>
  <UpdateClinicDialog :state="updateState" @confirm="updateData"/>    
</div>

</template>

<script>
import { TABLE_LIMIT } from "../api/config";
import { searchClinic, getDocument } from "../api/index";

export default {
    name: "ClinicTable",
    props: ["id"],
    components: {
        DeleteDialog: () => import("./DeleteDialog.vue"),
        NewClinicDialog: () => import("./NewClinicDialog.vue"),
        UpdateClinicDialog: () => import("./UpdateClinicDialog.vue"),
    },
    data() {
        return {
            tableData: [],
            search: "",
            total: 0,
            deleteState: {
                title: "Phòng khám",
                doc: "clinic",
                data: {},
                visible: false,
                confirmed: false,
            },
            updateState: {
                title: "Chỉnh sửa phòng khám",
                doc: "clinic",
                visible: false,
            },
            newState: {},
            page: 1,
        };
    },
    async mounted() {
        await this.getData(this.page);
    },
    methods: {
        async getData(page) {
            var start = (page - 1) * TABLE_LIMIT;
            var data = await searchClinic(this.search, this.id, start);
            this.tableData = [];
            if (data) {
                this.total = data.total;
                for (let i = 0; i < Math.min(TABLE_LIMIT, this.total - start); ++i) {
                    data.data[i].index = i + 1 + start;
                    if (data.data[i].images) {
                        data.data[i].images = data.data[i].images.map(i => {
                            return {
                                name: i.split("/")[i.split("/").length - 1],
                                url: i
                            }});
                    }
                }
                this.tableData = data.data;
            }
            if(this.id) {
                var response = await getDocument("hospital", this.id);
                this.newState.hospitalName = response.name;
                this.newState.hospital = {"$oid": this.id};
                this.updateState.hospitalName = response.name;
                this.updateState.hospital = {"$oid": this.id};
            }
        },
        handleDetail(index, row) {
            var id = row._id["$oid"];
            this.$router.push(`/doctor/${id}`)
        },
        handleEdit(index, row) {
            this.updateState.id = row._id;
            this.updateState.visible = true;
            this.updateState = {...this.updateState};
        },
        handleDelete(index, row) {
            this.deleteState.visible = true;
            this.deleteState.data = row;
        },
        handleNew() {
            this.newState = {
                title: "Tạo phòng khám",
                doc: "clinic",
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
