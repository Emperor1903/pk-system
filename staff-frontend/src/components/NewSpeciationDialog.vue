<template>
  <el-dialog
    v-loading="loading"
    :title="state.title"
    :visible.sync="state.visible"
    width="50%">
    <el-form :model="form">
      <el-form-item label="Tên chuyên khoa">
        <el-input autocomplete="off" v-model="form.name"></el-input>
      </el-form-item>
      <el-form-item label="Mô tả">
        <el-input
          type="textarea"
          :autosize="{ minRows: 4, maxRows: 4 }"
          placeholder="Nhập mô tả bệnh viện"
          v-model="form.desc"
        >
        </el-input>
      </el-form-item>
    </el-form>
    <span slot="footer" class="dialog-footer">
      <el-button @click="handleCancel">Thoát</el-button>
      <el-button type="primary" @click="handleConfirm">Xác nhận</el-button>
    </span></el-dialog>
</template>

<script>
import { newSpecializtion } from "../api/index.js";
export default {
  name: "NewSpeciationDialog",
  props: ["state"],
  data() {
    return {
      loading: false,
      form: {
        name: "",
        desc: "",
      },
    };
  },
  methods: {
    async handleConfirm() {
      this.loading = true;
      await newSpecializtion(this.form.name, this.form.desc);
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
