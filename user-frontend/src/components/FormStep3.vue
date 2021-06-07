<template>
  <el-form>
    <el-form-item label="Bác sĩ khám">
      <el-select v-model="form.doctor" @change="getShift">
        <el-option
          v-for="(item, index) in form.doctorData"
          :value="item.value"
          :label="item.label"
          :key="index"
        >
        </el-option>
      </el-select>
    </el-form-item>
  </el-form>
</template>

<script>
import { searchShift } from "../api/api";
export default {
  props: ["form"],
  data() {
    return {};
  },
  methods: {
    async getShift() {
      this.form.shiftData = [];
      let data = searchShift(this.form.doctor);
      this.form.shiftData = data.data;
    },
  },
};
</script>