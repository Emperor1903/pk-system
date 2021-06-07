<template>
  <el-form>
    <el-form-item label="Bác sĩ khám">
      <el-select v-model="form.doctor"> </el-select>
    </el-form-item>
   
  </el-form>
</template>

<script>
export default {
  props: ["form"],
};
</script>