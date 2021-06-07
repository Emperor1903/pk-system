<template>
  <el-form>
    <el-form-item label="Ca khám">
      <el-select v-model="form.shift"> </el-select>
    </el-form-item>
    
  </el-form>
</template>

<script>
export default {
  props: ["form"],
};
</script>