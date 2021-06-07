<template>
  <el-dialog
    v-loading="loading"
    :title="state.title"
    :visible.sync="state.visible"
  >
    <el-form>
      <el-form-item label="Họ và tên">
        <el-input v-model="form.name"> </el-input>
      </el-form-item>
      <el-form-item label="Ngày sinh">
        <el-date-picker
          v-model="form.date_of_birth"
          :picker-options="pickerOptions"
        ></el-date-picker>
      </el-form-item>
      <el-form-item label="Giới tính">
        <el-select v-model="form.gender_is_male">
          <el-option
            v-for="item in options"
            :key="item.value"
            :label="item.label"
            :value="item.value"
          >
          </el-option>
        </el-select>
      </el-form-item>
      <el-form-item label="Địa chỉ">
        <el-input v-model="form.address"> </el-input>
      </el-form-item>
      <el-form-item label="Số điện thoại">
        <el-input v-model="form.phone_num"> </el-input>
      </el-form-item>
      <el-form-item label="Email">
        <el-input v-model="form.email"> </el-input>
      </el-form-item>

      <el-form-item></el-form-item>
    </el-form>
    <span slot="footer" class="dialog-footer">
      <el-button @click="handleCancel">Thoát</el-button>
      <el-button type="primary" @click="handleConfirm">Xác nhận</el-button>
    </span>
  </el-dialog>
</template>

<script>
export default {
  props: ["state"],
  data() {
    return {
      form: {
        name: "",
        date_of_birth: null,
        phone_num: "",
        address: "",
        gender_is_male: true,
        email: "",
      },
      loading: false,
      pickerOptions: {
        disabledDate(time) {
          return time.getTime() > Date.now();
        },
      },
      options: [
        {
          label: "Nam",
          value: true,
        },
        {
          label: "Nữ",
          value: false,
        },
      ],
    };
  },
  methods: {
    async handleCancel() {
      this.state.visible = false;
    },
    async handleConfirm() {
      this.state.visible = false;
    },
  },
};
</script>