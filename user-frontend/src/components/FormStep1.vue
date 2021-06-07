<template>
<el-form>
  <el-form-item label="Họ và tên">
    <el-input v-model="form.name"> </el-input>
  </el-form-item>
  <el-form-item label="Ngày sinh">
    <el-date-picker
      :picker-options="pickerOptions"
      v-model="form.date_of_birth"
      ></el-date-picker>
  </el-form-item>
  <el-form-item label="Giới tính">
    <el-select v-model="form.gender_is_male">
      <el-option
        v-for="(item, index) in options"
        :value="item.value"
        :label="item.label"
        :key="index">
      </el-option>
    </el-select>
  </el-form-item>
  <el-form-item label="Địa chỉ">
      <el-input v-model="form.address"> </el-input>
    </el-form-item>
  </el-form>
</template>

<script>
export default {
    props: ["form"],
    data() {
        return {
            pickerOptions: {
                disabledDate(time) {
                    return time.getTime() > Date.now();
                },
            },
            form: {
                name: "",
                gender_is_male: true,
                address: "",
            },
            options: [
                {
                    value: true,
                    label: "Nam",
                },
                {
                    value: false,
                    label: "Nữ",
                },
            ],
        };
    },
};
</script>
