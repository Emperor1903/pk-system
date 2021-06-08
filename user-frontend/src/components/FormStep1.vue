<template>
  <el-form :model="form" label-width="120px">
    <el-form-item label="Họ và tên" class="item">
      <el-input v-model="form.name"> </el-input>
    </el-form-item>
    <el-form-item label="Ngày sinh" class="item">
      <el-date-picker
        :picker-options="pickerOptions"
        type="year"
        format="dd/MM/yyyy"
        placeholder="vd: 13/09/1999"
        value-format="timestamp"
        v-model="form.date_of_birth">
      </el-date-picker>
    </el-form-item>
    <el-form-item label="Giới tính" class="item">
      <el-select v-model="form.gender_is_male">
        <el-option label="Nam" :value="true"></el-option>
        <el-option label="Nữ" :value="false"></el-option>
      </el-select>
    </el-form-item>
    <el-form-item label="Địa chỉ" class="item">
      <el-input v-model="form.address"> </el-input>
    </el-form-item>    
  </el-form>
</el-form>
</template>

<style>
  .item {
  margin-left: 0;
  }
</style>  

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
        };
    },
};
</script>
