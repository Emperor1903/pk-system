<template>
<div>
  <el-card shadow="hover">
    <el-table :data="tableData" style="width: 100%">
      <el-table-column fixed label="STT" prop="index" width="50">
      </el-table-column>
      <el-table-column fixed label="Bệnh viện" prop="name" width="400">
      </el-table-column>
      <el-table-column label="Số điện thoại" prop="phone_num" width="300">
      </el-table-column>
      <el-table-column label="Địa chỉ" prop="address" width="600">
      </el-table-column>
      <el-table-column align="right">
        <template slot="header" slot-scope="scope">
          <el-input
            v-model="search"
            size="mini"
            placeholder="Tìm kiếm bệnh viện"
            @change="handleSearch"
          />
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
            type="text"
            @click="handleEdit(scope.$index, scope.row)"
            >Chỉnh sửa</el-button
          >

          <el-button
            size="mini"
            type="danger"
            @click="handleDelete(scope.$index, scope.row)"
            >Xóa</el-button
          >
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
  </el-card>

  <DeleteDialog :state="deleteState" />
</div>
</template>

<script>
import { searchHospital } from "../api/control";
import { TABLE_LIMIT } from "../api/config";

export default {
  name: "HospitalTable",
  data() {
    return {
      tableData: [],
      search: "",
      total: 0,
      deleteState: {
        title: "Bệnh viện",
        data: {},
        visible: true
      },
    };
  },
  components: {
    DeleteDialog: () => import("./DeleteDialog.vue"),
  },
  async mounted() {
    await this.getData(1);
  },
  methods: {
    async getData(page) {
      let start = (page - 1) * TABLE_LIMIT;
      let data = await searchHospital(this.search, start);
      this.total = data.total;
      for (let i = 0; i < Math.min(TABLE_LIMIT, this.total - start); ++i) {
        data.data[i].index = i + 1 + start;
      }
      this.tableData = data.data;
    },
    handleDetail(index, row) {},
    handleEdit(index, row) {
      console.log(index, row);
    },
    handleDelete(index, row) {
      console.log(index, row);
    },
    handlePageChange(page) {
      this.getData(page);
    },
    handleSearch() {
      this.getData(1);
    },
  },
};
</script>
