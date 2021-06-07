<template>
  <el-form>
    <el-form-item label="Email">
      <el-input v-model="form.email"> </el-input>
    </el-form-item>
    <el-form-item label="Số điện thoại">
      <el-input v-model="form.phone_num"> </el-input>
    </el-form-item>
  </el-form>
</template>

<script>
export default {
  props: ["form"],
};
</script>