<template>
  <el-dialog
    v-loading="loading"
    :title="state.title"
    :visible.sync="state.visible"
    width="50%"
    ><el-form :model="form">
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
import { updateProvince } from "../api/admin.js";
export default {
  name: "UpdateProvinceDialog",
  props: ["state"],
  data() {
    return {
      form: {
        _id: {},
        name: "",
      },
      formLabelWidth: "120px",
      loading: false,
    };
  },
  watch: {
    state: function (val) {
      this.form = val.data;
      console.log(this.form);
    },
  },
  methods: {
    async handleConfirm() {
      this.loading = true;
      await updateProvince(this.form._id, this.form.name);
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