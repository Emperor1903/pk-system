<template>
  <el-form>
    <el-form-item label="Khu vực khám">
      <el-select v-model="form.province"> </el-select>
    </el-form-item>
    <el-form-item label="Khu vực khám">
      <el-select v-model="form.hospital"> </el-select>
    </el-form-item>
    <el-form-item label="Khu vực khám">
      <el-select v-model="form.clinic"> </el-select>
    </el-form-item>
  </el-form>
</template>

<script>
export default {
  props: ["form"],
};
</script>