<template>
  <el-dialog :title="state.title" :visible.sync="state.visible" width="50%"
    ><el-form :model="form">
      <el-form-item label="Tên chuyên khoa" :label-width="formLabelWidth">
        <el-input v-model="form.name" autocomplete="off"></el-input>
      </el-form-item>
      <el-form-item label="Mô tả" :label-width="formLabelWidth">
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
    </span>
  </el-dialog>
</template>

<script>
import { updateSpecialization } from "../api/index.js";

export default {
  name: "UpdateSpeciationDialog",
  props: ["state"],
  data() {
    return {
      form: {
        _id: {},
        name: "",
        desc: "",
      },
      formLabelWidth: "120px",
    };
  },
  watch: {
    state: function (val) {
      this.form = val.data;
      console.log(val);
    },
  },
  methods: {
    async handleConfirm() {
      await updateSpecialization(this.form._id, this.form.name, this.form.desc);
      this.state.visible = false;
      this.$emit("confirm");
    },
    async handleCancel() {
      this.state.visible = false;
    },
  },
};
</script>