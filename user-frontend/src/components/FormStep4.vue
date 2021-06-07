<template>
  <el-form>
    <el-form-item label="Ca khám">
      <el-select v-model="form.shift">
        <el-option
          v-for="(item, index) in form.shiftData"
          :value="item.value"
          :label="item.label"
          :key="index"
        >
      </el-select>
    </el-form-item>
    
  </el-form>
</template>

<script>
export default {
  props: ["form"],
};
</script>