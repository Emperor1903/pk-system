<template>
  <div>
    <el-table :data="tableData" style="width: 100%">
      <el-table-column fixed label="STT" prop="index" width="50">
      </el-table-column>
      <el-table-column label="Tỉnh/Thành phố" prop="name"> </el-table-column>
      <el-table-column fixed="right" align="right" width="250">
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
            placeholder="Tìm kiếm bệnh viện"
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
    <NewProvinceDialog :state="newState" @confirm="updateData" />
    <UpdateProvinceDialog :state="updateState" @confirm="updateData" />
  </div>
</template>

<script>
import { TABLE_LIMIT } from "../api/config";
import { searchProvince } from "../api/admin";

export default {
  name: "ProvinceTable",
  components: {
    NewProvinceDialog: () => import("./NewProvinceDialog"),
    UpdateProvinceDialog: () => import("./UpdateProvinceDialog"),
  },
  data() {
    return {
      tableData: [],
      searchValue: "",
      page: 1,
      newState: {
        title: "Tỉnh/thành phố",
        doc: "province",
        visible: false,
        confirmed: false,
      },
      updateState: {
        title: "Tỉnh/thành phố",
        doc: "province",
        data: {},
        visible: false,
        confirmed: false,
      },
      total: 0,
    };
  },
  async mounted() {
    await this.getData(this.page);
  },
  methods: {
    async getData(page) {
      var start = (page - 1) * TABLE_LIMIT;
      var data = await searchProvince(this.searchValue, start);

      console.log(data);
      this.total = data.total;
      data.data.forEach((element, index) => {
        element.index = index + 1 + start;
      });
      this.tableData = data.data;
    },
    handleNew() {
      this.newState.visible = true;
    },
    handleEdit(index, row) {
      this.updateState = {
        title: "Tỉnh/thành phố",
        doc: "province",
        data: row,
        visible: true,
        confirmed: false,
      };
    },
    async updateData() {
      await this.getData(this.page);
    },
    async handlePageChange(page) {
      this.page = page;
      await this.getData(page);
    },
    async handleSearch() {
      await this.getData(1);
    },
  },
};
</script>