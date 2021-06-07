<template>
  <div>
    <el-table :data="tableSpecialization" style="width: 100%">
      <el-table-column fixed label="STT" prop="index" width="50">
      </el-table-column>
      <el-table-column fixed label="Chuyên khoa" prop="name" width="300">
      </el-table-column>
      <el-table-column label="Mô tả" prop="desc"> </el-table-column>
      <el-table-column align="right">
        <template slot="header" slot-scope="scope">
          <el-button
            size="mini"
            icon="el-icon-plus"
            @click="handleNew(scope.$index, scope.row)"
          >
          </el-button>
          <el-input
            v-model="searchValue"
            size="mini"
            placeholder="Tìm kiếm chuyên khoa"
            @change="handleSearch"
          />
        </template>
        <template slot-scope="scope">
          <el-button
            size="mini"
            icon="el-icon-edit"
            @click="handleEdit(scope.$index, scope.row)"
          >
          </el-button>

          <el-button
            size="mini"
            type="danger"
            icon="el-icon-delete"
            @click="handleDelete(scope.$index, scope.row)"
          >
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
    <DeleteDialog :state="deleteState" @confirm="updateData" />
    <UpdateSpeciationDialog
      :state="updateState"
      @confirm="updateData"
    ></UpdateSpeciationDialog>
    <NewSpeciationDialog
      :state="newState"
      @confirm="updateData"
    ></NewSpeciationDialog>
  </div>
</template>

<script>
import { TABLE_LIMIT } from "../api/config";
import { searchSpecialization } from "../api/admin";
export default {
  name: "SpecializationTable",
  components: {
    UpdateSpeciationDialog: () => import("./UpdateSpeciationDialog"),
    NewSpeciationDialog: () => import("./NewSpeciationDialog"),
    DeleteDialog: () => import("./DeleteDialog.vue"),
  },
  data() {
    return {
      tableSpecialization: [],
      searchValue: "",
      total: 0,
      page: 1,
      newState: {
        title: "Chuyên khoa",
        doc: "specialization",
        data: {},
        visible: false,
        confirmed: false,
      },
      updateState: {
        title: "Chuyên khoa",
        doc: "specialization",
        data: {},
        visible: false,
        confirmed: false,
      },
      deleteState: {
        title: "Chuyên khoa",
        doc: "specialization",
        data: {},
        visible: false,
        confirmed: false,
      },
    };
  },
  async mounted() {
    await this.getData(this.page);
  },
  methods: {
    async getData(page) {
      let start = (page - 1) * TABLE_LIMIT;
      let data = await searchSpecialization(this.searchValue, start);
      console.log(data);
      this.total = data.total;
      data.data.forEach((element, index) => {
        element.index = index + 1 + start;
      });
      this.tableSpecialization = data.data;
    },
    async handlePageChange(page) {
      this.page = page;
      await this.getData(page);
    },
    async handleSearch() {
      await this.getData(1);
    },
    handleNew() {
      this.newState.visible = true;
    },
    handleEdit(index, row) {
      this.updateState = {
        title: "Chuyên khoa",
        doc: "specialization",
        data: row,
        visible: true,
        confirmed: false,
      };
    },
    handleDelete(index, row) {
      this.deleteState.visible = true;
      this.deleteState.data = row;
    },
    async updateData() {
      await this.getData(this.page);
    },
  },
};
</script>