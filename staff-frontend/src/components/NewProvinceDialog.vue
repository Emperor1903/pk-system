<template>
  <el-dialog
    v-loading="loading"
    :title="state.title"
    :visible.sync="state.visible"
    width="50%"
  >
    <el-form :model="form">
      <el-form-item label="Tên tỉnh/thành phố" :label-width="formLabelWidth">
        <el-input v-model="form.name" autocomplete="off"></el-input>
      </el-form-item>
    </el-form>
    <span slot="footer" class="dialog-footer">
      <el-button @click="handleCancel">Thoát</el-button>
      <el-button type="primary" @click="handleConfirm">Xác nhận</el-button>
    </span>
  </el-dialog>
</template>

<script>
import { newProvince } from "../api/index.js";
export default {
  name: "NewProvinceDialog",
  props: ["state"],
  data() {
    return {
      form: {
        name: "",
      },
      loading: false,
      formLabelWidth: "120px",
    };
  },
  methods: {
    async handleConfirm() {
      this.loading = true;
      await newProvince(this.form.name);
      this.loading = false;
      this.state.visible = false;
      this.$emit("confirm");
    },
    async handleCancel() {
      this.state.visible = false;
    },
  },
};
</script>